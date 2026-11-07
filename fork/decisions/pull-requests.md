---
type: Decision
title: Pull request descriptions
description: Each fork change is a pull request whose description says how to reapply it when a merge conflict cannot be resolved.
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
---

# Pull request descriptions

- Each fork change is a pull request.
- Its description documents the idea behind the change and how to reapply it, so that the change can be redone when a merge conflict with upstream cannot be resolved. It is not a copy of the changeset, which the commits already hold.
- The knowledge behind a change (why it exists, what it depends on, what it means for later work) is kept in its record under `changes/`, not in the pull request.
