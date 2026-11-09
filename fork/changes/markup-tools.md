---
type: Fork Change
title: Markup tools and keys follow the criteria
description: The bar next to a text selection follows the offered-commands list, and the Delete or Backspace key asks the engine's delete rule before it deletes a selected markup item.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Markup tools and keys follow the criteria

## Why
Batch 3 criteria of "Mark a passage with text markup", "Place a note at a point on a page", "Draw freehand on a page", "Draw a shape around content", "Delete a markup item" and "Undo and redo markup changes".

## What it changes
- `crates/ui-egui/src/canvas/selection.rs`: each entry of the bar next to a text selection is shown only when `commands::offered` accepts its command (comment.note, comment.highlight, comment.underline, comment.strikeout, comment.replace). In the web build "Replace text" is not shown; natively `offered` is true for all. The bar stays disabled without annotate permission (`add_enabled(can_comment, ...)`, unchanged).
- `crates/ui-egui/src/tools/comments/input.rs`: `keys` takes a new argument `deletable(page, index)`. Delete or Backspace on a selection deletes only when it holds for every selected item: a locked item, or a note with a locked reply, is not deleted, the selection stays, no notice, no edit.
- `crates/ui-egui/src/canvas.rs` (one line, ~3317): the call passes `&|page, index| doc.annotation_deletable(page, index)` (`pdfcraft_annot::deletable`).

## Held already, no change
- A text field with focus wins over Delete/Backspace: `keys` returns at once when `ctx.egui_wants_keyboard_input()`, and the drawing-point Backspace in `click_points` has the same guard.
- Without annotate permission the key does nothing (`allowed &&`).
- A text selection has one page (`Selection.page`, `selection_quads` returns that page only), so a drag across a page break marks one page; a page without text gives no quads and creates nothing.
- Polygon, connected lines and cloud finish by double-click, Enter, the menu's Complete or (polygon, cloud, three points) a click on the first point (`click_points`, `closes_at`); `drawn_shape` returns nothing for the slips.
- Redo is Ctrl+Shift+Z (`edit.redo`, `Shortcut::cmd_shift("Z")`, engine `commands.rs:199`); undo is Ctrl+Z. `edit.undo` and `edit.redo` are enabled only when there is something to undo or redo (`is_enabled`), and the host refuses them otherwise with "command <id> is not available now" (`check_host_command`).

## Tests
- `drawn_shapes_ignore_slips` (`tools/comments/tests.rs`) extended for every listed slip.
- `delete_and_backspace_keep_an_item_that_is_not_deletable_and_do_nothing_without_permission` (`tools/comments/tests.rs`), on `keys`.
- `a_new_markup_change_after_an_undo_clears_redo` and `undo_and_redo_are_not_available_when_there_is_nothing_to_undo_or_redo` (`tests/it/editing.rs`).
- `a_text_markup_records_the_author_set_by_the_host` and `the_selection_bar_is_disabled_when_the_pdf_forbids_annotating` (`tests/it/comments.rs`): the author criterion (`set_author`, then a highlight records that name, empty contents, its page) and the disabled bar on an encrypted PDF with text.
- The selection bar's entries: not tested natively (`offered` is true there); left to the e2e tests.
