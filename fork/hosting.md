---
type: Overview
title: How Workspace hosts the editor
description: The editor as Workspace uses it, from load to save: what the client gives the editor, what it reads back, and what stays with Workspace. Every fork change to the editor's interface with the client is designed against this.
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
sources:
  - id: backlog
    resource: /sources/workspace-backlog.md
  - id: mods
    resource: /sources/modifications.md
  - id: plan
    resource: /sources/workspace-plan.md
---

# How Workspace hosts the editor

A backlog item names one function of the editor's interface. Workspace uses them together, in the order below. A fork change to that interface is designed for this whole use, not only for the call its item names.

## The parts
- The client: Workspace's CSR web application in the browser. It owns the layout, login, permissions, documents, versions, drafts, stamps, watermarks, signatures and every message to the user.[^backlog]
- The editor: the fork's WASM package, one part of the client's layout. The client imports it as a module and talks to it only through the exported JS API: no iframe, no postMessage bridge.[^plan][^backlog]
- The editor never contacts the fileserver or the existing APIs; everything it works on, the client passes to it, and everything it produces, it returns to the client.[^mods]

## Load and mount
- The client supplies a canvas element and mounts the editor on it; the editor renders there and receives the pointer and keyboard input there.[^mods]
- A failure to load or mount is returned to the client as a result. The client shows "the editor could not be loaded" with the browser's or the editor's error text unchanged, and a way to try again; a browser without WebAssembly is named as unsupported.[^backlog]
- While the editor is not loaded, the rest of Workspace stays usable and the actions that need the editor are unavailable. A failure of the editor while a document is open is reported the same way.[^backlog]

## Settings the client owns
- Interface language: set by Workspace; the editor's Preferences are not offered.[^mods] Workspace supports English, French and Arabic, right to left (the product owner).
- Theme: light, dark or follow the system, chosen in Workspace; the editor uses the same theme, and with follow the system it follows a change of the system setting.[^backlog]
- Author name: the logged-in user's display name; every markup item and reply records it, and it cannot be changed inside the editor.[^mods]
- These values exist before the editor does, so the client gives them when it starts the editor, and changes them while it runs (the product owner, 2026-11-07).

## Open a document
- The client passes the complete bytes of a PDF (never a path or URL); the result tells it whether the document opened. A failure carries the editor's error text, which the client shows unchanged.[^mods][^backlog]
- With a password: "password required" is a result distinct from every other failure; the password goes nowhere but the editor.[^mods]
- Given a document is open and opening another one fails, the open document stays open and unchanged.[^backlog]
- The editor shows no tab strip and no home screen; which document is open is the client's choice.[^mods]

## Work on the document
- The user edits on upstream's editing canvas.[^mods]
- Actions Workspace triggers from outside the editor run as editor commands (backed by `PdfCraftApp::run_command`) and change the document exactly as the same action inside the editor does.[^mods]
- Content Workspace produces or obtains is passed in by the client: the drawn signature image, other images (QR codes, barcodes), stamps and watermarks, the bytes of another PDF to insert, files to combine.[^mods]

## State the client reads
- can_undo, can_redo, permissions, is_signed, edit_generation, the page count, and the document information (title, author, subject, keywords).[^mods]
- The client's controls follow them: its undo and redo actions, its unsaved changes indicator (from edit_generation), the page count it shows, the suggested name in Save as new Item.[^backlog]
- After every change, and after a different document is opened, the values the client gets are the new ones.[^mods] How the client learns that a value changed is not settled by the backlog; it is a design decision of the fork.

## Save
- The client asks the editor to save; the editor returns the bytes, containing every edit up to the call, and sends them nowhere. The client sends them to the backend for Save, Save as a Version or Save as new Item.[^mods][^backlog]
- The editor offers no Save and no Save As command; saving is done with Workspace's buttons outside it: Save (overwrites the current version), Save as a Version, and Download.[^mods]
- Unsaved changes: none right after opening; after a successful save, only later changes count; after a failed save, the changes stay unsaved.[^backlog]
- Unsaved work is kept by Workspace's Draft Management, not by the editor's autosave and crash recovery, which are hidden.[^backlog]

## Today's build
The library build with this API is deferred until Workspace's stack is decided (product owner, 2026-11-09).[^plan] The editor is upstream's web build `apps/pdfcraft-web`, hosted by the Mock/Stub:
- `data-host="page"` on the canvas `pdfcraft`, with CustomEvents on it: `pdfcraft:ready`, `pdfcraft:open` (the page sends `{ name, bytes }`), `pdfcraft:dirty`, `pdfcraft:failed`, and the fork's `pdfcraft:state` (the editor state, each time it changes). `pdfcraft:save` is sent only by the editor's own Save, which the web build no longer offers.
- The fork's functions on `window.pdfcraft`, present on every page: `setLanguage`, `setTheme`, `setAuthor` (also given at start as `data-language`, `data-theme`, `data-author` on the canvas), `save()`, `runCommand(id)` and `getState()` ([Host API](/changes/host-api.md)).
- Upstream's page command API on `window.pdfcraft` (`apps/pdfcraft-web/src/api.rs`: `execute`, `docInspect`, `uiInspect` and the others), present only when the page is opened with `?control`; the Mock/Stub's tests use it to inspect the editor, and the client does not need it.

Where today's build cannot do what this page describes, that is the gap the backlog items close.

[^backlog]: [Workspace backlog](/sources/workspace-backlog.md): "Workspace imports the editor package", "Report that the editor could not be loaded", "Choose a reading theme", "Deliver fileserver bytes to the editor", "Report why a document could not be opened", "Use the editor state for controls outside the editor", "Store saved bytes on the fileserver", "Show whether the document has unsaved changes", "Draft Management".
[^mods]: [Fork items of the backlog](/sources/modifications.md): the feature "Exported JS API", "Hide upstream interface parts Workspace does not offer", "Hide upstream functions that overlap Workspace concerns", "Keep fork changes to what Workspace needs", "Edit on the PdfCraft editing canvas".
[^plan]: [Workspace plan](/sources/workspace-plan.md), "WASM library build".
