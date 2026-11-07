---
type: Upstream Document
title: docs/architecture.md
description: How PdfCraft is built on upstream main. Accurate for the fork's code.
resource: /../docs/architecture.md
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
---

# docs/architecture.md (upstream)

## What it covers
How PdfCraft is built: one Cargo workspace with library crates in `crates/`, binaries in `apps/` (including the web build `apps/pdfcraft-web`), build tooling in `xtask/` and patched third-party crates in `vendor/`; the layers; the data model and editing; rendering and text; file I/O; agent control; safety; build and quality gates.

## How it relates to the fork
- Accurate for the fork's code, which is upstream's plus the changes under `changes/`.
- It refers to `plan/architecture.md` and `plan/adr/` for rationale; those are upstream's local-only files and are not in the repository.
