---
type: Source
title: Workspace plan
description: Outline of Workspace and the fork: goal, repositories, the fork's work, the app. Copied unchanged below this header.
status: stable
resource: WORKSPACE-PLAN.md
---

# Workspace

## Goal

Use PdfCraft as the PDF editor inside our own workspace application, while continuing to receive upstream updates and fixes.

## Repositories

1. **Fork of PdfCraft** (`github.com/storytold/pdfcraft`) - the PDF editor, built to WASM.
2. **App repo** - the workspace application.

## 1. Fork of PdfCraft

### Keeping in sync with upstream
- Upstream updates and fixes are pulled into the fork regularly.
- Upstream `CLAUDE.md` and `AGENTS.md` do not apply to this project. They are removed in the fork; when upstream changes them, the conflict is resolved at sync time.
- Upstream trademarks must be removed from the fork (README; `docs/brand/LICENSE-brand.txt`).

### WASM library build
- A new library crate in the fork (`cdylib`, `wasm-bindgen`), built with `wasm-pack` into a package the CSR app imports directly.
- No iframe and no `postMessage` bridge.
- Exported JS API, to cover at least:
  - mount the editor on a canvas supplied by the page
  - open a document from bytes
  - save, returning the bytes to the page (the page sends them to the server)
  - run commands (`PdfCraftApp::run_command` exists)
  - place the drawn signature image
  - the state the page's controls need (for example `can_undo` / `can_redo`, `permissions`, `is_signed`, `edit_generation`, which the engine exposes)

### Editor UI
- Keep the PdfCraft editing canvas (page view, selection, annotations, text editing).
- Replace upstream menus and toolbars with our own, built in the fork.

## 2. The app

A CSR web application with a backend.

### Layout
- The PDF editor (the fork's WASM package) is one part of the layout.
- Actions, versions and signatures are in the layout, outside the editor.

### Features
- Login and permissions.
- Documents read from the fileserver.
- Stamps and watermarks from the existing APIs.
- Versioning.
- E-signature: the user draws on a canvas; the editor places the result in the document.
- Digital signature: a backend action, using certificates available through the API.
