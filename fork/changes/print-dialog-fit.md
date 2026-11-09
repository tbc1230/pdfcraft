---
type: Fork Change
title: The Print dialog fits the editor area
description: The Print dialog is never wider or taller than the editor area; its body scrolls, the Print and Cancel row stays in view, and below 860 points the preview moves under the settings.
resource: uncommitted on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-sonnet-5-5, at: 2026-11-09T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# The Print dialog fits the editor area

## Why
In the Mock/Stub's 760 x 596 editor area the Print dialog was 820 points wide and taller than the area, with no scroll, so its Print and Cancel buttons lay outside it (Issue 5 in `fork/issues.md`). The product owner decided that the dialog scrolls and its buttons stay reachable at any editor area size, with no lower bound; the criteria are those marked "Issue 5" in the story "Print the document".[^mods]

## What it changes
Layout only; what the dialog does is unchanged.
- `crates/ui-egui/src/workflows/print_ui.rs`: the constants of the layout and `dialog_width(ctx)`, the content width: 820 points, or the editor area's width less 48 (the card's padding and outline) where that is less, never below 40.
- `crates/ui-egui/src/dialogs.rs`: the width of `Dialog::Print` in `dialogs` is `dialog_width`; no other dialog and not `show_modal` changed.
- `crates/ui-egui/src/workflows/print_ui/body.rs`:
  - the title stays on top and the Print and Cancel row stays at the bottom, both outside the scroll; the settings and the preview lie in a vertical scroll area (`scrolled_body`, as `preferences_scroll` in `dialogs.rs`: `max_height` and `min_scrolled_height` of the height left after the title and the row, so the dialog opens at its final size and does not grow in later frames; the scrollbar floats, so it takes no width);
  - the height of the scroll is the editor area's height less 150 points for the card's padding, title, row and gaps, never below 24;
  - from an editor area 860 points wide up the settings (470 wide) and the preview (320 wide) are side by side as before; below it the preview lies under the settings;
  - below 860 the settings column is at most 470 and the preview at most 320 wide, never wider than the dialog; the preview pane and its paper shrink with its width;
  - where the settings column is under 430 wide each caption of the top grid lies above its control; combo boxes are no wider than the column; the rows of radio buttons, check boxes and combo boxes wrap.
- At 1400 x 900 the dialog is as before: 820 wide, two columns, nothing scrolls (the body's height is below its limit).

## Thresholds
860 points (two columns from there), 430 points (captions above controls), 150 points (what the card takes besides the body), 24 points (least body height), 48 points (card padding and outline off the width). All are constants at the top of `print_ui.rs`.

## Tests
In `crates/ui-egui/tests/it/print.rs`: `the_print_dialog_lies_inside_the_editor_area_at_any_size`, `the_settings_scroll_and_the_print_row_stays_put`, `below_860_points_the_preview_is_under_the_settings_and_nothing_lies_outside_horizontally`, `at_1400_by_900_the_dialog_keeps_two_columns_820_wide_without_scrolling`, `the_print_dialog_opens_at_its_final_size_and_does_not_move_afterwards`, `the_print_button_prints_at_760_by_596`; sizes 760 x 596, 1400 x 900, 520 x 400, 400 x 300, 300 x 200. Not yet built or run.

## Not covered
- The French and Arabic criterion and the e2e criteria (1280 x 720 and 1024 x 600 windows) are for the Mock/Stub; no test here.
- At a width where one control is wider than the column (the driver options panel's 200 point boxes, a long caption word) that control is clipped by the scroll area rather than wrapped; the dialog itself stays inside the area.
- The preview's rasters are still requested at the full pane size.

## Depends on upstream
- egui's `ScrollArea` (`min_scrolled_height`, `max_height`) and `Modal`; the Preferences dialog's scroll in `dialogs.rs` is the precedent.

[^mods]: [Fork items of the backlog](/sources/modifications.md).
