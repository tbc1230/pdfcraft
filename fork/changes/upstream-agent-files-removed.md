---
type: Fork Change
title: Upstream AGENTS.md and CLAUDE.md removed
description: Upstream's agent instruction files are deleted in the fork; the fork's agent context is .claude/CLAUDE.md and this bundle.
resource: commit 51bff10e on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Upstream AGENTS.md and CLAUDE.md removed

## Why
Upstream's AGENTS.md and CLAUDE.md are written for upstream's contributors and do not apply to the fork. Backlog item: "Remove upstream CLAUDE.md and AGENTS.md".[^mods] The instructions from them that are useful for working on the fork's code were extracted into `.claude/CLAUDE.md`.

## What it changes
- `AGENTS.md` and `CLAUDE.md` at the repository root are deleted.

## On every upstream merge
When upstream has changed either file, the merge stops with a modify/delete conflict. Resolve it by deleting the file again. Check whether the change contains something useful for working on the fork's code; if so, carry it into `.claude/CLAUDE.md`.

## Related
- `docs/development.md` (upstream) still says working instructions are in AGENTS.md and CLAUDE.md; see [its record](/upstream/development.md).

[^mods]: [Fork items of the backlog](/sources/modifications.md), "Remove upstream CLAUDE.md and AGENTS.md".
