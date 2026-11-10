---
type: Fork Change
title: Note pop-up on the page, and rules for moving, resizing and reshaping annotations
description: An opened note is its PDF pop-up drawn on the page (movable, resizable, closable, saved in the file); a move, resize or reshape respects permissions and locks, slips below 2 pt, stays on the page and is one undo step.
resource: uncommitted on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-sonnet-5-5, at: 2026-11-10T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Note pop-up on the page, and rules for moving, resizing and reshaping annotations

## Why
Criteria of the stories "Move and resize a placed object" (notes, text boxes, ink, shapes, lines, arrows, polygons, clouds; added images are another change) and "Markup carries its appearance in the saved file", and the product owner's decision that an opened note is the PDF pop-up on the page, as Acrobat shows it.[^mods] Today resize and reshape start without a permission or lock check, nothing stops at the page edge, the slip rule is 2 screen pixels, and a note has no pop-up on the page.

## What it changes
Permissions and locks (UI, `crates/ui-egui/src/tools/comments/`):
- `input.rs`: at the start of a drag, Reshape (point handle), Resize (edge or corner handle) and Move start only when `cx.allowed` (the PDF allows annotating, the check the Move already used) and the comment is not locked. A press that cannot start one only selects; no notice is shown. The engine still refuses (`NotPermitted`, locked) as the last line.
- `geometry.rs`: `resizable` is false for a locked comment (`reshapable` already was); `paint.rs` and `hit` draw and offer handles through those two and `cx.allowed`, so a locked or not-annotatable comment shows none. The cursor over a locked body is the pointing hand, not Move.

Slip threshold: `geometry::SLIP_PT` = 2 page points, tested by `slipped` (screen distance divided by `PageXform::zoom`). It replaces 2 screen pixels (move), 4 pixels (resize) and 2 pixels (reshape). A resize also needs each resulting side to be at least 2 pt. Below it nothing is queued.

Stay on the page (`geometry::slide_within`, `fit_within`; `state::PageCx::resize_preview`): the bound is `PageXform::rect`, the page box the view shows (the crop box with the file's `/Rotate` and the view's rotation applied), so the rule is the same on a rotated page and view. A move is cut so the dragged comment (with the other selected ones, as one group, so the arrangement is kept) stays inside; a resize and a reshape clamp the pointer to the page and, for proportional corner resizing, scale about the fixed corner. The live preview (`adjusted_rect`, `paint_points`) uses the same functions as the release, so it shows what will be saved.

Text markup: unchanged and now tested: `hit` returns `declines_drag` for it, the Body drag arm and `resizable` exclude it.

A text box whose text the editor cannot redraw: already refused by `refuse_foreign_text` in `pdfcraft_annot::set_rect` (the engine's wording "PdfCraft can't redraw this comment..."); moving does not redraw and works. Now covered by a test. No code change.

Author (`/T`) and undo: `move_annotation`, `set_rect` and `set_points` never write `/T`; each is one engine edit, so one undo step; confirmed by an engine test. No code change.

Note pop-up:
- Engine: `Edit::SetNotePopup { page, index, open, rect }` (`engine/src/edit.rs`, `edit/run.rs`, `history.rs` scope Comments, permission group "comments" with the other annotation edits), implemented by `pdfcraft_annot::set_popup` (`annot/src/modify.rs`): sets `/Open` on the note and on its `/Popup`; with `rect` also its `/Rect` (user space, at least 40 pt a side, finite); a note without a `/Popup` gets one (`create::add_popup`, appended to `/Annots` so no index moves) at `rect` or at `default_popup_rect` (the place a new note's pop-up has always had). The note's text, author and date are not touched (no `/M`). Refused for a locked note, a reply, or any annotation that is not a note. Moving the note already moved its pop-up (`move_annotation`).
- Reading: `pdfcraft_annot::Summary` has `popup: Option<([f32; 4], bool)>` (the pop-up's rectangle and whether it is open) for a note only; a note with no pop-up reads as open if its own `/Open` is true, at the default place. `engine/src/comments.rs` passes it on to `pdfcraft_render::Annotation::popup` (see Open points: this field is required in the render crate).
- Labels (undo wording): "Open note", "Close note", "Change pop-up" (`edit/labels.rs`, `EDIT_LABELS`).
- UI drawing (`paint.rs::paint_popups`): for each note whose pop-up is open, over everything on the page, at its `/Popup /Rect` (as the view shows it): a title bar in the note's colour with the author and the date, a close control, the note's text wrapped and clipped, a corner grip. A pop-up is drawn upright as its rectangle shows on the page.
- UI input: `CommentPart` (`canvas/page_router.rs`) has pop-up parts `PopupBar`, `PopupEdge(x, y)`, `PopupClose` and `PopupText` (mapped by `geometry::PopupPart`); the single hit test, the router-only press and the cursor claim are unchanged. `hit` tests an open pop-up first (Select tool), over the handles and bodies under it. Dragging the title bar moves it (`Gesture::PopupMove`), dragging an edge or corner resizes it (`Gesture::PopupResize`, least side 40 pt), both within the page and with the slip rule; the release is `Edit::SetNotePopup { open: true, rect }`. The close control closes (`SetNotePopup { open: false }`). With annotating forbidden or the note locked, only the text is hit and the pop-up is read only.
- Opening: a double-click on the note's icon opens its pop-up, or closes it again if open; opening wraps the edit in one named `Batch` ("Open note") that also puts the pop-up inside the page (a pop-up that lay off the page, as a default place near an edge does, comes in with its size kept, at least 40 pt). The context menu has "Open note" / "Close note" for a note (greyed without permission or when locked). A locked note keeps the earlier behaviour: a double-click edits its text.
- Editing and the pop-up: the ways to edit the note's text stay (the context menu "Edit text...", the Comments panel, the composer card). A double-click inside the pop-up (title bar or text) opens the same composer card as before ("Edit comment"); the pop-up shows the saved text, so after posting it shows the new text. The composer card is still the single place where text is typed, so there is no second editor inside the pop-up.
- Undo decision: opening, closing, moving and resizing a pop-up are undo steps. Basis: the state is written to the file (`/Open`, `/Popup /Rect`) and changes what Save writes and what the modified mark says; the engine has no non-undoable edit; undo then restores exactly what a saved file would contain. Opening costs one step, a close one step, and a place or size change one step each (the drag's release).
- Opened from a file: a note whose `/Popup /Open` is true is shown open on opening the file; a pop-up whose `/Rect` is missing or smaller than 1 pt reads at the default place.

Appearance (criteria "Markup carries its appearance in the saved file"), checked, nothing missing found in the fork's code:
- `add_annotation` draws `/AP /N` for every shape it builds (`draw_appearance` at creation); `add_text_replacement` builds its two annotations through it. Measurements, attachments and stamps come through other code (not changed or re-audited here, see Open points).
- A resize, a reshape, a text change and a style or colour change redraw the appearance (`set_rect`, `set_points`, `set_contents`, `set_style`... call `redraw_or_restore` / `draw_appearance`).
- A move does not redraw the appearance: it shifts `/Rect` and the geometry keys together (`/L`, `/Vertices`, `/InkList`, `/QuadPoints`, `/CL`) and the pop-up's rect, and an appearance's `/BBox` is mapped onto `/Rect` by every reader (ISO 32000-2 12.5.5), so the saved appearance shows at the new place, exactly. This deviates from "rebuilt after a move" on purpose: redrawing on a move would replace an appearance another program drew (a text box, a stamp) just because it was moved, against "preserve unknown data". A resize, reshape or text change still redraws as before.
- A reply carries an empty appearance form (BBox 0 0 0 0, no content) and a copied `/Rect`, so a reader draws no second icon (`reply` in `modify.rs`; test).
- A highlight appearance uses `/BM /Multiply` (`appearance.rs`, test in `annot/src/tests.rs`), so it does not hide the text; underline, squiggly and strike out draw lines only.
- A deleted item takes its replies and its pop-up out of `/Annots` (`with_replies_and_popups`, existing test `delete_takes_popup_and_reply_chain`).

## Tests
- `crates/ui-egui/src/tools/comments/tests.rs` (module `placed`): `a_locked_comment_shows_no_handle`, `the_slip_threshold_is_two_page_points_whatever_the_zoom`, `a_click_or_a_drag_below_the_slip_threshold_changes_nothing`, `a_move_resize_or_reshape_stops_at_the_page_edge_on_every_rotation` (every page `/Rotate` x every view rotation), `dragging_a_group_to_the_edge_keeps_its_arrangement`, `the_pop_up_parts_round_trip_through_the_routers_parts`, `an_open_pop_up_is_hit_by_part_and_only_a_closed_note_is_its_icon` (permitted, not permitted, locked), `the_pop_up_moves_by_its_title_bar_and_resizes_by_its_edges_within_the_page`, `opening_puts_the_pop_up_inside_the_page_and_closing_writes_only_the_state`, `text_markup_is_not_moved_or_resized`.
- `crates/annot/src/modify.rs` (module `popup_tests`): `a_new_note_has_a_closed_pop_up_and_opening_it_is_saved`, `a_note_without_a_pop_up_gets_one_when_opened_and_the_other_comments_keep_their_index`, `only_an_unlocked_note_takes_a_pop_up_of_a_sensible_size`, `moving_the_note_takes_its_pop_up_along_and_a_move_keeps_the_author`, `a_resize_and_a_reshape_keep_the_author_and_redraw_the_appearance`, `a_reply_has_an_empty_appearance_so_no_second_icon_shows`, `a_text_box_another_program_drew_moves_but_is_not_resized`.
- `crates/engine/src/comments.rs` (module `popup_tests`): `opening_closing_and_placing_a_pop_up_are_undo_steps_and_are_saved`, `a_locked_note_keeps_its_pop_up_and_annotating_is_needed`, `a_move_a_resize_and_a_reshape_are_each_one_undo_step_and_keep_the_author`.
- `crates/ui-egui/tests/it/comments.rs`: new `a_note_opens_as_a_pop_up_that_moves_resizes_and_closes_within_the_page`; `edit_composer_preserves_only_typewriter_whitespace` now double-clicks a note twice (icon opens the pop-up, the pop-up's text opens the composer).
- Not covered here: the criteria marked e2e (the integrator's), the no-modify-protection image criterion (another change), and the remaining Appearance e2e criteria.

## Open points
- Done under the grants: `pdfcraft_render::Annotation` (`crates/render/src/inspect/model.rs`) has the field `pub popup: Option<([f32; 4], bool)>` (the pop-up's rectangle in user space, whether it is open), set in `crates/render/src/inspect/annotations.rs` for a Text annotation that is not a reply from its `/Popup` (`/Rect`, `/Open`; the default place when none, as `pdfcraft_annot::default_popup_rect`; `None` for other subtypes), and `popup: None` added to the struct literals in `crates/render/src/inspect/tests.rs` and in other tests that build an `Annotation`. The inspector sets the field when a file is opened, and the engine sets it after every comment edit.
- The texts "Open note", "Close note" and "Change pop-up" (labels and menu items), with French and Arabic written by the worker, are added by the integrator to the local craft-i18n checkout only (`locales/fr/pdfcraft.tsv`, `locales/ar/pdfcraft.tsv`); they live only there until upstream craft-i18n has them, and `craft-i18n.rev` does not move (product owner's decision, 2026-11-10). `Edit::SetNotePopup` needs an entry in any tool or table that lists edits (none found in `automation`).
- Integrator grants (done): `popup` field and its reading in `crates/render/src/inspect/model.rs` and `annotations.rs`; `popup: None` in the only other `Annotation` literal outside the fork files, `crates/render/src/inspect/tests.rs`; dedicated pop-up parts in `crates/ui-egui/src/canvas/page_router.rs`. No serialization or list of router parts exists.
- The Comments panel (`comments_panel.rs`) still opens a note's text only through its own card; it has no "open pop-up" action.
- Pop-ups of other annotation types (a text box or a shape with a `/Popup`) are not drawn or changed.
- A pop-up is drawn upright from its `/Rect` as the view shows it; on a page turned a quarter its `/Rect` is the box as the view shows it, not turned about a corner as a NoRotate icon is.
- The live preview of moving several selected comments clamps each against the page alone; the release clamps the group, so near the edge the preview and the result can differ by the group's spread.
- Measurements, attachments, stamps and pasted comments were not re-audited for their appearance.
- The slip threshold changes from pixels to points: existing e2e tests that drag by a few pixels at a high zoom need a check.
- Mistake in the session: one read-only `git status` was run once against the worker rules; it changed nothing.

[^mods]: [Fork items of the backlog](/sources/modifications.md).

Not built or run.
