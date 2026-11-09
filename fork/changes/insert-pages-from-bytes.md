---
type: Fork Change
title: Insert pages from bytes, and the image size rule as an engine function
description: Edit::InsertPagesFrom takes an optional password for the inserted PDF; PdfCraftApp::insert_pages_from_bytes inserts the pages of a PDF given as bytes without a file chooser, dialog or toast; the size an added image is placed at is a public engine function.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
  - id: hosting
    resource: /hosting.md
  - id: host-api
    resource: /changes/host-api.md
---

# Insert pages from bytes, and the image size rule as an engine function

## Why
- Backlog "Expose insert pages from bytes": Workspace passes the bytes of another PDF and the editor inserts its pages as "Insert pages from another PDF" does. This change is the editor side; exporting it on `window.pdfcraft` (the Promise, the result, the event) is another change.
- Backlog "Place an image on a page": the preview at the pointer needs the natural size of an image before the edit is made, by the same rule as the edit.

## What it changes
- Engine, `Edit::InsertPagesFrom` has a new field `password: Option<SourcePassword>`. It opens the inserted PDF (user or owner password) and is used for nothing else.
  - `SourcePassword` (`crates/engine/src/edit.rs`, exported from the crate root) wraps the string; its `Debug` prints `SourcePassword(..)`, as `Protection` hides its passwords, so an edit in a log or a test failure does not show it. It is in no label (the label is `Insert pages from {name}`), no error text.
  - Every constructor was updated: `crates/automation/src/assemble.rs` (the `insert_pages` tool, `password: None`), `crates/ui-egui/src/editing.rs` (paste pages), `crates/ui-egui/src/workflows/files.rs` (insert files), the engine tests.
  - A PDF that needs a password: with none given the edit fails with `EditError::Source("<name>: it is password-protected")` (the engine's existing text and error), and `pdfcraft_engine::combine_source_check(bytes, None)` returns `SourceProblem::Password` for it; with a wrong one it fails with the existing `EditError::WrongPassword`. The check runs only after a failed open (`open_inserted` in `crates/engine/src/edit/run.rs`), so a successful insert opens the file once.
  - The user password of a PDF that withholds page assembly opens it but does not allow copying its pages (`SourceProblem::NotPermitted`); the owner password does. Unchanged engine behaviour.
- Engine, the image size rule in `crates/engine/src/edit/run.rs`, exported from the crate root:
  - `pub fn added_image_natural_size(name: &str, bytes: &[u8]) -> Result<(f64, f64), EditError>`: pixels * 72 / dpi, 72 dpi when the image has none. It embeds the image in a scratch document with `embed_image`, the call the Add image edit makes, so the decoding and the limits are the same; a panic in a decoder is caught and returned as `EditError::Source`. Bytes that are empty, not an image, truncated or over the limits return the engine's error text.
  - `pub fn fit_added_image(natural: (f64, f64), page: (f64, f64)) -> (f64, f64)`: 80 percent of the page on each side, the shape kept, never enlarged; a size that is not finite and positive is returned as it is.
  - The `Edit::AddImage` arm without a rect now calls `fit_added_image`: one rule.
- App, `PdfCraftApp::insert_pages_from_bytes(name: String, bytes: Vec<u8>, password: Option<String>) -> Result<usize, InsertFromBytesError>` in `crates/ui-egui/src/workflows/files.rs`:
  - inserts every page after the last selected page, or the current page when none is selected (not at the gap a "+" in the page grid chose), as one undo step, selects the inserted pages, and returns how many (the difference of the page counts);
  - `InsertFromBytesError` is `pub` in `files.rs` with `PasswordRequired` and `Failed(String)` (the engine's text unchanged). `PasswordRequired` is returned when the password is missing or wrong (the host asks again); it is decided from the engine's error (`WrongPassword`, or a `Source` error that `combine_source_check` calls `Password` or `WrongPassword`);
  - no document open: `Failed("no document is open")`; bytes that are not a PDF, empty or random: `Failed` with the engine's text, document unchanged; permissions that forbid assembly: `Failed("the document's security settings don't allow page changes; ...")`, document unchanged;
  - no file chooser, dialog or toast: `PdfCraftApp::apply_edit` was split into `apply_edit` (toasts a failure, as before) and `apply_edit_quiet` (`crates/ui-egui/src/editing.rs`), which returns the engine's error and leaves the failure bookkeeping of the view as it was;
  - the open document's information is its own: the edit does not touch it.
  - The first parameter is the inserted file's name, used as on the picker path (the undo step's label, the title without extension of the one bookmark that holds its bookmarks, the engine's error text); blank, it is `document.pdf`.

## Tests
- Engine (`crates/engine/src/tests.rs`): natural size of a 480 x 180 px PNG with no resolution (480 x 180 pt), with a resolution, equal to what Add image embeds, refusing empty, text, PDF and truncated bytes; `fit_added_image` (fits, too wide for a small page, too tall, unusable sizes); Add image of a 480 x 180 px image on a 200 x 300 page lands at `[20, 120, 180, 180]`; a locked PDF with no, a wrong and the right (owner) password, the document unchanged after the failures, one undo step; the password in no `Debug`, label or error; empty, text and random bytes fail as `EditError::Source` with the document, undo history and edit generation unchanged.
- App (`insert_bytes_tests` in `workflows/files.rs`): no document; after the current page, selected, one undo step, no toast or dialog; after the last selected page; the title stays the open document's; bytes that are not a PDF; a locked PDF (none, wrong, right); a document that forbids assembly; the bookmarks of a file come under one bookmark named after it (`Report.pdf` -> `Report`), a file with none adds none, a blank name is `document.pdf`.
- Not built or run (workers do not build).

## Not covered
- `window.pdfcraft.insertPages...`, its result and its e2e tests, and exporting `InsertFromBytesError` from the crate root (`crates/ui-egui/src/lib.rs` line 71 lists the `files` exports): worker-host-functions.
- A dedicated `EditError` variant for a needed password: `crates/engine/src/error.rs` was not in this task's file list, so the engine tells the cases apart by the existing variants (above).
- The picker path does not take a password (the file chooser flow shows no password prompt for insert, as before).

## Depends on upstream
Nothing new: `open_source_with`, `combine_source_check`, `embed_image`, `Document::new_empty`.
