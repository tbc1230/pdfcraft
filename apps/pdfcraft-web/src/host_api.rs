//! `window.pdfcraft.save()`, `runCommand(id)`, `getState()` and the `pdfcraft:state` event: what the
//! hosting page uses to take the document's bytes, run editor commands and follow the editor state.
//! Installed always (not only with `?control`, not only when hosted). Each call queues a
//! `HostCall` for the app and wakes it; the app answers on a channel that is polled about once a
//! frame (`api::poll_reply`). No `#[wasm_bindgen]` export: the workspace forbids `unsafe_code`.
//!
//! - `save()`: a Promise of `{ ok: true, name, bytes: Uint8Array, doc, edit_generation }` or
//!   `{ ok: false, error }`; it never rejects. `edit_generation` is the value `getState()` reports
//!   while nothing has changed since these bytes. The editor downloads and posts nothing.
//! - `runCommand(id)`: a Promise of `{ ok: true }` or `{ ok: false, error }`.
//! - `placeSignature(bytes)`: a Promise of `{ ok: true }` (the placement started) or
//!   `{ ok: false, error }`; `bytes` is a `Uint8Array` or `ArrayBuffer`, at most 32 MiB.
//! - `insertPages(bytes, { name?, password? })`: a Promise of `{ ok: true, pages }` or
//!   `{ ok: false, error, password_required }`; `bytes` at most 512 MiB. The password is not logged.
//! - `getState()`: the last state snapshot, a plain object (the no-document state before the first frame).
//! - `pdfcraft:state`: a `CustomEvent` on the canvas, `detail` = the snapshot, each time it changes.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::channel;

use eframe::wasm_bindgen::{JsCast as _, JsValue, closure::Closure};
use pdfcraft_ui_egui::{HostCall, HostCalls, HostPassword, HostStateFn, InsertFromBytesError, SavedBytes, no_document_state};
use serde_json::{Value, json};

use crate::api::poll_reply;

/// The app id in `window.<app>` and the event names.
const APP: &str = "pdfcraft";

/// Put the functions on `window.pdfcraft` and return what the app calls with each new state.
pub fn install(calls: HostCalls, ctx: egui::Context) -> HostStateFn {
    let last: Rc<RefCell<Value>> = Rc::new(RefCell::new(no_document_state()));
    if let Some(api) = api_object() {
        let save_calls = calls.clone();
        let save_ctx = ctx.clone();
        let save = Closure::<dyn Fn() -> js_sys::Promise>::new(move || {
            let (calls, ctx) = (save_calls.clone(), save_ctx.clone());
            wasm_bindgen_futures::future_to_promise(async move { Ok(save_reply(&calls, &ctx).await) })
        });
        set(&api, "save", save);
        let command_ctx = ctx.clone();
        let command_calls = calls.clone();
        let run_command = Closure::<dyn Fn(JsValue) -> js_sys::Promise>::new(move |id: JsValue| {
            let (calls, ctx) = (command_calls.clone(), command_ctx.clone());
            wasm_bindgen_futures::future_to_promise(async move {
                let reply = match id.as_string() {
                    Some(id) => run_command_reply(&calls, &ctx, id).await,
                    None => failure("the command id must be a string"),
                };
                Ok(to_js(&reply))
            })
        });
        set(&api, "runCommand", run_command);
        let place_calls = calls.clone();
        let place_ctx = ctx.clone();
        let place_signature = Closure::<dyn Fn(JsValue) -> js_sys::Promise>::new(move |image: JsValue| {
            let (calls, ctx) = (place_calls.clone(), place_ctx.clone());
            wasm_bindgen_futures::future_to_promise(async move { Ok(to_js(&place_signature_reply(&calls, &ctx, &image).await)) })
        });
        set(&api, "placeSignature", place_signature);
        let insert_calls = calls.clone();
        let insert_ctx = ctx.clone();
        let insert_pages = Closure::<dyn Fn(JsValue, JsValue) -> js_sys::Promise>::new(move |pdf: JsValue, options: JsValue| {
            let (calls, ctx) = (insert_calls.clone(), insert_ctx.clone());
            wasm_bindgen_futures::future_to_promise(async move { Ok(to_js(&insert_pages_reply(&calls, &ctx, &pdf, &options).await)) })
        });
        set(&api, "insertPages", insert_pages);
        let read = last.clone();
        let get_state = Closure::<dyn Fn() -> JsValue>::new(move || to_js(&read.borrow()));
        set(&api, "getState", get_state);
    }
    Box::new(move |state: &Value| {
        *last.borrow_mut() = state.clone();
        dispatch_state(state);
    })
}

/// `window.pdfcraft`, made when the page has none (the other installers keep what exists).
fn api_object() -> Option<JsValue> {
    let window = web_sys::window()?;
    let key = JsValue::from_str(APP);
    match js_sys::Reflect::get(&window, &key) {
        Ok(existing) if existing.is_object() => Some(existing),
        _ => {
            let made: JsValue = js_sys::Object::new().into();
            js_sys::Reflect::set(&window, &key, &made).ok()?;
            Some(made)
        }
    }
}

/// Put `function` on `api` under `name`; it lives as long as the page.
fn set<F: ?Sized + eframe::wasm_bindgen::closure::WasmClosure>(api: &JsValue, name: &str, function: Closure<F>) {
    if js_sys::Reflect::set(api, &JsValue::from_str(name), function.as_ref().unchecked_ref::<js_sys::Function>()).is_ok() {
        function.forget();
    }
}

fn failure(error: &str) -> Value {
    json!({ "ok": false, "error": error })
}

fn to_js(value: &Value) -> JsValue {
    js_sys::JSON::parse(&value.to_string()).unwrap_or(JsValue::NULL)
}

/// Queue a call and wake the app; false when the queue is unavailable.
fn queue(calls: &HostCalls, ctx: &egui::Context, call: HostCall) -> bool {
    let queued = calls.lock().map(|mut q| q.push_back(call)).is_ok();
    ctx.request_repaint();
    queued
}

async fn save_reply(calls: &HostCalls, ctx: &egui::Context) -> JsValue {
    let (tx, rx) = channel();
    if !queue(calls, ctx, HostCall::Save(tx)) {
        return to_js(&failure("the editor is not available"));
    }
    match poll_reply(rx).await {
        Some(Ok(saved)) => saved_object(saved),
        Some(Err(error)) => to_js(&failure(&error)),
        None => to_js(&failure("the app dropped the request")),
    }
}

async fn run_command_reply(calls: &HostCalls, ctx: &egui::Context, id: String) -> Value {
    let (tx, rx) = channel();
    if !queue(calls, ctx, HostCall::RunCommand(id, tx)) {
        return failure("the editor is not available");
    }
    match poll_reply(rx).await {
        Some(Ok(())) => json!({ "ok": true }),
        Some(Err(error)) => failure(&error),
        None => failure("the app dropped the request"),
    }
}

/// The largest image `placeSignature` takes, and the largest PDF `insertPages` takes (the host
/// open's limit, `MAX_OPEN_BYTES` in `main.rs`).
const MAX_IMAGE_BYTES: u64 = 32 << 20;
const MAX_PDF_BYTES: u64 = 512 << 20;

/// Copy a `Uint8Array` or `ArrayBuffer` into a `Vec`; refused before copying when over `limit`.
/// `Err` is the text for the result's `error`.
fn bytes_of(value: &JsValue, limit: u64, what: &str, kind: &str) -> Result<Vec<u8>, String> {
    let array = if let Some(array) = value.dyn_ref::<js_sys::Uint8Array>() {
        array.clone()
    } else if let Some(buffer) = value.dyn_ref::<js_sys::ArrayBuffer>() {
        js_sys::Uint8Array::new(buffer)
    } else {
        return Err(format!("the {what} must be bytes (Uint8Array)"));
    };
    if u64::from(array.length()) > limit {
        return Err(format!("the {kind} is over the limit of {} MiB", limit >> 20));
    }
    Ok(array.to_vec())
}

/// `window.pdfcraft.placeSignature(bytes)`: `{ ok: true }` or `{ ok: false, error }`.
async fn place_signature_reply(calls: &HostCalls, ctx: &egui::Context, image: &JsValue) -> Value {
    let bytes = match bytes_of(image, MAX_IMAGE_BYTES, "image", "image") {
        Ok(bytes) => bytes,
        Err(error) => return failure(&error),
    };
    let (tx, rx) = channel();
    if !queue(calls, ctx, HostCall::PlaceSignature(bytes, tx)) {
        return failure("the editor is not available");
    }
    match poll_reply(rx).await {
        Some(Ok(())) => json!({ "ok": true }),
        Some(Err(error)) => failure(&error),
        None => failure("the app dropped the request"),
    }
}

fn insert_failure(error: &str, password_required: bool) -> Value {
    json!({ "ok": false, "error": error, "password_required": password_required })
}

/// A string property of the options object; a missing, non-object or non-string value is `None`.
fn option_string(options: &JsValue, key: &str) -> Option<String> {
    if !options.is_object() {
        return None;
    }
    js_sys::Reflect::get(options, &JsValue::from_str(key)).ok()?.as_string()
}

/// `window.pdfcraft.insertPages(bytes, { name?, password? })`. The password goes into the queued
/// call and nowhere else: it is not logged, not formatted and not part of any result.
async fn insert_pages_reply(calls: &HostCalls, ctx: &egui::Context, pdf: &JsValue, options: &JsValue) -> Value {
    let bytes = match bytes_of(pdf, MAX_PDF_BYTES, "PDF", "PDF") {
        Ok(bytes) => bytes,
        Err(error) => return insert_failure(&error, false),
    };
    let name = option_string(options, "name").unwrap_or_default();
    let password = option_string(options, "password").map(HostPassword::new);
    let (tx, rx) = channel();
    if !queue(calls, ctx, HostCall::InsertPages { name, bytes, password, reply: tx }) {
        return insert_failure("the editor is not available", false);
    }
    match poll_reply(rx).await {
        Some(Ok(pages)) => json!({ "ok": true, "pages": pages }),
        Some(Err(InsertFromBytesError::PasswordRequired)) => insert_failure("the PDF needs a password", true),
        Some(Err(InsertFromBytesError::Failed(error))) => insert_failure(&error, false),
        None => insert_failure("the app dropped the request", false),
    }
}

/// `{ ok: true, name, bytes: Uint8Array, doc, edit_generation }`.
fn saved_object(saved: SavedBytes) -> JsValue {
    let meta = json!({ "ok": true, "name": saved.name, "doc": saved.doc, "edit_generation": saved.edit_generation });
    let object = to_js(&meta);
    let bytes = js_sys::Uint8Array::from(saved.bytes.as_slice());
    if js_sys::Reflect::set(&object, &JsValue::from_str("bytes"), &bytes).is_err() {
        return to_js(&failure("the saved bytes could not be returned"));
    }
    object
}

/// Dispatch `pdfcraft:state` on the canvas with the snapshot as `detail`. The page's own listeners
/// run inside the dispatch, so a failure here is logged, never raised into the app.
fn dispatch_state(state: &Value) {
    let Some(canvas) = crate::app_canvas() else { return };
    let init = web_sys::CustomEventInit::new();
    init.set_detail(&to_js(state));
    match web_sys::CustomEvent::new_with_event_init_dict("pdfcraft:state", &init) {
        Ok(event) => {
            let _ = canvas.dispatch_event(&event);
        }
        Err(e) => eframe::web_sys::console::warn_1(&format!("PdfCraft: couldn't make the pdfcraft:state event: {e:?}").into()),
    }
}
