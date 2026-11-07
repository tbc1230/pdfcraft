---
type: Fork Change
title: Tab strip hidden in the web build
description: The web build does not draw the tab strip; the desktop build is unchanged.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Tab strip hidden in the web build

## Why
Backlog item "Hide upstream interface parts Workspace does not offer": "The editor does not show ... the tab strip."[^mods] This change covers only the tab strip; the home screen and Preferences are not part of it.

## What it changes
- `PdfCraftApp::ui` (`crates/ui-egui/src/app.rs`): `chrome::tab_strip` is called only when `cfg!(not(target_arch = "wasm32"))`. The code stays compiled in both builds.
- The desktop build is unchanged.

## Depends on upstream
- The strip also holds the Home and Open buttons and the Discord link; in the web build they are not shown either. Opening and switching documents then relies on other entry points (menus, commands).
- The mode bar and panels are laid out below the strip and do not depend on it.

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Hide upstream interface parts Workspace does not offer".
