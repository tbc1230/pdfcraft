//! The comment list and its summary.

/// The comment list, read from the object graph (as `inspect` lists comments).
pub(crate) fn comment_list(doc: &pdfcraft_cos::Document) -> Vec<pdfcraft_render::Annotation> {
    let mut list: Vec<pdfcraft_render::Annotation> = pdfcraft_annot::summaries(doc)
        .into_iter()
        .map(|s| pdfcraft_render::Annotation {
            page: s.page,
            subtype: s.subtype,
            author: s.author,
            contents: s.contents,
            modified: s.modified.map(|m| pdfcraft_render::pretty_date(&m)),
            name: s.name,
            in_reply_to: s.in_reply_to,
            parent: s.parent,
            grouped: s.grouped,
            rect: s.rect,
            color: s.color,
            index: s.index,
            state: s.state,
            quads: s.quads,
            locked: s.locked,
            intent: s.intent,
            fill_sign: s.fill_sign,
            no_rotate: s.no_rotate,
            points: s.points,
            popup: s.popup,
        })
        .collect();
    pdfcraft_render::link_replies(&mut list);
    list
}

/// Order of Summarize Comments.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SummarySort {
    #[default]
    Page,
    Author,
    Date,
    Type,
}

impl SummarySort {
    pub const ALL: [SummarySort; 4] = [SummarySort::Page, SummarySort::Author, SummarySort::Date, SummarySort::Type];
    pub fn name(self) -> &'static str {
        match self {
            SummarySort::Page => "Page",
            SummarySort::Author => "Author",
            SummarySort::Date => "Date",
            SummarySort::Type => "Type",
        }
    }
    pub fn parse(s: &str) -> Option<SummarySort> {
        Self::ALL.into_iter().find(|k| k.name().eq_ignore_ascii_case(s))
    }
}

/// Acrobat's name for a comment type, as comment lists show it.
pub fn comment_type_name(subtype: &str) -> &str {
    match subtype {
        "Text" => "Sticky Note",
        "FreeText" => "Text Box",
        "Highlight" => "Highlight",
        "Underline" => "Underline",
        "StrikeOut" => "Strikethrough",
        "Squiggly" => "Squiggly",
        "Square" => "Rectangle",
        "Circle" => "Oval",
        "Line" => "Line",
        "Ink" => "Pencil",
        "Polygon" => "Polygon",
        "PolyLine" => "Polygonal Line",
        "Stamp" => "Stamp",
        "Caret" => "Inserted Text",
        "FileAttachment" => "File Attachment",
        "Sound" => "Sound",
        "Redact" => "Redaction",
        other => other,
    }
}

/// [`comment_type_name`], telling callouts, clouds and typewriter text apart by `/IT`.
pub fn comment_kind(a: &pdfcraft_render::Annotation) -> &str {
    match a.intent.as_deref() {
        Some("FreehandHighlight") if a.subtype == "Ink" => "Freehand Highlight",
        Some("FreeTextCallout") => "Callout",
        Some("PolygonCloud") => "Cloud",
        Some("FreeTextTypeWriter") => "Typewriter",
        _ => comment_type_name(&a.subtype),
    }
}

/// The text of a comment summary: one block per comment (replies indented under it), with a
/// "Page N" heading when sorted by page. Checkmarks and status replies are left out, as are
/// pop-ups; the number is the comment's position on its page.
pub fn comment_summary(name: &str, all: &[pdfcraft_render::Annotation], sort: SummarySort) -> String {
    use std::fmt::Write;
    let top: Vec<&pdfcraft_render::Annotation> = all.iter().filter(|a| a.is_comment()).collect();
    // Each reply under the annotation its `/IRT` points at, in document order, gathered once.
    type Children<'a> = std::collections::HashMap<(usize, usize), Vec<&'a pdfcraft_render::Annotation>>;
    let mut children = Children::new();
    for r in all.iter().filter(|r| r.state.is_none()) {
        if let Some(parent) = r.parent {
            children.entry(parent).or_default().push(r);
        }
    }
    // Write a comment's replies, indenting each level. A reply to a reply keeps its `/IRT` on the
    // direct parent, so following one level would leave the deeper replies out of the summary.
    fn write_replies(out: &mut String, children: &Children<'_>, parent: &pdfcraft_render::Annotation, depth: usize) {
        use std::fmt::Write;
        if depth > 64 {
            return;
        }
        for r in children.get(&(parent.page, parent.index)).into_iter().flatten() {
            let indent = "    ".repeat(depth + 1);
            let _ =
                writeln!(out, "{indent}Author: {}  Subject: Reply  Date: {}", r.author.as_deref().unwrap_or(""), r.modified.as_deref().unwrap_or(""));
            for line in r.contents.as_deref().unwrap_or("").lines() {
                let _ = writeln!(out, "{indent}{line}");
            }
            write_replies(out, children, r, depth + 1);
        }
    }
    let mut numbered: Vec<(usize, &pdfcraft_render::Annotation)> = Vec::new();
    let mut last = usize::MAX;
    let mut n = 0;
    for a in &top {
        if a.page != last {
            last = a.page;
            n = 0;
        }
        n += 1;
        numbered.push((n, a));
    }
    match sort {
        SummarySort::Page => {}
        SummarySort::Author => numbered.sort_by(|x, y| x.1.author.cmp(&y.1.author)),
        SummarySort::Date => numbered.sort_by(|x, y| x.1.modified.cmp(&y.1.modified)),
        SummarySort::Type => numbered.sort_by(|x, y| comment_kind(x.1).cmp(comment_kind(y.1))),
    }
    let mut out = format!("Summary of Comments on {name}\n\n");
    if numbered.is_empty() {
        out.push_str("This document has no comments.\n");
        return out;
    }
    let mut heading = usize::MAX;
    for (n, a) in numbered {
        if sort == SummarySort::Page && a.page != heading {
            heading = a.page;
            let _ = writeln!(out, "Page: {}", a.page + 1);
        }
        let _ = write!(out, "Number: {n}  Author: {}  Subject: {}", a.author.as_deref().unwrap_or(""), comment_kind(a));
        if sort != SummarySort::Page {
            let _ = write!(out, "  Page: {}", a.page + 1);
        }
        let _ = writeln!(out, "  Date: {}", a.modified.as_deref().unwrap_or(""));
        if let Some(c) = a.contents.as_deref().filter(|c| !c.is_empty()) {
            let _ = writeln!(out, "{c}");
        }
        write_replies(&mut out, &children, a, 0);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod popup_tests {
    use std::sync::Arc;

    use crate::edit::check_permission;
    use crate::{DocId, Edit, EditError, NewAnnotation, NoteIcon, Session, Shape, Style};

    fn session_with_note() -> (Session, DocId) {
        let mut s = Session::new().with_clock(|| 1_700_000_000);
        let id = s.open("fixture.pdf", None, Arc::new(crate::tests::fixture(1)), None).expect("opens");
        let shape = Shape::Note { at: [50.0, 200.0], icon: NoteIcon::Comment };
        let style = Style::default_for(&shape);
        s.apply(id, Edit::AddAnnotation(NewAnnotation { page: 0, shape, style, contents: "Hello".into(), author: "Ada".into() })).unwrap();
        (s, id)
    }

    fn popup(s: &Session, id: DocId) -> ([f32; 4], bool) {
        s.get(id).unwrap().info.annotations.iter().find(|a| a.subtype == "Text").and_then(|a| a.popup).expect("a note with a pop-up")
    }

    #[test]
    fn opening_closing_and_placing_a_pop_up_are_undo_steps_and_are_saved() {
        let (mut s, id) = session_with_note();
        assert!(!popup(&s, id).1, "a new note is closed");
        s.apply(id, Edit::SetNotePopup { page: 0, index: 0, open: true, rect: None }).unwrap();
        assert_eq!(s.get(id).unwrap().can_undo(), Some("Open note"));
        assert!(popup(&s, id).1);
        s.apply(id, Edit::SetNotePopup { page: 0, index: 0, open: true, rect: Some([60.0, 40.0, 190.0, 160.0]) }).unwrap();
        assert_eq!(s.get(id).unwrap().can_undo(), Some("Change pop-up"));
        assert_eq!(popup(&s, id), ([60.0, 40.0, 190.0, 160.0], true));
        // The saved file opens with the pop-up open, where it was left.
        let bytes = s.save_bytes(id).unwrap();
        let mut again = Session::new();
        let reopened = again.open("saved.pdf", None, bytes, None).expect("reopens");
        assert_eq!(popup(&again, reopened), ([60.0, 40.0, 190.0, 160.0], true));
        s.undo(id).unwrap();
        assert_ne!(popup(&s, id).0, [60.0, 40.0, 190.0, 160.0], "one undo takes the placement back");
        assert!(popup(&s, id).1, "and leaves it open");
        s.redo(id).unwrap();
        assert_eq!(popup(&s, id), ([60.0, 40.0, 190.0, 160.0], true));
        s.apply(id, Edit::SetNotePopup { page: 0, index: 0, open: false, rect: None }).unwrap();
        assert_eq!(s.get(id).unwrap().can_undo(), Some("Close note"));
        assert!(!popup(&s, id).1);
        s.undo(id).unwrap();
        assert!(popup(&s, id).1);
    }

    #[test]
    fn a_locked_note_keeps_its_pop_up_and_annotating_is_needed() {
        let (mut s, id) = session_with_note();
        s.apply(id, Edit::LockAnnotation { page: 0, index: 0, locked: true }).unwrap();
        assert!(s.apply(id, Edit::SetNotePopup { page: 0, index: 0, open: true, rect: None }).is_err());
        assert!(!popup(&s, id).1);
        let none = pdfcraft_cos::Permissions { bits: 0, owner: false, revision: 3 };
        let edit = Edit::SetNotePopup { page: 0, index: 0, open: true, rect: None };
        assert_eq!(check_permission(&edit, &none), Err(EditError::NotPermitted("comments")));
        assert_eq!(crate::history::scope_of(&edit), crate::history::Scope::Comments);
    }

    #[test]
    fn a_move_a_resize_and_a_reshape_are_each_one_undo_step_and_keep_the_author() {
        let mut s = Session::new().with_clock(|| 1_700_000_000);
        let id = s.open("fixture.pdf", None, Arc::new(crate::tests::fixture(1)), None).expect("opens");
        for shape in [
            Shape::Rectangle { rect: [20.0, 20.0, 60.0, 60.0] },
            Shape::Line { from: [20.0, 100.0], to: [80.0, 100.0], start: crate::LineEnding::None, end: crate::LineEnding::None },
        ] {
            let style = Style { color: [1.0, 0.0, 0.0], opacity: 1.0, width: 2.0, fill: None, ..Style::default() };
            s.apply(id, Edit::AddAnnotation(NewAnnotation { page: 0, shape, style, contents: "x".into(), author: "Other user".into() })).unwrap();
        }
        let author = |s: &Session| -> Vec<Option<String>> { s.get(id).unwrap().info.annotations.iter().map(|a| a.author.clone()).collect() };
        let rect = |s: &Session, i: usize| s.get(id).unwrap().info.annotations.iter().find(|a| a.index == i).unwrap().rect;
        let before = rect(&s, 0);
        for (edit, label) in [
            (Edit::MoveAnnotation { page: 0, index: 0, dx: 10.0, dy: 5.0 }, "Move comment"),
            (Edit::ResizeAnnotation { page: 0, index: 0, rect: [30.0, 25.0, 90.0, 80.0] }, "Resize comment"),
            (Edit::ReshapeAnnotation { page: 0, index: 1, points: vec![[20.0, 100.0], [120.0, 140.0]] }, "Reshape comment"),
        ] {
            s.apply(id, edit).unwrap();
            assert_eq!(s.get(id).unwrap().can_undo(), Some(label));
            assert_eq!(author(&s), [Some("Other user".to_owned()), Some("Other user".to_owned())], "{label} keeps the author");
        }
        assert_eq!(rect(&s, 0), [30.0, 25.0, 90.0, 80.0]);
        s.undo(id).unwrap();
        assert_eq!(rect(&s, 0), [30.0, 25.0, 90.0, 80.0], "undoing the reshape leaves the square as resized");
        s.undo(id).unwrap();
        assert_eq!(rect(&s, 0), [before[0] + 10.0, before[1] + 5.0, before[2] + 10.0, before[3] + 5.0], "undoing the resize leaves it moved");
        s.undo(id).unwrap();
        assert_eq!(rect(&s, 0), before);
        s.redo(id).unwrap();
        assert_eq!(rect(&s, 0), [before[0] + 10.0, before[1] + 5.0, before[2] + 10.0, before[3] + 5.0], "redo repeats the move");
    }
}
