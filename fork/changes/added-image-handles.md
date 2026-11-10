---
type: Fork Change
title: Added images stop at the page edge and ignore a slip
description: Moving or resizing a picture placed with Add image (drawn signature, image from bytes, code image) stops at the page edge, a drag under 2 pt only selects, and a corner handle keeps the picture's proportions.
resource: uncommitted on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-sonnet-5-5, at: 2026-11-10T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Added images stop at the page edge and ignore a slip

## Why
The criteria "Move and resize a placed object", "Place a QR code or barcode on a page" and "Place an image from bytes" ask that an added image can be moved and resized, stays on its page, is not changed by a slip under 2 pt, keeps its proportions at a corner handle, and is not moved when the PDF forbids modification.[^mods]

## Where an added image is moved and resized
An added image is a page image to the editor. It is selected, moved and resized in the Edit text and images tool by `image_input` (`crates/ui-egui/src/tools/edit_text_ui/image.rs`), not by `crates/ui-egui/src/tools/content_ui.rs` (whose `hit` and `page_input` only handle added text). The gesture ends in `Edit::EditPageImage` with `ImageEdit::Move` or `ImageEdit::Transform`; the engine (`run_edit_page_image` in `crates/engine/src/edit/run.rs`) finds the picture's added item and updates it with `update_content`, so the picture stays an added item until a rotation, crop or arrange promotes it. `canvas/page_input.rs` runs `image_input` only when `can_modify` (`Document::allows_modification`) is true, and `check_permission` refuses `EditPageImage` under a no-modify protection, so with modification forbidden no handle shows and nothing is applied (unchanged code; now covered by a test). The editor has no locked state for images.

## What it changes
All in `crates/ui-egui/src/tools/edit_text_ui/image.rs`, in `image_input`:
- Added pictures only: the slip threshold and the page-edge limit below apply only when the selected picture is an added item (`placed` in `image_input`: `view.content.selected` is on the page, which `sync_image_selection` sets only for a picture that is the one added item showing its XObject). Pictures and form artwork that were in the page content keep upstream's behaviour: no limit, no threshold; a picture filling the page can still be moved (`check_existing_artwork` in `crates/ui-egui/tests/it/editing.rs`, unchanged).
- Slip threshold (`SLIP_PT`, 2 pt): on release a drag whose pointer went less than 2 pt (in page points, at any zoom) from the press applies nothing and records no settled frame, for the upright path and the path of a turned picture.
- Stay on the page: the drag is limited to the page rectangle shown by the view (`PageXform::rect`, already turned by the view rotation, so a /Rotate page and a rotated view work the same) joined with the picture's own box at the press, so a picture already partly outside may come back but not go further out. A move slides along the edge (`slide_into`); a resize is cut back to the largest part of the drag that fits (`fit_to`, bisection). The preview shows the limited box.
- Proportions: the existing corner behaviour is kept (corner handles keep the proportions, Shift frees them, edge handles are free); the limit above keeps a corner resize square at the edge too.
- A move of a turned picture now sends the shift actually applied (after the limit), not the pointer's.

Unchanged: one `EditPageImage` is one undo step and redo repeats it. Rendering: the page renderer draws an image without `/Interpolate` with nearest-neighbour sampling when enlarged, and does not smooth when reduced (`non_interpolated_images_are_not_smoothed_when_minified` in `crates/render/src/raster/tests.rs`), so module colours of a code image are kept.

## Tests
- `crates/ui-egui/src/tools/edit_text_ui/image.rs` (unit): `a_move_slides_along_the_page_edge_and_may_not_leave_it`, `a_proportional_corner_resize_stops_at_the_page_edge_and_stays_square`, `the_slip_threshold_is_two_points_at_any_zoom`.
- `crates/ui-egui/tests/it/content.rs`: `an_added_picture_dragged_past_the_page_edge_stops_there_and_is_one_undo_step`, `an_added_picture_enlarged_by_a_corner_stays_square_and_on_the_page`, `an_added_picture_dragged_under_two_points_does_not_change`, `an_added_picture_is_not_moved_or_resized_when_the_pdf_forbids_modification`.
- `editing_existing_images_on_the_page` and `editing_existing_form_artwork_on_the_page` (unchanged) prove that a page-filling existing picture can still be moved a quarter page.
- Not covered by a test here: a page with /Rotate or a rotated view (the e2e criterion); saved-file rendering by PDFium.

## Open points
- An added picture that a rotation, crop or arrange has promoted to ordinary page content (the engine drops its added marks), or one XObject drawn twice, counts as existing and is not limited.
- Under 2 pt is hard to reach with the mouse because egui starts a drag only after a few pixels; the threshold matters at high zoom-out and for scripted drags.
- A turned picture that is cut back at the edge is cut back along the drag's own direction, not slid.
- The page rectangle is the view's page box (the box the page is drawn with); the exact box (CropBox or MediaBox) is what `DocInfo` reports for the view, not decided here.

Not built or run.

[^mods]: [Fork items of the backlog](/sources/modifications.md).
