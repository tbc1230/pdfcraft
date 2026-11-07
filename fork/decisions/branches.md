---
type: Decision
title: Branches and syncing
description: main mirrors upstream; fork changes live on their own branch; upstream is merged into it.
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
---

# Branches and syncing

- `main` mirrors upstream (github.com/storytold/pdfcraft) exactly. It moves only by syncing with upstream and carries no fork commits.
- Fork changes live on their own branch, currently `wasm-bundle`. Upstream is merged into it regularly; merge conflicts appear only there.
- Contributions to upstream are made from branches based on upstream's `main`, so they carry no fork changes.
