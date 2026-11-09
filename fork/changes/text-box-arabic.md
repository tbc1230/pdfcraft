---
type: Fork Change
title: Arabic text in a text box is drawn in the Arabic face
description: A text box (FreeText) with Arabic text, in a build with craft-fonts, is shaped, ordered right to left, drawn from the craft-fonts Arabic face, and sized and wrapped with the Arabic widths.
resource: working tree on wasm-bundle (not built or committed by the worker)
status: draft
generated: { by: claude-sonnet-5-5, at: 2026-11-09T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Arabic text in a text box is drawn in the Arabic face

## Why
The criteria of "Write a remark in a text box on a page" (batch 3) require that Arabic typed into a text box, in a build with craft-fonts, appears in the Arabic face, shaped and read right to left, with the box sized and wrapped by the Arabic widths and no /PCUndrawn mark. Before, every text box used Helvetica with WinAnsi encoding, so Arabic came out as `?` and was marked.[^mods]

## What it changes
- `crates/annot/src/appearance/arabic.rs` (new): lays out a line with the Unicode bidirectional algorithm (`unicode-bidi`), shapes Arabic words with `pdfcraft_fonts::shape_arabic`, and draws each shaped cluster from a Type 3 font built from `pdfcraft_fonts::shaped_glyph` outlines, with a ToUnicode map and an `/ActualText` span per line. The code follows `crates/edit/src/added` (the edit crate sits above annot, so the part needed is kept here, not shared).
- `crates/annot/src/appearance.rs`: `text_width` and `wrap` are now functions that measure Arabic with the shaped widths and anything else with Helvetica as before; `undrawable_text` for a text box asks the Arabic layout when the text has Arabic and the build has the face; `draw_free_text` draws Arabic through the new module and adds its fonts to the form's resources. A right-to-left paragraph with no `/Q` is aligned to the right edge; an explicit `/Q` is kept.
- `crates/annot/src/draw.rs`: `apply_appearance` stores streams held inside the appearance's resources (the glyph procedures) as objects of their own.
- `crates/annot/Cargo.toml`: new dependency `unicode-bidi = "0.3"` (already used by `fonts`, `edit` and `ui-egui`; `Cargo.lock` needs its entry for annot).
- The box measured at creation (`crates/ui-egui/src/tools/comments/geometry.rs`) and the box sizing of an edit (`crates/annot/src/modify.rs`) call `appearance::text_width` and `appearance::wrap`, so they follow without a change of their own.

## Not covered
- Without craft-fonts, and for any script with no face (Cyrillic, ...), nothing changes: /Contents is kept, `?` is drawn, /PCUndrawn is set and the editor shows its notice.
- Hebrew and Thai are not drawn in a text box; no other face is added.
- Mixed Arabic and Latin text is laid out with the bidirectional algorithm; Latin pieces keep Helvetica. Characters outside WinAnsi and the Arabic face in such a text are reported as undrawable.

## Depends on upstream
- `pdfcraft_fonts::{shape_arabic, shaped_glyph, arabic_has, document_arabic_font}` and the craft-fonts Arabic face.
- `appearance::build` having no document: Type 3 glyph streams are inline until `apply_appearance` stores them.

[^mods]: /sources/modifications.md, "Write a remark in a text box on a page".
