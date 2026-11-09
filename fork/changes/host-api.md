---
type: Fork Change
title: Save, commands and editor state for the client
description: window.pdfcraft.save(), runCommand(id) and getState() and the pdfcraft:state event, always installed (no ?control needed); the editor returns the saved bytes, runs offered commands and reports its state.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
  - id: hosting
    resource: /hosting.md
---

# Save, commands and editor state for the client

## Why
Three backlog items of the feature "Exported JS API", on the API side only:
- "Save and return the bytes to the client": the client asks for a save and gets the bytes, sent nowhere else.
- "Report editor state to the client": can_undo, can_redo, permissions, is_signed, edit_generation, and with them the page count and the document information, as the client's controls need them.
- "Run commands from the client": the client runs an editor command, backed by `PdfCraftApp::run_command`.

Before this change the page's functions (`execute`, `docInspect`, `documentInfo`) existed only with `?control`, which keeps the accessibility tree of every frame. Workspace must not need `?control`. The page count and the document information that the fork had added to `docInspect` and `documentInfo` are removed (see "Second revision" below), so the host API is the only way to read them. Hiding Save, Save As and the stand-alone commands is another change.

## What it changes
- `crates/ui-egui/src/app/host_calls.rs` (new): `HostCalls`, a shared queue of `HostCall` (`Save`, `RunCommand`), each with a reply channel; `PdfCraftApp::serve_host_calls` serves it once a frame, from `logic`. Fields `host_calls`, `host_state` (`HostStateFn`) and `host_state_last` on `PdfCraftApp`.
- Save: `PdfCraftApp::host_save_call` commits typing (`commit_typing_in`), flattens Fill & Sign marks when the preference is on, takes `session.save_bytes`, then `mark_saved` and `view.document_changed`. That path is one function pair in `crates/ui-egui/src/editing/save.rs`, `take_save_bytes` and `finish_save`, which the web build's own Save now uses too (its behaviour is unchanged). The call does not download, does not use `host_save` and posts no `pdfcraft:save`; the editor's toasts during it are held back and its failure text is returned. No document open: `no document is open`.
- runCommand: refused with an error, and nothing runs, when the id is not in `pdfcraft_engine::commands::command`, when `crate::commands::offered(id)` is false, or when `command_enabled` is false (the text is `command <id> is not available now`; the editor's own `disabled_reason` is private to `commands.rs`, which another change owns). Otherwise `PdfCraftApp::run_command(id)` runs and the reply is ok.
- State snapshot, built by `PdfCraftApp::host_state_snapshot` and handed to `host_state` only when it differs from the last one sent:
  `{ open, doc, page_count, can_undo, can_redo, edit_generation, is_signed, permissions, info }`. `doc` is the `DocId` of the active document; `permissions` is an object for every open document: `encrypted` plus one boolean per `pdfcraft_cos::Permissions` method (`print`, `modify`, `copy`, `annotate`, `fill_forms`, `extract_for_accessibility`, `assemble`, `print_high_quality`) and `owner`; for a document that is not encrypted `encrypted` is false and every other value is true (the engine allows everything for it); `info` is `{ title, author, subject, keywords }`. With no document: `open` false, `can_undo` and `can_redo` false, the rest null.
- The snapshot is taken twice a frame: at the end of `logic` (after the host calls are served, which also covers a frame whose `ui` does not run) and at the end of `ui`, after the control channel's `finish_frame`. A `ui` pass that returns early (full screen and the other early returns) does not report; its edits are in the snapshot of the next `logic`.
- `apps/pdfcraft-web/src/host_api.rs` (new, wasm32): puts these on `window.pdfcraft`, installed in `main.rs` before `pdfcraft:ready`, always:
  - `save()`: a Promise of `{ ok: true, name, bytes: Uint8Array, doc, edit_generation }` or `{ ok: false, error }`; it never rejects. `edit_generation` is the document's value after the save was recorded, which is what the next snapshot reports while nothing has changed (recording a save raises the edit generation by one, `crates/engine/src/session/save.rs` in `mark_saved`). The client compares it with the snapshot's `edit_generation` to know there are no edits since these bytes.
  - `runCommand(id)`: a Promise of `{ ok: true }` or `{ ok: false, error }`; a non-string id is `{ ok: false, error }` without a frame.
  - `getState()`: the last snapshot as a plain object, synchronously; before the first frame, the no-document state.
  - event `pdfcraft:state`: a `CustomEvent` on the canvas `pdfcraft`, `detail` = the snapshot, each time it changes. It is built with web-sys (the host crate has no custom-event sender); the web-sys features it needs were already enabled.
  - Every call wakes the app (`request_repaint`); replies are polled about once a frame with `api::poll_reply`, which `api.rs` now shares.
- The `?control` functions keep what upstream had; the fork's additions are removed (see "Second revision"). The desktop build is unchanged.

## Second revision (worker-state-save), per criterion
- "The result carries the edit_generation value the returned bytes correspond to": `save()` returns one generation, the value after the save was recorded, as `edit_generation`; `clean_generation` is removed (`SavedBytes`, `host_api.rs`, tests). Why: the client compares it with `getState().edit_generation`, which is that value while nothing has changed since the bytes.
- "permissions has a value for every open document": an unencrypted document reports every permission true, `owner` true and `encrypted: false`; an encrypted one reports the engine's values with `encrypted: true`; no document: `permissions` null. Why: the engine treats a document that is not encrypted as allowing everything (`allows_printing` and the others in `crates/engine/src/document.rs`); `encrypted` lets the client tell the cases apart.
- "No second way to read these values": removed `active.pages` and `active.info` from `ui.state` (`crates/ui-egui/src/control/host.rs`), `window.pdfcraft.documentInfo()` with its helper and tests (`apps/pdfcraft-web/src/api.rs`), and the `active.pages` assertion in `state_and_view_options` (`crates/ui-egui/tests/it/control.rs`). Upstream's `documents[].pages` stays. The records page-count-reported and document-info-reported are deprecated.
- "Given the editor cannot produce the bytes, the call returns a failure result with the editor's error text": every failure path of `host_save_call` returns text: an engine error (`take_save_bytes` Failed, `finish_save`), or the toast the editor raised during the call (typing that cannot be committed, a refused flatten); the generic text `the document could not be saved` appears only when no toast was raised. Findings on which documents fail are in the log entry of 2026-11-08 (worker-state-save).
- "No event without a change": `report_host_state` sends only when the snapshot differs; the generation raised by `mark_saved` gives one event after a save (test `a_save_changes_the_state_once`).
- Tests added or changed in `host_calls::tests`: the generation of the save result equals the next snapshot's; permissions for an unencrypted, an encrypted and no document; a save failing with the engine's text (the engine no longer has the document), also found through the flatten step; one event after a save.

## placeSignature and insertPages (worker-host-functions)
Two more functions on `window.pdfcraft`, always installed, in `apps/pdfcraft-web/src/host_api.rs`. Both return a Promise that never rejects, use the existing reply polling (`api::poll_reply`) and wake the app with `request_repaint`. Neither toasts: the host owns every message (`start_image_placement` and `insert_pages_from_bytes` raise none).
- `placeSignature(bytes)`: resolves `{ ok: true }` when the placement started (the pointer placement of `fork/changes/image-placement.md`), or `{ ok: false, error }`.
  - `bytes` must be a `Uint8Array` or an `ArrayBuffer`; anything else gives `{ ok: false, error: "the image must be bytes (Uint8Array)" }` with no frame.
  - Over 32 MiB is refused before copying: `{ ok: false, error: "the image is over the limit of 32 MiB" }`.
  - `HostCall::PlaceSignature(bytes, reply)` calls `start_image_placement("signature.png", bytes)`; its refusals (`no document is open`, `command edit.image is not available now`, the engine's text for bytes that are not an image) are the `error`.
- `insertPages(bytes, options)`: `options` is optional, `{ name?: string, password?: string }`; a name that is not a string counts as missing (the app falls back to `document.pdf`), as does a password that is not a string. Resolves:
  - `{ ok: true, pages }`, `pages` the number inserted;
  - `{ ok: false, error: "the PDF needs a password", password_required: true }` when the password is missing or wrong (`InsertFromBytesError::PasswordRequired`);
  - `{ ok: false, error, password_required: false }` for `InsertFromBytesError::Failed(text)`, the text unchanged;
  - bytes that are not a `Uint8Array` or `ArrayBuffer`: `{ ok: false, error: "the PDF must be bytes (Uint8Array)", password_required: false }` with no frame; over 512 MiB (the host open's limit, `MAX_OPEN_BYTES` in `main.rs`): refused before copying, `password_required: false`.
  - `HostCall::InsertPages { name, bytes, password, reply }` calls `insert_pages_from_bytes`.
- The password: it travels in the queued call as `HostPassword` (`app/host_calls.rs`), whose `Debug` prints `HostPassword(..)`; `HostCall` derives no `Debug`; the password is never formatted, logged, put in an error text, a result or the state snapshot. The app hands it on to the engine, which wraps it in `SourcePassword` (also hidden in `Debug`). `InsertFromBytesError` and `HostPassword` are exported from the crate root.
- `canvas/image_place.rs` is added to `PAGE_MODULES` in `canvas/page_pointer/tests.rs` (38 entries).
- Tests in `host_calls::tests`: placeSignature with no document, with bytes that are not an image, started; insertPages with no document, inserting with the count, the password-required mapping (none and wrong) with the hidden `Debug`, a failure with the engine's text. The right password is covered by the app tests of `insert_pages_from_bytes` and the e2e tests. Not built or run.

## Tests
Native unit tests in `host_calls::tests` (`cargo test -p pdfcraft-ui-egui`): the snapshot with and without a document, an edit, undo and redo changing it, reporting only on change; the three runCommand refusals (the not-offered case through the injected `offered` check, because `offered` refuses only `app.preferences` on wasm32); save with no document; save returning bytes that start with `%PDF`, contain the edit, leave the document with no unsaved changes and raise no toast. The web functions run only in the browser and are checked in the Mock/Stub by the integrator.

## Not covered
- Not built or run (workers do not build).
- Hiding Save, Save As and the stand-alone commands from menus, command search and shortcuts.
- The other functions of the feature (open with a password and the rest); insert pages and the drawn signature are in "placeSignature and insertPages".
- A command's own toasts and dialogs still appear when it is run through `runCommand`.

## Depends on upstream
Nothing new: it uses `Session::save_bytes`, `mark_saved`, `edit_generation`, `can_undo`, `can_redo`, `is_signed`, `permissions` and `commands::command` as they are.
