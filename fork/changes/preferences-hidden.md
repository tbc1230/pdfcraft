---
type: Fork Change
title: Preferences hidden in the web build
description: The web build does not offer Preferences; the desktop build is unchanged.
resource: working tree change (no commit, no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Preferences hidden in the web build

## Why
Workspace sets the interface language and the other settings itself. Related backlog item: "Hide upstream interface parts Workspace does not offer" ("The editor does not show ... Preferences").[^mods]

## What it changes
- `offered(id)` (`crates/ui-egui/src/commands.rs`): false for `app.preferences` when the target is `wasm32`, true otherwise.
- The command is not offered in: the Edit menu (`menus.rs`), command search (`dialogs/palette.rs`), keyboard shortcuts, including the Ctrl+, binding (`commands.rs`) and the shortcuts list (`dialogs.rs`), the hotbar tool list (`hotbar.rs`), the Edit panel's gear button (`panels/edit.rs`) and the title bar's Appearance settings item (`chrome.rs`).
- `execute` returns false for it, and the `dialog=preferences` control option returns an error (`app/control.rs`).
- The desktop build is unchanged.
- Extended by [offered-commands](/changes/offered-commands.md): `offered(id)` is now one list of commands, with `app.preferences` in it.

## Depends on upstream
- `Dialog::Preferences` and its body stay in the code, so upstream sync stays simple; nothing in the web build opens it.

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Hide upstream interface parts Workspace does not offer".
