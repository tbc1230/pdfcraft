---
type: Overview
title: The Workspace fork of PdfCraft
description: What this fork is, what Workspace needs from it, how it follows upstream, and how its changes are made and tested.
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
sources:
  - id: plan
    resource: /sources/workspace-plan.md
  - id: backlog
    resource: /sources/workspace-backlog.md
---

# The Workspace fork of PdfCraft

## What the fork is
- Workspace is the organisation's web application (client-side rendered, with a backend) for opening, reading, reviewing, marking up, organising, stamping, signing and saving PDF and image documents.[^backlog]
- This repository is the organisation's fork of github.com/storytold/pdfcraft. The editor component of Workspace is built from it: a WASM package that shows pages and performs editing on the open document.[^backlog]
- Workspace owns login, permissions, document metadata, versioning, the fileserver connection, stamps and watermarks from the existing APIs, digital signatures and all messages to the user. None of these are implemented in the fork.[^backlog]
- The fork changes PdfCraft only where an item of the backlog with Component "PdfCraft fork" or "Workspace and PdfCraft fork" requires it, or the plan's fork work does (sync, removing upstream agent files, the WASM library build, the fork's own menus and toolbars), or the rename to Workspace does.[^backlog]

## The editor's WASM
- Today: PdfCraft's existing web build, `apps/pdfcraft-web`, built with trunk. A hosting page exchanges files and saves with it through `data-host="page"` events on its canvas, and since upstream `9dbede87` can drive it through `window.pdfcraft` (page command API, opt-in with `?control`, `apps/pdfcraft-web/src/api.rs`).
- Deferred: a library crate in the fork (cdylib, wasm-bindgen) built with wasm-pack into a package Workspace imports directly.[^plan] It waits until Workspace's stack is decided (the spike "Workspace framework and language"); what Workspace needs from it (loading the editor as a module, no iframe, no postMessage bridge, one API) is met by the web build above (product owner, 2026-11-09).

## Following upstream
- See [Branches and syncing](/decisions/branches.md).
- Upstream's documents describe upstream; see [Upstream documents stay unchanged](/decisions/upstream-documents.md) and the records under `upstream/`.

## Making a change
- Each fork change is a pull request; see [Pull request descriptions](/decisions/pull-requests.md).
- Each fork change has a record under `changes/`, listed in [index](/index.md).

## Testing
- Tests of PdfCraft features go in this repository.
- The editor's WASM is also tested end to end in the Mock/Stub, a disposable stand-in for Workspace outside this repository (the `workspace/` folder next to the fork's folder). It serves the WASM build, stands in for the backend and the fileserver, and holds its own end-to-end tests.

[^backlog]: [Workspace backlog](/sources/workspace-backlog.md), "Terms" and "Keep fork changes to what Workspace needs".
[^plan]: [Workspace plan](/sources/workspace-plan.md), "WASM library build".
