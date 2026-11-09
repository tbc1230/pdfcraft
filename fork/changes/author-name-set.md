---
type: Fork Change
title: Author name set by the host for markup
description: The web page sets the author name with window.pdfcraft.setAuthor(name); new markup items and replies record it, and the editor cannot change it.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Author name set by the host for markup

## Why
Backlog item "Set the author name for markup": the exported JS API has a function that takes a display name and sets it as the author name; every markup item and reply the user creates records it; the editor cannot change it.[^mods]

## What it changes
- `PdfCraftApp::set_author(name)` (`crates/ui-egui/src/app/prefs.rs`): trims the name, cuts it to `MAX_AUTHOR_CHARS`, refuses a blank one (returns false), otherwise sets `comment_prefs.author` (which every new annotation, reply, status and stamp already reads) and sets `author_locked`.
- While `author_locked` is set: the Preferences "Name on new comments" field and the comment properties Author field are disabled (`dialogs/preferences.rs`, `dialogs/comment_props.rs`); `restore` ignores a saved author (`app/prefs.rs`); the control option `author` returns an error (`app/control.rs`).
- `PdfCraftApp::author_inbox` (`hooks.rs`, `lib.rs`, `app.rs`): a slot the page writes to; the app takes it at the start of each frame and calls `set_author`.
- `install_set_author` (`apps/pdfcraft-web/src/main.rs`): puts `setAuthor(name)` on `window.pdfcraft` (the object is created when absent; `api::install` keeps existing functions). Returns false for a blank or non-string name, true otherwise; the name takes effect on the next frame.
- The desktop build is unchanged unless something calls `set_author`.

## Depends on upstream
- The function is not a `#[wasm_bindgen]` export: the workspace sets `unsafe_code = "forbid"`, which the macro's generated code would likely violate. It is a closure installed with `js-sys`, like the rest of the page API.
- The planned library crate (cdylib, wasm-bindgen) does not exist yet; the exported function belongs there when it does, calling `set_author` directly. Open point.
- The `?author=` query parameter (upstream) still sets a default name when the user has not chosen one; it does not lock.
- Documents already saved keep the author they were saved with; only new items use the name.

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Set the author name for markup".
