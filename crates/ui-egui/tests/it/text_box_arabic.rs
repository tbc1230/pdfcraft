//! A new text box with Arabic text is sized with the Arabic face's own widths.

use pdfcraft_ui_egui::comments::text_box_rect;

/// The advance of `text` (Arabic words and the spaces between them) in the craft-fonts Arabic face
/// at `size`, from the shaper alone: each word and each space shaped on its own.
fn shaped_width(text: &str, size: f64) -> f64 {
    let advance = |s: &str| -> f64 { pdfcraft_fonts::shape_arabic(s, true).unwrap().iter().map(|c| c.advance * size).sum() };
    let words: Vec<&str> = text.split(' ').collect();
    words.iter().map(|w| advance(w)).sum::<f64>() + advance(" ") * (words.len() - 1) as f64
}

#[test]
fn the_box_of_new_arabic_text_has_the_width_of_the_arabic_words() {
    if pdfcraft_fonts::document_arabic_font().is_none() {
        eprintln!("skipping the Arabic text box sizing: built without a craft-fonts Arab face (set CRAFT_FONTS_DIR)");
        return;
    }
    let size = 12.0;
    let text = "\u{645}\u{631}\u{62d}\u{628}\u{627} \u{628}\u{627}\u{644}\u{639}\u{627}\u{644}\u{645}";
    let r = text_box_rect([100.0, 700.0], text, size, 0);
    let want = (shaped_width(text, size) + 2.0 * 2.0 + 4.0).clamp(40.0, 300.0);
    assert!(((r[2] - r[0]) - want).abs() <= 1.0, "box {} wide, the Arabic widths say {want}", r[2] - r[0]);
    // One line tall.
    let h = size * 1.2 + 2.0 * 2.0 + 2.0;
    assert!(((r[3] - r[1]) - h).abs() <= 1.0, "{r:?}");
    // The Arabic widths are what size it, not the Latin ones the same characters would have.
    let latin = (pdfcraft_fonts::helvetica_width(text, size) + 8.0).clamp(40.0, 300.0);
    assert!(((r[2] - r[0]) - latin).abs() > 1.0 || (want - latin).abs() <= 1.0, "{r:?} {latin}");
}
