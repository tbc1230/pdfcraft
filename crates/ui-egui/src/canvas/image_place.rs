//! Placing an image on a page: an image the reader chose (or Workspace handed over) follows the
//! pointer over the pages at the size it will have, half transparent, and the next click on a page
//! puts it there as one Add image edit, its box centred on the click and kept inside the page.
//! Escape, a click beside the pages, or switching documents drops it and nothing changes.
//!
//! While an image is pending the pages take no other pointer input (the page view skips its tool
//! input, as for a cancelled press), so the click is this mode's alone; the click itself is read
//! once per frame by `canvas::draw_pages` and handed in as a point.

use std::sync::Arc;

use egui::{Color32, ColorImage, CornerRadius, CursorIcon, Pos2, Stroke, StrokeKind, TextureHandle, TextureOptions};
use pdfcraft_render::DocInfo;

use crate::canvas::{DocView, PageXform};
use crate::page_cursor::Layer;

/// How opaque the preview is: half.
const PREVIEW_ALPHA: u8 = 128;

/// The image formats the engine can place (what `pdfcraft_create`'s image reader decodes, by the
/// file's content): PNG, JPEG, JPEG 2000, TIFF, GIF, BMP and PCX. The web build's file chooser
/// offers exactly these extensions.
#[cfg(target_arch = "wasm32")]
pub(crate) const PLACEABLE_EXTENSIONS: [&str; 11] = ["png", "jpg", "jpeg", "jp2", "j2k", "jpx", "tif", "tiff", "gif", "bmp", "pcx"];

/// The image waiting for the click that places it.
pub(crate) struct PendingImage {
    pub name: String,
    pub bytes: Arc<Vec<u8>>,
    /// Its natural size in points (`pdfcraft_engine::added_image_natural_size`). The size it is
    /// placed at is this fitted to the page under the pointer (`fit_added_image`).
    pub natural: (f64, f64),
}

/// An image to add where the click was: the page, and its box in user space.
pub(crate) struct Placed {
    pub page: usize,
    pub rect: [f64; 4],
    pub name: String,
    pub bytes: Arc<Vec<u8>>,
}

/// The state of image placement on one document's view.
#[derive(Default)]
pub(crate) struct ImagePlace {
    pending: Option<PendingImage>,
    /// The image's own pixels for the preview, decoded once when the placement starts (see
    /// `decode_preview`); none when they could not be had (a frame is drawn instead).
    preview: Option<ColorImage>,
    /// The preview as a texture; made on the first frame that draws it.
    texture: Option<TextureHandle>,
    texture_tried: bool,
    /// The last frame this view drew the mode: a view that was not drawn since (another document
    /// was shown) drops the mode.
    seen_pass: Option<u64>,
    /// Clicked: the edit waiting for the app to apply it.
    placed: Option<Placed>,
    /// Where the press now held began (egui forgets it by the frame of the release).
    press: Option<Pos2>,
}

impl ImagePlace {
    /// Start placing `image`; whatever was pending is replaced.
    pub(crate) fn start(&mut self, image: PendingImage, preview: Option<ColorImage>) {
        let press = self.press;
        *self = Self { pending: Some(image), preview, press, ..Self::default() };
    }

    /// Drop the mode; nothing is placed.
    pub(crate) fn cancel(&mut self) {
        *self = Self { press: self.press, ..Self::default() };
    }

    /// A press began at `at`.
    pub(crate) fn note_press(&mut self, at: Pos2) {
        self.press = Some(at);
    }

    /// The press ends: where it began, if it was seen.
    pub(crate) fn take_press(&mut self) -> Option<Pos2> {
        self.press.take()
    }

    pub(crate) fn is_pending(&self) -> bool {
        self.pending.is_some()
    }

    /// The edit the last click made, once.
    pub(crate) fn take_placed(&mut self) -> Option<Placed> {
        self.placed.take()
    }
}

/// The box, in user space, of the pending image centred on `at` (screen) on `page`, and kept inside
/// the page: the size is `natural` fitted to the page, as the engine fits it.
fn place_rect(info: &DocInfo, page: usize, xf: &PageXform, at: Pos2, natural: (f64, f64)) -> Option<[f64; 4]> {
    let p = info.pages.get(page)?;
    let (w, h) = pdfcraft_engine::fit_added_image(natural, (f64::from(p.width), f64::from(p.height)));
    if !(w.is_finite() && h.is_finite() && w > 0.0 && h > 0.0 && at.is_finite()) {
        return None;
    }
    let (vx, vy) = xf.screen_to_view(at);
    let (pw, ph) = (f64::from(xf.pw), f64::from(xf.ph));
    let x0 = (f64::from(vx) - w / 2.0).clamp(0.0, (pw - w).max(0.0));
    let y0 = (f64::from(vy) - h / 2.0).clamp(0.0, (ph - h).max(0.0));
    let a = p.view_to_user(x0 as f32, y0 as f32);
    let b = p.view_to_user((x0 + w) as f32, (y0 + h) as f32);
    let rect = [a[0].min(b[0]), a[1].min(b[1]), a[0].max(b[0]), a[1].max(b[1])].map(f64::from);
    rect.iter().all(|v| v.is_finite()).then_some(rect)
}

/// The mode's frame, after the pages are drawn (their screen transforms are known): drops it on
/// Escape or when the view was not shown, turns a click into the edit (`ImagePlace::take_placed`),
/// and otherwise draws the preview at the pointer and claims the cursor.
/// `pointer` is where the pointer is over the pages, `click` where a click came up.
pub(crate) fn after_pages(ui: &egui::Ui, view: &mut DocView, info: &DocInfo, pointer: Option<Pos2>, click: Option<Pos2>, unobstructed: bool) {
    if !view.image_place.is_pending() {
        return;
    }
    let pass = ui.ctx().cumulative_pass_nr();
    let skipped = view.image_place.seen_pass.is_some_and(|seen| pass.saturating_sub(seen) > 1);
    if skipped || (unobstructed && ui.ctx().input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))) {
        view.image_place.cancel();
        return;
    }
    view.image_place.seen_pass = Some(pass);
    let page_at = |view: &DocView, p: Pos2| view.screen_xforms.iter().find(|(_, xf)| xf.rect.contains(p)).map(|(page, xf)| (*page, *xf));
    if let Some(c) = click {
        // A click on a page places the image there; one beside the pages cancels. Either way the
        // mode is over.
        if let Some((page, xf)) = page_at(view, c)
            && let Some(pending) = view.image_place.pending.as_ref()
            && let Some(rect) = place_rect(info, page, &xf, c, pending.natural)
        {
            view.image_place.placed = Some(Placed { page, rect, name: pending.name.clone(), bytes: Arc::clone(&pending.bytes) });
        }
        view.image_place.pending = None;
        view.image_place.texture = None;
        return;
    }
    let Some(at) = pointer else { return };
    let Some((page, xf)) = page_at(view, at) else { return };
    view.cursor.claim(Layer::Editor, CursorIcon::Crosshair);
    let Some(natural) = view.image_place.pending.as_ref().map(|p| p.natural) else { return };
    let (Some(rect), Some(page_info)) = (place_rect(info, page, &xf, at, natural), info.pages.get(page)) else { return };
    if !view.image_place.texture_tried {
        view.image_place.texture_tried = true;
        view.image_place.texture =
            view.image_place.preview.clone().map(|image| ui.ctx().load_texture("image-place-preview", image, TextureOptions::LINEAR));
    }
    let painter = ui.painter();
    let screen = xf.user_rect_f64(info, page, rect);
    match &view.image_place.texture {
        Some(texture) => xf.paint_user_image(painter, texture.id(), page_info, rect, 0, Color32::from_white_alpha(PREVIEW_ALPHA)),
        None => {
            painter.rect_filled(screen, CornerRadius::ZERO, Color32::from_rgba_unmultiplied(0x3A, 0x7B, 0xF0, PREVIEW_ALPHA / 2));
        }
    }
    painter.rect_stroke(screen, CornerRadius::ZERO, Stroke::new(1.0, Color32::from_rgba_unmultiplied(0x3A, 0x7B, 0xF0, 200)), StrokeKind::Outside);
}

/// The largest file the preview decodes (bytes). A larger one that the engine still places shows
/// the frame only.
const PREVIEW_MAX_BYTES: usize = 16 << 20;

/// The longest side of the preview raster, in pixels.
const PREVIEW_MAX_SIDE: f64 = 1024.0;

/// The most the preview raster is enlarged (pixels per point): a small picture is drawn sharper
/// than its natural size but never blown up to the longest side.
const PREVIEW_MAX_SCALE: f64 = 8.0;

/// The image's pixels for the preview, for every format the engine places. The bytes are turned
/// into a one-page PDF by the engine's own image reader (`Session::convert_to_pdf`, the code
/// `Edit::AddImage` embeds with, behind its panic guard) and page 0 is rendered by the page
/// renderer on a transparent ground, so the preview shows what the placed image will show (EXIF
/// turn, JPEG 2000, GIF transparency and so on). The raster is at most `PREVIEW_MAX_SIDE` pixels
/// on a side. `None` (a file over `PREVIEW_MAX_BYTES`, a picture that cannot be decoded or drawn)
/// leaves the frame alone to show where it goes; the placement does not depend on it.
fn decode_preview(session: &pdfcraft_engine::Session, name: &str, bytes: &Arc<Vec<u8>>, natural: (f64, f64)) -> Option<ColorImage> {
    if bytes.len() > PREVIEW_MAX_BYTES || !(natural.0.is_finite() && natural.1.is_finite() && natural.0 > 0.0 && natural.1 > 0.0) {
        return None;
    }
    let (_, pdf) = session.convert_to_pdf(name, bytes).ok()?;
    let longest = natural.0.max(natural.1);
    let wanted = (PREVIEW_MAX_SIDE / longest).min(PREVIEW_MAX_SCALE) as f32;
    let scale = pdfcraft_render::effective_scale(natural.0 as f32, natural.1 as f32, wanted);
    let config = pdfcraft_render::RenderConfig { transparent: true, ..Default::default() };
    let mut renderer = pdfcraft_render::PageRenderer::new(pdf, config);
    let shot = renderer.render(pdfcraft_render::RenderRequest { page: 0, kind: pdfcraft_render::RequestKind::Pixels, scale, ..Default::default() });
    let (w, h) = (usize::try_from(shot.width).ok()?, usize::try_from(shot.height).ok()?);
    if shot.error.is_some() || w == 0 || h == 0 || shot.rgba.len() != w.checked_mul(h)?.checked_mul(4)? {
        return None;
    }
    Some(ColorImage::from_rgba_premultiplied([w, h], &shot.rgba))
}

impl crate::PdfCraftApp {
    /// Start placing an image on the active document: it follows the pointer over the pages and a
    /// click on a page puts it there as one Add image edit (see the module). Nothing in the
    /// document changes before the click. Refused, with the reason, when no document is open, the
    /// document's permissions do not allow Add image (the same text as the command), or the bytes
    /// are not a placeable image (the engine's own text: not an image, empty, over its limits).
    pub fn start_image_placement(&mut self, name: String, bytes: Vec<u8>) -> Result<(), String> {
        let Some((index, _)) = self.active_ids() else { return Err("no document is open".into()) };
        const COMMAND: &str = "edit.image";
        if !pdfcraft_engine::commands::command(COMMAND).is_some_and(|spec| self.command_enabled(spec)) {
            return Err(format!("command {COMMAND} is not available now"));
        }
        let natural = pdfcraft_engine::added_image_natural_size(&name, &bytes).map_err(|e| e.to_string())?;
        // One placement at a time, on one document.
        for view in &mut self.views {
            view.image_place.cancel();
        }
        let bytes = Arc::new(bytes);
        let preview = decode_preview(&self.session, &name, &bytes, natural);
        if let Some(view) = self.views.get_mut(index) {
            view.image_place.start(PendingImage { name, bytes, natural }, preview);
        }
        if let Some(ctx) = &self.ctx {
            ctx.request_repaint();
        }
        Ok(())
    }

    /// The picture the pointer preview shows for the placement now pending (its pixels and size),
    /// or `None` when none is pending or its picture could not be decoded within the preview's
    /// limits (the preview is then a frame).
    pub fn image_placement_preview(&self) -> Option<&ColorImage> {
        self.views.iter().find(|v| v.image_place.is_pending()).and_then(|v| v.image_place.preview.as_ref())
    }

    /// Add an image that waits for the reader's click: the edit, then the same selection Add image
    /// leaves (the Edit tool, the new image selected).
    pub(crate) fn place_clicked_image(&mut self, placed: Placed) {
        let Placed { page, rect, name, bytes } = placed;
        self.add_image_at(page, Some(rect), name, bytes);
    }
}

/// An image the browser's file chooser returned, for the document that was active when it was asked.
#[cfg(target_arch = "wasm32")]
struct Picked {
    doc: pdfcraft_engine::DocId,
    name: String,
    bytes: Vec<u8>,
}

#[cfg(target_arch = "wasm32")]
type PickedQueue = Arc<std::sync::Mutex<Vec<Picked>>>;

/// The queue the chooser's future fills and the next frame reads, kept in egui's memory because the
/// future outlives the call that made it.
#[cfg(target_arch = "wasm32")]
fn picked_queue(ctx: &egui::Context) -> PickedQueue {
    ctx.data_mut(|d| d.get_temp_mut_or_insert_with(egui::Id::unique("image-place-picked"), PickedQueue::default).clone())
}

#[cfg(target_arch = "wasm32")]
impl crate::PdfCraftApp {
    /// Add image on the web: the browser's file chooser (an `<input type=file>` that `rfd` builds,
    /// as for the PDF choosers) offering only the placeable formats; the chosen file is read and
    /// its placement starts on a later frame (`take_picked_image`).
    pub(crate) fn pick_image_to_place(&mut self) {
        use crate::file_filter::AddFilterAnyCase as _;
        let Some((_, doc)) = self.active_ids() else {
            self.notify_tr("Open a document first");
            return;
        };
        let Some(ctx) = self.ctx.clone() else { return };
        let queue = picked_queue(&ctx);
        wasm_bindgen_futures::spawn_local(async move {
            let dialog = rfd::AsyncFileDialog::new().add_filter_any_case(tl!("Images"), &PLACEABLE_EXTENSIONS);
            let Some(handle) = dialog.pick_file().await else { return };
            let name = handle.file_name();
            let bytes = handle.read().await;
            if let Ok(mut q) = queue.lock() {
                q.push(Picked { doc, name, bytes });
            }
            // The read may finish while the app is idle: wake it to use the file.
            ctx.request_repaint();
        });
    }

    /// Start the placement of an image the chooser returned, if its document is still the active
    /// one. A failure is reported as any failed edit is, and nothing changes.
    pub(crate) fn take_picked_image(&mut self) {
        let Some(ctx) = self.ctx.clone() else { return };
        let picked: Vec<Picked> = match picked_queue(&ctx).lock() {
            Ok(mut q) => std::mem::take(&mut *q),
            Err(_) => return,
        };
        for Picked { doc, name, bytes } in picked {
            if self.active_ids().map(|(_, id)| id) != Some(doc) {
                continue;
            }
            if let Err(e) = self.start_image_placement(name, bytes) {
                self.notify_error(e);
            }
        }
    }
}
