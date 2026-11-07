---
type: Procedure
title: Integrate worker changes
description: The integrator's steps after workers report: review, checks, WASM build, Mock/Stub tests, failures, commits, log.
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
---

# Integrate worker changes

Done by the integrator (see [How work on the fork is divided](/decisions/work-architecture.md)) once the workers it dispatched have reported. Workers have changed code and docs in the shared working tree without git and without building.

## 1. Collect
- Each worker's report: the files it touched and how it says it checked the change.
- Each worker's entry in [log](/log.md) and its record under `changes/`; complete any that is missing (the schema in `.claude/CLAUDE.md` lists the fields).
- `git status` and `git diff`: every changed file belongs to a worker's report. A changed file no worker reported is investigated before going on.

## 2. Checks
Run from the repository root, after all workers have finished (any build compiles every worker's edits).
1. `cargo fmt --check`
2. `cargo clippy -p <crate> --all-targets -- -D warnings`, for each crate touched.
3. `cargo test -p <crate>`, for each crate touched.
4. `cargo xtask layers`, `cargo xtask assets`, `cargo xtask wasm`.

A failure that also occurs on `main` (upstream) is not a worker's; note it in the log and go on.

## 3. WASM build
Build PdfCraft's web app (`apps/pdfcraft-web`) into the Mock/Stub, which lives outside this repository in `D:\tmp\aac\pdfcraft-main\workspace`:

```
trunk --config D:/tmp/aac/pdfcraft-main/pdfcraft/apps/pdfcraft-web/Trunk.toml build --release --filehash false --dist D:/tmp/aac/pdfcraft-main/workspace/pkg --public-url /pkg/
```

A release build takes several minutes even when one crate changed.

## 4. Mock/Stub tests
From `D:\tmp\aac\pdfcraft-main\workspace`:

```
.venv\Scripts\python -m pytest tests -v
```

The tests start their own stub server with fresh data. The Mock/Stub and its tests are described in `D:\tmp\aac\pdfcraft-main\workspace\README.md`.

## 5. Failures
- Use the log (files touched per worker) to find the change concerned.
- Send the failure, with the command and its output, back to the worker that made the change, by resuming it; it fixes its change without building. Then repeat from step 2.
- If the worker can no longer be reached, or the cause lies between two workers' changes, the integrator fixes it and logs who fixed what.

## 6. Commits
- Code first: one commit for the code changes (several if workers' changes are unrelated), then a separate commit for `fork/`.
- Each fork change becomes a pull request when the product owner asks; its description holds the idea and how to reapply it (see [Pull request descriptions](/decisions/pull-requests.md)).
- Nothing is pushed unless the product owner asks.

## 7. Log
For each worker's entry, add the build result (checks, WASM build, Mock/Stub tests) and the commit.
