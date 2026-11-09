---
type: Fork Change
title: A document opened by the host is shown
description: In the web build a document the host opens becomes the active document instead of a background tab.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
---

# A document opened by the host is shown

## Why
Task "Show a document opened by the host". Opening another document added a background tab instead of showing it, and the tab strip is hidden in the web build[^tabs], so the user had no way to reach it.

## What it changes
- `PdfCraftApp::ui` (`crates/ui-egui/src/app.rs`): files taken from `startup_inbox` are opened with `activate = !self.startup_superseded || cfg!(target_arch = "wasm32")`. In the web build every file from that queue (host bridge, `?file=`) is activated.
- The desktop build is unchanged: a startup file still does not take over once the user has done something (`startup_superseded`).

## Not closed by this change
- The previously open document stays open in another tab, now hidden in the web build. The open issue (one document open at a time) remains.

[^tabs]: [Tab strip hidden in the web build](/changes/tab-strip-hidden.md).
