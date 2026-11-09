---
type: Fork Change
title: Page, bookmark, print, search and link items checked against the code
description: Seven backlog items (insert blank pages, add and rename a bookmark, delete selected pages, print, search as I type, follow a link) checked criterion by criterion in the web build; one delete rule for every surface, a rename that selects the whole title, a bookmark title from the selected text, a print refused without permission everywhere, and a result list laid out only as far as it is in view.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
  - id: hosting
    resource: /hosting.md
---

# Page, bookmark, print, search and link items checked against the code

## Why
Seven items of the backlog ("Insert blank pages", "Add a bookmark", "Rename a bookmark", "Delete selected pages", "Print the document", "Search the whole document as I type", "Follow a link to another place in the document") each have criteria the product owner added from the hosting design.[^mods] Each criterion was read against the code. This record says which hold, which were fixed, and which do not hold yet. Nothing was built or run: the Rust tests listed below are written and still have to be run by the integrator.

## What it changes
- One delete rule. `DocView::pages_deletable(assembly, pages, count)` (`canvas/organize.rs`) is true when the permissions allow page changes, the pages exist and at least one page remains; `DocView::can_delete_pages(doc)` applies it to the selection or the current page. `command_enabled` (`commands.rs`) has a `page.delete` arm that calls it (`delete_pages_allowed`), and `disabled_reason` says "At least one page must remain" when assembly is allowed but the rule refuses. The Pages panel (Delete key, thumbnail menu, header menu), the Organize grid (Delete key, hover button) and the Organize pages panel (button) call the same function. Command search, the menus and the host's `runCommand` follow `command_enabled`. `disabled_reason` is now `pub(crate)` so the host call can give the reason.
- `PdfCraftApp::add_bookmark` (`commands.rs`) is New bookmark for every way to ask: the title is the selected text on one line, at most 128 characters (`one_line_title`, `DocView::bookmark_title` in `canvas/navigation.rs`), or "Untitled"; the destination is `bookmark_position` as before. The command `bookmark.add` calls it. The panel button and the page context menu still go through `bookmark_action(BmAction::New)` in `editing.rs` (see "Required outside the file list").
- A rename opens with the whole title selected (`select_all` in `panels/bookmarks.rs`, set on the frame that gives the field the focus), so after an add, from the row's menu and by double-click typing replaces the title.
- `open_print` and `print_now` (`workflows/print_ui.rs`) refuse a document that forbids printing, with the reason, so the page menu, a form button and a script cannot open the dialog. A browser print that cannot start because the job has too many sheets, a sheet without a size, or too many picture bytes now downloads the print-ready PDF, as a refused print does; `web_print_failed` returns whether it did. The download is not a save: no `pdfcraft:save`, no change to the document.
- The Search panel lays out only the result rows in view (`panels/search_ui.rs`, row heights kept in `Find::row_heights`, `canvas/find.rs`) and lists up to 20 000 results (it listed 2 000, all laid out every frame); a longer list says so.

## Criteria
"met already" names where; "fixed" says what; "not met" says why.

### Insert blank pages
| Criterion | Status |
|---|---|
| N+1 pages, the new page empty | met already: `commands.rs` `page.insert_blank` (`Edit::InsertBlankPage`) |
| Undo gives N pages | met already: one edit, one undo step (test `a_blank_page_follows_the_last_selected_or_current_page_with_its_size`) |
| After the last selected page, or the current page, with that page's size | met already: `commands.rs` `run_page_command` (`targets.last()`, crop box); the Organize pages panel does the same. The size is the crop box, not turned by the page's /Rotate. Test added in `tests/it/editing.rs` |
| `runCommand` and the editor's entries give the same result | met already: the Organize pages panel builds the same edit for the same page as the command |
| `getState().page_count` is N+1, one `pdfcraft:state` | met already (host API, `app/host_calls.rs`); the end-to-end test is the integrator's |
| Refused without assembly, with its reason | met already: `Needs::Assembly` in `command_enabled`, reason "The document's security settings don't allow this change"; the panels are disabled by `editable`. Test added in `tests/it/editing.rs` |
| `save()` bytes reopen with N+1 pages | met already (engine edit, saved with the document); the end-to-end test is the integrator's |

### Add a bookmark
| Criterion | Status |
|---|---|
| No selection: "Untitled" at the top of the view | met already: `bookmark_position` (`canvas/navigation.rs`) |
| Selected text: its title on one line, at most 128 characters, at the selection's top-left | fixed: the title was always "Untitled"; `bookmark_title` and `one_line_title` added and `add_bookmark` uses them. It applies to the command, Ctrl+B, the menu and command search at once; the panel button and the context menu use it through `editing.rs`. Unit tests in `canvas/navigation.rs`; a selection end to end is the integrator's |
| The new title is open for renaming | met already: `bookmark_rename` is set after the edit (now only when the edit succeeded) |
| Unavailable without assembly or when read-only, on every surface | met already: menu, command search and Ctrl+B follow `command_enabled` (`Needs::Assembly`); the panel button follows `bm_editable` (`panels/right.rs:26`, assembly and not read-only); the page context menu uses `perms.can_assemble` (`canvas.rs:886`, `allows_assembly()`), the same rule. Test added in `tests/it/bookmarks.rs` |
| `runCommand`, the panel button and Ctrl+B make the same bookmark | fixed for the command and Ctrl+B; not met for the panel button until `editing.rs` calls `add_bookmark` (required, below). The test `the_command_the_button_and_ctrl_b_add_the_same_bookmark` fails on a selected-text title via the button until then (with no selection all three give "Untitled" already) |
| `save()` bytes contain the outline entry | met already (engine edit); the end-to-end test is the integrator's |
| After the add `edit_generation` differs, `can_undo`, one undo removes it | met already: `Edit::AddBookmark` through `apply_edit` |
| Same commands on every surface | met already as above |

### Rename a bookmark
| Criterion | Status |
|---|---|
| Starting a rename selects the whole title | fixed: typing replaced nothing before (the tests pressed Ctrl+A first); `select_all` added. All three starts share one editor. Test `every_rename_start_selects_the_whole_title` |
| Confirmed title is shown and the bookmark opens the same page | met already: `Edit::RenameBookmark` keeps the destination |
| Whole title selected at all three starts | fixed, as above |
| No rename start without assembly | met already: menu and double-click are drawn only when `cx.editable` (`panels/bookmarks.rs`). Test `bookmarks_cannot_be_renamed_or_added_without_the_assembly_permission` |
| Escape cancels; empty keeps the old; trimmed | met already: `panels/bookmarks.rs` (Escape sets cancel before the lost focus is read; `text.trim()`, empty ignored). Tests: `escape_cancels_a_rename` (existing) and `a_renamed_title_is_trimmed_and_an_empty_one_keeps_the_old` |
| One undo step; `save()` bytes hold the new title | met already (one `RenameBookmark` edit); the end-to-end test is the integrator's |

### Delete selected pages
| Criterion | Status |
|---|---|
| Delete page 2 of 5 gives 4 pages | met already: `Edit::DeletePages` |
| Pages 2 and 4 selected, delete gives 3 | met already; test `a_selection_is_deleted_in_one_undo_step_and_survives_closing_the_panel` |
| A bookmark to a deleted page has no destination | met already in the engine (read only): `crates/organize/src/lib.rs` `delete_pages` calls `drop_destinations_to`, which removes /Dest or the GoTo action of bookmarks and links that went to a deleted page. No UI change |
| Markup handled as "Markup stays on its page" defines | not checked here: another item |
| No confirmation | met already: no prompt on any surface |
| Every page selected: unavailable, told that one page must remain | fixed: the Organize panel and the Pages panel already refused, `command_enabled` did not, and no reason said so. One rule and the reason "At least one page must remain" added. Test `delete_is_refused_when_no_page_would_remain` |
| Pages picked in the Pages panel stay selected after the panel closes | met already: the selection is `DocView::selected`; nothing clears it on close (only Escape and a plain click in the panel, and a delete). Test in `tests/it/editing.rs` |
| `runCommand("page.delete")` refused with every page selected and with no selection in a one-page document | fixed (`command_enabled` arm). The host's error text is "command page.delete is not available now"; the reason ("At least one page must remain") is in the toast only, unless `app/host_calls.rs:138` uses `disabled_reason` (required, below) |
| Page context menu: disabled in a one-page document, deletes the clicked page otherwise | not met in my files: `canvas.rs:905` enables it with `perms.can_assemble` only, so in a one-page document it is enabled and the engine refuses. worker-image-place owns `canvas.rs` and makes it follow `command_enabled`; that rule is on the selection, not on the clicked page (see below) |
| After a delete `page_count` is the new count, one `pdfcraft:state` | met already (host API); the end-to-end test is the integrator's |
| Deleting a selection is one undo step | met already: one `DeletePages` edit. Test in `tests/it/editing.rs` |
| Without assembly: unavailable on every surface, command refused with a reason | met already for the panels and the command ("The document's security settings don't allow this change"); the shared rule now includes the permission. Test `page_changes_are_refused_without_the_assembly_permission` |
| Same states on the Pages panel, Organize grid, Organize pages panel, context menu, command search, keys | fixed for all but the context menu (above): each calls `pages_deletable` or `command_enabled`. The Pages panel header menu entry reads the rule from the thumbnails' frame (egui data, `can_delete_key` in `panels/pages.rs`) because `panels/right.rs:86` passes it no view |

### Print the document
| Criterion | Status |
|---|---|
| The browser's print dialog opens with the pages in order | met already: `web_print.rs` `print_in_iframe` (`window.print()` on the frame), sheets in layout order. Not testable natively; the integrator checks it in the browser |
| The paper shows the document as in the editor, unsaved changes included | met already: `print_now` applies the user's pending edit first (`commit_form_typing`), then `Session::print_pdf` prints the current document |
| Markup prints by default; the option switches it off | met already: `PrintDraft` default `Content::DocumentAndMarkups`; the "Comments & Forms" choice (`print_ui/body.rs`). A choice made in one dialog is kept for the next one in the session |
| Pages as images at the dpi steps and caps | met already: `plan_dpi` (300, 200, 150 dpi; 64 million pixel cap; 1000 sheets) |
| Refused print downloads the print-ready PDF | met already for a browser that refuses (`web_print_failed`); fixed for a job that cannot be drawn as pictures (too many sheets, no usable size, too many bytes): it only showed a message that told the user to save the PDF; it now downloads it |
| Selected pages only | met already: `open_print` sets "Selected pages" when pages are picked; test `print_dialog_prints_the_selected_pages` (existing) |
| No print permission: button disabled with the editor's hover text; `print.dialog` refused | met already: `chrome.rs:528` uses `command_enabled` and `print_refused()`; `Needs::Printing`. Fixed: `open_print` itself had no check, so the page menu, a form button and a script opened the dialog; test `print_entry_points_outside_the_command_refuse_a_document_that_forbids_printing`. `print_restricted_documents_say_so_before_the_dialog` (existing) covers button and command. The host error text is "command print.dialog is not available now" (`app/host_calls.rs:139`) |
| A print that succeeds changes nothing; the fallback download changes nothing and sends no `pdfcraft:save` | met already: `print_pdf` takes `&self` and works on a copy; `editing::download` sends no event; nothing in the print path marks the document. The only change is the user's own pending form typing, applied before printing, which is an edit the user made. Test `printing_does_not_change_the_document` |

### Search the whole document as I type
| Criterion | Status |
|---|---|
| Matches on all pages found while typing | met already: `canvas.rs` requests every page's text while a query is set; `rerun_find`, `refresh_find_page` |
| Number of matches and which is current | met already: `canvas/find.rs` shows "{c} of {n}" |
| Columns, right-to-left, CJK found | met already in the text path: `crates/render/src/text/order.rs` (reading order of columns, right-to-left runs reordered to logical order) and `text/search.rs` (literal search in that text); render tests `rtl_runs_come_out_in_logical_order_and_cjk_has_no_spaces`, `two_columns_read_left_then_right_even_if_drawn_interleaved`. Whole-word on CJK treats letters and ideographs alike as word characters, with no word segmentation. The end-to-end test with ToUnicode maps is the integrator's |
| Case-insensitive by default; case-sensitive option | met already: `find.rs:276-279`, `render/text/search.rs` |
| Whole-word option | met already, same place |
| No other find option | met already: the bar and the Search panel offer only these two |
| Advanced search count equals n | met already: both read `find.matches.len()`. Fixed: the panel drew at most 2 000 rows while the count said all; it now lists up to 20 000 and says when it stops. Test `the_find_bar_and_the_search_panel_count_the_same_matches` |
| `runCommand("edit.find")` opens the bar, typing works | met already: `open_find` sets the focus request the bar honours next frame |
| Searching does not change the document | met already: nothing in `find.rs` or `search_ui.rs` edits; asserted in the test above |
| A late page is added to the count; the status ends without "+" | met already: `refresh_find_page` re-sorts and keeps the choice; "+" shows while `texts + text_failed < pages`; text of other pages is not let go while a query is set (`canvas/cache.rs:497`). The 100-page end-to-end test is the integrator's |

### Follow a link to another place in the document
| Criterion | Status |
|---|---|
| XYZ: position at the top-left, zoom applied | met already: `go_to_dest` (`canvas/navigation.rs`) sets the zoom (clamped 0.08 to 64) and places the point at the window's top-left (after the gutter). Existing unit test `destinations_set_the_zoom_mode_and_the_point_to_show` (`canvas/tests.rs`) |
| Fit page, fit width, fit height | met already: `Fit::Page`, `Fit::Width`, `Fit::Height`. FitV sets `Fit::Height`, which `fit_zoom` (`canvas/zoom.rs:289`) turns into (window height minus margins) over the page height. In a scrolling view the zoom fits the largest page of the document, in a single-page view the shown page, as the View menu's fit height does; so in a scrolling document a destination page shorter than the largest does not fill the height |
| Following a link changes nothing in the document | met already: `go_to_dest` touches only the view. Test `following_an_internal_link_changes_nothing_in_the_document` (`tests/it/pdf_links.rs`) |

## Done outside the first file list (integrator's permission)
- `editing.rs` `bookmark_action`, `A::New` arm: calls `add_bookmark()`, so the panel button and the page context menu give the same bookmark as the command and Ctrl+B.
- `canvas.rs`: the page context menu's Delete page is enabled by `DocView::pages_deletable(allows_assembly, one page, page count)`, the rule for the clicked page; `PageMenuPerms::can_delete_page` stays (now computed there) and the `document_area` computation of the command rule and the env field are removed.
- `app/host_calls.rs` `check_host_command`: the refusal is `command <id> is not available now: <reason>` (the bare text when the reason is empty).
- Still open: craft-i18n translations of "At least one page must remain" and "Only the first {n} results are listed" (English until added). `panels/right.rs:86` stays as is (optional).

## Not covered
- The Rust tests are written but not run; the end-to-end tests the criteria name (Mock/Stub) are the integrator's.
- A blank page takes the crop box of its neighbour; a neighbour turned by /Rotate gives a blank page that is not turned.
- The comments and forms print choice persists from one print dialog to the next in a session.

## Depends on upstream
- `Edit::DeletePages`, `Edit::AddBookmark`, `Edit::InsertBlankPage`, `Session::print_pdf` and the engine's removal of destinations to deleted pages; `PageText::find_opts`; egui `TextEdit::load_state`.

[^mods]: [Fork items of the backlog](/sources/modifications.md), the seven items named above.
