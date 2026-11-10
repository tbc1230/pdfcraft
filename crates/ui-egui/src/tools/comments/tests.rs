use super::*;

#[test]
fn freehand_highlight_and_draw_accept_widths_up_to_100() {
    assert_eq!(CommentTool::FreehandHighlight.group(), CommentTool::Highlight.group());
    assert_eq!(GROUPS.len(), 3);
    let mut prefs = CommentPrefs::default();
    assert_eq!(prefs.style(CommentTool::FreehandHighlight).width, 14.0);
    assert_eq!(prefs.style(CommentTool::FreehandHighlight).opacity, 0.35);
    for tool in [CommentTool::Ink, CommentTool::FreehandHighlight] {
        prefs.set_width(tool, 75.0);
        assert_eq!(prefs.style(tool).width, 75.0);
        prefs.set_width(tool, 200.0);
        assert_eq!(prefs.style(tool).width, 100.0);
        prefs.set_width(tool, f64::NAN);
        assert_eq!(prefs.style(tool).width, 100.0);
    }
}

/// A selected line is drawn where it is going: moved by its body (both ends follow the
/// pointer), with an end dragged (that end alone), and as released until the page is drawn.
#[test]
fn a_moved_or_reshaped_line_is_drawn_at_its_new_points() {
    let line = [[10.0, 10.0], [50.0, 10.0]];
    let to_user = |p: Pos2| [f64::from(p.x), f64::from(p.y)];
    let moving = Gesture::Move { page: 0, index: 3, from: pos2(20.0, 10.0) };
    let at = Some(pos2(25.0, 30.0));
    assert_eq!(changing_points(Some(&moving), None, (0, 3), &line, to_user, at, false), Some(vec![[15.0, 30.0], [55.0, 30.0]]));
    assert_eq!(changing_points(Some(&moving), None, (0, 4), &line, to_user, at, false), None, "another comment's move");
    let released = Edit::MoveAnnotation { page: 0, index: 3, dx: 5.0, dy: 20.0 };
    assert_eq!(changing_points(None, Some(&released), (0, 3), &line, to_user, None, false), Some(vec![[15.0, 30.0], [55.0, 30.0]]));
    let dragging = Gesture::Reshape { page: 0, index: 3, point: 1, points: line.to_vec() };
    let to = Some(pos2(60.0, 10.0));
    assert_eq!(changing_points(Some(&dragging), None, (0, 3), &line, to_user, to, false), Some(vec![[10.0, 10.0], [60.0, 10.0]]));
    assert_eq!(changing_points(None, None, (0, 3), &line, to_user, to, false), None, "not changing");
    // Released with the rest of the selection: its own move in the batch.
    let together = Edit::Batch {
        label: "Move comments".into(),
        edits: vec![Edit::MoveAnnotation { page: 0, index: 1, dx: 1.0, dy: 1.0 }, Edit::MoveAnnotation { page: 0, index: 3, dx: 5.0, dy: 20.0 }],
    };
    assert_eq!(changing_points(None, Some(&together), (0, 3), &line, to_user, None, false), Some(vec![[15.0, 30.0], [55.0, 30.0]]));
    assert_eq!(changing_points(None, Some(&together), (0, 2), &line, to_user, None, false), None, "not in the batch");
}

/// A sticky note on a page turned a quarter is drawn upright beside its `/Rect`: an edit
/// composer stays open while the drawn note is partly in view, even with `/Rect` out of it.
#[test]
fn an_upright_note_on_a_turned_page_is_in_view_where_it_is_drawn() {
    let note = |no_rotate| Annotation {
        page: 0,
        subtype: "Text".into(),
        author: None,
        contents: None,
        modified: None,
        name: None,
        in_reply_to: None,
        parent: None,
        grouped: false,
        rect: [40.0, 20.0, 70.0, 40.0],
        color: None,
        index: 0,
        state: None,
        quads: Vec::new(),
        locked: false,
        intent: None,
        fill_sign: false,
        no_rotate,
        points: Vec::new(),
        popup: None,
    };
    let page = PageInfo { width: 140.0, height: 200.0, label: String::new(), crop: [0.0, 0.0, 200.0, 140.0], rotation: 90 };
    let xf = PageXform { rect: Rect::from_min_size(Pos2::ZERO, vec2(140.0, 200.0)), rot: 0, pw: 140.0, ph: 200.0 };
    let prefs = CommentPrefs::default();
    // Screen x is user y here: `/Rect` spans x 20..40, the upright note x 40..70.
    let viewport = Rect::from_min_max(pos2(50.0, 0.0), pos2(140.0, 200.0));
    for (no_rotate, in_view) in [(true, true), (false, false)] {
        let info = DocInfo { pages: vec![page.clone()], annotations: vec![note(no_rotate)], ..Default::default() };
        let cx = PageCx {
            page: 0,
            xf: &xf,
            ptr: &PagePointer::default(),
            info: &info,
            tool: QuickTool::Select,
            prefs: &prefs,
            allowed: true,
            hidden: false,
            redact_mark: ([0.0; 3], 1.0),
        };
        assert_eq!(edited_in_view(&cx, 0, viewport), in_view, "no_rotate: {no_rotate}");
    }
}

/// A listed comment or reply at `index` on `page`, replying to `parent`.
fn listed(page: usize, index: usize, contents: &str, parent: Option<(usize, usize)>) -> Annotation {
    Annotation {
        page,
        subtype: "Text".into(),
        author: None,
        contents: Some(contents.into()),
        modified: None,
        name: None,
        in_reply_to: None,
        parent,
        grouped: false,
        rect: [0.0, 0.0, 20.0, 20.0],
        color: None,
        index,
        state: None,
        quads: Vec::new(),
        locked: false,
        intent: None,
        fill_sign: false,
        no_rotate: false,
        points: Vec::new(),
        popup: None,
    }
}

/// What stays selected or open in the editor after a delete is found where it moved to: the
/// reply's own replies go with it (pop-ups, unlisted, are gone too), and a page that lost
/// something else, or whose annotation in that place reads differently, keeps nothing.
#[test]
fn a_kept_comment_is_found_where_a_delete_moved_it() {
    // Page 0's `/Annots`: reply A, its pop-up (unlisted), the comment, reply B, a reply to A.
    let before = vec![
        listed(0, 0, "A", Some((0, 2))),
        listed(0, 2, "comment", None),
        listed(0, 3, "B", Some((0, 2))),
        listed(0, 4, "to A", Some((0, 0))),
        listed(1, 0, "other page", None),
    ];
    // Deleting A takes its pop-up and the reply to it: the comment and B move up two places.
    let after = vec![listed(0, 0, "comment", None), listed(0, 1, "B", Some((0, 0))), listed(1, 0, "other page", None)];
    let gone = (0, 0);
    assert_eq!(after_delete(&before, &after, gone, (0, 2)), Some((0, 0)), "the comment");
    assert_eq!(after_delete(&before, &after, gone, (0, 3)), Some((0, 1)), "the other reply");
    assert_eq!(after_delete(&before, &after, gone, (0, 4)), None, "the reply to A went with it");
    assert_eq!(after_delete(&before, &after, gone, gone), None, "the deleted one");
    assert_eq!(after_delete(&before, &after, gone, (1, 0)), Some((1, 0)), "another page keeps its places");
    // The page lost more than A and its replies: nothing is guessed.
    assert_eq!(after_delete(&before, &after[..1], gone, (0, 2)), None);
    // Another annotation in the comment's place.
    let changed = vec![listed(0, 0, "something else", None), listed(0, 1, "B", Some((0, 0)))];
    assert_eq!(after_delete(&before, &changed, gone, (0, 2)), None);

    let mut view = CommentView { selected: Some((0, 2)), editing: Some((0, 3, "typed".into())), ..Default::default() };
    view.annotation_deleted(&before, &after, gone);
    assert_eq!((view.selected, view.editing), (Some((0, 0)), Some((0, 1, "typed".into()))), "both follow");
    let mut view = CommentView { editing: Some((0, 4, "typed".into())), editor_focus: Some(EditorFocus::Take), ..Default::default() };
    view.annotation_deleted(&before, &after, gone);
    assert_eq!((view.editing, view.editor_focus), (None, None), "the editor closes with the reply it was open on");
}

use pdfcraft_render::TextGlyph;

/// One glyph per character, `w` points wide, on a line of `em`-point text whose baseline is at `y`, drawn
/// along `direction` (a unit vector in view space, y down).
fn line(chars: &str, x: f32, y: f32, w: f32, em: f32, direction: [f32; 2]) -> PageText {
    let [dx, dy] = direction;
    // "Up" for the glyph: its writing direction turned a quarter counterclockwise (y down).
    let [ux, uy] = [dy, -dx];
    let glyphs = chars
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let start = [x + dx * w * i as f32, y + dy * w * i as f32];
            let at = |along: f32, up: f32| [start[0] + dx * along + ux * up, start[1] + dy * along + uy * up];
            let quad = [at(0.0, -0.2 * em), at(w, -0.2 * em), at(w, 0.8 * em), at(0.0, 0.8 * em)];
            let xs = quad.map(|p| p[0]);
            let ys = quad.map(|p| p[1]);
            let rect = [
                xs.into_iter().fold(f32::INFINITY, f32::min),
                ys.into_iter().fold(f32::INFINITY, f32::min),
                xs.into_iter().fold(f32::NEG_INFINITY, f32::max),
                ys.into_iter().fold(f32::NEG_INFINITY, f32::max),
            ];
            TextGlyph { text: c.to_string(), rect, quad, direction }
        })
        .collect::<Vec<_>>();
    let n = glyphs.len();
    PageText { glyphs, line_of: vec![0; n], space_before: vec![false; n] }
}

fn annotation(subtype: &str, contents: Option<&str>, fill_sign: bool) -> Annotation {
    Annotation {
        page: 0,
        subtype: subtype.into(),
        author: None,
        contents: contents.map(str::to_string),
        modified: None,
        name: None,
        in_reply_to: None,
        parent: None,
        grouped: false,
        rect: [0.0; 4],
        color: None,
        index: 0,
        state: None,
        quads: Vec::new(),
        locked: false,
        intent: None,
        fill_sign,
        no_rotate: false,
        points: Vec::new(),
        popup: None,
    }
}

#[test]
fn the_hover_tip_skips_fill_and_sign_marks_replies_and_bare_highlights_when_asked() {
    let prefs = CommentPrefs::default();
    assert_eq!(hover_tip(&annotation("FreeText", Some("Hello"), false), &prefs).as_deref(), Some("FreeText\nHello"));
    assert_eq!(hover_tip(&annotation("FreeText", Some("Hello"), true), &prefs), None);
    let reply = Annotation { in_reply_to: Some("1 0 R".into()), parent: Some((0, 1)), ..annotation("Text", Some("Agreed"), false) };
    assert_eq!(hover_tip(&reply, &prefs), None);
    let bare = Annotation { author: Some("Ana".into()), ..annotation("Highlight", None, false) };
    assert_eq!(hover_tip(&bare, &prefs).as_deref(), Some("Ana"));
    let hide = CommentPrefs { hide_empty_highlight_hover: true, ..CommentPrefs::default() };
    assert_eq!(hover_tip(&bare, &hide), None);
    assert_eq!(hover_tip(&annotation("Highlight", Some("Check this"), false), &hide).as_deref(), Some("Highlight\nCheck this"));
}

fn near(a: Option<([f32; 2], f32)>, b: ([f32; 2], f32)) -> bool {
    a.is_some_and(|([x, y], s)| (x - b.0[0]).abs() < 1e-3 && (y - b.0[1]).abs() < 1e-3 && (s - b.1).abs() < 1e-3)
}

#[test]
fn the_caret_snaps_to_the_nearer_edge_on_the_baseline() {
    let text = line("abcd", 20.0, 100.0, 10.0, 14.0, [1.0, 0.0]);
    // "c" spans 40..50: its left half snaps before it, its right half after it.
    assert!(near(caret_snap(&text, 43.0, 95.0), ([40.0, 100.0], 7.0)));
    assert!(near(caret_snap(&text, 47.0, 92.0), ([50.0, 100.0], 7.0)));
    // Just past the last character, on the line: after it.
    assert!(near(caret_snap(&text, 63.0, 99.0), ([60.0, 100.0], 7.0)));
}

#[test]
fn the_snapped_caret_follows_the_page_into_user_space() {
    let text = line("abcd", 20.0, 100.0, 10.0, 14.0, [1.0, 0.0]);
    let page = |rotation, user_unit: f32| PageInfo {
        width: 300.0 * user_unit,
        height: 200.0 * user_unit,
        label: "1".into(),
        crop: [0.0, 0.0, 300.0, 200.0],
        rotation,
    };
    // An unrotated page: view y = 100 is user y = 100 (y up from the bottom of a 200 pt page).
    let (tip, side) = snapped_caret(&page(0, 1.0), &text, 43.0, 95.0).expect("snaps");
    assert!((tip[0] - 40.0).abs() < 1e-3 && (tip[1] - 100.0).abs() < 1e-3 && (side - 7.0).abs() < 1e-3);
    // /UserUnit 2: the view is twice as large, so the same text is half as many user units.
    let doubled = line("abcd", 40.0, 200.0, 20.0, 28.0, [1.0, 0.0]);
    let (tip, side) = snapped_caret(&page(0, 2.0), &doubled, 86.0, 190.0).expect("snaps");
    assert!((tip[0] - 40.0).abs() < 1e-3 && (tip[1] - 100.0).abs() < 1e-3 && (side - 7.0).abs() < 1e-3, "{tip:?} {side}");
    // A rotated page: the caret would lie on its side, so it stays at the click.
    for rotation in [90, 180, 270] {
        assert_eq!(snapped_caret(&page(rotation, 1.0), &text, 43.0, 95.0), None, "/Rotate {rotation}");
    }
}

#[test]
fn away_from_upright_horizontal_text_the_caret_stays_at_the_click() {
    let text = line("abcd", 20.0, 100.0, 10.0, 14.0, [1.0, 0.0]);
    assert_eq!(caret_snap(&text, 200.0, 100.0), None, "far to the right");
    assert_eq!(caret_snap(&text, 40.0, 40.0), None, "far above the line");
    assert_eq!(caret_snap(&PageText::default(), 40.0, 100.0), None, "no text");
    for (direction, what) in [([0.0, 1.0], "vertical"), ([-1.0, 0.0], "upside down"), ([0.995, 0.0998], "sloped")] {
        let turned = line("abcd", 20.0, 100.0, 10.0, 14.0, direction);
        let [x0, y0, x1, y1] = turned.glyphs[2].rect;
        assert_eq!(caret_snap(&turned, (x0 + x1) / 2.0, (y0 + y1) / 2.0), None, "{what}");
    }
}

#[test]
fn recent_custom_colours_dedupe_cap_and_skip_swatches() {
    // #1193: "Other colour…" remembers what it applied, newest first.
    let ctx = egui::Context::default();
    assert!(recent_custom_colours(&ctx).is_empty());
    for i in 0..7 {
        remember_custom_colour(&ctx, [0.1 * f64::from(i), 0.3, 0.7]);
    }
    let recent = recent_custom_colours(&ctx);
    assert_eq!(recent.len(), RECENT_CUSTOM, "capped");
    assert!(same_colour(&recent[0], &[0.6, 0.3, 0.7]), "newest first: {recent:?}");
    remember_custom_colour(&ctx, [0.2, 0.3, 0.7]);
    let again = recent_custom_colours(&ctx);
    assert_eq!(again.len(), RECENT_CUSTOM);
    assert!(
        same_colour(&again[0], &[0.2, 0.3, 0.7]) && again.iter().filter(|c| same_colour(c, &[0.2, 0.3, 0.7])).count() == 1,
        "moved, not repeated"
    );
    // Swatches and unusable values are not remembered; out-of-range ones are clamped.
    remember_custom_colour(&ctx, SWATCHES[0].1);
    remember_custom_colour(&ctx, [f64::NAN, 0.0, 0.0]);
    assert!(same_colour(&recent_custom_colours(&ctx)[0], &[0.2, 0.3, 0.7]));
    remember_custom_colour(&ctx, [1.7, -0.5, 0.25]);
    assert!(same_colour(&recent_custom_colours(&ctx)[0], &[1.0, 0.0, 0.25]));
}

#[test]
fn other_colour_applies_the_picked_colour_once() {
    // #1193: the swatch grid has a full picker; Apply returns its colour and remembers it.
    use egui_kittest::kittest::Queryable;
    let current = [0.2, 0.4, 0.6];
    let mut h = egui_kittest::Harness::new_ui_state(
        |ui, picked: &mut Vec<Rgb>| {
            if let Some(c) = swatch_grid(ui, Some(current)) {
                picked.push(c);
            }
        },
        Vec::new(),
    );
    h.run();
    assert!(h.query_by_label("Apply").is_none(), "the picker starts closed");
    assert!(h.query_by_label("Recent colours").is_none());
    h.get_by_label("Other colour…").click();
    h.run();
    h.get_by_label("Apply").click();
    h.run();
    let picked = h.state().clone();
    assert_eq!(picked.len(), 1, "one choice: {picked:?}");
    assert!(same_colour(&picked[0], &current), "the picker opens on the current colour: {picked:?}");
    assert!(h.query_by_label("Apply").is_none(), "applying closes the picker");
    assert!(h.query_by_label("Recent colours").is_some(), "the colour is offered again");
}

#[test]
fn two_grids_in_one_parent_open_their_pickers_separately() {
    // Redaction properties and Prepare a form draw two grids in one parent ui.
    use egui_kittest::kittest::Queryable;
    let mut h = egui_kittest::Harness::new_ui(|ui| {
        let _ = swatch_grid(ui, Some([0.2, 0.4, 0.6]));
        let _ = swatch_grid(ui, Some([0.6, 0.4, 0.2]));
    });
    h.run();
    let plus = h.query_all_by_label("Other colour…").collect::<Vec<_>>();
    assert_eq!(plus.len(), 2);
    plus[0].click();
    h.run();
    assert_eq!(h.query_all_by_label("Apply").count(), 1, "only the first grid's picker opens");
}

#[test]
fn other_colour_opens_the_picker_inside_the_quick_bar_popup() {
    use egui_kittest::kittest::Queryable;
    let mut h = egui_kittest::Harness::new_ui_state(
        |ui, prefs: &mut CommentPrefs| {
            let _ = quick_bar_controls(ui, CommentTool::Highlight, prefs);
        },
        CommentPrefs::default(),
    );
    h.run();
    h.get_by_label("Colour").click();
    h.run();
    h.get_by_label("Other colour…").click();
    h.run();
    assert!(h.query_by_label("Apply").is_some(), "the popup stays open with the picker showing");
    h.get_by_label("Apply").click();
    h.run();
    assert!(h.query_by_label("Other colour…").is_none(), "applying a colour closes the popup");
}

#[test]
fn other_colour_opens_the_picker_inside_the_quick_bar_fill_popup() {
    use egui_kittest::kittest::Queryable;
    let prefs = CommentPrefs::default();
    let fill_label = if prefs.style(CommentTool::Rectangle).fill.is_some() { "Fill colour" } else { "Fill colour: none" };
    let mut h = egui_kittest::Harness::new_ui_state(
        |ui, prefs: &mut CommentPrefs| {
            let _ = quick_bar_controls(ui, CommentTool::Rectangle, prefs);
        },
        prefs,
    );
    h.ctx.set_fonts(theme::font_definitions());
    h.run();
    h.get_by_label(fill_label).click();
    h.run();
    h.get_by_label("Other colour…").click();
    h.run();
    assert!(h.query_by_label("Apply").is_some(), "the popup stays open with the picker showing");
    h.get_by_label("Apply").click();
    h.run();
    assert!(h.query_by_label("No fill").is_none(), "applying a colour closes the popup");
    assert!(h.state().style(CommentTool::Rectangle).fill.is_some());
}

#[test]
fn the_context_menu_has_no_colour_entry() {
    use egui_kittest::kittest::Queryable;
    let info = DocInfo {
        pages: vec![PageInfo { width: 300.0, height: 400.0, label: String::new(), crop: [0.0, 0.0, 300.0, 400.0], rotation: 0 }],
        annotations: vec![Annotation {
            page: 0,
            subtype: "Square".into(),
            author: None,
            contents: None,
            modified: None,
            name: None,
            in_reply_to: None,
            parent: None,
            grouped: false,
            rect: [10.0, 10.0, 50.0, 50.0],
            color: None,
            index: 0,
            state: None,
            quads: Vec::new(),
            locked: false,
            intent: None,
            fill_sign: false,
            points: Vec::new(),
            no_rotate: false,
            popup: None,
        }],
        ..Default::default()
    };
    let mut view = DocView::new(pdfcraft_engine::DocId(1), &info, crate::canvas::ViewDefaults::default());
    view.comments.selected = Some((0, 0));
    let prefs = CommentPrefs::default();
    let mut h = egui_kittest::Harness::new_ui_state(
        |ui, (view, actions): &mut (DocView, Vec<CanvasAction>)| {
            ui.button("Page").context_menu(|ui| {
                let rights = MenuRights { allowed: true, can_paste: false, can_modify: true };
                if let Some(a) = context_menu(ui, view, &info, &|_, _| true, &prefs, rights) {
                    actions.push(a);
                }
            });
        },
        (view, Vec::new()),
    );
    h.run();
    h.get_by_label("Page").click_secondary();
    h.run();
    h.get_by_label("Edit text…");
    assert!(h.query_by_label("Colour").is_none(), "the context menu has no Colour entry");
    assert!(h.state().1.is_empty(), "nothing was asked for");
}

#[test]
fn shift_keeps_the_current_proportions_unless_the_box_has_its_own() {
    let r = Rect::from_min_max(pos2(100.0, 100.0), pos2(196.0, 132.0));
    assert_eq!(corner_ratio(r, None, false), None);
    assert_eq!(corner_ratio(r, None, true), Some(3.0));
    assert_eq!(corner_ratio(r, Some(2.0), false), Some(2.0));
    assert_eq!(corner_ratio(r, Some(2.0), true), Some(2.0));
    // A degenerate box has no proportions to keep (no division by zero).
    let flat = Rect::from_min_max(pos2(100.0, 100.0), pos2(196.0, 100.0));
    assert_eq!(corner_ratio(flat, None, true), None);
    // Dragging a corner with Shift held scales the box instead of stretching it.
    let out = resized(r, (1, 1), vec2(48.0, 0.0), corner_ratio(r, None, true));
    assert_eq!(out.min, r.min);
    assert!((out.width() / out.height() - 3.0).abs() < 0.001);
}

#[test]
fn proportional_corners_keep_the_opposite_anchor_and_edges_stretch_one_axis() {
    let r = Rect::from_min_max(pos2(100.0, 100.0), pos2(196.0, 132.0));
    for (hx, hy) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
        for (dx, dy) in [(24.0, 0.0), (0.0, 16.0), (-48.0, -20.0), (-200.0, -200.0)] {
            let out = resized(r, (hx, hy), vec2(f32::from(hx) * dx, f32::from(hy) * dy), Some(3.0));
            assert_eq!(handle_pos(out, (-hx, -hy)), handle_pos(r, (-hx, -hy)));
            assert!((out.width() / out.height() - 3.0).abs() < 0.001);
            assert!(out.width() >= 4.0 && out.height() >= 4.0);
        }
    }
    for handle in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let out = resized(r, handle, vec2(24.0, 16.0), Some(3.0));
        assert_eq!(out, resized(r, handle, vec2(24.0, 16.0), None));
        assert_eq!(if handle.0 == 0 { out.width() } else { out.height() }, if handle.0 == 0 { r.width() } else { r.height() });
    }
    // Ordinary comment corners keep their existing independent width/height behavior.
    let free = resized(r, (1, 1), vec2(24.0, 0.0), None);
    assert_eq!(free.size(), vec2(120.0, 32.0));
}

#[test]
fn tools_round_trip_through_their_commands() {
    for t in ALL {
        assert_eq!(CommentTool::from_command(t.command()), Some(t));
        assert!(pdfcraft_engine::commands::command(t.command()).is_some(), "{} is registered", t.command());
        assert!(icons::exists(t.icon()), "icon {}", t.icon());
        assert!(GROUPS[t.group()].contains(&t));
    }
}

#[test]
fn drawn_shapes_ignore_slips() {
    assert_eq!(drawn_shape(CommentTool::Rectangle, &[[0.0, 0.0], [1.0, 1.0]]), None);
    assert_eq!(drawn_shape(CommentTool::Rectangle, &[[10.0, 0.0], [0.0, 10.0]]), Some(Shape::Rectangle { rect: [0.0, 0.0, 10.0, 10.0] }));
    assert_eq!(drawn_shape(CommentTool::Ink, &[[0.0, 0.0]]), Some(Shape::Ink { strokes: vec![vec![[0.0, 0.0]]] }));
    assert!(matches!(
        drawn_shape(CommentTool::Arrow, &[[0.0, 0.0], [5.0, 0.0]]),
        Some(Shape::Line { start: LineEnding::None, end: LineEnding::OpenArrow, .. })
    ));
    // Polygon and cloud need three points, connected lines two.
    for tool in [CommentTool::Polygon, CommentTool::Cloud] {
        assert_eq!(drawn_shape(tool, &[[0.0, 0.0]]), None);
        assert_eq!(drawn_shape(tool, &[[0.0, 0.0], [10.0, 0.0]]), None);
        assert!(matches!(drawn_shape(tool, &[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]]), Some(Shape::Polygon { .. })));
    }
    assert_eq!(drawn_shape(CommentTool::PolyLine, &[[0.0, 0.0]]), None);
    assert!(matches!(drawn_shape(CommentTool::PolyLine, &[[0.0, 0.0], [10.0, 0.0]]), Some(Shape::PolyLine { .. })));
    // A flat drag (a side under 2 pt) places no rectangle and no oval.
    for tool in [CommentTool::Rectangle, CommentTool::Oval] {
        assert_eq!(drawn_shape(tool, &[[0.0, 0.0], [100.0, 0.0]]), None);
        assert_eq!(drawn_shape(tool, &[[0.0, 0.0], [0.0, 100.0]]), None);
        assert_eq!(drawn_shape(tool, &[[0.0, 0.0], [100.0, 1.9]]), None);
    }
    // A line or arrow shorter than 2 pt, and a click without a drag (one point), place nothing.
    for tool in [CommentTool::Line, CommentTool::Arrow] {
        assert_eq!(drawn_shape(tool, &[[0.0, 0.0], [1.0, 1.0]]), None);
        assert_eq!(drawn_shape(tool, &[[5.0, 5.0]]), None);
        assert_eq!(drawn_shape(tool, &[]), None);
    }
    for tool in [CommentTool::Rectangle, CommentTool::Oval] {
        assert_eq!(drawn_shape(tool, &[[5.0, 5.0]]), None);
    }
}

#[test]
fn shift_click_selects_the_range_from_the_comment_with_the_handles() {
    let order = [(0, 3), (0, 1), (0, 0), (1, 2), (1, 0)];
    let mut cv = CommentView::default();
    cv.select_only((0, 1));
    cv.select_range(&order, (1, 2));
    assert_eq!(cv.selection(), vec![(0, 1), (0, 0), (1, 2)], "from the anchor down, the anchor keeps the handles");
    // Another ⇧-click from the same anchor changes the range, upwards too.
    cv.select_range(&order, (0, 3));
    assert_eq!(cv.selection(), vec![(0, 1), (0, 3)]);
    // Nothing to start from, or a card not in the list: added as by ⌘/Ctrl-click.
    let mut none = CommentView::default();
    none.select_range(&order, (0, 0));
    assert_eq!(none.selection(), vec![(0, 0)]);
    cv.select_range(&order, (9, 9));
    assert_eq!(cv.selected, Some((9, 9)));
}

#[test]
fn text_boxes_fit_their_text() {
    let r = text_box_rect([100.0, 500.0], "Hi", 12.0, 0);
    assert_eq!((r[0], r[3]), (100.0, 500.0));
    assert!(r[2] - r[0] < 60.0);
    let long = text_box_rect([0.0, 500.0], &"word ".repeat(80), 12.0, 0);
    assert_eq!(long[2], 300.0);
    assert!(long[3] - long[1] > 50.0);
}

/// The hover outline goes round the comment a press under the pointer would take (the top hit
/// there): not a highlight under a form field, which takes the press.
#[test]
fn the_hover_outline_goes_round_what_a_press_would_take() {
    use super::paint::hover_outline;
    use crate::page_cursor::Layer;
    use crate::page_router::{CommentPart, Hit, Owner};
    let page = PageInfo { width: 200.0, height: 200.0, label: String::new(), crop: [0.0, 0.0, 200.0, 200.0], rotation: 0 };
    let xf = PageXform { rect: Rect::from_min_size(Pos2::ZERO, vec2(200.0, 200.0)), rot: 0, pw: 200.0, ph: 200.0 };
    let highlight = Annotation { rect: [50.0, 50.0, 150.0, 150.0], ..annotation("Highlight", None, false) };
    let info = DocInfo { pages: vec![page], annotations: vec![highlight], ..Default::default() };
    let prefs = CommentPrefs::default();
    // The pointer is over the highlight.
    let ptr = PagePointer { pos: Some(pos2(100.0, 100.0)), ..PagePointer::default() };
    let cx = PageCx {
        page: 0,
        xf: &xf,
        ptr: &ptr,
        info: &info,
        tool: QuickTool::Select,
        prefs: &prefs,
        allowed: true,
        hidden: false,
        redact_mark: ([0.0; 3], 1.0),
    };
    let on = |owner, layer| Hit::new(owner, layer, egui::CursorIcon::PointingHand);
    let comment = on(Owner::Comment { index: 0, part: CommentPart::Body }, Layer::Comment);
    let field = on(Owner::Field { field: 0, widget: 0 }, Layer::Field);
    let cv = CommentView::default();
    assert!(hover_outline(&cx, &cv, Some(&comment)).is_some(), "the highlight a press there selects");
    assert!(hover_outline(&cx, &cv, Some(&field)).is_none(), "not under a field that takes the press");
    assert!(hover_outline(&cx, &cv, None).is_none(), "nor where nothing would take it");
    let selected = CommentView { selected: Some((0, 0)), ..CommentView::default() };
    assert!(hover_outline(&cx, &selected, Some(&comment)).is_none(), "the selected comment has its own frame");
}

/// Runs one frame in which `key` is pressed, with `keys` for a selected item at (0, 0).
fn press_delete_key(key: egui::Key, allowed: bool, deletable: bool) -> (Option<Edit>, Option<(usize, usize)>) {
    let info = DocInfo {
        pages: vec![PageInfo { width: 300.0, height: 400.0, label: String::new(), crop: [0.0, 0.0, 300.0, 400.0], rotation: 0 }],
        ..Default::default()
    };
    let mut view = DocView::new(pdfcraft_engine::DocId(1), &info, crate::canvas::ViewDefaults::default());
    view.comments.selected = Some((0, 0));
    let ctx = egui::Context::default();
    let mut tool = QuickTool::Select;
    let input = egui::RawInput {
        events: vec![egui::Event::Key { key, physical_key: None, pressed: true, repeat: false, modifiers: egui::Modifiers::NONE }],
        ..Default::default()
    };
    ctx.run_ui(input, |ui| keys(ui.ctx(), &mut view, &mut tool, allowed, &|_, _| deletable)).drop_without_applying_deltas();
    (view.pending_edit.take(), view.comments.selected)
}

/// "Delete a markup item": the key asks the engine's delete rule first. A locked item, or a note
/// with a locked reply (`deletable` is false for both), is not deleted: no edit, the selection
/// stays and no notice is raised. Without annotate permission the key does nothing either.
#[test]
fn delete_and_backspace_keep_an_item_that_is_not_deletable_and_do_nothing_without_permission() {
    for key in [egui::Key::Delete, egui::Key::Backspace] {
        let (edit, selected) = press_delete_key(key, true, true);
        assert!(matches!(edit, Some(Edit::DeleteAnnotation { page: 0, index: 0 })), "a deletable item is deleted by {key:?}");
        assert_eq!(selected, None);
        let (edit, selected) = press_delete_key(key, true, false);
        assert!(edit.is_none(), "a locked item or a note with a locked reply is not deleted by {key:?}");
        assert_eq!(selected, Some((0, 0)), "the selection stays");
        let (edit, selected) = press_delete_key(key, false, true);
        assert!(edit.is_none(), "no annotate permission: {key:?} does nothing");
        assert_eq!(selected, Some((0, 0)));
    }
}

// ── moving, resizing and reshaping on the page; note pop-ups ────────────────────────────────

mod placed {
    use super::*;
    use crate::canvas::{DocView, ViewDefaults};
    use crate::page_router::{CommentPart, Hit, Owner};
    use pdfcraft_engine::DocId;

    use super::super::geometry::{PopupPart, reshapable, resizable, slipped};

    /// A 200 x 100 page (crop [0 0 200 100]) turned `rotation`, shown `rot` more by the view.
    /// 1 screen pixel is 1 point, the page's top-left at the origin.
    fn setup(rotation: u16, rot: u16, annotations: Vec<Annotation>) -> (DocInfo, PageXform) {
        let (w, h) = if rotation % 180 == 90 { (100.0, 200.0) } else { (200.0, 100.0) };
        let page = PageInfo { width: w, height: h, label: String::new(), crop: [0.0, 0.0, 200.0, 100.0], rotation };
        let (sw, sh) = if rot % 180 == 90 { (h, w) } else { (w, h) };
        let xf = PageXform { rect: Rect::from_min_size(Pos2::ZERO, vec2(sw, sh)), rot, pw: w, ph: h };
        (DocInfo { pages: vec![page], annotations, ..Default::default() }, xf)
    }

    fn comment(subtype: &str, index: usize, rect: [f32; 4]) -> Annotation {
        Annotation { subtype: subtype.into(), index, rect, ..listed(0, index, "text", None) }
    }

    fn cx<'a>(info: &'a DocInfo, xf: &'a PageXform, prefs: &'a CommentPrefs, ptr: &'a PagePointer, allowed: bool) -> PageCx<'a> {
        PageCx { page: 0, xf, ptr, info, tool: QuickTool::Select, prefs, allowed, hidden: false, redact_mark: ([0.0; 3], 1.0) }
    }

    fn view(info: &DocInfo) -> DocView {
        DocView::new(DocId(1), info, ViewDefaults::default())
    }

    /// The edit a gesture leaves when the pointer is released at `to` (screen).
    fn dragged(cx: &PageCx<'_>, view: &mut DocView, gesture: Gesture, to: Pos2) -> Option<Edit> {
        view.comments.gesture = Some(gesture);
        view.pending_edit = None;
        finish_select_gesture(false, cx, view, to);
        view.pending_edit.take()
    }

    fn near(a: [f64; 4], b: [f64; 4]) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 0.01)
    }

    fn inside(r: Rect, page: Rect) -> bool {
        r.min.x >= page.min.x - 0.01 && r.min.y >= page.min.y - 0.01 && r.max.x <= page.max.x + 0.01 && r.max.y <= page.max.y + 0.01
    }

    #[test]
    fn a_locked_comment_shows_no_handle() {
        let mut square = comment("Square", 0, [10.0, 10.0, 50.0, 50.0]);
        assert!(resizable(&square));
        square.locked = true;
        assert!(!resizable(&square), "no resize handle on a locked square");
        let mut line = comment("Line", 1, [10.0, 10.0, 50.0, 50.0]);
        line.points = vec![[10.0, 10.0], [50.0, 50.0]];
        assert!(reshapable(&line));
        line.locked = true;
        assert!(!reshapable(&line), "no end handle on a locked line");
    }

    #[test]
    fn the_slip_threshold_is_two_page_points_whatever_the_zoom() {
        let o = pos2(10.0, 10.0);
        assert!(!slipped(o, pos2(11.9, 10.0), 1.0));
        assert!(slipped(o, pos2(12.0, 10.0), 1.0));
        // At 4 pixels per point, 2 pt is 8 px.
        assert!(!slipped(o, pos2(17.9, 10.0), 4.0));
        assert!(slipped(o, pos2(18.0, 10.0), 4.0));
        assert!(!slipped(o, pos2(100.0, 10.0), 0.0), "a page with no size on screen moves nothing");
    }

    #[test]
    fn a_click_or_a_drag_below_the_slip_threshold_changes_nothing() {
        let square = comment("Square", 0, [10.0, 10.0, 50.0, 50.0]);
        let (info, xf) = setup(0, 0, vec![square]);
        let (prefs, ptr) = (CommentPrefs::default(), PagePointer::default());
        let c = cx(&info, &xf, &prefs, &ptr, true);
        let mut v = view(&info);
        let from = pos2(30.0, 70.0);
        assert!(dragged(&c, &mut v, Gesture::Move { page: 0, index: 0, from }, from + vec2(1.5, 0.0)).is_none());
        assert!(dragged(&c, &mut v, Gesture::Move { page: 0, index: 0, from }, from).is_none(), "a click");
        // A handle dragged one point: not a resize.
        let corner = pos2(50.0, 90.0);
        let resize = Gesture::Resize { page: 0, index: 0, handle: (1, 1), from: corner, aspect_ratio: None };
        assert!(dragged(&c, &mut v, resize.clone(), corner + vec2(1.0, 1.0)).is_none());
        assert!(matches!(dragged(&c, &mut v, resize, corner + vec2(10.0, 10.0)), Some(Edit::ResizeAnnotation { .. })));
        let mut line = comment("Line", 1, [10.0, 10.0, 50.0, 50.0]);
        line.points = vec![[10.0, 10.0], [50.0, 50.0]];
        let (info, xf) = setup(0, 0, vec![line]);
        let c = cx(&info, &xf, &prefs, &ptr, true);
        let end = c.to_screen([50.0, 50.0]);
        let reshape = Gesture::Reshape { page: 0, index: 0, point: 1, points: vec![[10.0, 10.0], [50.0, 50.0]] };
        assert!(dragged(&c, &mut v, reshape.clone(), end + vec2(1.0, 0.0)).is_none());
        assert!(matches!(dragged(&c, &mut v, reshape, end + vec2(8.0, 0.0)), Some(Edit::ReshapeAnnotation { .. })));
    }

    /// Whatever the page's `/Rotate` and the view's rotation, a move, a resize and a reshape stop at
    /// the page edge: the object ends where dropped, inside the page, with the dragged size.
    #[test]
    fn a_move_resize_or_reshape_stops_at_the_page_edge_on_every_rotation() {
        for rotation in [0u16, 90, 180, 270] {
            for rot in [0u16, 90, 180, 270] {
                let square = comment("Square", 0, [80.0, 30.0, 120.0, 60.0]);
                let (info, xf) = setup(rotation, rot, vec![square.clone()]);
                let (prefs, ptr) = (CommentPrefs::default(), PagePointer::default());
                let c = cx(&info, &xf, &prefs, &ptr, true);
                let mut v = view(&info);
                let at = c.screen_rect(&square);
                let size = at.size();
                // Dragged far beyond every side in turn.
                for far in [vec2(1000.0, 0.0), vec2(-1000.0, 0.0), vec2(0.0, 1000.0), vec2(0.0, -1000.0), vec2(1000.0, 1000.0)] {
                    let from = at.center();
                    let Some(Edit::MoveAnnotation { dx, dy, .. }) = dragged(&c, &mut v, Gesture::Move { page: 0, index: 0, from }, from + far) else {
                        panic!("a move is made (page {rotation}, view {rot})");
                    };
                    let moved = [80.0 + dx as f32, 30.0 + dy as f32, 120.0 + dx as f32, 60.0 + dy as f32];
                    let r = xf.user_rect(&info, 0, moved);
                    assert!(inside(r, xf.rect), "{r:?} is inside {:?} (page {rotation}, view {rot}, drag {far:?})", xf.rect);
                    assert!((r.width() - size.x).abs() < 0.01 && (r.height() - size.y).abs() < 0.01, "the size is kept");
                    if far.x > 0.0 && far.y == 0.0 {
                        assert!((r.max.x - xf.rect.max.x).abs() < 0.01, "stops at the right edge");
                    }
                    if far.x < 0.0 {
                        assert!((r.min.x - xf.rect.min.x).abs() < 0.01, "stops at the left edge");
                    }
                }
                // Resized from the lower right corner far outside.
                let corner = at.right_bottom();
                let resize = Gesture::Resize { page: 0, index: 0, handle: (1, 1), from: corner, aspect_ratio: None };
                let Some(Edit::ResizeAnnotation { rect, .. }) = dragged(&c, &mut v, resize, corner + vec2(1000.0, 1000.0)) else {
                    panic!("a resize is made (page {rotation}, view {rot})");
                };
                let r = xf.user_rect(&info, 0, rect.map(|x| x as f32));
                assert!(inside(r, xf.rect), "resized {r:?} inside {:?} (page {rotation}, view {rot})", xf.rect);
                assert!((r.min - at.min).length() < 0.01, "the opposite corner stays");
            }
        }
        // A line's end dragged off the page stops on its edge.
        let mut line = comment("Line", 0, [20.0, 20.0, 60.0, 60.0]);
        line.points = vec![[20.0, 20.0], [60.0, 60.0]];
        let (info, xf) = setup(0, 0, vec![line]);
        let (prefs, ptr) = (CommentPrefs::default(), PagePointer::default());
        let c = cx(&info, &xf, &prefs, &ptr, true);
        let mut v = view(&info);
        let reshape = Gesture::Reshape { page: 0, index: 0, point: 1, points: vec![[20.0, 20.0], [60.0, 60.0]] };
        let Some(Edit::ReshapeAnnotation { points, .. }) = dragged(&c, &mut v, reshape, pos2(5000.0, -5000.0)) else { panic!("a reshape is made") };
        assert!((points[1][0] - 200.0).abs() < 0.01 && (points[1][1] - 100.0).abs() < 0.01, "the end stops at the page's corner: {points:?}");
    }

    #[test]
    fn dragging_a_group_to_the_edge_keeps_its_arrangement() {
        let (info, xf) = setup(0, 0, vec![comment("Square", 0, [10.0, 10.0, 30.0, 30.0]), comment("Circle", 1, [150.0, 10.0, 190.0, 30.0])]);
        let (prefs, ptr) = (CommentPrefs::default(), PagePointer::default());
        let c = cx(&info, &xf, &prefs, &ptr, true);
        let mut v = view(&info);
        v.comments.also.push((0, 1));
        let from = pos2(20.0, 80.0);
        let Some(Edit::Batch { edits, .. }) = dragged(&c, &mut v, Gesture::Move { page: 0, index: 0, from }, from + vec2(500.0, 0.0)) else {
            panic!("a batch");
        };
        // The right-most one reaches the edge (10 more points) and the other goes the same way.
        let dxs: Vec<f64> = edits.iter().map(|e| if let Edit::MoveAnnotation { dx, .. } = e { *dx } else { f64::NAN }).collect();
        assert!(dxs.iter().all(|d| (d - 10.0).abs() < 0.01), "{dxs:?}");
    }

    fn note(popup: Option<([f32; 4], bool)>) -> Annotation {
        Annotation { popup, author: Some("Ada".into()), contents: Some("Hello".into()), ..comment("Text", 0, [20.0, 60.0, 40.0, 80.0]) }
    }

    #[test]
    fn the_pop_up_parts_round_trip_through_the_routers_parts() {
        for part in [PopupPart::Close, PopupPart::Text, PopupPart::Bar, PopupPart::Edge(1, 1), PopupPart::Edge(-1, 0), PopupPart::Edge(0, 1)] {
            assert_eq!(PopupPart::of(part.part()), Some(part));
        }
        assert_eq!(PopupPart::of(CommentPart::Body), None);
    }

    #[test]
    fn an_open_pop_up_is_hit_by_part_and_only_a_closed_note_is_its_icon() {
        // The pop-up: page x 60..160, y 20..80, which is screen y 20..80 on this 100 high page.
        let open = note(Some(([60.0, 20.0, 160.0, 80.0], true)));
        let (info, xf) = setup(0, 0, vec![open]);
        let (prefs, ptr) = (CommentPrefs::default(), PagePointer::default());
        let v = view(&info);
        let owner = |allowed: bool, p: Pos2| {
            let c = cx(&info, &xf, &prefs, &ptr, allowed);
            hit(&c, &v, p, false).map(|h: Hit| h.owner)
        };
        let part = |part| Some(Owner::Comment { index: 0, part });
        assert_eq!(owner(true, pos2(100.0, 30.0)), part(CommentPart::PopupBar), "the title bar");
        assert_eq!(owner(true, pos2(100.0, 60.0)), part(CommentPart::PopupText), "the text");
        assert_eq!(owner(true, pos2(150.0, 31.0)), part(CommentPart::PopupClose), "the close control");
        assert_eq!(owner(true, pos2(160.0, 50.0)), part(CommentPart::PopupEdge(1, 0)), "the right edge");
        assert_eq!(owner(true, pos2(160.0, 80.0)), part(CommentPart::PopupEdge(1, 1)), "the corner");
        // Not permitted: only the text, no grip.
        assert_eq!(owner(false, pos2(100.0, 30.0)), part(CommentPart::PopupText));
        assert_eq!(owner(false, pos2(165.0, 85.0)), None);
        // The icon: page x 20..40, y 60..80 is screen y 20..40.
        assert_eq!(owner(true, pos2(30.0, 30.0)), part(CommentPart::Body));
        // A locked note's pop-up is only readable.
        let mut locked = note(Some(([60.0, 20.0, 160.0, 80.0], true)));
        locked.locked = true;
        let (info, xf) = setup(0, 0, vec![locked]);
        let c = cx(&info, &xf, &prefs, &ptr, true);
        assert_eq!(hit(&c, &v, pos2(100.0, 30.0), false).map(|h| h.owner), part(CommentPart::PopupText));
    }

    #[test]
    fn the_pop_up_moves_by_its_title_bar_and_resizes_by_its_edges_within_the_page() {
        let (info, xf) = setup(0, 0, vec![note(Some(([60.0, 20.0, 160.0, 80.0], true)))]);
        let (prefs, ptr) = (CommentPrefs::default(), PagePointer::default());
        let c = cx(&info, &xf, &prefs, &ptr, true);
        let mut v = view(&info);
        let bar = pos2(100.0, 30.0);
        let Some(Edit::SetNotePopup { open: true, rect: Some(r), .. }) =
            dragged(&c, &mut v, Gesture::PopupMove { page: 0, index: 0, from: bar }, bar + vec2(1000.0, 1000.0))
        else {
            panic!("the pop-up is moved")
        };
        // Pushed to the right and down it ends in the page's lower right corner (page y 0..60).
        assert!(near(r, [100.0, 0.0, 200.0, 60.0]), "{r:?}");
        assert!(dragged(&c, &mut v, Gesture::PopupMove { page: 0, index: 0, from: bar }, bar + vec2(1.0, 0.0)).is_none(), "below the slip threshold");
        let corner = pos2(160.0, 80.0);
        let Some(Edit::SetNotePopup { rect: Some(r), .. }) =
            dragged(&c, &mut v, Gesture::PopupResize { page: 0, index: 0, handle: (1, 1), from: corner }, corner + vec2(1000.0, 1000.0))
        else {
            panic!("the pop-up is resized")
        };
        assert!(near(r, [60.0, 0.0, 200.0, 80.0]), "the corner stops at the page edge: {r:?}");
        // Shrunk to nothing it keeps its least size.
        let Some(Edit::SetNotePopup { rect: Some(r), .. }) =
            dragged(&c, &mut v, Gesture::PopupResize { page: 0, index: 0, handle: (1, 1), from: corner }, pos2(65.0, 25.0))
        else {
            panic!("the pop-up is resized")
        };
        assert!(near([r[2] - r[0], r[3] - r[1], 0.0, 0.0], [40.0, 40.0, 0.0, 0.0]), "{r:?}");
    }

    #[test]
    fn opening_puts_the_pop_up_inside_the_page_and_closing_writes_only_the_state() {
        // Its default place (right of the icon) lies off the page's right edge.
        let closed = note(Some(([180.0, 20.0, 380.0, 80.0], false)));
        let (info, xf) = setup(0, 0, vec![closed.clone()]);
        let (prefs, ptr) = (CommentPrefs::default(), PagePointer::default());
        let c = cx(&info, &xf, &prefs, &ptr, true);
        let Edit::Batch { edits, label } = popup_edit(&c, &closed, true) else { panic!("one named step") };
        assert_eq!(label, "Open note");
        let [Edit::SetNotePopup { open: true, rect: Some(r), .. }] = &edits[..] else { panic!("{edits:?}") };
        assert!(r[0] >= 0.0 && r[2] <= 200.0 && r[1] >= 0.0 && r[3] <= 100.0, "{r:?} is inside the page");
        assert!(near([r[2] - r[0], r[3] - r[1], 0.0, 0.0], [200.0, 60.0, 0.0, 0.0]), "its size is kept: {r:?}");
        assert!(matches!(popup_edit(&c, &closed, false), Edit::SetNotePopup { page: 0, index: 0, open: false, rect: None }));
    }

    #[test]
    fn text_markup_is_not_moved_or_resized() {
        let mut highlight = comment("Highlight", 0, [10.0, 10.0, 90.0, 24.0]);
        highlight.quads = vec![[10.0, 24.0, 90.0, 24.0, 10.0, 10.0, 90.0, 10.0]];
        assert!(!resizable(&highlight));
        let (info, xf) = setup(0, 0, vec![highlight]);
        let (prefs, ptr) = (CommentPrefs::default(), PagePointer::default());
        let v = view(&info);
        let c = cx(&info, &xf, &prefs, &ptr, true);
        let h = hit(&c, &v, pos2(50.0, 83.0), false).expect("the highlight is under the pointer");
        assert!(h.declines_drag, "a drag across a highlight selects text; it is not moved");
    }
}
