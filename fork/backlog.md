---
type: Convention
title: How to read the backlog
description: The backlog's acceptance criteria describe the target behaviour; which items concern the fork; where the terms are defined.
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
sources:
  - id: backlog
    resource: /sources/workspace-backlog.md
  - id: mods
    resource: /sources/modifications.md
---

# How to read the backlog

## Acceptance criteria are the target behaviour
The backlog writes acceptance criteria in the present tense, as the behaviour Workspace and the fork must have. They are not descriptions of how the code behaves today and not bug reports. "The editor does not show the tab strip" means: make the editor not show the tab strip. Where the code does not yet behave as a criterion says, that gap is the work. Where it already does, say so.

## Which items concern the fork
Every item has a Component:[^backlog]
- "Workspace": the web application, client and backend. Not done in the fork.
- "PdfCraft fork": a change in this repository.
- "Workspace and PdfCraft fork": a Workspace feature that needs a named change in this repository; only that change is done here.

The items with Component "PdfCraft fork" or "Workspace and PdfCraft fork" are collected in [Fork items of the backlog](/sources/modifications.md).[^mods]

## Terms and open points
- The backlog's "Terms" section defines the words used in the items (the editor, the client, the backend, document, draft, markup, and so on). Read an item with those meanings.[^backlog]
- The backlog's "Open points" are product questions no source answers; no item settles what an open point leaves open.[^backlog]

[^backlog]: [Workspace backlog](/sources/workspace-backlog.md), introduction, "Terms" and "Open points".
[^mods]: [Fork items of the backlog](/sources/modifications.md).
