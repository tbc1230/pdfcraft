---
type: Issues
title: Issues found in the editor
description: Problems found while testing the editor's WASM in the Mock/Stub, with steps, cause and the backlog they go against. Paths starting with workspace/ are in the Mock/Stub, D:\tmp\aac\pdfcraft-main\workspace.
status: draft
---

# Issues

## 1. Opening another document does not show it in the editor

- Found: 2026-11-06, in the Mock/Stub (`workspace/`), with PdfCraft's existing WASM build (`apps/pdfcraft-web`, fork at `f42fda73`).
- Component: PdfCraft fork.

### Observed
Clicking a document in the document list of `/editor/` sends it to PdfCraft with `pdfcraft:open`. The first document opens and is shown. Every later document opens in a new tab in the background: the editor keeps showing the first document. Opening a document that is already open adds a second tab for it.

### Steps
1. Start the Mock/Stub, open `/editor/`, log in as `alice`.
2. Click Contract: Contract is shown.
3. Click Invoice: an Invoice tab is added; Contract is still shown.
4. Click Contract: a second Contract tab is added; the first Contract tab is still shown.

The page sent all three documents (three `pdfcraft:open` events with the right names and PDF bytes). Screenshots: `workspace/.debug/0-Contract.png`, `1-Invoice.png`, `2-Contract.png`; script: `workspace/.debug/repro.py`.

### Cause
- `crates/ui-egui/src/app.rs:493` sets `startup_superseded` as soon as a document view exists.
- `crates/ui-egui/src/app.rs:542` opens each file from `startup_inbox` with `activate = !self.startup_superseded`, so every file after the first opens without being activated.
- `apps/pdfcraft-web/src/main.rs` (`queue_open`) puts host opens (`pdfcraft:open`) into the same `startup_inbox` as a `?file=` URL at startup. The rule that keeps a late startup file from taking the focus therefore also applies to every host open.

### Backlog
The backlog has one document open in the editor at a time; opening another document replaces it:
- Terms: "Document: the PDF open in the editor"; the editor "performs editing on the open document".
- "Hide upstream interface parts Workspace does not offer" (PdfCraft fork): "The editor does not show the home screen, the tab strip, the All tools catalogue or Preferences."
- "Show whether the document is signed": "When the user opens another document, then the state shown is that of the newly opened document."
- "Report why a document could not be opened": "Given a document is already open and opening another one fails, Then the already open document stays open and unchanged."
- "Combine several files in chosen order": several files become "one document" in the editor.

### Tests
The e2e tests in `workspace/tests/` open one document per page and do not cover opening a second document.

### Status
Open, partly fixed. The fork change `host-open-shown` (`fork/changes/host-open-shown.md`, in the working tree, not committed) shows each document the page opens; its e2e test in `workspace/tests/test_fork_functions_e2e.py` passed on 2026-11-07. Still open: the previous document is not replaced; it stays open in a tab that is not shown, so opening another document does not replace it as the backlog has it. The e2e test checks only which document is shown.

## 2. The mouse wheel does not scroll the document until it is clicked

- Found: 2026-11-07, in the Mock/Stub (`workspace/`), with PdfCraft's existing WASM build (`apps/pdfcraft-web`, branch `wasm-bundle` with the uncommitted worker changes).
- Component: PdfCraft fork.

### Observed
With a document open in the editor, turning the mouse wheel over its pages does nothing. After a click on the pages, the wheel scrolls. After a click anywhere in the page outside the editor (document list, actions, right panel), the wheel stops scrolling again until the pages are clicked.

### Steps
1. Start the Mock/Stub, open `/editor/`, log in as `alice`.
2. Click Contract, move the pointer over its page, turn the wheel: nothing scrolls.
3. Click the page, turn the wheel: it scrolls.
4. Click Invoice in the document list, move back over the page, turn the wheel: nothing scrolls.

### Cause
- `crates/ui-egui/src/canvas.rs:2998`: `navigation_enabled` requires `ui.input(|i| i.focused)`; the wheel (`wheel_enabled`, line 3020) and pinch zoom (line 3000) depend on it.
- On the desktop app `focused` is the window's focus. On the web, eframe sets it only while the canvas (or its text agent) has the browser's keyboard focus (`eframe/src/web/app_runner.rs:241-256`, egui checkout `6d518fc`). eframe's wheel listener on the canvas passes every wheel event on with no focus check (`eframe/src/web/events.rs:836`); PdfCraft drops them.
- The web build gives the canvas no focus (`apps/pdfcraft-web/index.html`, `apps/pdfcraft-web/src/`), so upstream's web build behaves the same.

### Backlog
- "Read every page of the document": "Given a document is open in the editor, when the user scrolls from the first page to the last page, then each page of the document is displayed in document order."
- "Work in the editor as one part of Workspace": the editor is one part of the layout, so focus moves between Workspace's controls and the editor all the time.

### Resolution
In the web build, page navigation does not depend on the canvas having the keyboard focus. The wheel, pinch zoom, middle-button drag and auto-scroll act on the pages while the pointer is over them, whatever has the browser's focus. Keyboard input still goes to the editor only while it has the focus. The desktop app keeps the window's focus as its rule. (Integrator, 2026-11-07.)

### Tests
The e2e tests in `workspace/tests/` do not use the mouse wheel.

### Status
Closed. Fixed by the fork change `wheel-without-canvas-focus` (commit `f706da14`); the fix was verified by the product owner in the Mock/Stub before 2026-11-09 (recorded 2026-11-09).

## 3. Arabic letters are shown disconnected in the WASM build

- Found: 2026-11-07, in the Mock/Stub (`workspace/`), with PdfCraft's existing WASM build (`apps/pdfcraft-web`, built with `CRAFT_I18N_DIR` and `CRAFT_I18N_REQUIRED=1`, without `CRAFT_FONTS_DIR`).
- Component: PdfCraft fork.

### Observed
With the interface language set to Arabic, the editor shows the Arabic texts, but their letters are drawn disconnected (each letter in its isolated form) instead of joined. PdfCraft itself does not show the problem (product owner). The product owner's screenshot of the build with `CRAFT_I18N_DIR` and without `CRAFT_FONTS_DIR` showed it; the screenshot was removed from the repository on 2026-11-08.

### Steps
1. Start the Mock/Stub, open `/editor/`, log in as `alice`.
2. Choose Arabic in the language selector.
3. Look at any Arabic text in the editor: the letters are not joined.

### Cause
- Every build of the WASM so far set `CRAFT_I18N_DIR` but not `CRAFT_FONTS_DIR`, and there is no craft-fonts checkout next to the fork. craft-fonts holds PdfCraft's Arabic interface faces (`ui_arabic_fonts`, `crates/fonts/src/craft.rs:46-49`: "Empty when built without craft-fonts"); `crates/fonts/build.rs:99-108` keeps them in the web build when craft-fonts is given.
- The desktop app can fall back to an Arabic font installed on the machine (`crates/ui-egui/src/platform/system_fonts.rs:1-9`: "such as an Arabic file name in a build without craft-fonts"); the web build cannot read installed fonts.
- Without an Arabic face, the shaper finds no glyph for the Arabic letters and asks for each grapheme cluster, one letter, separately (`epaint/src/text/text_layout.rs:498-520`, egui checkout `6d518fc`). eframe's web build draws such clusters with the browser's own font (`eframe/src/web/app_runner.rs:50`, `eframe/src/web/canvas_glyphs.rs:67-120`). A letter drawn alone has no neighbours, so the browser draws its isolated form.
- With craft-fonts, the web build has Noto Sans Arabic in every font family (`crates/ui-egui/src/theme.rs:388-405`); Arabic text is then shaped as whole runs with the font's joining rules (`epaint/src/text/text_layout.rs`, `layout_shaped_run`).

### Resolution
The WASM is built with craft-fonts: `CRAFT_FONTS_DIR` set to a craft-fonts checkout and `CRAFT_FONTS_REQUIRED=1`, as `CRAFT_I18N_DIR` and `CRAFT_I18N_REQUIRED=1` are for the translations. No code change.

### Backlog
- Workspace supports English, French and Arabic, right to left (product owner).
- "Set the interface language of the editor": "every text the editor shows to the user is shown in that language where the editor has a translation for it."

### Tests
The e2e language test checks that `ar` takes effect, not how the text is drawn.

### Status
Fixed, confirmed by the product owner in the Mock/Stub (2026-11-08).

## 4. Control+Page Down scrolls instead of stepping when Ctrl is released in the same frame

- Found: 2026-11-08, in the Mock/Stub (`workspace/`), by the e2e test `workspace/tests/test_navigation_e2e.py::test_the_keys_step_pages_after_a_click_on_the_pages`, with the WASM built on 2026-11-08 at 17:27 (branch `wasm-bundle` with the uncommitted worker changes). Investigated by worker-nav-inspect (Sonnet).
- Component: PdfCraft fork (upstream key handling; no fork change involved).

### Observed
With continuous scrolling, Control+Page Down and Control+Page Up step one page when the keys arrive with time between them. When the Ctrl key-up reaches the editor in the same frame as the Page Down or Page Up key, the editor scrolls by a screen instead of stepping one page, and the page position ends one page off.

### Steps
1. Open a document of several pages in the editor, click the pages.
2. Send Ctrl down, Page Down down, Page Down up and Ctrl up within one frame (a test driver does this; worker-nav-inspect synthesised the events in the page).
3. The view scrolls by a screen instead of stepping to the next page. With Ctrl released about 300 ms later, the same keys step one page.

### Cause
- `crates/ui-egui/src/canvas/navigation.rs:431` reads Ctrl as `ctx.input(|i| i.modifiers.command)`, the modifier state when the frame runs, not the state carried by the key event. Page Down and Page Up step a page only when `command` is true or the view does not scroll (`navigation.rs`, the `ArrowRight` / `PageDown` and `ArrowLeft` / `PageUp` branches); otherwise they scroll a screen.
- eframe's web build updates the modifiers on every key-down and key-up (`eframe/src/web/events.rs`, egui checkout `6d518fc`, as read by worker-nav-inspect), so a frame that receives the Ctrl key-up as well sees Ctrl as not held.

### Backlog
- "Step to the next or previous page": "Stepping works with the editor's page controls, and with the keyboard after the user has clicked the pages."

### Tests
The e2e key test waits for each step before sending the next key (`wait_until` in `workspace/tests/conftest.py`), so it does not send keys faster than frames. No test sends a key and the Ctrl release in one frame.

### Status
Open. Only keys sent faster than frames trigger it; a person's Ctrl release comes tens of milliseconds after the key. Not changed: upstream key handling, and no criterion of the backlog requires it.

## 5. The Print dialog does not fit a small editor area, and its Print button cannot be reached

- Found: 2026-11-09, in the Mock/Stub (`workspace/`), by the e2e tests in `workspace/tests/test_item_print.py`, with the WASM built on 2026-11-08 at 21:14 (branch `wasm-bundle` with the uncommitted worker changes). Confirmed by the product owner in Chrome.
- Component: PdfCraft fork (the editor's Print dialog); not investigated further.

### Observed
In a 1280 x 720 browser window the Mock/Stub gives the editor a canvas of 760 x 596 points. The editor's Print dialog is taller and wider than that: its top and left edges are cut off, and its Print button lies at y 627-655, below the bottom of the canvas. The button cannot be clicked. The product owner had to zoom the browser page out to reach it; the browser's print dialog then opened with the right pages.

### Steps
1. Open the Mock/Stub in a 1280 x 720 window and open a document.
2. Run Print (`runCommand("print.dialog")` or the toolbar's Print).
3. The Print dialog's Print and Cancel buttons are below the editor area.

### Cause
Investigated by worker-print-dialog-inspect (Sonnet, read only), 2026-11-09. Upstream code; the fork's print changes (`fork/changes/page-items.md`) do not touch the layout.
- The dialog is an `egui::Modal` (`widgets::show_modal`, `crates/ui-egui/src/dialogs.rs:130`) centred on the screen, with no size limit and no `constrain`. A card larger than the screen is placed with a negative left and top.
- Its width is fixed at 820 points (`dialogs.rs:131-133` and `workflows/print_ui/body.rs:92`), about 860 with the card padding: a settings column of 470 (`body.rs:101`) and a preview of 320 x 380 (`print_ui.rs:21-22`) side by side.
- The body has no scroll area; the Print and Cancel row is plain content at its bottom (`body.rs:535-546`).
- Preferences solves the same problem: its body scrolls within the screen height and its OK row stays outside the scroll (`dialogs.rs:1371-1405`, test `tests/it/preferences_layout.rs`). The Print dialog does not follow it.
- Narrowest fix proposed by the inspector (not made): cap the width to the screen, stack the preview under the settings when narrower than the two columns, and put the body in a vertical scroll area capped to the screen height with the button row outside it; a new test in `tests/it/print.rs` at about 760 x 596.

### Backlog
- "Print the document".

### Tests
`workspace/tests/test_item_print.py` uses a 1700 x 1300 window, in which the dialog fits. No test checks the dialog at a smaller size.

### Status
Open. Whether the dialog must fit a smaller editor area is the product owner's decision.

## 6. Characters inserted one at a time end up in reverse order in a text field

- Found: 2026-11-09, in the Mock/Stub (`workspace/`), by the e2e test `workspace/tests/test_item_search.py::test_c3_columns_right_to_left_and_cjk_text`, with the WASM built on 2026-11-08 at 21:14.
- Component: PdfCraft fork web build (text input); not investigated.

### Observed
Playwright's `keyboard.type` sends characters that are not on a US keyboard as text insertions, one per character. In the find bar, Arabic "نص" arrives as "صن" and Japanese "検索" as "索検": each inserted character goes in front of the previous one. Latin text typed as key presses arrives in order. A whole word inserted at once arrives in order and is found.

### Steps
1. Open a document, run `edit.find`.
2. In Playwright, `page.keyboard.type("検索")`.
3. The find field holds "索検".

### Cause
Not investigated. Probably the cursor is not moved past text that arrives as an insertion.

### Backlog
- "Search the whole document as I type" (typing a query in a right-to-left or CJK language).

### Tests
`test_item_search.py` inserts a non-ASCII query as one insertion, as an input method commits a word. Not checked with a real Arabic keyboard layout or a real input method, which may deliver characters differently.

### Status
Open. Whether a person typing with an Arabic layout or a CJK input method sees it is not known; the product owner can check by hand.
