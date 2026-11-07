# Log

## 2026-11-07
- Bundle created: overview, how to read the backlog, three decisions, three fork changes (commit 51bff10e on `wasm-bundle`), four upstream documents, three sources copied from the Workspace project.

## 2026-11-07 (tab strip)
- Added change record tab-strip-hidden: the web build does not draw the tab strip.

## 2026-11-07 (upstream sync)
- Agent: integrator (main session). Task: sync `wasm-bundle` with upstream.
- `main` fast-forwarded to upstream `9dbede87` (40 commits); merged into `wasm-bundle` as `c0f37734`, no conflicts. Uncommitted work (9 files) was stashed before the merge and reapplied after it, no conflicts.
- Upstream did not change AGENTS.md, CLAUDE.md, .gitignore or the upstream documents recorded under `upstream/`.
- Checked: every record under `changes/` still matches the code; `cargo check` passes for `pdfcraft-web` (wasm32) and `pdfcraft-ui-egui` with tests (native).
- New from upstream: `apps/pdfcraft-web/src/api.rs`, `window.pdfcraft`, a page command API in the web build, opt-in with `?control` (commands, execute, docInspect, uiInspect, screenshot).

## 2026-11-07 (integrate procedure)
- Added procedure integrate: the integrator's steps after workers report. The Quality gates and Testing sections of `.claude/CLAUDE.md` now say they are the integrator's.

## 2026-11-07 (sync procedure)
- Added procedure upstream-sync, from the steps of the first sync.

## 2026-11-07 (work architecture)
- Added decision work-architecture: the integrator builds, tests and commits; workers change code, record it and add a log entry; the log links each change to its agent.
