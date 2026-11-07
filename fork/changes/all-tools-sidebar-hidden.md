---
type: Fork Change
title: All tools sidebar hidden by default in the web build
description: The web build starts with the All tools sidebar closed, at every start; the user can still open it.
resource: commit 51bff10e on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# All tools sidebar hidden by default in the web build

## Why
The editor shows the document, not upstream's tool catalogue, when it opens. Related backlog item: "Hide upstream interface parts Workspace does not offer" ("The editor does not show ... the All tools catalogue").[^mods] This change covers only the default: the catalogue still opens from the All tools tab.

## What it changes
- `PdfCraftApp::new` (`crates/ui-egui/src/app.rs`): `left_open` starts as `cfg!(not(target_arch = "wasm32"))`, so false in the web build.
- Preference restore (`crates/ui-egui/src/app/prefs.rs`): in the web build, the saved Tools pane state (`side_panels::restore(..).tools_open`) is not applied, so the sidebar is closed at every start.
- The desktop build is unchanged.

## Depends on upstream
- The Tools pane shows when `left_open` is true and the mode is not Read (`open_panes` in `crates/ui-egui/src/panels/side_panels.rs`).
- Anything that calls `reveal_tools()` (the All tools tab, a tool chosen from command search, `view.tools_pane`) opens it again; this is intended.

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Hide upstream interface parts Workspace does not offer".
