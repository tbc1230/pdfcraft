---
type: Fork Change
title: Six tool groups hidden from All tools in the web build
description: All tools (the sidebar and the expanded view) does not list export, create, stamp, standards, javascript and ai in the web build.
resource: commit 51bff10e on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Six tool groups hidden from All tools in the web build

## Why
The product owner chose these tool groups to hide from All tools: `export` (Export a PDF), `create` (Create a PDF), `stamp` (Add a stamp), `standards` (Apply PDF standards), `javascript` (Use JavaScript), `ai` (AI assistant). Related backlog items: "Hide upstream interface parts Workspace does not offer" (the Tools panel is shown only as far as backlog actions need it) and "Hide upstream functions that overlap Workspace concerns".[^mods]

## What it changes
- `crates/ui-egui/src/panels.rs`: the list `WORKSPACE_HIDDEN_TOOLS` and `tool_listed(id)`, which is true for every group in the desktop build and false for the listed groups in the web build.
- `crates/ui-egui/src/panels/tools.rs`, `all_tools`: the sidebar lists only groups that pass `tool_listed`; "View more" and the first rows count from that list.
- `crates/ui-egui/src/panels/all_tools_ui.rs`: the expanded All tools view filters each category the same way.

## Not covered
The hidden groups are still reachable outside All tools: the Convert tab opens Export (`select_mode` in `crates/ui-egui/src/app/commands.rs`), command search lists tool groups (`crates/ui-egui/src/dialogs/palette.rs`), the Tools menu (`crates/ui-egui/src/menus.rs`), the page context menu's Export PDF (`crates/ui-egui/src/canvas.rs`), the Edit panel's other tools and Home's pinned tools.

The list was extended to 19 groups, and the Tools pane trimmed, by [interface-parts-hidden](/changes/interface-parts-hidden.md).

## Depends on upstream
- The tool catalogue `TOOL_GROUPS` and its group ids (`crates/engine/src/catalog.rs`).
- The All tools sidebar and expanded view iterate the catalogue in the two places above.

[^mods]: [Fork items of the backlog](/sources/modifications.md).
