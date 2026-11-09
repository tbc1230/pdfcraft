---
type: Fork Change
title: Commands offered in the web build
description: One allowlist of the commands the web build offers, each mapped to the backlog story that needs it, followed by every surface; the desktop build is unchanged.
resource: working tree change (no commit, no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
  - id: hosting
    resource: /hosting.md
---

# Commands offered in the web build

## Why
Workspace hosts the editor, so the editor offers only commands that apply to working in Workspace. Related backlog items: "Run commands from the client" ("The editor offers only commands that apply to working in Workspace"; commands of the stand-alone application are not shown, enabled or disabled; the commands that change what Workspace sets, the interface language and the theme, are not offered), "Save and return the bytes to the client" (the editor offers no Save and no Save As), "Hide upstream interface parts Workspace does not offer" (the Tools and Inspector panels are shown only as far as actions that items of the backlog define need them; no hidden part can be opened from anywhere in the editor) and "Hide upstream functions that overlap Workspace concerns" (one list of offered commands that every surface follows).[^mods]

The first version of this change listed the commands the web build refuses. The list is turned around: the web build offers only the commands on an allowlist, and every command that no backlog story needs is refused, including a command upstream adds later.

## What it changes
- `offered(id)` (`crates/ui-egui/src/commands.rs`) is `!wasm32 || OFFERED_ON_THE_WEB.contains(&id)`: in the web build only a listed id is offered; on the desktop every command is. `OFFERED_ON_THE_WEB` replaces `NOT_OFFERED_ON_THE_WEB`, and extends the one `app.preferences` change of [preferences-hidden](/changes/preferences-hidden.md). The constant is grouped by backlog story, each group commented with the story's exact title.
- Everything the earlier refusal list refused stays refused, because none of it is on the allowlist: Save and Save As; the commands of the stand-alone application (New blank PDF, Create PDF from file, Create PDF from Multiple Files, Open Recent, Clear Recent Files, Pin Folder to Home, Tab Order, and the commands that open, create, close or locate files, manage tabs or quit); Preferences; the theme commands; the commands that only open a hidden panel (`form.fields`, `sign.panel`, `a11y.tags`, `a11y.check`, `a11y.find_untagged`, `doc.compare`, `a11y.autotag`, `form.prepare`, `sign.validate`); the network and upstream links (`help.check_updates`, `help.discord`, `help.github`, `help.website`, `help.app_page`).
- 46 commands are offered, in the table below.
- Surfaces that follow `offered()`, each checked: menus (`menus.rs`, including the View > Tools submenu and `run_menu_command`, which also serves the native macOS bar), command search (`dialogs/palette.rs`), keyboard shortcuts and their dispatch (`commands.rs`: `registry_shortcuts` filters the registry by `offered`; `dispatch_command`, the shortcuts deferred while a form field has the keyboard, and the menus all end in `execute`, whose `execute_unguarded` returns false for an id that is not offered before anything else runs), the shortcuts list (`dialogs.rs`), the hotbar (`hotbar.rs`: the tool picker, the saved tool buttons and `set_tools`), context menus, the control channel (`control/host.rs`: `ui.command` returns an error and `ui.commands` omits them; `app/control.rs`: `dialog=compare-files`), `PdfCraftApp::run_command` (`app/commands.rs`: no file picker is asked for first, and with no document a tool says "Open a document first" instead of opening a file picker in the web build) and `execute`, which returns false.
- Follows worker-hide-parts' predicates (`panels.rs`): the View > Side panels submenu skips panels `panel_offered` refuses; the `panel` control option refuses them with an error; command search and the View > Tools submenu skip tool groups `tool_listed` refuses; the page context menu's "Export this PDF" (`canvas.rs`) is not drawn when `tool_listed("export")` is false.
- The Organize pages toolbar's "Save pages" button (`canvas/organize.rs`) is not drawn, and its queued action (`editing.rs`) saves nothing, where Save is not offered.
- Tests (`commands.rs`): the list names registered commands once; every listed command is offered in both builds; every other registered command is offered only on the desktop, with `file.save`, `create.blank`, `view.theme.dark`, `app.preferences`, `form.tab_order.row` and `help.discord` named and checked as not on the list; `execute` of every registered command that is not listed returns false and opens no dialog (checked in the web build only, where it can run without side effects).
- The desktop build is unchanged.

## Offered commands, by story
The story is the backlog story or technical story (Component "PdfCraft fork" or "Workspace and PdfCraft fork") that needs the user to run the command inside the editor.

| Command id | Label | Story that needs it |
|---|---|---|
| `view.select_tool` | Select | Edit on the PdfCraft editing canvas |
| `view.hand_tool` | Hand | Edit on the PdfCraft editing canvas |
| `view.marquee_zoom` | Marquee zoom | Make pages larger or smaller |
| `view.fit_width_scrolling` | Fit to width scrolling | Fit pages to the document view |
| `view.fit_one_page` | Fit one full page | Fit pages to the document view |
| `view.layout.single` | Single page | Choose how pages are arranged |
| `view.layout.two_up` | Two-page view | Choose how pages are arranged |
| `view.continuous` | Continuous scrolling | Choose how pages are arranged |
| `view.read_mode` | Read mode | Read without distractions |
| `view.full_screen` | Full screen mode | Present the document on the full screen |
| `print.dialog` | Print... | Print the document |
| `view.focus_page_input` | Go to page... | Go directly to a page |
| `edit.find` | Find... | Search the whole document as I type; Step through the matches |
| `edit.advanced_search` | Advanced search... | List all matches |
| `view.palette` | Find tools and commands... | Find and run an action by name; Run frequent editor actions from the keyboard |
| `comment.highlight` | Highlight text | Mark a passage with text markup |
| `comment.underline` | Underline text | Mark a passage with text markup |
| `comment.squiggly` | Squiggly underline text | Mark a passage with text markup |
| `comment.strikeout` | Strikethrough text | Mark a passage with text markup |
| `comment.note` | Add a sticky note | Place a note at a point on a page |
| `comment.freetext` | Add a text box | Write a remark in a text box on a page |
| `comment.ink` | Draw freehand | Draw freehand on a page |
| `comment.square` | Draw a rectangle | Draw a shape around content |
| `comment.circle` | Draw an oval | Draw a shape around content |
| `comment.line` | Draw a line | Draw a shape around content |
| `comment.arrow` | Draw an arrow | Draw a shape around content |
| `comment.polygon` | Draw a polygon | Draw a shape around content |
| `comment.polyline` | Draw connected lines | Draw a shape around content ("each shape tool the editor provides"; integrator's decision) |
| `comment.cloud` | Draw a cloud | Draw a shape around content; Move and resize a placed object (names clouds; integrator's decision) |
| `comment.list` | Comments panel | List all markup in the comments panel |
| `edit.edit_text` | Edit text & images | Correct wording in a Latin-script paragraph |
| `edit.image` | Add image... | Place an image on a page |
| `edit.link` | Add or edit links | Add a link over an area of a page |
| `bookmark.add` | New bookmark | Add a bookmark |
| `page.organize` | Organize pages | See all pages in the Organize pages view |
| `page.delete` | Delete pages | Delete selected pages |
| `page.insert_blank` | Insert blank page | Insert blank pages |
| `page.insert` | Insert pages from file... | Insert pages from another PDF |
| `edit.undo` | Undo | Undo and redo markup changes; Undo and redo page changes |
| `edit.redo` | Redo | Undo and redo markup changes; Undo and redo page changes |
| `redact.mark` | Redact text and images | Expose the editor's redaction in the WASM library build |
| `redact.apply` | Apply redactions... | Expose the editor's redaction in the WASM library build |
| `file.properties` | Document properties... | See the document's properties |
| `edit.copy` | Copy | Redact confidential content from the result ("when the user copies the text of the page"; integrator's decision) |
| `view.tools_pane` | Show or hide the Tools panel | Hide upstream interface parts Workspace does not offer (the toggle opens and closes the panel the item trims; integrator's decision) |
| `view.side_panel` | Show or hide the side panel | Hide upstream interface parts Workspace does not offer (the toggle opens and closes the Inspector; integrator's decision) |

Basis of the less obvious rows:
- `view.palette` opens command search.
- `edit.advanced_search` opens the Search panel, which holds the find bar's search as its results list; `canvas.rs` hands the search back to the find bar when the panel closes.
- The shape rows are the shape tools "Draw a shape around content" names (a rectangle, and "each shape tool the editor provides") and "Move and resize a placed object" names (lines, arrows, polygons, clouds).
- `edit.edit_text` is the Edit text & images tool.
- `page.insert` stays offered as the integrator earlier decided; the story's source is a file the user chooses.
- `redact.mark` and `redact.apply` cover the mark types selected text, a dragged area and an image, and applying the marks.

## Commands left off
By the integrator's decision, with the reason "no story names it", each is refused in the web build:

| Command id | Label | What it does |
|---|---|---|
| `view.fit_visible` | Fit visible | Fits the visible content of the page to the view |
| `view.page_gaps` | Show gaps between pages | Toggles the gap between pages |
| `view.layout.cover` | Show cover page in two-page view | Toggles the cover page in two-page view |
| `view.previous_view`, `view.next_view` | Previous view, Next view | Go back and forward through the view history |
| `comment.freehand_highlight` | Freehand-Highlight | Draws a translucent freehand stroke |
| `comment.callout` | Add a callout | Adds a text box with a leader line |
| `comment.caret`, `comment.replace` | Insert text, Replace text | Text markup of the insert and replace kinds |
| `comment.eraser` | Erase drawings | Erases ink drawings |
| `edit.text` | Add text | Adds text to the page content |
| `edit.column_select`, `edit.snapshot` | Column select, Take a snapshot | Select a column, copy an area |
| `page.copy`, `page.cut`, `page.paste` | Copy pages, Cut pages, Paste pages | Move pages through a page clipboard |
| `redact.clear` | Clear redaction marks | Removes the redaction marks |
| `protect.properties` | Security properties... | Opens Document properties on the Security tab |
| `file.revert` | Revert | Discards the changes since the last save |
| `ui.dock.reset` | Reset side panels | Resets the panel layout |
| `help.shortcuts` | Keyboard shortcuts | Opens the shortcuts list |
| `help.about` | About PdfCraft | Opens the About dialog |
| `edit.attach`, `comment.attach` | Attach a file... | Attach a file (attachments are hidden by "Hide upstream functions that overlap Workspace concerns") |

Every other registered command is refused too; no story needs it.

## Actions of a story that no command provides
These stories are served by the canvas, the rail or a panel, not by a registered command, so the allowlist has nothing to offer for them:
- zoom in and zoom out (view keys `ZOOM_IN` and `ZOOM_OUT`, and the rail); fit height;
- view rotation (`ROTATE_CW` and `ROTATE_CCW`; the `page.rotate` commands change the document, which the story rules out);
- stepping to the next or previous page (`PAGE_PREV`, `PAGE_NEXT` and the page controls) and to the next or previous match (`FIND_NEXT`, `FIND_PREV`);
- the layers switches (the Layers panel) and the page thumbnails panel;
- moving pages and moving a selection earlier or later (dragging in the Organize pages view);
- moving, resizing and deleting a placed object or markup item, replies and explanations (the canvas and the Comments panel), renaming and following a bookmark (the Bookmarks panel), following a link, and form fields (interactive on the canvas).

"Draw a signature in Workspace", "Place a QR code or barcode on a page" and the stamp and watermark story go through the exported JS API.

## Not covered
- Interface parts in files this change does not touch can still show a control for a command that is not offered; `execute` refuses it when clicked. These belong to the hiding of interface parts and to the surfaces change: the title bar's Open, Preferences, theme, Discord and Save buttons and the Combine button (`chrome.rs`), the home screen (`home.rs`, `home/folders_ui.rs`) and the tab strip's context menu (`menus/tab_context.rs`).
- A script's `app.saveAs` and a form push button "SaveAs" (`workflows/js_ui.rs`, `canvas/area.rs`) call `run_command("file.save_as")`, which does nothing in the web build: Save As is not offered, per "Save and return the bytes to the client".
- Ctrl+S and Ctrl+Shift+S are not bound in the web build. eframe's web backend calls `preventDefault` on a Ctrl or Cmd + S key press whatever the app does with it, so the browser's Save Page dialog does not open while the canvas has the keyboard focus (eframe `web/events.rs`, `should_prevent_default_for_key`, also for O, P and the comma). With the focus elsewhere on the page the browser gets the key; that is the client's page.
- The earlier version kept some commands by the integrator's decision that the allowlist now leaves off: `edit.attach`, `comment.attach`, `file.revert`, `ui.dock.reset`, `help.shortcuts`, `help.about`, and the export and JavaScript commands (their tool groups are hidden).

## Depends on upstream
- The registry table `crates/engine/src/commands.rs` is unchanged; the list names its ids. A command id upstream renames is no longer offered in the web build until someone updates the list, and the list test fails for an id that no longer exists; a command upstream adds is not offered until someone lists it.
- `dispatch_command`, `execute_unguarded`, the menu builders and the palette keep upstream's structure; each gets one `offered` check.
- eframe's key handling in `web/events.rs` (egui fork `storytold/egui`, rev `6d518fc`) decides the Ctrl+S finding above.

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Save and return the bytes to the client", "Run commands from the client", "Hide upstream interface parts Workspace does not offer", "Hide upstream functions that overlap Workspace concerns".
