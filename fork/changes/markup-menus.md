---
type: Fork Change
title: The comments panel and menus offer what the criteria allow
description: No reply on a text box, only Edit, Reply, Properties and Delete in the card and context menus, no checkmark button or status and checkmark filter, no Copy text on a reply, and tests for locks, permissions and the author field.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
---

# The comments panel and menus offer what the criteria allow

## Why
Batch 3 criteria of "Reply to a PDF file comment", "List all markup in the comments panel", "Add or change the explanation of a markup item" and "Delete a markup item" (panel and menu side). Product owner's answers: "not offered" covers every surface; Copy text is removed from a reply's own menu too.

## What it changes
- `crates/ui-egui/src/panels/comments_panel.rs`:
  - new `takes_replies(a)` (false for a FreeText, a callout included): the card shows no "Add a reply" field for a text box. Replies already in a file on a text box are still listed in its thread.
  - the card's "More" menu offers "Edit text..." (renamed from "Edit", as the context menu), Reply (not on a text box), Properties and Delete; "Set status", "Mark with checkmark", "Colour" (never there), "Copy text" and "Make Current Properties Default" are removed. The menu's Reply selects the card and the field takes the keyboard (`reply_focus_id`, a one-frame flag in egui's memory).
  - the card's checkmark toggle button is removed; a checkmark already in the file still shows as a mark, a status as a badge.
  - the filter menu has no "Status" and "Checkmark" sections. The view's `hidden_statuses` and `hidden_checks` fields and their use in the card list stay, so that nothing outside the menu breaks; nothing in the interface sets them.
  - a reply's own "..." menu offers Edit and Delete (Copy text removed).
- `crates/ui-egui/src/tools/comments/menu.rs`: the item's context menu has no Reply entry on a text box; it offers "Edit text...", Reply, (Copy comment, Paste comments and the attachment entries as before, each gated by `offered`), Delete and Properties; "Set status", "Colour", "Mark with checkmark", "Copy text" and "Make Current Properties Default" are removed. The Reply entry also asks the panel's field for the keyboard. The `prefs` argument of `context_menu` is now unused (`_prefs`).
- `crates/ui-egui/src/tools/comments.rs`: `defaultable` removed (no caller left); the module doc names the new menu entries.
- `crates/ui-egui/src/tools/comments/tests.rs`: the test `context_menu_colour_submenu_keeps_its_picker_open_and_a_swatch_closes_the_menu` is now `the_context_menu_has_no_colour_entry` (the context menu has no Colour entry).
- `crates/ui-egui/tests/it/control.rs` (~1421): the right-click test queries "Edit text..." instead of "Set status".
- `crates/ui-egui/tests/it/comments.rs`: the card menu's Edit is clicked as "Edit text..."; existing tests that used the removed entries were changed to the new behaviour (the menu entry "Copy text" is no marker any more, "Properties..." is used; "Make Current Properties Default" is called through `make_comment_default`; the checkmark and status filter steps check their absence).

## Held already, no change
- A locked item, or a note with a locked reply, cannot be deleted: the card menu, the reply menu and the context menu enable Delete from `Document::annotation_deletable` (`pdfcraft_annot::deletable`), the context menu for every selected comment. Explanations and replies of a locked item stay editable: the engine's `set_contents` and `reply_to` have no lock check.
- Without annotate permission: `allowed` is false, so the panel draws no "Add a comment" box, no reply field, no "More" button and no reply "..." button, a double-click does not open the editor, and the context menu's Edit text, Reply, Properties and Delete are disabled. The engine refuses `ReplyToAnnotation` and `SetAnnotationContents` (`check_permission`).
- The Properties dialog's author field is disabled when `author_locked` (`dialogs/comment_props.rs`, `ui.add_enabled(!author_locked, ...)`).
- Plain Enter, Ctrl+Enter post a reply and Shift+Enter adds a line (`submit_key`).

## Open points
- "Copy comment", "Paste comments" and the attachment entries stay in the menus: they are gated by `offered()` and are not offered in the web build, so they are hidden there.

## Tests
In `crates/ui-egui/tests/it/comments.rs`: `plain_enter_posts_a_reply_from_the_card`, `the_card_offers_a_reply_field_on_every_markup_but_a_text_box`, `the_context_menu_offers_reply_on_every_markup_but_a_text_box`, `the_menus_offer_only_edit_reply_properties_and_delete`, `a_replys_menu_has_edit_and_delete_only`, `a_locked_comment_and_a_note_with_a_locked_reply_are_not_deleted_from_the_menus`, `the_properties_author_field_is_read_only_after_set_author`, `a_pdf_that_forbids_annotating_offers_no_commenting`. Not built or run.
