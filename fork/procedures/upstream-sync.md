---
type: Procedure
title: Sync the fork branch with upstream
description: The integrator's steps to bring upstream into the fork branch while keeping committed and uncommitted work.
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
---

# Sync the fork branch with upstream

Done by the integrator (see [How work on the fork is divided](/decisions/work-architecture.md)). The inputs can be committed fork work, uncommitted work in the working tree, and new upstream commits; the steps keep all three. First run: 2026-11-07, see [log](/log.md).

## Before
- Remote `upstream` is github.com/storytold/pdfcraft; `git fetch upstream` has been run.
- The fork branch (currently `wasm-bundle`) is checked out.

## Steps
1. **Set uncommitted work aside**: `git stash push -m "<why>"`. Tracked files only; untracked files such as `fork/` stay in place (upstream has no `fork/`, so the merge does not touch it).
2. **Update `main`**: `git fetch . upstream/main:main`. This only fast-forwards; `main` carries no fork commits, so it never fails unless someone committed to `main`.
3. **Merge `main` into the fork branch**: `git merge main`. Conflicts appear only here.
   - `AGENTS.md` or `CLAUDE.md` changed upstream: modify/delete conflict; delete the file again (`git rm`). If the change has something useful for working on the fork's code, carry it into `.claude/CLAUDE.md`. See [the change](/changes/upstream-agent-files-removed.md).
   - A conflict in a file a fork change touches: resolve it from the change's record under `changes/`; if that fails, reapply the change from its pull request description (see [Pull request descriptions](/decisions/pull-requests.md)).
4. **Reapply uncommitted work**: `git stash pop`. If it conflicts, resolve, then `git stash drop` only once the work is back.
5. **Check the build**: at least `cargo check -p pdfcraft-web --target wasm32-unknown-unknown` and `cargo check -p pdfcraft-ui-egui --tests`.
6. **Check the docs**: every record under `changes/` against the code (the functions and files it names still exist and still do what it says) and every record under `upstream/` against its document (`git diff <old> <new> -- <document>`). Update a record, or set `status: deprecated` where upstream made a change unnecessary.
7. **Log it** in [log](/log.md): the agent, the old and new `main`, the merge commit, conflicts and how they were resolved, the checks and their results, and anything new from upstream that matters to the fork.

## Not part of the sync
- Pushing. `main` and the fork branch are pushed to `origin` only when the product owner asks.
- The release WASM build and the Mock/Stub end-to-end tests; the integrator runs them before a commit or pull request.
