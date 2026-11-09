---
type: Fork Change
title: Document information reported to the client
description: window.pdfcraft.documentInfo() returns the title, author, subject and keywords stored in the open document; ui.state carries them too.
resource: working tree change, not committed (no pull request yet)
status: deprecated
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Document information reported to the client

Deprecated: the [Host API](/changes/host-api.md) replaces this change, because the backlog item "Report editor state to the client" requires that the fork adds no second way to read these values. `active.info` is removed from `ui.state` and `window.pdfcraft.documentInfo()` is removed from `apps/pdfcraft-web/src/api.rs`; the document information is `info` in `getState()` and in the event `pdfcraft:state`. The rest of this record describes what the change was.

## Why
Backlog item "Report the document information to the client": Workspace uses the PDF's title as the suggested name in "Save as new Item".[^mods]

## What it changes
- `Host::state` for `PdfCraftApp` (`crates/ui-egui/src/control/host.rs`): the `active` entry of `ui.state` has `info`, an object with `title`, `author`, `subject` and `keywords` of the active document (a string, or null when the document has none). They come from the document's `info`, which the engine reads from `/Info`, else XMP.
- `apps/pdfcraft-web/src/api.rs`: `window.pdfcraft.documentInfo()`, added after the page API is installed, returns a promise of `{ok: true, result: {title, author, subject, keywords}}`, or `{ok: false, error}` when no document is open or the state cannot be read. It is built on `ui.state` through `document_info`; tests cover both outcomes.
- Nothing is written to the document; the call is a read.

## Depends on upstream
- Like the rest of `window.pdfcraft`, the function exists only with `?control` in the page address. Without `?control`, the same values are `info` in `getState()` and `pdfcraft:state` ([Host API](/changes/host-api.md)), which the Mock/Stub's client uses.
- `craft-web-api` is a git dependency and has no document-information function, so the function is added by PdfCraft's `api.rs` on the object the crate installs. It is not added if the crate's install fails.
- The values are those the engine read when the document was opened or its information last changed.

## Not verified yet
Not built or run (workers do not build). The integrator must build the wasm target and try `window.pdfcraft.documentInfo()` in the Mock/Stub.

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Report the document information to the client".
