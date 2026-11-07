---
type: Decision
title: Upstream documents stay unchanged
description: The fork never edits upstream's Markdown files; its own knowledge lives in this bundle.
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
---

# Upstream documents stay unchanged

- Upstream's Markdown files (README.md, ROADMAP.md, ATTRIBUTION.md, docs/ and the rest) are never edited in the fork, so that they never cause merge conflicts.
- They describe upstream PdfCraft. They are read as sources: accurate about the code they document, not a statement of the fork's goals.
- The fork's own knowledge lives in this bundle (`fork/`, Open Knowledge Format v0.2). Records under `upstream/` say how an upstream document relates to the fork.
- The file loaded for every agent is `.claude/CLAUDE.md`; it points here.
