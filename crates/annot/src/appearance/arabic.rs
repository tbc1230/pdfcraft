//! Arabic text in a text box (FreeText): laid out and shaped with the craft-fonts Arabic face.
//!
//! Lines are ordered with the Unicode bidirectional algorithm (UAX #9), Arabic words are shaped
//! with `pdfcraft_fonts::shape_arabic` (letters take their joining forms), and each shaped cluster
//! is drawn with a Type 3 font built from the face's glyph outlines, with a ToUnicode map and an
//! `/ActualText` span per line so copy and search get the text as typed. This follows the way the
//! edit crate draws Arabic in added text (`crates/edit/src/added`); the annotation crate sits
//! below it, so the small part it needs is kept here. Everything works when the build has no
//! craft-fonts: [`applies`] is then false and a text box draws as before (Arabic as `?`, marked).

use std::collections::HashMap;

use pdfcraft_cos::{Dict, Object, PdfString, Stream};
use pdfcraft_fonts::{
    GlyphOutline, ShapedCluster, ShapedScript, arabic_has, document_arabic_font, first_non_win_ansi, helvetica_width, literal, shape_arabic,
    shaped_glyph, win_ansi,
};
use unicode_bidi::{Level, ParagraphBidiInfo};

use crate::n;

/// Most glyphs in one Type 3 font: its codes are single bytes, 1-240.
const GLYPHS_PER_FONT: usize = 240;

/// Arabic letters, marks, digits and punctuation.
pub(crate) fn is_arabic(c: char) -> bool {
    matches!(u32::from(c), 0x0600..=0x06FF | 0x0750..=0x077F | 0x0870..=0x08FF | 0xFB50..=0xFDFF | 0xFE70..=0xFEFC)
}

/// Direction marks, embeddings, overrides and isolates: they steer the order and are not drawn.
fn is_bidi_control(c: char) -> bool {
    matches!(c, '\u{061C}' | '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}')
}

fn shaped_arabic(c: char) -> bool {
    is_arabic(c) && !is_bidi_control(c)
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
        '\u{ab}' => '\u{bb}',
        '\u{bb}' => '\u{ab}',
        other => other,
    }
}

/// Whether `text` is drawn with the Arabic face: it has Arabic and the build has the face.
pub(crate) fn applies(text: &str) -> bool {
    document_arabic_font().is_some() && text.chars().any(is_arabic)
}

/// A piece of a line, in drawing order.
enum Piece {
    /// Drawn in Helvetica, WinAnsi.
    Latin(String),
    Shaped(Vec<ShapedCluster>),
}

/// A laid-out line: its text as typed, whether its paragraph runs right to left, its pieces in
/// drawing order and its width at the size.
pub(crate) struct Line {
    pub text: String,
    pub rtl: bool,
    pieces: Vec<Piece>,
    pub width: f64,
}

fn paragraph_rtl(para: &str) -> bool {
    para.chars().any(is_arabic) && unicode_bidi::get_base_direction(para) == unicode_bidi::Direction::Rtl
}

/// `line` in drawing order and its width at `size`. Arabic (and the spaces and punctuation between
/// it) is shaped; what the face lacks, and Latin text, stays in Helvetica.
fn layout_line(line: &str, rtl: bool, size: f64) -> (Vec<Piece>, f64) {
    // unicode-bidi indexes the first level of a line, so an empty one would panic.
    if line.is_empty() {
        return (Vec::new(), 0.0);
    }
    let base = if rtl { Level::rtl() } else { Level::ltr() };
    let info = ParagraphBidiInfo::new(line, Some(base));
    let (levels, runs) = info.visual_runs(0..line.len());
    let mut pieces = Vec::new();
    for run in runs {
        let run_rtl = levels.get(run.start).is_some_and(Level::is_rtl);
        let Some(text) = line.get(run) else { continue };
        let mut segments: Vec<(bool, String)> = Vec::new();
        for c in text.chars().filter(|c| !is_bidi_control(*c)) {
            let arabic = is_arabic(c) || (run_rtl && !c.is_alphanumeric() && arabic_has(c));
            match segments.last_mut() {
                Some((kind, s)) if *kind == arabic => s.push(c),
                _ => segments.push((arabic, c.to_string())),
            }
        }
        if run_rtl {
            segments.reverse();
        }
        for (arabic, s) in segments {
            let latin = |s: String| {
                if run_rtl { Piece::Latin(s.chars().rev().map(mirrored).collect()) } else { Piece::Latin(s) }
            };
            pieces.push(if arabic {
                match shape_arabic(&s, run_rtl) {
                    Ok(clusters) => Piece::Shaped(clusters),
                    // A character the face lacks: reported by `first_undrawable`, drawn as `?`.
                    Err(_) => latin(s),
                }
            } else {
                latin(s)
            });
        }
    }
    let width = pieces
        .iter()
        .map(|p| match p {
            Piece::Latin(s) => helvetica_width(s, size),
            Piece::Shaped(clusters) => clusters.iter().map(|c| c.advance * size).sum::<f64>(),
        })
        .sum();
    (pieces, width)
}

/// The width of one line of `text` at `size`, shaped when `text` is Arabic.
pub(crate) fn line_width(text: &str, size: f64) -> f64 {
    layout_line(text, paragraph_rtl(text), size).1
}

/// `text` broken into lines of at most `width` points (a word wider than that keeps its own line),
/// each laid out. Paragraphs split on line breaks; words are shaped one by one (joining stops at a
/// space), so a line's width is the sum of its words' and the work stays linear.
pub(crate) fn wrap_lines(text: &str, size: f64, width: f64) -> Vec<Line> {
    let mut out = Vec::new();
    for para in text.replace("\r\n", "\n").split(['\n', '\r']) {
        let rtl = paragraph_rtl(para);
        let measure = |s: &str| layout_line(s, rtl, size).1;
        let space = measure(" ");
        let mut rows: Vec<String> = Vec::new();
        let (mut line, mut line_w) = (String::new(), 0.0);
        let mut fresh = true;
        for word in para.split(' ') {
            let word_w = measure(word);
            let candidate_w = if fresh { word_w } else { line_w + space + word_w };
            if !fresh && candidate_w > width {
                rows.push(std::mem::take(&mut line));
                line = word.to_string();
                line_w = word_w;
            } else {
                if !fresh {
                    line.push(' ');
                }
                line.push_str(word);
                line_w = candidate_w;
            }
            fresh = false;
        }
        rows.push(line);
        for text in rows {
            let (pieces, width) = layout_line(&text, rtl, size);
            out.push(Line { text, rtl, pieces, width });
        }
    }
    out
}

/// The first character of `text` that would be drawn as `?` when the Arabic face draws its Arabic:
/// Arabic the face lacks, or outside WinAnsi where Helvetica draws it. `text` is for [`applies`].
pub(crate) fn first_undrawable(text: &str) -> Option<char> {
    if let Some(c) = text.chars().find(|c| shaped_arabic(*c) && !arabic_has(*c)) {
        return Some(c);
    }
    for para in text.split(['\n', '\r']) {
        let (pieces, _) = layout_line(para, paragraph_rtl(para), 1.0);
        let bad = pieces.iter().find_map(|p| match p {
            Piece::Latin(s) => first_non_win_ansi(s),
            Piece::Shaped(_) => None,
        });
        if bad.is_some() {
            return bad;
        }
    }
    None
}

/// What makes two clusters the same Type 3 glyph.
type ClusterKey<'a> = (&'a str, Vec<(u32, u64, u64)>, u64);

fn cluster_key(c: &ShapedCluster) -> ClusterKey<'_> {
    (c.text.as_str(), c.glyphs.iter().map(|(id, [x, y])| (*id, x.to_bits(), y.to_bits())).collect(), c.advance.to_bits())
}

/// Draw `text` inside `rect` (pad points from its edges) into the text object `out` (`/Helv` is
/// current), laid out as the Helvetica path does: `q` is the form's alignment (0 left, 1 centre,
/// 2 right; none: a right-to-left paragraph at the right edge, others at the left), `round` centres
/// the block up and down. Returns the `/Font` entries (Type 3 fonts with inline streams) the content
/// needs, to join the form's resources.
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_lines(out: &mut Vec<u8>, text: &str, size: f64, rect: [f64; 4], pad: f64, width: f64, q: Option<i64>, round: bool) -> Dict {
    let lines = wrap_lines(text, size, width);
    // Every distinct cluster is one glyph of one of the fonts.
    let mut keys: Vec<&ShapedCluster> = Vec::new();
    let mut index: HashMap<ClusterKey<'_>, usize> = HashMap::new();
    for c in lines.iter().flat_map(|l| &l.pieces).flat_map(|p| match p {
        Piece::Shaped(clusters) => clusters.as_slice(),
        Piece::Latin(_) => &[],
    }) {
        index.entry(cluster_key(c)).or_insert_with(|| {
            keys.push(c);
            keys.len() - 1
        });
    }
    let mut outlines: HashMap<u32, GlyphOutline> = HashMap::new();
    for c in &keys {
        for (id, _) in &c.glyphs {
            if let std::collections::hash_map::Entry::Vacant(e) = outlines.entry(*id)
                && let Ok(g) = shaped_glyph(ShapedScript::Arabic, *id)
            {
                e.insert(g);
            }
        }
    }
    let mut fonts = Dict::new();
    let mut names = Vec::new();
    for (i, chunk) in keys.chunks(GLYPHS_PER_FONT).enumerate() {
        let name = format!("PCAr{i}");
        fonts.set(name.clone().into_bytes(), type3(chunk, &outlines));
        names.push(name);
    }
    let mut y = rect[3] - pad - size * 0.9;
    if round {
        y -= ((rect[3] - rect[1] - 2.0 * pad - lines.len() as f64 * size * 1.2) / 2.0).max(0.0);
    }
    let mut current = String::from("Helv");
    for line in &lines {
        if y < rect[1] - size {
            break;
        }
        let x = match q.unwrap_or(if line.rtl { 2 } else { 0 }) {
            1 => rect[0] + pad + (width - line.width) / 2.0,
            2 => rect[2] - pad - line.width,
            _ => rect[0] + pad,
        };
        // The line as typed, for copy and search.
        let logical: String = line.text.encode_utf16().map(|u| format!("{u:04X}")).collect();
        out.extend(format!("/Span <</ActualText <FEFF{logical}>>> BDC\n").bytes());
        let mut pen = x;
        for piece in &line.pieces {
            match piece {
                Piece::Latin(s) => {
                    if current != "Helv" {
                        out.extend(format!("/Helv {} Tf ", n(size)).bytes());
                        current = "Helv".into();
                    }
                    out.extend(format!("1 0 0 1 {} {} Tm ", n(pen), n(y)).bytes());
                    out.extend(literal(&win_ansi(s)));
                    out.extend_from_slice(b" Tj\n");
                    pen += helvetica_width(s, size);
                }
                Piece::Shaped(clusters) => {
                    for c in clusters {
                        let Some(&k) = index.get(&cluster_key(c)) else { continue };
                        let Some(name) = names.get(k / GLYPHS_PER_FONT) else { continue };
                        let code = u8::try_from(k % GLYPHS_PER_FONT + 1).unwrap_or(1);
                        if current != *name {
                            out.extend(format!("/{name} {} Tf ", n(size)).bytes());
                            current = name.clone();
                        }
                        out.extend(format!("1 0 0 1 {} {} Tm ", n(pen), n(y)).bytes());
                        out.extend(literal(&[code]));
                        out.extend_from_slice(b" Tj\n");
                        pen += c.advance * size;
                    }
                }
            }
        }
        out.extend_from_slice(b"EMC\n");
        y -= size * 1.2;
    }
    fonts
}

/// A Type 3 font with `clusters` as codes 1.., each drawn from the Arabic face's `outlines`. Its
/// glyph procedures and ToUnicode map are inline streams; the caller stores them as objects.
fn type3(clusters: &[&ShapedCluster], outlines: &HashMap<u32, GlyphOutline>) -> Object {
    let mut charprocs = Dict::new();
    let mut differences = vec![Object::Int(1)];
    let mut widths = Vec::with_capacity(clusters.len());
    let mut bbox = [0.0f64; 4];
    let mut to_unicode = Vec::new();
    for (i, c) in clusters.iter().enumerate() {
        let code = i + 1;
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
        let [x0, y0, x1, y1] =
            ink.map_or([0.0; 4], |b| [(b[0] * 1000.0).floor(), (b[1] * 1000.0).floor(), (b[2] * 1000.0).ceil(), (b[3] * 1000.0).ceil()]);
        bbox = [bbox[0].min(x0), bbox[1].min(y0), bbox[2].max(x1), bbox[3].max(y1)];
        let mut proc = format!("{} 0 {} {} {} {} d1\n", n(c.advance * 1000.0), n(x0), n(y0), n(x1), n(y1)).into_bytes();
        if !paths.is_empty() {
            proc.extend(paths);
            proc.extend_from_slice(b"f\n");
        }
        let glyph = format!("g{code:02X}");
        charprocs.set(glyph.clone().into_bytes(), Object::Stream(Stream::flate(Dict::new(), &proc)));
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
    for block in to_unicode.chunks(100) {
        cmap.push_str(&format!("{} beginbfchar\n{}endbfchar\n", block.len(), block.concat()));
    }
    cmap.push_str("endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n");
    let family = document_arabic_font().map_or("Arabic", |f| f.family);
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
    font.set(b"FontDescriptor".to_vec(), Object::Dict(descriptor));
    font.set(b"FontBBox".to_vec(), Object::Array(bbox.iter().map(|v| Object::Real(*v)).collect()));
    font.set(
        b"FontMatrix".to_vec(),
        Object::Array(vec![Object::Real(0.001), Object::Int(0), Object::Int(0), Object::Real(0.001), Object::Int(0), Object::Int(0)]),
    );
    font.set(b"FirstChar".to_vec(), Object::Int(1));
    font.set(b"LastChar".to_vec(), Object::Int(clusters.len() as i64));
    font.set(b"Widths".to_vec(), Object::Array(widths));
    font.set(b"Encoding".to_vec(), Object::Dict(encoding));
    font.set(b"CharProcs".to_vec(), Object::Dict(charprocs));
    font.set(b"ToUnicode".to_vec(), Object::Stream(Stream::flate(Dict::new(), cmap.as_bytes())));
    Object::Dict(font)
}
