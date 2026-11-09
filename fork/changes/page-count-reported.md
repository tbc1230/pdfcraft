---
type: Fork Change
title: Page count reported to the client
description: The state that window.pdfcraft.docInspect() returns holds the page count of the active document as active.pages.
resource: working tree change, not committed (no pull request yet)
status: deprecated
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Page count reported to the client

Deprecated: the [Host API](/changes/host-api.md) replaces this change, because the backlog item "Report editor state to the client" requires that the fork adds no second way to read these values. `active.pages` is removed from `ui.state` (`documents[].pages` is upstream's and stays); the page count is `page_count` in `getState()` and in the event `pdfcraft:state`. The rest of this record describes what the change was.

## Why
Backlog item "Report the page count to the client": the exported JS API exposes the page count of the open document, the value after a page change is the new count, and after another document is opened it is that of the new document.[^mods]

## What it changes
- `Host::state` for the app (`crates/ui-egui/src/control/host.rs`): the `active` object of `ui.state` gets a `pages` field, the number of pages of the active document, read from the open document (`info.pages.len()`) on each call. It is `null` when no document is open.
- The test `state_and_view_options` (`crates/ui-egui/tests/it/control.rs`) asserts `active.pages`.
- No new JS function: `window.pdfcraft.docInspect()` in the web build is `ui.state` (`apps/pdfcraft-web/src/api.rs`), so the client reads `(await window.pdfcraft.docInspect()).active.pages`. The `documents[].pages` field already held the same value per tab.

## Depends on upstream
- `window.pdfcraft` exists only when the page is opened with `?control` (`apps/pdfcraft-web/src/main.rs`). Workspace must open the editor with it. Superseded for Workspace by the [Host API](/changes/host-api.md): `page_count` in `getState()` and the event `pdfcraft:state`, without `?control` and without polling.
- The value changes after a page change or a newly opened document only if the document's page list is refreshed by those operations; this change did not touch that code and it was not run (workers do not build). The integrator should check it in the Mock/Stub: move, delete, insert blank pages, insert from another PDF, then open another document.

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Report the page count to the client".
