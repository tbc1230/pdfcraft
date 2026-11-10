//! What the embedding page asks of the editor and reads back (web): save and get the bytes, run
//! a command, and the editor state. The page's functions (`apps/pdfcraft-web/src/host_api.rs`)
//! queue a [`HostCall`] with a reply channel; the app serves the queue once a frame (in `logic`)
//! and answers on the channel. The state snapshot is built at the end of each frame and handed to
//! [`HostStateFn`] when it differs from the last one sent. Nothing here toasts for a host call:
//! the host owns every message to the user.

use std::collections::VecDeque;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use pdfcraft_engine::commands::{self, CommandSpec};
use serde_json::{Value, json};

use crate::PdfCraftApp;
use crate::files::InsertFromBytesError;

/// The bytes a save call returns, and which state they hold.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SavedBytes {
    /// The document's name.
    pub name: String,
    /// The saved PDF.
    pub bytes: Vec<u8>,
    /// The document (its `DocId`).
    pub doc: u64,
    /// The `edit_generation` these bytes correspond to: the document's value after the save was
    /// recorded (recording a save raises it by one), which is what the state snapshot reports
    /// while nothing has changed since these bytes. The client compares it with the snapshot's
    /// `edit_generation`.
    pub edit_generation: u64,
}

/// What the page asks for.
pub enum HostCall {
    /// Save the active document and return its bytes; nothing is downloaded or posted.
    Save(Sender<Result<SavedBytes, String>>),
    /// Run an editor command by id.
    RunCommand(String, Sender<Result<(), String>>),
    /// Start the pointer placement of an image (`start_image_placement`): `placeImage`, and
    /// `placeSignature`, which is the same call with no name. The variant is named for what it
    /// does now that it serves both.
    PlaceImage {
        /// The image's name, when the page gave a string (see [`image_name`]).
        name: Option<String>,
        /// The image file.
        bytes: Vec<u8>,
        /// Whether the placement started, or why not.
        reply: Sender<Result<(), String>>,
    },
    /// Insert the pages of another PDF after the current page or selection.
    InsertPages {
        /// The inserted file's name (blank: the app falls back to `document.pdf`).
        name: String,
        /// The other PDF.
        bytes: Vec<u8>,
        /// The other PDF's password, if the page gave one.
        password: Option<HostPassword>,
        /// How many pages were inserted, or why not.
        reply: Sender<Result<usize, InsertFromBytesError>>,
    },
}

/// A password the page gave. `HostCall` does not derive `Debug`, and this type prints no text in
/// `Debug`, so the password cannot get into a log through either; it is never formatted anywhere
/// and is handed on only to the engine (as `SourcePassword`, which also hides it).
#[derive(Clone, PartialEq, Eq)]
pub struct HostPassword(String);

impl HostPassword {
    /// Wrap the password the page gave.
    pub fn new(password: String) -> Self {
        Self(password)
    }
}

impl std::fmt::Debug for HostPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HostPassword(..)")
    }
}

/// The name an image gets when the page gave none (`placeSignature`, or `placeImage` without a
/// name). The engine reads the format from the bytes, so the name only shows in its error texts.
const DEFAULT_IMAGE_NAME: &str = "signature.png";

/// The longest image name the editor keeps, in characters.
const MAX_IMAGE_NAME_CHARS: usize = 255;

/// The name an image is placed under: the page's string without control characters and cut to
/// [`MAX_IMAGE_NAME_CHARS`] characters, or [`DEFAULT_IMAGE_NAME`] when there is none (missing,
/// not a string, or nothing left of it).
pub(crate) fn image_name(given: Option<String>) -> String {
    let cleaned: String = given.unwrap_or_default().chars().filter(|c| !c.is_control()).take(MAX_IMAGE_NAME_CHARS).collect();
    if cleaned.trim().is_empty() { DEFAULT_IMAGE_NAME.to_string() } else { cleaned }
}

/// How many names of files the page sent to open are remembered while their outcome is unknown.
const MAX_TRACKED_OPENS: usize = 64;

/// The queue the page's calls wait in, which the app serves every frame, and the names of the
/// files the page sent to open (`pdfcraft:open`) whose outcome the page has not been told.
/// Clones share both.
#[derive(Clone, Default)]
pub struct HostCalls {
    shared: Arc<Mutex<Shared>>,
}

#[derive(Default)]
struct Shared {
    queue: VecDeque<HostCall>,
    opens: VecDeque<String>,
}

impl HostCalls {
    /// Queue a call; false when the queue is unavailable.
    pub fn push(&self, call: HostCall) -> bool {
        self.shared.lock().map(|mut shared| shared.queue.push_back(call)).is_ok()
    }

    fn take_calls(&self) -> Vec<HostCall> {
        self.shared.lock().map(|mut shared| shared.queue.drain(..).collect()).unwrap_or_default()
    }

    /// The page sent a file named `name` to open. Until the editor knows the outcome the name is
    /// remembered, so that a password prompt for it can be told from the editor's own prompts.
    pub fn expect_open(&self, name: &str) {
        if let Ok(mut shared) = self.shared.lock() {
            if shared.opens.len() >= MAX_TRACKED_OPENS {
                shared.opens.pop_front();
            }
            shared.opens.push_back(name.to_string());
        }
    }

    /// The outcome of the open of `name` is known (opened, failed or cancelled). True when the
    /// page had sent it.
    pub(crate) fn settle_open(&self, name: &str) -> bool {
        let Ok(mut shared) = self.shared.lock() else { return false };
        match shared.opens.iter().position(|n| n == name) {
            Some(at) => shared.opens.remove(at).is_some(),
            None => false,
        }
    }
}

/// What the editor tells the page without being asked.
#[derive(Clone, Debug, PartialEq)]
pub enum HostEvent {
    /// The editor state (see [`PdfCraftApp::host_state_snapshot`]), each time it changes.
    State(Value),
    /// The user cancelled the password prompt of the document `name`, which the page had sent
    /// to open: nothing opened. It carries no password, and none can be added to it.
    PasswordRequired {
        /// The name the page gave the file.
        name: String,
    },
}

impl HostEvent {
    /// The name of the page event (a `CustomEvent` on the canvas) this is sent as.
    pub fn event_name(&self) -> &'static str {
        match self {
            Self::State(_) => "pdfcraft:state",
            Self::PasswordRequired { .. } => "pdfcraft:open-failed",
        }
    }

    /// The event's `detail`. An open failure is `{ name, cause, error }`: `cause` names why the
    /// open failed ("password-required" is the only one sent so far; others can join without
    /// changing this shape). It never holds a password.
    pub fn detail(&self) -> Value {
        match self {
            Self::State(state) => state.clone(),
            Self::PasswordRequired { name } => json!({ "name": name, "cause": "password-required", "error": "the PDF needs a password" }),
        }
    }
}

/// Told what the editor reports: its state, and the open events.
pub type HostStateFn = Box<dyn FnMut(HostEvent)>;

/// The state with no document open.
pub fn no_document_state() -> Value {
    json!({
        "open": false, "doc": null, "page_count": null, "can_undo": false, "can_redo": false,
        "edit_generation": null, "is_signed": null, "permissions": null, "info": null,
    })
}

/// The `permissions` of the snapshot for an open document. `None` is a document that is not
/// encrypted: the engine allows everything for it (`Document::allows_printing` and the others),
/// so every permission is true and `owner` is true; `encrypted` tells the two cases apart.
fn permissions_value(permissions: Option<pdfcraft_cos::Permissions>) -> Value {
    match permissions {
        Some(p) => json!({
            "encrypted": true,
            "print": p.print(), "modify": p.modify(), "copy": p.copy(), "annotate": p.annotate(),
            "fill_forms": p.fill_forms(), "extract_for_accessibility": p.extract_for_accessibility(),
            "assemble": p.assemble(), "print_high_quality": p.print_high_quality(), "owner": p.owner,
        }),
        None => json!({
            "encrypted": false,
            "print": true, "modify": true, "copy": true, "annotate": true,
            "fill_forms": true, "extract_for_accessibility": true,
            "assemble": true, "print_high_quality": true, "owner": true,
        }),
    }
}

impl PdfCraftApp {
    /// Serve every queued host call, then report the state. One call in `logic`.
    pub(crate) fn serve_host_calls(&mut self) {
        for call in self.host_calls.take_calls() {
            match call {
                HostCall::Save(reply) => {
                    let _ = reply.send(self.host_save_call());
                }
                HostCall::RunCommand(id, reply) => {
                    let result = self.host_run_command(&id);
                    let _ = reply.send(result);
                }
                HostCall::PlaceImage { name, bytes, reply } => {
                    let _ = reply.send(self.start_image_placement(image_name(name), bytes));
                }
                HostCall::InsertPages { name, bytes, password, reply } => {
                    let result = self.insert_pages_from_bytes(name, bytes, password.map(|p| p.0));
                    let _ = reply.send(result);
                }
            }
        }
        self.report_host_state();
    }

    /// Save the active document and return the bytes, as Save does in the web build: typing is
    /// committed, Fill & Sign marks are flattened when the preference is on, the engine writes the
    /// bytes, and the document is marked saved. No toast, no download, no `pdfcraft:save`.
    pub(crate) fn host_save_call(&mut self) -> Result<SavedBytes, String> {
        let Some((_, id)) = self.active_ids() else { return Err("no document is open".to_string()) };
        // The editor's own messages are kept out of the way: the host owns every message.
        let held = (self.toast.take(), self.toast_script.take(), self.toast_dropped, self.script_messages_dropped);
        let result = self.host_save_quietly(id);
        let said = self.toast.take().map(|(text, _)| text);
        (self.toast, self.toast_script, self.toast_dropped, self.script_messages_dropped) = held;
        result.map_err(|e| e.or(said).unwrap_or_else(|| "the document could not be saved".to_string()))
    }

    /// The save itself. An error carries its text, or `None` when only a toast said why.
    fn host_save_quietly(&mut self, id: pdfcraft_engine::DocId) -> Result<SavedBytes, Option<String>> {
        // A correction typed and queued but not yet applied (`pending_edit`) must be in the bytes.
        // `commit_typing_in` applies it, but when the document refuses it the edit is dropped and
        // only a toast says so: Save would go on without it. The host's save fails instead. (A
        // paragraph still open in the editor, or a text box, is committed by the same call and
        // refused through its `false`.)
        let queued = self.views.iter().find(|v| v.id == id).and_then(|v| v.pending_edit.clone());
        let before = self.session.get(id).map(|d| d.edit_generation());
        if !self.commit_typing_in(id) {
            return Err(None);
        }
        if let Some(edit) = queued {
            let applied = self.session.get(id).map(|d| d.edit_generation()) != before;
            if !applied {
                // Left queued, as the editor's own frame would find it.
                if let Some(view) = self.views.iter_mut().find(|v| v.id == id) {
                    view.pending_edit.get_or_insert(edit);
                }
                return Err(None);
            }
        }
        let bytes = match self.take_save_bytes(id) {
            Ok(bytes) => bytes,
            Err(crate::editing::TakeBytes::Refused) => return Err(None),
            Err(crate::editing::TakeBytes::Failed(e)) => return Err(Some(e)),
        };
        let doc = self.session.get(id).ok_or_else(|| Some("no document is open".to_string()))?;
        let (name, before) = (doc.name.clone(), doc.edit_generation());
        self.finish_save(id, bytes.clone()).map_err(|e| Some(e.to_string()))?;
        // The value after the save was recorded: the one the next snapshot reports.
        let edit_generation = self.session.get(id).map_or(before, |d| d.edit_generation());
        Ok(SavedBytes { name, bytes: bytes.to_vec(), doc: id.0, edit_generation })
    }

    /// Run a command for the host: refused, with the reason, when the id is unknown, is not
    /// offered in this editor, or is disabled now; otherwise [`PdfCraftApp::run_command`].
    pub(crate) fn host_run_command(&mut self, id: &str) -> Result<(), String> {
        self.check_host_command(id, crate::commands::offered)?;
        self.run_command(id);
        Ok(())
    }

    /// The refusals of [`Self::host_run_command`]; `offered` is `crate::commands::offered`.
    pub(crate) fn check_host_command(&self, id: &str, offered: impl Fn(&str) -> bool) -> Result<&'static CommandSpec, String> {
        let spec = commands::command(id).ok_or_else(|| format!("unknown command: {id}"))?;
        if !offered(id) {
            return Err(format!("command {id} is not offered in this editor"));
        }
        if !self.command_enabled(spec) {
            let why = self.disabled_reason(spec);
            let mut error = format!("command {id} is not available now");
            if !why.is_empty() {
                error.push_str(": ");
                error.push_str(&why);
            }
            return Err(error);
        }
        Ok(spec)
    }

    /// The editor state the page reads: see `fork/changes/host-api.md` for each field.
    pub fn host_state_snapshot(&self) -> Value {
        let Some(view) = self.active.and_then(|i| self.views.get(i)) else { return no_document_state() };
        let Some(doc) = self.session.get(view.id) else { return no_document_state() };
        let permissions = permissions_value(doc.permissions());
        json!({
            "open": true,
            "doc": view.id.0,
            "page_count": doc.info.pages.len(),
            "can_undo": doc.can_undo().is_some(),
            "can_redo": doc.can_redo().is_some(),
            "edit_generation": doc.edit_generation(),
            "is_signed": doc.is_signed(),
            "permissions": permissions,
            "info": { "title": doc.info.title, "author": doc.info.author, "subject": doc.info.subject, "keywords": doc.info.keywords },
        })
    }

    /// Hand the snapshot to the page when it differs from the last one sent. Called at the end of
    /// `logic` (through [`Self::serve_host_calls`]) and at the end of `ui`, so an edit, undo, redo,
    /// page change or opened document made during a frame is in that frame's last snapshot.
    pub(crate) fn report_host_state(&mut self) {
        if self.host_state.is_none() {
            return;
        }
        let snapshot = self.host_state_snapshot();
        if self.host_state_last.as_ref() == Some(&snapshot) {
            return;
        }
        self.report_host_event(HostEvent::State(snapshot.clone()));
        self.host_state_last = Some(snapshot);
    }

    /// Hand an event to the page, if it listens.
    pub(crate) fn report_host_event(&mut self, event: HostEvent) {
        if let Some(report) = self.host_state.as_mut() {
            report(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pdfcraft_engine::{Edit, Session};

    fn app_with_document() -> PdfCraftApp {
        let mut app = PdfCraftApp::new();
        let pdf = Session::new().create_blank(595.0, 842.0, 2).unwrap().to_vec();
        app.open_bytes("doc.pdf", None, pdf).unwrap();
        app
    }

    #[test]
    fn the_snapshot_without_a_document() {
        let app = PdfCraftApp::new();
        assert_eq!(app.host_state_snapshot(), no_document_state());
        assert_eq!(app.host_state_snapshot()["open"], json!(false));
        assert_eq!(app.host_state_snapshot()["can_undo"], json!(false));
    }

    #[test]
    fn the_snapshot_with_a_document_follows_edits_undo_and_redo() {
        let mut app = app_with_document();
        let before = app.host_state_snapshot();
        assert_eq!(before["open"], json!(true));
        assert_eq!(before["page_count"], json!(2));
        assert_eq!(before["can_undo"], json!(false));
        assert_eq!(before["is_signed"], json!(false));
        assert!(before["permissions"].is_object(), "a document that is not encrypted has permissions too");
        assert!(before["info"].is_object());
        assert!(before["doc"].is_u64());
        assert!(app.apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 }));
        let after = app.host_state_snapshot();
        assert_ne!(after["edit_generation"], before["edit_generation"], "an edit changes the generation");
        assert_eq!(after["can_undo"], json!(true));
        assert_eq!(after["can_redo"], json!(false));
        assert!(app.execute("edit.undo"));
        let undone = app.host_state_snapshot();
        assert_eq!(undone["can_undo"], json!(false));
        assert_eq!(undone["can_redo"], json!(true));
        assert_ne!(undone["edit_generation"], after["edit_generation"]);
    }

    #[test]
    fn the_state_is_reported_only_when_it_changes() {
        let mut app = app_with_document();
        let seen: std::rc::Rc<std::cell::RefCell<Vec<Value>>> = Default::default();
        let sink = seen.clone();
        app.host_state = Some(Box::new(move |event| {
            if let HostEvent::State(s) = event {
                sink.borrow_mut().push(s);
            }
        }));
        app.report_host_state();
        app.report_host_state();
        assert_eq!(seen.borrow().len(), 1);
        assert!(app.apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 }));
        app.report_host_state();
        assert_eq!(seen.borrow().len(), 2);
    }

    #[test]
    fn run_command_refuses_unknown_not_offered_and_disabled() {
        let mut app = app_with_document();
        let unknown = app.host_run_command("no.such.command").unwrap_err();
        assert!(unknown.contains("unknown command"), "{unknown}");
        let hidden = app.check_host_command("edit.undo", |_| false).unwrap_err();
        assert!(hidden.contains("not offered"), "{hidden}");
        // Nothing to undo yet: disabled, and nothing runs.
        let disabled = app.host_run_command("edit.undo").unwrap_err();
        assert!(disabled.contains("not available"), "{disabled}");
        assert!(app.apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 }));
        assert_eq!(app.host_run_command("edit.undo"), Ok(()));
        assert_eq!(app.host_state_snapshot()["can_undo"], json!(false));
    }

    #[test]
    fn permissions_of_a_document_that_is_not_encrypted_allow_everything() {
        let app = app_with_document();
        let p = app.host_state_snapshot()["permissions"].clone();
        assert_eq!(p["encrypted"], json!(false));
        for key in ["print", "modify", "copy", "annotate", "fill_forms", "extract_for_accessibility", "assemble", "print_high_quality", "owner"] {
            assert_eq!(p[key], json!(true), "{key}");
        }
        assert_eq!(app.host_state_snapshot()["permissions"], permissions_value(None));
        assert_eq!(PdfCraftApp::new().host_state_snapshot()["permissions"], Value::Null, "no document: no permissions");
    }

    #[test]
    fn permissions_of_an_encrypted_document_are_the_engines() {
        // Opened with the user password, and the file allows printing only (bit 3 of /P).
        let restricted = pdfcraft_cos::Permissions { bits: 1 << 2, owner: false, revision: 3 };
        let p = permissions_value(Some(restricted));
        assert_eq!(p["encrypted"], json!(true));
        assert_eq!(p["print"], json!(true));
        assert_eq!(p["modify"], json!(false));
        assert_eq!(p["copy"], json!(false));
        assert_eq!(p["owner"], json!(false));
    }

    fn serve(app: &mut PdfCraftApp, call: HostCall) {
        assert!(app.host_calls.push(call));
        app.serve_host_calls();
    }

    fn png() -> Vec<u8> {
        crate::create_ui::png_from_rgba(1, 1, &[255, 0, 0, 255]).unwrap()
    }

    /// A call of `placeImage` (or `placeSignature`, which has no name) and its reply.
    fn place(app: &mut PdfCraftApp, name: Option<&str>, bytes: Vec<u8>) -> Result<(), String> {
        let (tx, rx) = std::sync::mpsc::channel();
        serve(app, HostCall::PlaceImage { name: name.map(str::to_string), bytes, reply: tx });
        rx.try_recv().unwrap()
    }

    /// A one-page PDF with the open password `pw` and the owner password `owner`, and the
    /// restrictions of Protection's defaults (no changes, no copying) for the user password.
    fn locked_pdf() -> Vec<u8> {
        let mut other = PdfCraftApp::new();
        other.open_bytes("o.pdf", None, Session::new().create_blank(595.0, 842.0, 1).unwrap().to_vec()).unwrap();
        assert!(other.apply_edit(Edit::Protect(pdfcraft_engine::Protection {
            open_password: Some("pw".into()),
            permissions_password: Some("owner".into()),
            ..Default::default()
        })));
        other.session.save_bytes(other.views[0].id).unwrap().to_vec()
    }

    type Events = std::rc::Rc<std::cell::RefCell<Vec<HostEvent>>>;

    /// Every event the app hands to the page from now on.
    fn record_events(app: &mut PdfCraftApp) -> Events {
        let seen: Events = Default::default();
        let sink = seen.clone();
        app.host_state = Some(Box::new(move |event| sink.borrow_mut().push(event)));
        seen
    }

    #[test]
    fn place_image_with_no_document_is_refused() {
        let mut app = PdfCraftApp::new();
        assert_eq!(place(&mut app, Some("code.png"), png()), Err("no document is open".to_string()));
        assert!(app.toast.is_none());
    }

    #[test]
    fn place_image_refuses_bytes_that_are_not_an_image_and_empty_bytes() {
        let mut app = app_with_document();
        let before = app.host_state_snapshot();
        for bytes in [b"not an image".to_vec(), Vec::new()] {
            assert!(place(&mut app, Some("x.png"), bytes).is_err());
            assert!(app.views.iter().all(|v| !v.image_place.is_pending()), "nothing started");
        }
        assert!(app.toast.is_none(), "no toast for a refusal");
        assert_eq!(app.host_state_snapshot(), before, "the document is unchanged");
    }

    #[test]
    fn place_image_is_refused_when_the_file_does_not_allow_changes() {
        // Opened with the user password: Protection's defaults allow no changes.
        let mut app = PdfCraftApp::new();
        app.open_bytes("l.pdf", None, locked_pdf()).unwrap();
        app.submit_password(Some("pw".into()));
        let error = place(&mut app, None, png()).unwrap_err();
        assert!(error.contains("command edit.image is not available now"), "{error}");
        assert!(app.views.iter().all(|v| !v.image_place.is_pending()));
    }

    #[test]
    fn place_image_starts_the_placement_with_or_without_a_name() {
        // The name is only text for the engine's messages: the format is read from the bytes, so a
        // name whose extension disagrees still places.
        for name in [None, Some("code.png"), Some("code.jpg"), Some(""), Some("a\nb\u{0}c")] {
            let mut app = app_with_document();
            assert_eq!(place(&mut app, name, png()), Ok(()), "{name:?}");
            assert!(app.views.iter().any(|v| v.image_place.is_pending()), "the placement started: {name:?}");
            assert!(app.toast.is_none());
        }
    }

    #[test]
    fn place_signature_is_place_image_without_a_name() {
        // `placeSignature(bytes)` queues `PlaceImage { name: None, .. }`: it starts the same
        // placement, and refuses what `placeImage` refuses, with the same texts.
        let mut app = PdfCraftApp::new();
        assert_eq!(place(&mut app, None, png()), Err("no document is open".to_string()));
        let mut app = app_with_document();
        assert!(place(&mut app, None, b"not an image".to_vec()).is_err());
        assert_eq!(place(&mut app, None, png()), Ok(()));
        assert!(app.views.iter().any(|v| v.image_place.is_pending()));
        assert_eq!(image_name(None), "signature.png", "the name placeSignature has always used");
    }

    #[test]
    fn the_image_name_falls_back_loses_control_characters_and_is_cut() {
        assert_eq!(image_name(None), "signature.png");
        assert_eq!(image_name(Some(String::new())), "signature.png");
        assert_eq!(image_name(Some("  \n\t".into())), "signature.png");
        assert_eq!(image_name(Some("code.png".into())), "code.png");
        assert_eq!(image_name(Some("a\nb\u{0}c.png".into())), "abc.png");
        let long = "\u{e9}".repeat(1000);
        assert_eq!(image_name(Some(long)).chars().count(), 255);
    }

    #[test]
    fn insert_pages_with_no_document_fails_saying_so() {
        let mut app = PdfCraftApp::new();
        let (tx, rx) = std::sync::mpsc::channel();
        let bytes = Session::new().create_blank(595.0, 842.0, 1).unwrap().to_vec();
        serve(&mut app, HostCall::InsertPages { name: "b.pdf".into(), bytes, password: None, reply: tx });
        assert_eq!(rx.try_recv().unwrap(), Err(InsertFromBytesError::Failed("no document is open".into())));
    }

    #[test]
    fn insert_pages_returns_the_count() {
        let mut app = app_with_document();
        let (tx, rx) = std::sync::mpsc::channel();
        let bytes = Session::new().create_blank(595.0, 842.0, 3).unwrap().to_vec();
        serve(&mut app, HostCall::InsertPages { name: "b.pdf".into(), bytes, password: None, reply: tx });
        assert_eq!(rx.try_recv().unwrap(), Ok(3));
        assert_eq!(app.host_state_snapshot()["page_count"], json!(5));
        assert!(app.toast.is_none() && app.dialog.is_none());
    }

    #[test]
    fn insert_pages_maps_a_needed_password_and_keeps_it_out_of_debug() {
        let mut app = app_with_document();
        // A PDF with an open password: none and a wrong one both ask for a password.
        let mut other = PdfCraftApp::new();
        other.open_bytes("o.pdf", None, Session::new().create_blank(595.0, 842.0, 1).unwrap().to_vec()).unwrap();
        assert!(other.apply_edit(Edit::Protect(pdfcraft_engine::Protection {
            open_password: Some("pw".into()),
            permissions_password: Some("owner".into()),
            ..Default::default()
        })));
        let locked = other.session.save_bytes(other.views[0].id).unwrap().to_vec();
        for password in [None, Some(HostPassword::new("wrong".into()))] {
            let (tx, rx) = std::sync::mpsc::channel();
            serve(&mut app, HostCall::InsertPages { name: "o.pdf".into(), bytes: locked.clone(), password, reply: tx });
            assert_eq!(rx.try_recv().unwrap(), Err(InsertFromBytesError::PasswordRequired));
        }
        assert_eq!(app.host_state_snapshot()["page_count"], json!(2), "unchanged");
        assert_eq!(format!("{:?}", HostPassword::new("secret".into())), "HostPassword(..)");
    }

    #[test]
    fn insert_pages_failure_carries_the_engines_text() {
        let mut app = app_with_document();
        let (tx, rx) = std::sync::mpsc::channel();
        serve(&mut app, HostCall::InsertPages { name: "b.pdf".into(), bytes: b"not a pdf".to_vec(), password: None, reply: tx });
        assert!(matches!(rx.try_recv().unwrap(), Err(InsertFromBytesError::Failed(text)) if !text.is_empty()));
        assert_eq!(app.host_state_snapshot()["page_count"], json!(2), "unchanged");
    }

    #[test]
    fn save_with_no_document_is_an_error() {
        let mut app = PdfCraftApp::new();
        assert_eq!(app.host_save_call().unwrap_err(), "no document is open");
    }

    #[test]
    fn save_failure_carries_the_engines_text() {
        // The tab is still there but the engine no longer has the document: `save_bytes` fails.
        let mut app = app_with_document();
        let (_, id) = app.active_ids().unwrap();
        app.session.close(id);
        let error = app.host_save_call().unwrap_err();
        assert_eq!(error, "no such document");
        assert!(app.toast.is_none(), "no toast for a host call");
    }

    #[test]
    fn a_refused_flatten_before_the_save_carries_the_editors_text() {
        // The same, found by the flatten step, which tells the user by a toast: the toast's text is returned.
        let mut app = app_with_document();
        app.flatten_fill_sign_on_save = true;
        let (_, id) = app.active_ids().unwrap();
        app.session.close(id);
        let error = app.host_save_call().unwrap_err();
        assert_eq!(error, "no such document");
        assert!(app.toast.is_none(), "the toast was kept out of the way");
    }

    #[test]
    fn a_save_changes_the_state_once() {
        let mut app = app_with_document();
        assert!(app.apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 }));
        let seen: std::rc::Rc<std::cell::RefCell<Vec<Value>>> = Default::default();
        let sink = seen.clone();
        app.host_state = Some(Box::new(move |event| {
            if let HostEvent::State(s) = event {
                sink.borrow_mut().push(s);
            }
        }));
        app.report_host_state();
        assert_eq!(seen.borrow().len(), 1);
        let saved = app.host_save_call().unwrap();
        app.report_host_state();
        app.report_host_state();
        assert_eq!(seen.borrow().len(), 2, "one event for the save, none for the repeat");
        assert_eq!(seen.borrow()[1]["edit_generation"], json!(saved.edit_generation));
    }

    #[test]
    fn save_returns_the_bytes_and_leaves_no_unsaved_changes() {
        let mut app = app_with_document();
        assert!(app.apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 }));
        let (_, id) = app.active_ids().unwrap();
        assert!(app.session.get(id).unwrap().dirty);
        let before = app.host_state_snapshot()["edit_generation"].clone();
        let saved = app.host_save_call().unwrap();
        assert!(saved.bytes.starts_with(b"%PDF"));
        assert_eq!(saved.name, "doc.pdf");
        assert_eq!(saved.doc, id.0);
        assert_ne!(json!(saved.edit_generation), before, "recording the save raises the generation");
        assert!(!app.session.get(id).unwrap().dirty, "no unsaved changes after the save");
        assert!(!app.has_unsaved_work(0));
        assert_eq!(
            app.host_state_snapshot()["edit_generation"],
            json!(saved.edit_generation),
            "the result's generation is the one the state reports"
        );
        assert!(app.toast.is_none(), "no toast for a host call");
        // The edit is in the bytes, and the document can still be undone.
        assert_eq!(app.host_state_snapshot()["can_undo"], json!(true));
        let mut check = Session::new();
        let reopened = check.open("x.pdf", None, Arc::new(saved.bytes), None).unwrap();
        assert_eq!(check.get(reopened).unwrap().info.pages[0].rotation, 90);
    }

    #[test]
    fn cancelling_the_password_prompt_of_a_host_file_tells_the_host_once() {
        let mut app = app_with_document();
        let events = record_events(&mut app);
        let before = app.host_state_snapshot();
        app.host_calls.expect_open("locked.pdf");
        app.open_bytes("locked.pdf", None, locked_pdf()).unwrap();
        assert!(app.password_prompt.is_some(), "the editor's own prompt asks");
        assert!(events.borrow().is_empty(), "nothing is reported while the user is asked");
        app.submit_password(None);
        assert_eq!(*events.borrow(), vec![HostEvent::PasswordRequired { name: "locked.pdf".into() }]);
        assert!(app.password_prompt.is_none());
        assert_eq!(app.views.len(), 1, "nothing opened");
        assert_eq!(app.host_state_snapshot(), before, "the open document is unchanged");
        app.submit_password(None);
        assert_eq!(events.borrow().len(), 1, "told once");
    }

    #[test]
    fn a_wrong_password_keeps_the_prompt_and_tells_the_host_nothing() {
        let mut app = app_with_document();
        let events = record_events(&mut app);
        app.host_calls.expect_open("locked.pdf");
        app.open_bytes("locked.pdf", None, locked_pdf()).unwrap();
        app.submit_password(Some("wrong-secret".into()));
        let prompt = app.password_prompt.as_ref().expect("the prompt stays for another try");
        assert_eq!(prompt.error.as_deref(), Some("Incorrect password. Try again."));
        assert!(events.borrow().is_empty());
        // The user tries again and cancels: the host hears it, with no password in the event.
        app.submit_password(Some("another-secret".into()));
        app.submit_password(None);
        assert_eq!(*events.borrow(), vec![HostEvent::PasswordRequired { name: "locked.pdf".into() }]);
        let shown = format!("{:?}", events.borrow());
        assert!(!shown.contains("wrong-secret") && !shown.contains("another-secret"), "{shown}");
        assert_eq!(app.views.len(), 1);
    }

    #[test]
    fn the_right_password_opens_the_host_file_and_tells_nothing() {
        let mut app = app_with_document();
        let events = record_events(&mut app);
        app.host_calls.expect_open("locked.pdf");
        app.open_bytes("locked.pdf", None, locked_pdf()).unwrap();
        app.submit_password(Some("pw".into()));
        assert!(app.password_prompt.is_none());
        assert_eq!(app.views.len(), 2);
        assert!(events.borrow().is_empty());
        assert!(!app.host_calls.settle_open("locked.pdf"), "the open is settled");
    }

    #[test]
    fn password_required_is_distinct_from_every_other_failure_and_from_the_editors_own_prompts() {
        let mut app = app_with_document();
        let events = record_events(&mut app);
        // A host file that cannot be opened at all fails as before and sends no password event.
        app.host_calls.expect_open("bad.pdf");
        assert!(app.open_bytes("bad.pdf", None, b"this is not a pdf".to_vec()).is_err());
        assert!(app.password_prompt.is_none());
        assert!(!app.host_calls.settle_open("bad.pdf"), "a failed open is settled");
        // A prompt the page did not cause (another file the editor opens) is not the page's.
        app.open_bytes("own.pdf", None, locked_pdf()).unwrap();
        app.submit_password(None);
        assert!(events.borrow().is_empty());
        assert_eq!(app.views.len(), 1);
    }

    #[test]
    fn the_host_events_and_calls_show_no_password() {
        let (tx, _rx) = std::sync::mpsc::channel();
        let call = HostCall::InsertPages { name: "o.pdf".into(), bytes: Vec::new(), password: Some(HostPassword::new("secret".into())), reply: tx };
        // `HostCall` has no `Debug`; its password does, and shows nothing.
        let HostCall::InsertPages { password, .. } = call else { return };
        assert_eq!(format!("{password:?}"), "Some(HostPassword(..))");
        // The event has a name and nothing else.
        let event = HostEvent::PasswordRequired { name: "locked.pdf".into() };
        assert_eq!(format!("{event:?}"), "PasswordRequired { name: \"locked.pdf\" }");
        assert_eq!(event.event_name(), "pdfcraft:open-failed");
        assert_eq!(
            event.detail(),
            json!({ "name": "locked.pdf", "cause": "password-required", "error": "the PDF needs a password" }),
            "a cause value, no password_required field"
        );
        assert_eq!(event.detail()["cause"], json!("password-required"));
        assert!(event.detail().get("password_required").is_none());
    }

    #[test]
    fn the_owner_password_gives_owner_and_every_right_the_user_password_does_not() {
        let mut app = PdfCraftApp::new();
        app.open_bytes("l.pdf", None, locked_pdf()).unwrap();
        app.submit_password(Some("owner".into()));
        let p = app.host_state_snapshot()["permissions"].clone();
        assert_eq!(p["encrypted"], json!(true));
        for key in ["print", "modify", "copy", "annotate", "fill_forms", "extract_for_accessibility", "assemble", "print_high_quality", "owner"] {
            assert_eq!(p[key], json!(true), "{key}");
        }
        let mut user = PdfCraftApp::new();
        user.open_bytes("l.pdf", None, locked_pdf()).unwrap();
        user.submit_password(Some("pw".into()));
        let p = user.host_state_snapshot()["permissions"].clone();
        assert_eq!(
            (p["encrypted"].clone(), p["owner"].clone(), p["modify"].clone(), p["copy"].clone()),
            (json!(true), json!(false), json!(false), json!(false))
        );
    }

    fn app_with_text() -> PdfCraftApp {
        let mut app = PdfCraftApp::new();
        let bytes = app.session.create_from_text("Preview", "Preview text").unwrap();
        app.open_bytes("preview.pdf", None, bytes.as_ref().clone()).unwrap();
        app
    }

    fn page_text(bytes: Vec<u8>) -> String {
        let mut check = Session::new();
        let id = check.open("x.pdf", None, Arc::new(bytes), None).unwrap();
        check.get(id).unwrap().text_lines(0).iter().map(|l| l.text.as_str()).collect()
    }

    #[test]
    fn a_queued_correction_is_in_the_bytes_save_returns() {
        let mut app = app_with_text();
        app.views[0].pending_edit =
            Some(Edit::EditTextBlock { page: 0, block: 0, text: "Text preview".into(), style: pdfcraft_engine::BlockStyle::default() });
        let saved = app.host_save_call().unwrap();
        assert_eq!(page_text(saved.bytes), "Text preview");
        assert!(app.views[0].pending_edit.is_none());
        assert!(app.toast.is_none(), "no toast for a host call");
    }

    #[test]
    fn a_queued_correction_that_cannot_be_applied_fails_the_save_with_the_editors_text() {
        let mut app = app_with_text();
        let (_, id) = app.active_ids().unwrap();
        let generation = app.session.get(id).unwrap().edit_generation();
        // Page 9 does not exist: the document refuses the edit.
        app.views[0].pending_edit =
            Some(Edit::EditTextBlock { page: 9, block: 0, text: "Text preview".into(), style: pdfcraft_engine::BlockStyle::default() });
        let error = app.host_save_call().unwrap_err();
        assert!(!error.is_empty() && error != "the document could not be saved", "{error}");
        assert_eq!(app.session.get(id).unwrap().edit_generation(), generation, "no save was recorded");
        assert!(!app.session.get(id).unwrap().dirty);
        assert!(app.views[0].pending_edit.is_some(), "the correction is not lost");
        assert!(app.toast.is_none(), "the toast was kept out of the way");
    }
}
