---
type: Fork Change
title: Interface parts Workspace does not offer hidden in the web build
description: The web build shows no home screen, no All tools catalogue view or tab, only five side panels, and a Tools pane trimmed to the four tool groups the backlog needs; the desktop build is unchanged.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Interface parts Workspace does not offer hidden in the web build

## Why
Backlog item "Hide upstream interface parts Workspace does not offer": the editor offers the parts a backlog item uses and does not show the home screen, the All tools catalogue or the panels no item defines; the Tools and Inspector panels are shown only as far as backlog actions need them.[^mods] This record is the rest of that item; the tab strip, Preferences, the closed All tools sidebar and the first six hidden tool groups are in [tab-strip-hidden](/changes/tab-strip-hidden.md), [preferences-hidden](/changes/preferences-hidden.md), [all-tools-sidebar-hidden](/changes/all-tools-sidebar-hidden.md) and [all-tools-hidden-groups](/changes/all-tools-hidden-groups.md). The product owner decided that with no document open the page area is empty, with no actions and no messages, because Workspace owns every message and decides what is opened ([hosting](/hosting.md)).

## Terms in upstream's code
- All tools catalogue: the tool table `TOOL_GROUPS` (`crates/engine/src/catalog.rs`) shown as a list of tool groups. It appears in two places: the All tools list in the left pane (`LeftPanel::AllTools`, `all_tools` in `panels/tools.rs`) and the expanded All tools view over the page area (`all_tools_full`, `panels/all_tools_ui.rs`), plus the All tools tab of the mode bar (`Mode::AllTools`).
- Tools panel: the left pane titled "Tools" (`Pane::Tools` in `panels/side_panels.rs`). It holds either the All tools list or the panel of one tool group (`LeftPanel::Tool(id)`, `tool_detail`, the Edit panel, the Organize pages panel).
- Inspector: the right pane titled "Inspector" (`Pane::Side`); it holds the one open `RightPanel` (Comments, Bookmarks, Pages and the others) next to the rail.
- Search (Advanced search results): the Inspector panel `RightPanel::Search` (`panels/search_ui.rs`). It lists every match of the find bar's search with its context and goes to the match on a click; the find bar has no list of its own (`canvas/find.rs` hides the bar while the panel is open). It is the "results list" of "List all matches", so it is kept.

## What it changes
- No home screen: `home::show` (`crates/ui-egui/src/home.rs`) returns at once under `cfg!(target_arch = "wasm32")`; the central panel (`app.rs`) already fills the page area with the pasteboard, so it stays empty.
- No Tools pane without a document: `open_panes` (`panels/side_panels.rs`) leaves the Tools pane out in the web build while no document is open, because choosing a tool then asks for a file.
- No All tools catalogue view: `side_panels::show` clears `all_tools_full` every frame in the web build, `all_tools_ui::show` returns at once and clears it, and the sidebar's Expand button is not drawn (`all_tools` in `panels/tools.rs`). Only the Edit tab in the mode bar (`mode_tab_offered` in `chrome.rs`): no All tools tab, and no Convert or E-Sign tab, which open Export a PDF and Fill & Sign, both hidden. The web build is not in `Mode::AllTools` at all (see below).
- The Tools pane lists only the four tool groups of the table below (`WORKSPACE_HIDDEN_TOOLS` and `tool_listed`, `panels.rs`). `tool_listed` is also false in the web build for a group none of whose tools runs a command `offered()` accepts, so a group with nothing to run is not shown; inside a shown group a tool shows only when `offered(tool.command)`, a section with no such tool has no heading, and the Edit panel's rows and the Redact footer follow `offered()` too (`panels/tools.rs`, `panels/edit.rs`). A tool group that is not listed is not opened from anywhere: `select_tool` ignores it, the `tool` control option refuses it, and one reached some other way (a saved choice) is replaced by the Tools pane's home (`left_panel`).
- No All tools catalogue, also not the list: the Tools pane's group list is `LeftPanel::AllTools`, drawn by `all_tools` titled "All tools", which is the catalogue (`TOOL_GROUPS`) as a list (see the terms above), so the web build never draws it. The web Tools pane starts from the Edit panel (`LeftPanel::Tool("edit")`, `tools_home` in `panels.rs`); every Back arrow of a tool panel, the Organize pages panel's Back and Close and `select_mode` go there, and `left_panel` replaces a stored `LeftPanel::AllTools` by it. The Edit panel's Other options links to every other listed group (Add comments, Redact a PDF), so each listed group is still reachable; the desktop keeps Combine files, Redact a PDF and Prepare a form there.
- The web build is never in `Mode::AllTools`: it has two workspaces, Edit and Read (`mode_in`, `offered_mode` in `panels.rs`; Convert and E-Sign also map to Edit, because they open Export a PDF and Fill & Sign). It starts in Edit and opens documents in Edit (`default_mode_in`: the saved `default_mode` and the control option `default-mode` cannot change it; `app.rs`, `app/prefs.rs`), with the Tools pane closed as before. Read is the only other mode (the story "Read mode"); leaving it (`view.read_mode`) returns to Edit. Edit was chosen because it is the only mode with the Tools pane of the listed groups beside the document: Read hides the tools, and the other three are the catalogue and two hidden groups. Every path to a mode goes through `enter_mode`, `select_mode` or the checks below: `enter_mode` leaves the current mode (or, from Read, becomes Edit without turning Edit text on) when asked for one the build lacks; `expand_all_tools` does nothing; the `mode` control option returns an error for such a mode, `default-mode` for anything but edit, `tools=full` an error; `side_panels::show` also maps `app.mode` every frame and clears `all_tools_full`; `app/files.rs` maps an explicit mode override.
- No document open: the page area is empty and the editor shows no message. `home::show` draws nothing, `open_panes` leaves the Tools pane out, and `run_command` does nothing, with no toast, for a command that needs a document (it used to say "Open a document first" or ask for a file), and for an id that is not a registry command (it used to say "ships in milestone").
- Side panels: in the web build only Comments, Bookmarks, Pages, Layers and Search are offered (`panel_offered` in `panels.rs`, the one predicate for the side panels). The rail has no button for the others (`right_rail` in `chrome.rs`), and `side_panels::show` closes any other panel that something opened (a command, a notice, a restored choice) before the rail and the panes draw, and resets `last_right` so F4 does not reopen one. Fields, Attachments, Signatures, Accessibility, Tags and Compare are therefore not shown; form fields stay interactive on the page canvas, which this change does not touch.
- Tests: `panels::tests` (the side panels offered; the hidden ids are catalogue groups, listed once; the web build keeps exactly edit, organize, comment and redact; the desktop build lists every group), `chrome::mode_tab_tests`. They check the choice functions with a `web` argument, so they run natively.
- Every surface that shows or runs a command asks `crate::commands::offered` (the web build allows 46 commands, `OFFERED_ON_THE_WEB`, `commands.rs`) and is not drawn when it is refused. Checked and changed: the menus (`menus.rs`: registry items by `offered`, the Fill & Sign submenu, Cut and Paste, the Window menu and the list of open documents), command search (`dialogs/palette.rs`), the hotbar (`hotbar.rs`: Select, Hand, each comment tool and group, Edit text & images, the whole Fill & Sign button, and the settings list), the mode bar (Print, Document properties, Save), the title bar's buttons, the rail (`chrome.rs`: Previous and Next view, the Page display and View menus), the page context menu, the comment and image context menus (`canvas.rs`, `tools/comments/menu.rs`, `tools/edit_text_ui/image.rs`), the Pages panel and the Organize pages grid (menus, hover buttons, gap buttons, Delete, copy, cut and paste keys, drag to copy), the Organize pages panel (rotate, delete, extract, Insert, Replace, Split and the rows; a planned row is not drawn), the Bookmarks, Comments and Pages panels' menus (`panels/right.rs`, `panels/comments_panel.rs`, `panels/pages.rs`), the Edit panel and the tool panels, and the All tools search by command names. Rotate view, zoom, the page box, Next and Previous page and the fit toggle are view controls with stories and stay. Not drawn in the web build and unreachable: the tab strip and its menu, the home screen, Combine files (its commands are not offered; its menus show no command).
- `tab_strip` (`chrome.rs`) returns at once in the web build, so its Open, Combine, Preferences, theme and Discord buttons and the tab menu cannot draw even if it is called. The home screen's open, recent and pin controls are not drawn because `home::show` returns first; `home/folders_ui.rs` is called only from there, and only in the desktop build.
- The desktop build is unchanged.

## Tool groups in the Tools pane of the web build
The Tools pane is shown only as far as backlog actions need it. A group is kept when a story needs one of its tools; the tools in a kept group that no story needs are listed below, and are not hidden one by one.

| Tool group (id) | Web build | Backlog story that needs it |
|---|---|---|
| Export a PDF (export) | hidden (product owner) | none |
| Fill & Sign (fill_sign) | hidden (integrator's decision) | none: "Draw a signature in Workspace" has Component Workspace and draws the signature in Workspace's canvas; "Place the drawn signature image" places it through the exported JS API, not through Fill & Sign or upstream's signature pad |
| Edit a PDF (edit) | kept | "Correct wording in a paragraph", "Place an image on a page", "Add a link over an area of a page", "Insert blank pages", "Insert pages from another PDF", "Delete selected pages", "See all pages in the Organize pages view" |
| Create a PDF (create) | hidden (product owner) | none |
| Combine files (combine) | hidden | none: "Expose Combine to Workspace" reaches Combine through the exported JS API; the editor's own Combine picks files itself, while the client passes in the files to combine ([hosting](/hosting.md)) |
| Organize pages (organize) | kept | "See all pages in the Organize pages view", "Move pages to a new position", "Delete selected pages", "Undo and redo page changes", "Insert blank pages", "Insert pages from another PDF" |
| Add comments (comment) | kept | "Mark a passage with text markup", "Place a note at a point on a page", "Write a remark in a text box on a page", "Draw freehand on a page", "Draw a shape around content", "List all markup in the comments panel" |
| Scan & OCR (scan) | hidden | none |
| Protect a PDF (protect) | hidden | none: "Hide upstream functions that overlap Workspace concerns" removes adding and removing passwords; "Honour the restrictions set inside a PDF" only reads them |
| Redact a PDF (redact) | kept | "Expose the editor's redaction in the WASM library build" |
| Compress a PDF (compress) | hidden | none |
| Prepare a form (form) | hidden | none: the Fields panel is not shown until an item defines form filling |
| Add a stamp (stamp) | hidden (product owner) | none: "Place a stamp or watermark received from the client" is passed in by the client |
| Use a certificate (certificate) | hidden | none: upstream digital signing is hidden, "Draw a signature in Workspace" is not a digital signature |
| Use print production (print_production) | hidden | none |
| Measure objects (measure) | hidden | none |
| Compare files (compare) | hidden | none |
| Use guided actions (actions) | hidden | none |
| Prepare for accessibility (accessibility) | hidden | none |
| Apply PDF standards (standards) | hidden (product owner) | none |
| Add search index (search_index) | hidden | none |
| Use JavaScript (javascript) | hidden (product owner) | none |
| AI assistant (ai) | hidden (product owner) | none |

Tools in a kept group that no story needs (still shown, a decision of the product owner could hide them one by one):
- Edit a PDF: Rotate pages (the backlog turns the view only), Extract pages, Text (the text box is Add comments), Header and footer, Watermark, Bates numbering, Background, Attach file (attachments are hidden by "Hide upstream functions that overlap Workspace concerns"). The Edit panel's Other options rows for Combine files and Prepare a form go with their groups; the row for Redact a PDF stays.
- Organize pages: Rotate, Extract, Replace, Split, Set page boxes, Crop pages, Duplicate pages, Reverse pages, Page labels, Page transitions.
- Add comments: Cloud, Stamp (both planned in upstream), Flatten comments.
- Redact a PDF: Redact pages, Find text and redact, Set properties, Sanitize document ("Expose the editor's redaction" names the mark types selected text, a dragged area and an image).

## Inspector
The Inspector has no list of tool groups; its content is the open side panel. It is trimmed by the side panel rule above: Comments, Bookmarks, Pages, Layers and Search are kept; Fields, Attachments, Signatures, Accessibility, Tags and Compare are not offered. The Inspector's Comments, Bookmarks and Pages panels (and the Layers and Search panels) show only the commands `offered()` accepts: their header menus, card menus, thumbnail menu and buttons.

## Not covered
- With no document open the mode bar still shows the menus (`menus.rs`), the Edit tab, Print and Document properties (disabled) and the "Find tools and commands" box, which opens command search (`dialogs/palette.rs`); the rail and the Inspector are not drawn. Combine files still shows over the page area when `combine_showing()` is true (`combine_ui::page`, `app.rs`). Toasts and dialogs still appear.
- The command search box and the Edit tab are the chrome that remains with no document, with the menus (the commands that need a document are disabled), Print and Document properties (disabled), the Shortcuts and About dialogs from the Help menu, and the "Find tools and commands" box; toasts for opening a file that failed (`app.rs`) are not changed here.
- `commands.rs` (not changed here): `execute` shows the reason a command is disabled, including "no document", when something calls it directly with no document open; the surfaces above never do.
- The hidden tool groups remain reachable through commands, command search, the Tools menu and the page context menu's Export PDF (`canvas.rs`) until `offered()` hides their commands ([all-tools-hidden-groups](/changes/all-tools-hidden-groups.md), "Not covered"). The commands that only open a hidden side panel (`form.fields`, `sign.panel`, `a11y.tags` and the like) are for `offered()` to hide; this change only closes the panels themselves. `edit.advanced_search` opens Search, which is kept.
- The Mode `Mode::Edit` is entered at start with the Select tool, not Edit text & images; the Edit tab turns that on, as before.

## Depends on upstream
- `TOOL_GROUPS` group ids; `RightPanel` variants; `LeftPanel`, `Mode` and the single place that draws the Tools pane and the Inspector (`side_panels::show`).
- Upstream code that sets `app.right` or `app.left` directly stays as it is; the normalising in `side_panels::show` and `left_panel` is what keeps the hidden parts closed.

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Hide upstream interface parts Workspace does not offer".
