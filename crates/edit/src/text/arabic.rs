//! The editor's own Arabic face for a corrected Arabic paragraph.
//!
//! When the paragraph's font cannot draw the corrected text (an embedded subset has no joining
//! tables, or lacks a letter of the new word), the paragraph is set with the craft-fonts Arabic
//! face. That face is a variable font, which cannot be embedded as a font file
//! (`pdfcraft_fonts::embedding::validate`), so its shaped glyphs are written as Type 3 fonts drawn
//! from its outlines, as added text does (`crate::added`). Each line is shaped in logical order,
//! laid out in drawing order (UAX #9) and wrapped in an `/ActualText` span holding the line as
//! typed, so Find and copy read the text back in typing order.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use pdfcraft_cos::{Dict, Document, ObjRef, Object, PdfString, Stream};
use pdfcraft_fonts::{GlyphError, GlyphOutline, ShapedCluster, ShapedScript};
use unicode_bidi::{Level, ParagraphBidiInfo};

use super::rewrite::{FontChoice, LineCursor};
use super::{EditError, Op, needs_type3};

/// Most glyphs in one Type 3 font: its codes are single bytes, 1 to 240.
const PER_FONT: usize = 240;

/// Arabic letters, marks, digits and punctuation (the range added text shapes with the face).
pub(super) fn is_arabic(c: char) -> bool {
    matches!(u32::from(c), 0x0600..=0x06FF | 0x0750..=0x077F | 0x0870..=0x08FF | 0xFB50..=0xFDFF | 0xFE70..=0xFEFC)
}

/// Direction marks, embeddings, overrides and isolates: they steer the order and are not drawn.
fn is_bidi_control(c: char) -> bool {
    matches!(c, '\u{061C}' | '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}')
}

fn mirrored(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        other => other,
    }
}

/// A piece of a line, in drawing order.
enum Piece {
    /// Shown in the paragraph's standard font (Latin letters, and what the Arabic face lacks).
    Latin(String),
    Shaped(Vec<ShapedCluster>),
}

/// What makes two clusters the same glyph of a Type 3 font: text, glyphs, offsets and advance.
type ClusterKey = (String, Vec<(u32, u64, u64)>, u64);

fn cluster_key(c: &ShapedCluster) -> ClusterKey {
    (c.text.clone(), c.glyphs.iter().map(|(id, [x, y])| (*id, x.to_bits(), y.to_bits())).collect(), c.advance.to_bits())
}

fn glyph_error(e: GlyphError, text: &str) -> EditError {
    EditError::Invalid(match e {
        GlyphError::NoFont => "Arabic text needs PdfCraft's Arabic font, which this build doesn't include \
                               (to build it in, set CRAFT_FONTS_DIR to a craft-fonts checkout that has an Arab face)"
            .into(),
        GlyphError::Missing => format!("neither the paragraph's font nor the Arabic font the editor provides can show \"{text}\""),
        GlyphError::TooComplex => format!("the Arabic font's glyphs for \"{text}\" are too complex"),
    })
}

/// The fonts written for a paragraph: where each cluster is, by font and code.
struct Registered {
    index: HashMap<ClusterKey, (usize, u8)>,
    names: Vec<String>,
    latin: String,
}

/// The Arabic face as a paragraph's font: widths for wrapping, then, once the lines are known,
/// the fonts and the operators that show a line.
pub(super) struct ArabicFace {
    family: crate::added::Family,
    bold: bool,
    italic: bool,
    laid: RefCell<HashMap<String, Rc<Vec<Piece>>>>,
    registered: RefCell<Option<Registered>>,
}

impl ArabicFace {
    pub(super) fn new(family: crate::added::Family, bold: bool, italic: bool) -> Self {
        Self { family, bold, italic, laid: RefCell::new(HashMap::new()), registered: RefCell::new(None) }
    }

    fn latin_width(&self, s: &str) -> f64 {
        self.family.std14(self.bold, self.italic).text_width(s, 1.0)
    }

    /// The pieces of `line` (typed in logical order) in drawing order.
    fn lay(&self, line: &str) -> Result<Rc<Vec<Piece>>, EditError> {
        if let Some(done) = self.laid.borrow().get(line) {
            return Ok(Rc::clone(done));
        }
        let pieces = Rc::new(Self::lay_out(line)?);
        self.laid.borrow_mut().insert(line.to_owned(), Rc::clone(&pieces));
        Ok(pieces)
    }

    fn lay_out(line: &str) -> Result<Vec<Piece>, EditError> {
        // unicode-bidi indexes the first level of a line, so an empty one is left out.
        if line.is_empty() {
            return Ok(Vec::new());
        }
        let base = if crate::bidi::rtl_base(line) { Level::rtl() } else { Level::ltr() };
        let info = ParagraphBidiInfo::new(line, Some(base));
        let (levels, runs) = info.visual_runs(0..line.len());
        let mut pieces = Vec::new();
        for run in runs {
            let run_rtl = levels.get(run.start).is_some_and(Level::is_rtl);
            let Some(text) = line.get(run) else { continue };
            // Arabic letters take the face; so do the spaces and punctuation between them. The
            // rest (Latin text, and what the face lacks) is shown in the standard font.
            let mut segments: Vec<(bool, String)> = Vec::new();
            for c in text.chars().filter(|c| !is_bidi_control(*c)) {
                let arabic = is_arabic(c) || (run_rtl && !c.is_alphanumeric() && pdfcraft_fonts::arabic_has(c));
                match segments.last_mut() {
                    Some((kind, s)) if *kind == arabic => s.push(c),
                    _ => segments.push((arabic, c.to_string())),
                }
            }
            if run_rtl {
                segments.reverse();
            }
            for (arabic, s) in segments {
                pieces.push(if arabic {
                    Piece::Shaped(pdfcraft_fonts::shape_arabic(&s, run_rtl).map_err(|e| glyph_error(e, &s))?)
                } else if run_rtl {
                    Piece::Latin(s.chars().rev().map(mirrored).collect())
                } else {
                    Piece::Latin(s)
                });
            }
        }
        Ok(pieces)
    }

    /// The width of `s` (a logical string) in em units; the standard font's estimate when it can't
    /// be shaped (writing the paragraph then reports why).
    pub(super) fn width_em(&self, s: &str) -> f64 {
        match self.lay(s) {
            Ok(pieces) => pieces
                .iter()
                .map(|piece| match piece {
                    Piece::Latin(s) => self.latin_width(s),
                    Piece::Shaped(clusters) => clusters.iter().map(|c| c.advance).sum::<f64>(),
                })
                .sum(),
            Err(_) => self.latin_width(s),
        }
    }

    /// Writes the fonts for the wrapped `lines` into `doc` and `fonts`, and returns the font the
    /// paragraph starts in. Every outline and every line is read before the document is touched.
    pub(super) fn register(&self, doc: &mut Document, fonts: &mut Dict, lines: &[(String, bool, usize)]) -> Result<FontChoice, EditError> {
        let mut keys: Vec<ShapedCluster> = Vec::new();
        let mut seen: HashMap<ClusterKey, usize> = HashMap::new();
        let mut any_latin = false;
        for (line, _, _) in lines {
            for piece in self.lay(line)?.iter() {
                match piece {
                    Piece::Latin(s) => {
                        if needs_type3(s) {
                            return Err(EditError::Invalid(format!(
                                "\"{s}\" has characters that neither the Arabic font the editor provides nor {} can show",
                                self.family.base_font(self.bold, self.italic)
                            )));
                        }
                        any_latin = true;
                    }
                    Piece::Shaped(clusters) => {
                        for c in clusters {
                            seen.entry(cluster_key(c)).or_insert_with(|| {
                                keys.push(c.clone());
                                keys.len().saturating_sub(1)
                            });
                        }
                    }
                }
            }
        }
        let mut outlines: HashMap<u32, GlyphOutline> = HashMap::new();
        for c in &keys {
            for (id, _) in &c.glyphs {
                if !outlines.contains_key(id) {
                    let outline = pdfcraft_fonts::shaped_glyph(ShapedScript::Arabic, *id).map_err(|e| glyph_error(e, &c.text))?;
                    outlines.insert(*id, outline);
                }
            }
        }
        let mut names: Vec<String> = Vec::new();
        for chunk in keys.chunks(PER_FONT) {
            let font = type3_font(doc, chunk, &outlines);
            // A name no font on the page has, so paragraphs never replace each other's fonts.
            let mut name = format!("PCAr{}", font.num);
            let mut suffix = 0usize;
            while fonts.get(name.as_bytes()).is_some() || names.contains(&name) {
                suffix = suffix.saturating_add(1);
                name = format!("PCAr{}_{suffix}", font.num);
            }
            fonts.set(name.clone().into_bytes(), Object::Ref(font));
            names.push(name);
        }
        let mut latin = String::new();
        if any_latin {
            let base = self.family.base_font(self.bold, self.italic);
            let mut font = Dict::new();
            font.set(b"Type".to_vec(), Object::name("Font"));
            font.set(b"Subtype".to_vec(), Object::name("Type1"));
            font.set(b"BaseFont".to_vec(), Object::name(base));
            font.set(b"Encoding".to_vec(), Object::name("WinAnsiEncoding"));
            latin = format!("PCEd{}", base.replace('-', ""));
            fonts.set(latin.clone().into_bytes(), Object::Dict(font));
        }
        let show_font = names
            .first()
            .cloned()
            .or_else(|| (!latin.is_empty()).then(|| latin.clone()))
            .ok_or_else(|| EditError::Invalid("the corrected text has nothing the Arabic font can draw".into()))?;
        let index = seen.into_iter().filter_map(|(key, k)| u8::try_from(k % PER_FONT + 1).ok().map(|code| (key, (k / PER_FONT, code)))).collect();
        *self.registered.borrow_mut() = Some(Registered { index, names, latin });
        let family = pdfcraft_fonts::document_arabic_font().map_or("Arabic", |f| f.family);
        Ok(FontChoice { show_font, encode: Box::new(|_: &str| None), substituted: Some(format!("{family} Type3")) })
    }

    /// The operators that show `line` (logical order) at `size`, after [`ArabicFace::register`].
    pub(super) fn show_line(&self, line: &str, size: f64, cursor: &mut LineCursor, ops: &mut Vec<Op>) -> Result<(), EditError> {
        let pieces = self.lay(line)?;
        let guard = self.registered.borrow();
        let registered = guard.as_ref().ok_or_else(|| EditError::Invalid("the Arabic font is not ready".into()))?;
        let n = pdfcraft_content::num;
        let mut properties = Dict::new();
        properties.set(b"ActualText".to_vec(), Object::String(PdfString::text(line)));
        ops.push(Op::new("BDC", vec![Object::name("Span"), Object::Dict(properties)]));
        for piece in pieces.iter() {
            match piece {
                Piece::Latin(s) => {
                    if cursor.shown_font != registered.latin {
                        ops.push(Op::new("Tf", vec![Object::name(&registered.latin), n(size)]));
                        cursor.shown_font = registered.latin.clone();
                    }
                    ops.push(Op::new("Tj", vec![Object::String(PdfString::literal(pdfcraft_fonts::win_ansi(s)))]));
                }
                Piece::Shaped(clusters) => {
                    for c in clusters {
                        let Some(&(font, code)) = registered.index.get(&cluster_key(c)) else { continue };
                        let Some(name) = registered.names.get(font) else { continue };
                        if cursor.shown_font != *name {
                            ops.push(Op::new("Tf", vec![Object::name(name), n(size)]));
                            cursor.shown_font = name.clone();
                        }
                        ops.push(Op::new("Tj", vec![Object::String(PdfString::literal(vec![code]))]));
                    }
                }
            }
        }
        ops.push(Op::new("EMC", vec![]));
        Ok(())
    }
}

/// A Type 3 font with `clusters` as codes 1 and up, each drawn from the Arabic face's outlines.
fn type3_font(doc: &mut Document, clusters: &[ShapedCluster], outlines: &HashMap<u32, GlyphOutline>) -> ObjRef {
    let n = crate::n;
    let mut charprocs = Dict::new();
    let mut differences = vec![Object::Int(1)];
    let mut widths = Vec::with_capacity(clusters.len());
    let mut bbox = [0.0f64; 4];
    let mut to_unicode = Vec::new();
    for (i, c) in clusters.iter().enumerate() {
        let code = i + 1;
        // Glyph space is 1000 units per em.
        let mut paths = Vec::new();
        let mut ink: Option<[f64; 4]> = None;
        for (id, [dx, dy]) in &c.glyphs {
            let Some(g) = outlines.get(id) else { continue };
            for contour in &g.contours {
                for (j, p) in contour.iter().enumerate() {
                    paths.extend(format!("{} {} {}\n", n((p[0] + dx) * 1000.0), n((p[1] + dy) * 1000.0), if j == 0 { "m" } else { "l" }).bytes());
                }
                paths.extend_from_slice(b"h\n");
                let b = [g.bbox[0] + dx, g.bbox[1] + dy, g.bbox[2] + dx, g.bbox[3] + dy];
                ink = Some(ink.map_or(b, |a| [a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])]));
            }
        }
        // Rounded outwards so no ink is clipped.
        let [x0, y0, x1, y1] =
            ink.map_or([0.0; 4], |b| [(b[0] * 1000.0).floor(), (b[1] * 1000.0).floor(), (b[2] * 1000.0).ceil(), (b[3] * 1000.0).ceil()]);
        bbox = [bbox[0].min(x0), bbox[1].min(y0), bbox[2].max(x1), bbox[3].max(y1)];
        let mut proc = format!("{} 0 {} {} {} {} d1\n", n(c.advance * 1000.0), n(x0), n(y0), n(x1), n(y1)).into_bytes();
        if !paths.is_empty() {
            proc.extend(paths);
            proc.extend_from_slice(b"f\n");
        }
        let glyph = format!("g{code:02X}");
        charprocs.set(glyph.clone().into_bytes(), Object::Ref(doc.add(Object::Stream(Stream::flate(Dict::new(), &proc)))));
        differences.push(Object::name(&glyph));
        widths.push(Object::Real((c.advance * 1000.0).round()));
        if !c.text.is_empty() {
            let hex: String = c.text.encode_utf16().map(|u| format!("{u:04X}")).collect();
            to_unicode.push(format!("<{code:02X}> <{hex}>\n"));
        }
    }
    let mut cmap = String::from(
        "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CMapType 2 def\n1 begincodespacerange\n<01> <FF>\nendcodespacerange\n",
    );
    // A CMap block holds at most 100 entries.
    for block in to_unicode.chunks(100) {
        cmap.push_str(&format!("{} beginbfchar\n{}endbfchar\n", block.len(), block.concat()));
    }
    cmap.push_str("endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n");
    let cmap = doc.add(Object::Stream(Stream::flate(Dict::new(), cmap.as_bytes())));
    let family = pdfcraft_fonts::document_arabic_font().map_or("Arabic", |f| f.family);
    let mut descriptor = Dict::new();
    descriptor.set(b"Type".to_vec(), Object::name("FontDescriptor"));
    descriptor.set(b"FontName".to_vec(), Object::name(&family.replace(' ', "")));
    descriptor.set(b"FontFamily".to_vec(), Object::String(PdfString::literal(family.as_bytes().to_vec())));
    descriptor.set(b"Flags".to_vec(), Object::Int(4));
    descriptor.set(b"ItalicAngle".to_vec(), Object::Int(0));
    let mut encoding = Dict::new();
    encoding.set(b"Type".to_vec(), Object::name("Encoding"));
    encoding.set(b"Differences".to_vec(), Object::Array(differences));
    let mut font = Dict::new();
    font.set(b"Type".to_vec(), Object::name("Font"));
    font.set(b"Subtype".to_vec(), Object::name("Type3"));
    font.set(b"FontDescriptor".to_vec(), Object::Ref(doc.add(Object::Dict(descriptor))));
    font.set(b"FontBBox".to_vec(), Object::Array(bbox.iter().map(|v| Object::Real(*v)).collect()));
    font.set(
        b"FontMatrix".to_vec(),
        Object::Array(vec![Object::Real(0.001), Object::Int(0), Object::Int(0), Object::Real(0.001), Object::Int(0), Object::Int(0)]),
    );
    font.set(b"FirstChar".to_vec(), Object::Int(1));
    font.set(b"LastChar".to_vec(), Object::Int(i64::try_from(clusters.len()).unwrap_or(i64::MAX)));
    font.set(b"Widths".to_vec(), Object::Array(widths));
    font.set(b"Encoding".to_vec(), Object::Dict(encoding));
    font.set(b"CharProcs".to_vec(), Object::Dict(charprocs));
    font.set(b"ToUnicode".to_vec(), Object::Ref(cmap));
    doc.add(Object::Dict(font))
}
