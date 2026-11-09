---
type: Fork Change
title: Image placed where the reader clicks
description: An image chosen in the browser (or handed over by Workspace) follows the pointer over the pages at its placed size, half transparent, and one click on a page adds it there as one undoable edit; the page context menu's Delete page follows the page.delete command.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
  - id: hosting
    resource: /hosting.md
---

# Image placed where the reader clicks

## Why
Backlog items "Place an image on a page" (the added criteria: Add image opens the browser's file chooser, the image follows the pointer, a click places it, Esc cancels, a file that is not an image changes nothing) and "Place the drawn signature image" (the editor shows the image at the pointer at its natural size and the click on a page places it; cancelling changes nothing; refusals for no document, permissions and bad bytes). Before this change the web build only showed "Adding images arrives on the web with file pickers for images". The host function that hands Workspace's bytes to the editor is wired by a later change; it calls `PdfCraftApp::start_image_placement`, defined here. The page context menu's Delete page (criterion "In a one-page document the page context menu entry is disabled; in a document of several pages it deletes the clicked page") is changed in the same pass.

## What it changes
- `crates/ui-egui/src/canvas/image_place.rs` (new), declared in `canvas.rs`; the state is `DocView::image_place` (`ImagePlace`, holding a `PendingImage { name, bytes, natural }`, the natural size in points from `pdfcraft_engine::added_image_natural_size`).
  - `PdfCraftApp::start_image_placement(name, bytes) -> Result<(), String>`:
    - no document open: `Err("no document is open")`;
    - the document does not allow Add image (the `edit.image` command is not enabled; its rule is the Modification permission, `Edit::AddImage` in `edit/run.rs`): `Err("command edit.image is not available now")`, the text of a refused `runCommand`;
    - bytes that are not a placeable image, empty, or over the engine's limits: `Err` with the text of `added_image_natural_size` (the `EditError` display);
    - otherwise any placement pending on any view is dropped, this one starts and the result is `Ok(())`. Nothing in the document changes before the click.
  - The preview: while the pointer is over a page, the image is drawn at the pointer through `PageXform::paint_user_image` in the box it would get, at 50 percent opacity with a thin frame, so its size follows the view's zoom and it turns with the page's `/Rotate` and the view rotation as the added image will. The picture comes from the engine's own image reader, so the preview and the placed image agree for every placeable format (PNG, JPEG, JPEG 2000, TIFF first page, GIF, BMP, PCX): once, when `start_image_placement` runs, `Session::convert_to_pdf` turns the bytes into a one-page PDF (the code `Edit::AddImage` embeds with, behind its panic guard) and `PageRenderer` renders page 0 on a transparent ground (`decode_preview`). The raster is at most 1024 pixels on its longest side and enlarged at most 8 pixels per point; it is kept as a `ColorImage` in `ImagePlace::preview` and turned into a texture on the first frame that draws it. A file over 16 MiB, or a picture that cannot be decoded or drawn, shows the frame only, filled half transparent. The placed image does not depend on the preview. `PdfCraftApp::image_placement_preview` returns the pending preview picture (for tests). No Cargo.toml and no engine file changed.
  - The size: `fit_added_image(natural, page)` for the page under the pointer (the 80 percent cap; the rule is the engine's and is not repeated here). The box is centred on the pointer in the page's view space (points as displayed), moved inside the page when it would stick out, then converted to user space (`PageInfo::view_to_user`), so a click near an edge keeps the whole image on the page.
  - The click: `canvas::draw_pages` reads one frame's primary press and release from `PagePointer` (`origin` and `released`, no more than 6 points apart) and hands the point to `image_place::after_pages`. A click on a page stores one `Placed { page, rect, name, bytes }`; the document view then calls `PdfCraftApp::place_clicked_image`, which applies ONE `Edit::AddImage { page, rect: Some(rect), name, bytes }` (so `can_undo` is true and one undo removes it) and leaves the same selection Add image leaves (the Edit tool, the new image selected). A click on the page view beside the pages (between or around them) drops the mode, nothing placed. A failure of the edit is reported as any failed edit is (the notice of `apply_edit`). Either way the mode is over.
  - Esc (when no dialog or palette is open) drops the mode before any other handler of the key. Switching documents drops it: a view that was not drawn for a pass since it last drew the mode (another document was shown, or Organize pages) drops it on its next draw, and starting a placement drops the pending one on every other view.
  - The router and the cursor: while an image is pending the page view skips `page_tool_input` for every page (as for a cancelled press), so no tool, text selection or comment takes the press and the router decides no owner; the cursor is claimed through `view.cursor.claim(Layer::Editor, CursorIcon::Crosshair)` while the pointer is over a page. The module reads no pointer state: the point comes in as a value, from `PagePointer` read in `canvas.rs`. It has none of the marks that make a file belong in `PAGE_MODULES` (`canvas/page_pointer/tests.rs`), so that list is unchanged; adding the file there would still be right.
- `crates/ui-egui/src/tools/content_ui.rs`: `add_image_dialog` on the web calls `pick_image_to_place` (instead of the message); `add_image` now calls the new `add_image_at(page, rect, name, bytes)`, the same code with the box given, which the click uses. The desktop build keeps its picker and its placement in the middle of the current page.
- The web file chooser (`image_place.rs`, wasm32 only): the same mechanism `pick_files` uses for PDFs, `rfd::AsyncFileDialog`, which builds the hidden `<input type=file>` in the browser (this crate's `web_sys` has no input element feature enabled, and `Cargo.toml` is not part of this change). Its `accept` list is exactly: `.png, .jpg, .jpeg, .jp2, .j2k, .jpx, .tif, .tiff, .gif, .bmp, .pcx`. That is what the engine's image reader (`pdfcraft_create::embed`, used by `embed_image`, which `Edit::AddImage` calls) decodes: PNG, JPEG, JPEG 2000 (JP2 and raw codestream), TIFF (its first page), GIF, BMP and PCX, chosen by the file's content, not its name. The desktop picker's list (`png jpg jpeg tif tiff gif bmp jp2 j2k jpx`) is unchanged and leaves PCX out. The chosen file is read in a future that puts it in a queue kept in egui's memory; the next frame of the document area (`take_picked_image`) starts the placement if the document that was active when the chooser opened is still the active one (a document opened meanwhile gets nothing, as for the insert-pages chooser, #167), and reports a refusal with `notify_error`. A chooser the reader cancels does nothing.
- `crates/ui-egui/src/canvas.rs`, page context menu: Delete page was enabled by `perms.can_assemble` alone; it is now enabled by `PageMenuPerms::can_delete_page`, set from `PdfCraftApp::command_enabled` for the `page.delete` command spec (computed once a frame in `document_area`, handed to the page frame as `PageEnv::can_delete_page`). The rule is the command's, in `commands.rs`, which another change owns: this change calls it and does not copy it. The entry still deletes the clicked page (`Edit::DeletePages { pages: [clicked page] }`). `can_assemble` stays in `PageMenuPerms` for Add bookmark.

## Placed image is an ordinary added image
The placed image is added with `Edit::AddImage`, so it is page content in `Document::added()` with no comments panel entry. The existing Edit tool selects it (it is selected after the click), moves it by dragging and resizes it by its handles, the corner handles keeping its proportions (`dragged` in `content_ui.rs`, `keep_aspect` for images). Nothing in this change touches that code. Not run in this change.

## Tests
`crates/ui-egui/tests/it/content.rs` (egui_kittest, native):
- `an_image_placement_is_refused_without_a_document_or_an_image`: no document, a text file named `x.png`, and no bytes are refused; a click afterwards places nothing and the edit generation is unchanged.
- `a_cancelled_image_placement_changes_nothing`: start, Esc, then a click on the page: no image, same edit generation, `can_undo` false.
- `a_click_places_the_pending_image_as_one_undoable_edit`: one added image on page 0, natural size 30 x 20 points, centred on the click, `can_undo` true, the edit generation raised, a second click places nothing more, one `edit.undo` removes it.
- `a_click_at_the_page_edge_keeps_the_image_inside_the_page`.
- `a_480_by_180_pixel_png_is_placed_at_480_by_180_points` on a 1000 x 1000 point page.
- `the_preview_of_a_placed_bmp_is_its_picture`, `..._gif_...`, `..._tiff_...`, `..._pcx_...`, `..._jpeg_2000_image_...`, `..._png_...` (full names `the_preview_of_a_placed_<format>_is_its_picture`): the pending preview picture has the shape of the image and is not blank. Fixtures are built in the test (hand-written BMP, GIF, TIFF and PCX bytes; a contributor-original raw JPEG 2000 codestream whose every sample is mid grey); no files added. Not run in this change.
The refusal for a document whose permissions do not allow the change needs the encrypted fixture (Mock/Stub e2e); the web chooser and the look of the preview run only in the browser.

## Not covered
- Not built or run (workers do not build).
- The host function that calls `start_image_placement` with Workspace's bytes (`app/host_calls.rs`, `apps/pdfcraft-web`) is a later change; until it is wired, only the web Add image path and Rust callers start a placement.
- A press that moves more than 6 points before its release is not a click: the mode stays until a click, Esc, a click beside the pages or a switch of documents.

## Depends on upstream
`rfd::AsyncFileDialog`, `SignatureImage::from_bytes`, `PageXform::paint_user_image`, `PageInfo::view_to_user`, and the engine's `added_image_natural_size` and `fit_added_image` (added by the change "Insert pages from bytes, and the image size rule as an engine function"), as they are.
