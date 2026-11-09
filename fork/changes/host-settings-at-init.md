---
type: Fork Change
title: Language, theme and author name given by the host at init
description: The page gives the editor its interface language, theme and author name as data-language, data-theme and data-author on the canvas, read once at start; the setters exist before pdfcraft:ready.
resource: working tree change, not committed (no pull request yet)
status: draft
sources:
  - id: language
    resource: /changes/language-set.md
  - id: theme
    resource: /changes/theme-set.md
  - id: author
    resource: /changes/author-name-set.md
---

# Language, theme and author name given by the host at init

## Why
A page could set the language, theme and author name only through `setLanguage`, `setTheme` and `setAuthor` after `pdfcraft:ready`, so the editor's first frames showed the defaults. The page can now give them when the editor starts.

## What it changes
- Attributes on the `pdfcraft` canvas, read once when the editor starts (`apps/pdfcraft-web/src/main.rs`): `data-language` (a language code or `auto`), `data-theme` (`light`, `dark` or `system`), `data-author` (the author name).
- They go through the same checks and the same path as the setters: the checks are now shared functions in `apps/pdfcraft-web/src/host_settings.rs` (`language`, `theme`, `author`), used by the three setters and by the attributes; the values are written to the same inboxes (`language_inbox`, `theme_inbox`, `author_inbox`), which the app takes on its first frame. The author name therefore goes through `PdfCraftApp::set_author`: it is trimmed, cut to its limit, saved as the comment author and locked, as in [author-name-set](/changes/author-name-set.md).
- A missing or invalid attribute changes nothing: the editor stays as it is without it.
- The three setters are installed before `pdfcraft:ready` is posted (they were installed after the host bridge), so a page can call them from its ready handler. The setters themselves are unchanged in behaviour.
- The `?author=` query parameter still sets a default name that does not lock; `data-author` overrides it.
- The desktop build is unchanged.

## Tests
- `host_settings::tests` (native, `cargo test -p pdfcraft-web`): the three checks accept what the setters accepted and refuse the rest (blank author, unknown language code, any theme other than `light`, `dark`, `system`).
- Reading the attributes and the order of install and ready run only in the browser and are not covered by a test; the integrator checks them in the Mock/Stub.

## Not done
- Not built or run (workers do not build).

[^language]: [Interface language set by the host](/changes/language-set.md).
[^theme]: [Theme set by the host](/changes/theme-set.md).
[^author]: [Author name set by the host for markup](/changes/author-name-set.md).
