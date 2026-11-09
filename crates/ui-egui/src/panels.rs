//! Left tool panel (All tools + tool sub-panels, generated from the engine catalogue) and the
//! right-hand panels (Comments, Bookmarks, Pages, Fields, Layers, Attachments).

use egui::{Align, Align2, Color32, CornerRadius, Layout, Rect, Sense, Stroke, pos2, vec2};
use pdfcraft_engine::catalog::{self, Availability, TOOL_GROUPS, ToolGroup, ToolItem};
use pdfcraft_render::{DocInfo, FieldKind, OutlineItem};

use crate::theme::{self, Tokens};
use crate::{LeftPanel, Mode, PdfCraftApp, RightPanel, icons, widgets};

pub(super) mod all_tools_ui;
mod bookmarks;
pub(super) mod comments_panel;
mod edit;
pub(super) mod format_text_ui;
pub(super) mod organize_panel;
pub(super) mod page_selection_ui;
mod pages;
mod right;
pub(super) mod search_ui;
pub mod side_panels;
mod tools;

pub use bookmarks::BmAction;
pub(crate) use bookmarks::ellipsized_prefix;
pub use right::right_panel;
pub use tools::left_panel;
pub(crate) use tools::panel_header;

/// Workspace: tool groups the web build leaves out of the Tools pane. The first six were chosen by
/// the product owner; the rest are groups that no story of the backlog needs (the table is in
/// `fork/changes/interface-parts-hidden.md`). The groups left are `edit`, `organize`, `comment`
/// and `redact`.
const WORKSPACE_HIDDEN_TOOLS: &[&str] = &[
    "export",
    "create",
    "stamp",
    "standards",
    "javascript",
    "ai",
    "fill_sign",
    "combine",
    "scan",
    "protect",
    "compress",
    "form",
    "certificate",
    "print_production",
    "measure",
    "compare",
    "actions",
    "accessibility",
    "search_index",
];

/// Whether the Tools pane lists this tool group: `web` is the web build.
fn listed_in(web: bool, id: &str) -> bool {
    listed_with(web, id, crate::commands::offered)
}

/// [`listed_in`] with the command rule given: in the web build a group is listed when it is not
/// one of [`WORKSPACE_HIDDEN_TOOLS`] and at least one of its tools runs a command `offered` accepts,
/// so a group whose tools are all not offered is not shown either.
fn listed_with(web: bool, id: &str, offered: impl Fn(&str) -> bool) -> bool {
    !web || (!WORKSPACE_HIDDEN_TOOLS.contains(&id)
        && catalog::group(id).is_some_and(|g| g.sections.iter().flat_map(|s| s.items.iter()).any(|item| offered(item.command))))
}

/// Whether the Tools pane (its list and the tool panels) offers this tool group.
pub(crate) fn tool_listed(id: &str) -> bool {
    listed_in(cfg!(target_arch = "wasm32"), id)
}

/// The workspace modes the web build has: Edit, which shows the document with the Tools pane of
/// the listed groups, and Read, which shows the document alone. It has no All tools catalogue and
/// no Convert or E-Sign workspace (they open tool groups it does not list), so it is never in
/// [`Mode::AllTools`], [`Mode::Convert`] or [`Mode::Sign`]. `web` is the web build.
pub(crate) fn mode_in(web: bool, mode: Mode) -> Mode {
    if web && !matches!(mode, Mode::Read | Mode::Edit) { Mode::Edit } else { mode }
}

/// The mode this build has for `mode` (see [`mode_in`]).
pub(crate) fn offered_mode(mode: Mode) -> Mode {
    mode_in(cfg!(target_arch = "wasm32"), mode)
}

/// The mode a document opens in, and the mode the app starts in: the one the saved choice and the
/// control option `default-mode` can no longer change in the web build, where it is Edit, and
/// `saved` (All tools unless chosen) elsewhere. `web` is the web build.
pub(crate) fn default_mode_in(web: bool, saved: Mode) -> Mode {
    if web { Mode::Edit } else { saved }
}

/// The panel the Tools pane shows when no tool group is open: the list of tool groups, which is
/// the All tools list (`LeftPanel::AllTools`), on the desktop. The web build has no All tools
/// catalogue, in the list or expanded, so its Tools pane starts from the Edit panel, which reaches
/// the other listed groups. `web` is the web build.
pub(crate) fn tools_home_in(web: bool) -> LeftPanel {
    if web { LeftPanel::Tool("edit") } else { LeftPanel::AllTools }
}

/// [`tools_home_in`] for this build.
pub(crate) fn tools_home() -> LeftPanel {
    tools_home_in(cfg!(target_arch = "wasm32"))
}

/// The panel the Tools pane draws for `left`: the web build never draws the All tools list.
pub(crate) fn left_in(web: bool, left: LeftPanel) -> LeftPanel {
    if web && left == LeftPanel::AllTools { tools_home_in(web) } else { left }
}

/// Whether the build offers this side panel: `web` is the web build. The web build offers
/// Comments, Bookmarks, Pages, Layers and Search (the find bar's list of all matches); the desktop
/// build offers all of them.
fn panel_offered_in(web: bool, panel: RightPanel) -> bool {
    !web || matches!(panel, RightPanel::Comments | RightPanel::Bookmarks | RightPanel::Pages | RightPanel::Layers | RightPanel::Search)
}

/// The one predicate for the side panels: false in the web build for Fields, Attachments,
/// Signatures, Accessibility, Tags and Compare.
pub(crate) fn panel_offered(panel: RightPanel) -> bool {
    panel_offered_in(cfg!(target_arch = "wasm32"), panel)
}

// ───────────────────────────────────────────────────────────────────────────── right panels

pub(crate) enum Nav {
    Page(usize),
    /// A bookmark's destination: its page and where on it.
    Dest(usize, pdfcraft_render::DestView),
    Flash(usize, [f32; 4]),
}

pub fn human_size(n: usize) -> String {
    match n {
        n if n >= 1 << 20 => format!("{:.1} MB", n as f64 / (1u64 << 20) as f64),
        n if n >= 1 << 10 => format!("{:.1} KB", n as f64 / (1u64 << 10) as f64),
        n => byte_count(n),
    }
}

/// An exact size in bytes, with the count's own plural form.
pub fn byte_count(n: usize) -> String {
    crate::i18n::trn(crate::i18n::current(), n as u64, "{n} byte", "{n} bytes")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_PANELS: [RightPanel; 11] = [
        RightPanel::Comments,
        RightPanel::Bookmarks,
        RightPanel::Pages,
        RightPanel::Fields,
        RightPanel::Layers,
        RightPanel::Attachments,
        RightPanel::Signatures,
        RightPanel::Accessibility,
        RightPanel::Tags,
        RightPanel::Search,
        RightPanel::Compare,
    ];

    #[test]
    fn the_desktop_build_offers_every_side_panel() {
        assert!(ALL_PANELS.iter().all(|p| panel_offered_in(false, *p)));
    }

    #[test]
    fn the_web_build_offers_only_the_side_panels_the_backlog_uses() {
        let offered: Vec<RightPanel> = ALL_PANELS.into_iter().filter(|p| panel_offered_in(true, *p)).collect();
        assert_eq!(offered, [RightPanel::Comments, RightPanel::Bookmarks, RightPanel::Pages, RightPanel::Layers, RightPanel::Search]);
    }

    #[test]
    fn hidden_tool_groups_are_catalogue_groups_and_listed_once() {
        for id in WORKSPACE_HIDDEN_TOOLS {
            assert!(catalog::group(id).is_some(), "{id} is not a tool group of the catalogue");
            assert_eq!(WORKSPACE_HIDDEN_TOOLS.iter().filter(|other| *other == id).count(), 1, "{id} is listed twice");
        }
    }

    #[test]
    fn the_web_build_keeps_only_the_groups_the_backlog_needs() {
        let kept: Vec<&str> = TOOL_GROUPS.iter().map(|g| g.id).filter(|id| listed_in(true, id)).collect();
        assert_eq!(kept, ["edit", "organize", "comment", "redact"]);
    }

    #[test]
    fn the_desktop_build_lists_every_group() {
        assert!(TOOL_GROUPS.iter().all(|g| listed_in(false, g.id)));
    }

    #[test]
    fn a_group_none_of_whose_tools_is_offered_is_not_listed_in_the_web_build() {
        let none = |_: &str| false;
        for g in TOOL_GROUPS {
            assert!(!listed_with(true, g.id, none), "{}", g.id);
            assert!(listed_with(false, g.id, none), "the desktop lists {} whatever is offered", g.id);
        }
        // A group is listed as soon as one of its tools is offered.
        let organize = catalog::group("organize").expect("catalogue group");
        let first = organize.sections.iter().flat_map(|s| s.items.iter()).next().expect("a tool").command;
        assert!(listed_with(true, "organize", |id| id == first));
        assert!(!listed_with(true, "organize", |id| id == "does.not.exist"));
        // A hidden group stays hidden when all its tools are offered.
        assert!(!listed_with(true, "export", |_| true));
    }

    #[test]
    fn the_web_build_is_only_ever_in_edit_or_read() {
        for mode in [Mode::AllTools, Mode::Convert, Mode::Sign] {
            assert_eq!(mode_in(true, mode), Mode::Edit, "{mode:?}");
            assert_eq!(mode_in(false, mode), mode, "{mode:?}");
        }
        for mode in [Mode::Edit, Mode::Read] {
            assert_eq!(mode_in(true, mode), mode, "{mode:?}");
        }
    }

    #[test]
    fn the_web_build_starts_in_edit_whatever_was_saved() {
        for saved in [Mode::AllTools, Mode::Read, Mode::Edit, Mode::Convert, Mode::Sign] {
            assert_eq!(default_mode_in(true, saved), Mode::Edit, "{saved:?}");
            assert_eq!(default_mode_in(false, saved), saved, "{saved:?}");
        }
        let app = PdfCraftApp::new();
        assert_eq!(app.mode, default_mode_in(cfg!(target_arch = "wasm32"), Mode::AllTools));
        assert_eq!(app.default_mode, app.mode);
        assert_eq!(app.left, tools_home());
    }

    #[test]
    fn the_web_build_never_draws_the_all_tools_list() {
        assert_eq!(tools_home_in(true), LeftPanel::Tool("edit"));
        assert_eq!(tools_home_in(false), LeftPanel::AllTools);
        assert_eq!(left_in(true, LeftPanel::AllTools), LeftPanel::Tool("edit"));
        assert_eq!(left_in(true, LeftPanel::Tool("organize")), LeftPanel::Tool("organize"));
        assert_eq!(left_in(false, LeftPanel::AllTools), LeftPanel::AllTools);
        assert!(listed_in(true, "edit"), "the Edit panel the web Tools pane starts from is listed");
    }

    #[test]
    fn leaving_a_mode_the_build_does_not_have_keeps_the_current_one() {
        // On the desktop every mode exists, so asking for All tools enters it.
        let mut app = PdfCraftApp::new();
        app.enter_mode(Mode::Edit);
        app.enter_mode(Mode::AllTools);
        assert_eq!(app.mode, if cfg!(target_arch = "wasm32") { Mode::Edit } else { Mode::AllTools });
        app.expand_all_tools();
        assert_eq!(app.all_tools_full, cfg!(not(target_arch = "wasm32")));
    }

    #[test]
    fn the_six_groups_the_product_owner_chose_stay_hidden_in_the_web_build() {
        for id in ["export", "create", "stamp", "standards", "javascript", "ai"] {
            assert!(!listed_in(true, id), "{id}");
        }
    }
}
