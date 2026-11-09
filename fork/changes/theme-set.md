---
type: Fork Change
title: Theme set by the host
description: The web page sets the editor's theme with window.pdfcraft.setTheme("light" | "dark" | "system"); the editor follows the browser's setting by default.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Theme set by the host

## Why
Backlog item "Set the editor's theme": the exported JS API has a function that takes one of light, dark and follow the system and sets the editor's theme to it; the editing canvas is shown in that theme.[^mods]

## What it found
- The editor already follows the browser's setting without any call: a fresh install has `ThemePreference::System`, and `sync_theme` (`crates/ui-egui/src/app/frame.rs`) resolves it from `ctx.system_theme()`, which eframe's web backend reads from `prefers-color-scheme`. This is from reading the code; it has not been run in a browser. A saved choice from the user's earlier visit (restored settings) overrides it until the host calls `setTheme`.

## What it changes
- `PdfCraftApp::theme_inbox` (`hooks.rs`, `lib.rs`, `app.rs`): a slot the page writes to; at the start of each frame the app takes it, sets `theme_preference`, and the existing `sync_theme` applies the colours (the same path as the Appearance choice and the `view.theme.*` commands).
- `install_set_theme` (`apps/pdfcraft-web/src/main.rs`): puts `setTheme(theme)` on `window.pdfcraft` (the object is created when absent; `api::install` keeps existing functions). `theme` is "light", "dark" or "system"; anything else returns false and changes nothing; otherwise it returns true and the theme applies on the next frame.
- The desktop build is unchanged.

## Depends on upstream
- Not a `#[wasm_bindgen]` export, for the same reason as `setAuthor` (see [author-name-set](/changes/author-name-set.md)): `unsafe_code = "forbid"`. It belongs in the planned library crate when that exists. Open point.
- The editor's own theme controls (title bar, View menu) are still offered in the web build, so a user can change the theme after the host set it; the backlog item does not ask to lock it. Open point for the product owner.
- The helper that finds or creates `window.pdfcraft` is repeated from `install_set_author`; it can be shared when both changes are integrated.

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Set the editor's theme".
