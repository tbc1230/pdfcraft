---
type: Fork Change
title: Page navigation works without the canvas focus in the web build
description: In the web build the wheel, pinch zoom, middle-button drag and auto-scroll work while the pointer is over the pages, even when the browser's keyboard focus is elsewhere in the hosting page.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: issues
    resource: /issues.md
---

# Page navigation works without the canvas focus in the web build

## Why
Issue 2 in [issues](/issues.md). The viewer's navigation required egui's `input.focused`. In a browser eframe sets that only while the canvas has the keyboard focus, so in Workspace, where focus moves between the editor and the client's own controls, the wheel, the middle button and pinch zoom did nothing over the pages until the pages were clicked. A middle press is also handled by eframe before it learns of a focus change, so the press frame counted as unfocused.

## What it changes
One rule, `navigation_has_focus` (`crates/ui-egui/src/canvas.rs`): navigation is live when the window is focused, or, in the web build (`target_arch = "wasm32"`), when the pointer is over the pages (or, for the middle button, a pan or latch is already under way).
- `navigation_enabled` (`canvas.rs`) uses it, so the wheel, pinch zoom, the presentation click and the selection edge scroll follow.
- `AutoScroll::update_latched` and `update_drag` (`crates/ui-egui/src/canvas/autoscroll.rs`) use it for the cancel and drop-the-drag checks; the press position counts as the pointer, so the press frame is live. Once a latch or pan is under way, the pointer may leave the pages.
- `PdfCraftApp::logic` (`crates/ui-egui/src/app.rs`), through the helper `middle_scroll_stops`: the per-frame cancel of every view's auto-scroll no longer fires on missing focus in the web build (a modal or the palette still cancels it). Without this the latch was cancelled on the next frame.
- Tests: `crates/ui-egui/src/canvas/tests.rs` for the rule; `crates/ui-egui/src/canvas/autoscroll.rs` for the middle drag and the latch without focus in a browser, and for the desktop staying as before.
- The desktop build is unchanged.

## Deliberately left on the real focus
- Keyboard input (shortcuts, text fields, page input) and egui's own keyboard focus.
- `PdfCraftApp::logic` `blocked` for `view.objects.set_input_blocked` (`app.rs`): it blocks clicks on page objects, not navigation. In the web build the first click on an object while another control holds the focus is therefore still ignored; not part of Issue 2, not changed.
- `attend_save_prompt` (`crates/ui-egui/src/editing/close.rs`): asks the operating system for the user's attention for a window in the background; no meaning in a browser.

## Not changed
The canvas does not take the browser focus on the wheel, so a Workspace text field keeps its focus while the user scrolls the pages.
