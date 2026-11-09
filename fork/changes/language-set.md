---
type: Fork Change
title: Interface language set by the host
description: The web page sets the editor's interface language with window.pdfcraft.setLanguage(code); the editor shows its texts in that language where it has a translation.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Interface language set by the host

## Why
Workspace sets the interface language instead of the editor's Preferences, which the web build does not offer. Backlog item: "Set the interface language of the editor".[^mods]

## What it changes
- `window.pdfcraft.setLanguage(code)` (`apps/pdfcraft-web/src/main.rs`, `install_set_language`): takes a language code, any case (such as `de`, `pt-br`), or `auto` for the browser's language. It returns true when accepted and false for a code the editor has no translation for, in which case nothing changes. It exists on every page, not only with `?control`, and keeps the functions `api::install` adds.
- `PdfCraftApp::language_inbox` (`LanguageInbox` in `hooks.rs`): the accepted code waits there; `app.rs` takes it each frame and stores it as the `language` setting, the same value Preferences and the `language` control option set. The frame's `logic` then applies it to the catalogs, so every text is shown in that language where a translation exists and the others stay English.
- The code is validated in the web crate with `i18n::normalize_pref` and again when taken.
- The desktop build is unchanged.

## Depends on upstream
- `i18n::normalize_pref`, the `language` setting and `i18n::set_current` in `logic`.
- The pattern is the same as the author name's ([author-name-set](/changes/author-name-set.md)).

## Not done
- Not built or run. Whether the page's own setting is stored over the user's saved language across reloads is the intended behaviour (the host calls it at each start); the integrator should check it in the Mock/Stub.

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Set the interface language of the editor".
