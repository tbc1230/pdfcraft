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

## 2026-11-07 (page count)
- Agent: worker-page-count. Task: backlog item "Report the page count to the client".
- Files touched: `crates/ui-egui/src/control/host.rs`, `crates/ui-egui/tests/it/control.rs`, `fork/changes/page-count-reported.md`, `fork/index.md`, `fork/log.md`.
- Change record: [page-count-reported](/changes/page-count-reported.md). Not built or run (workers do not build).

## 2026-11-07 (document information)
- Agent: worker-doc-info. Task: backlog item "Report the document information to the client".
- Files touched: `crates/ui-egui/src/control/host.rs`, `apps/pdfcraft-web/src/api.rs`, `fork/changes/document-info-reported.md`, `fork/index.md`, `fork/log.md`.
- Change record: [document-info-reported](/changes/document-info-reported.md). Not built or run (workers do not build).

## 2026-11-07 (author name)
- Agent: worker-author. Task: backlog item "Set the author name for markup".
- Files touched: `apps/pdfcraft-web/src/main.rs`, `crates/ui-egui/src/hooks.rs`, `crates/ui-egui/src/lib.rs`, `crates/ui-egui/src/app.rs`, `crates/ui-egui/src/app/prefs.rs`, `crates/ui-egui/src/app/control.rs`, `crates/ui-egui/src/dialogs/preferences.rs`, `crates/ui-egui/src/dialogs/comment_props.rs`, `fork/changes/author-name-set.md`, `fork/index.md`, `fork/log.md`.
- Change record: [author-name-set](/changes/author-name-set.md). Not built or run (workers do not build).

## 2026-11-07 (theme)
- Agent: worker-theme. Task: backlog item "Set the editor's theme".
- Files touched: `apps/pdfcraft-web/src/main.rs`, `crates/ui-egui/src/hooks.rs`, `crates/ui-egui/src/lib.rs`, `crates/ui-egui/src/app.rs`, `fork/changes/theme-set.md`, `fork/index.md`, `fork/log.md`.
- Change record: [theme-set](/changes/theme-set.md). Not built or run (workers do not build).

## 2026-11-07 (interface language)
- Agent: worker-language. Task: backlog item "Set the interface language of the editor".
- Files touched: `apps/pdfcraft-web/src/main.rs`, `crates/ui-egui/src/hooks.rs`, `crates/ui-egui/src/lib.rs`, `crates/ui-egui/src/app.rs`, `fork/changes/language-set.md`, `fork/index.md`, `fork/log.md`.
- Change record: [language-set](/changes/language-set.md). Not built or run (workers do not build).

## 2026-11-07 (host open shown)
- Agent: worker-host-open. Task: "Show a document opened by the host".
- Files touched: `crates/ui-egui/src/app.rs`, `fork/changes/host-open-shown.md`, `fork/index.md`, `fork/log.md`.
- Change record: [host-open-shown](/changes/host-open-shown.md). Not built or run (workers do not build).

## 2026-11-07 (fmt all tools)
- Agent: worker-fmt-all-tools. Task: fix `cargo fmt --check` failure from commit 51bff10e. Files touched: crates/ui-egui/src/panels/all_tools_ui.rs (reformatted the `found` iterator chain, no behaviour change). Change record: none new (formatting only).

## 2026-11-07 (integrate procedure, build variables)
- Agent: integrator. Task: fix the WASM build command in the integrate procedure, which did not set the craft-i18n variables.
- Files touched: `fork/procedures/integrate.md` (step 3 now sets `CRAFT_I18N_DIR` and `CRAFT_I18N_REQUIRED=1` before `trunk`), `fork/log.md`. Change record: none (procedure only).

## 2026-11-07 (host settings at init)
- Agent: worker-host-init. Task: let the hosting page give the interface language, theme and author name at init (data-language, data-theme, data-author on the canvas) and install the setters before pdfcraft:ready.
- Files touched: `apps/pdfcraft-web/src/main.rs`, `apps/pdfcraft-web/src/host_settings.rs` (new), `fork/changes/host-settings-at-init.md`, `fork/index.md`, `fork/log.md`.
- Change record: [host-settings-at-init](/changes/host-settings-at-init.md). Not built or run (workers do not build).

## 2026-11-07 (hosting)
- Agent: the product owner's session. Task: describe how Workspace hosts the editor, so that changes to the editor's interface are designed for the whole use and not only for the call an item names.
- Files touched: `fork/hosting.md` (new), `fork/index.md`, `fork/log.md`. From the backlog, the fork items and the plan; the start-up values and the supported languages from the product owner.

## 2026-11-07 (issues)
- Agent: the product owner's session. Task: move the issues list into the fork, so that sessions working in the repository can read it.
- Files touched: `fork/issues.md` (moved from `ISSUES.md` next to the fork's folder; Issues 2 and 3 added and Issue 1's status corrected the same day), `fork/index.md`, `fork/log.md`, `.claude/STATE.md`.

## 2026-11-07 (wheel focus)
- Agent: worker-wheel. Task: Issue 2, page navigation (wheel, pinch, middle button) does not work until the canvas is clicked.
- Files touched: `crates/ui-egui/src/canvas.rs`, `crates/ui-egui/src/canvas/tests.rs`, `crates/ui-egui/src/canvas/autoscroll.rs`, `crates/ui-egui/src/app.rs`, `fork/changes/wheel-without-canvas-focus.md`, `fork/index.md`, `fork/log.md`.
- Change record: [wheel-without-canvas-focus](/changes/wheel-without-canvas-focus.md). Not built or run (workers do not build).

## 2026-11-07 (issue 2 resolution)
- Agent: integrator. Task: state how Issue 2 is resolved.
- Files touched: `fork/issues.md` (Issue 2: Resolution added), `fork/log.md`.

## 2026-11-08 (issue 3 cause and resolution)
- Agent: integrator, from the Issue 3 worker's investigation (no code change). Task: state the confirmed cause and the resolution of Issue 3.
- Files touched: `fork/issues.md` (Issue 3: Cause confirmed, Resolution added; Observed kept, with a pointer to `Screenshot.png`), `fork/log.md`.

## 2026-11-08 (craft-fonts in the WASM build)
- Agent: integrator. Task: resolve Issue 3 by building the WASM with the craft-fonts checkout the product owner provided (`D:\tmp\aac\pdfcraft-main\craft-fonts`, storytold/craft-fonts at `aced413`).
- Files touched: `fork/procedures/integrate.md` (step 3 sets `CRAFT_FONTS_DIR` and `CRAFT_FONTS_REQUIRED=1`), `.claude/CLAUDE.md` (craft-fonts checkout and WASM build variables), `fork/log.md`.
- Result: the product owner confirmed in the Mock/Stub (build of 2026-11-08 14:48) that Arabic letters are joined and that the label hidden behind the search bar is fully visible. `fork/issues.md` Issue 3 status set to fixed.

## 2026-11-08 (backlog criteria)
- Agent: the product owner's session. Task: add the product owner's criteria to two fork items.
- "Save and return the bytes to the client": the editor offers no Save and no Save As; saving is Workspace's Save (overwrites the current version), Save as a Version and Download, outside the editor.
- "Run commands from the client": commands of the stand-alone application (New blank PDF, Create PDF from file, Create PDF from Multiple Files, Open Recent, Clear Recent Files, Pin Folder to Home, Tab Order, and others) are not shown, enabled or disabled.
- Files touched: `modifications.md` and `workspace-backlog.md` next to the fork's folder, their copies `fork/sources/modifications.md` and `fork/sources/workspace-backlog.md`, `fork/hosting.md`, `fork/log.md`.

## 2026-11-08 (host API: save, commands, state)
- Agent: worker-host-api. Task: the API side of "Save and return the bytes to the client", "Report editor state to the client" and "Run commands from the client": `save()`, `runCommand(id)`, `getState()` and the `pdfcraft:state` event, without `?control`.
- Files touched: `crates/ui-egui/src/app/host_calls.rs` (new), `crates/ui-egui/src/app.rs`, `crates/ui-egui/src/lib.rs`, `crates/ui-egui/src/editing.rs`, `crates/ui-egui/src/editing/save.rs`, `apps/pdfcraft-web/src/host_api.rs` (new), `apps/pdfcraft-web/src/api.rs`, `apps/pdfcraft-web/src/main.rs`, `fork/changes/host-api.md`, `fork/index.md`, `fork/log.md`.
- Change record: [host-api](/changes/host-api.md).

## 2026-11-08 (offered commands)
- Agent: worker-offered-commands. Task: turn `offered()` into one list of commands the web build does not offer (Save, commands of the stand-alone application, theme, commands that only open hidden panels) and check every surface follows it.
- Files touched: `crates/ui-egui/src/commands.rs`, `crates/ui-egui/src/menus.rs`, `crates/ui-egui/src/hotbar.rs`, `crates/ui-egui/src/dialogs/palette.rs`, `crates/ui-egui/src/canvas.rs` (page context menu, one place), `crates/ui-egui/src/control/host.rs`, `crates/ui-egui/src/app/control.rs`, `crates/ui-egui/src/app/commands.rs`, `crates/ui-egui/src/canvas/organize.rs`, `crates/ui-egui/src/editing.rs`, `fork/changes/offered-commands.md` (new), `fork/changes/preferences-hidden.md`, `fork/index.md`, `fork/log.md`.
- Change record: [offered-commands](/changes/offered-commands.md). Not built or run (workers do not build).

## 2026-11-08 (interface parts hidden)
- Agent: worker-hide-parts. Task: the rest of "Hide upstream interface parts Workspace does not offer": no home screen, no All tools catalogue view or tab, only five side panels, the Tools pane trimmed to the tool groups the backlog needs (`WORKSPACE_HIDDEN_TOOLS` extended from 6 to 19 groups; Fill & Sign hidden by the integrator's decision; `panels::panel_offered`).
- Files touched: `crates/ui-egui/src/home.rs`, `crates/ui-egui/src/chrome.rs`, `crates/ui-egui/src/state.rs`, `crates/ui-egui/src/panels.rs`, `crates/ui-egui/src/panels/tools.rs`, `crates/ui-egui/src/panels/side_panels.rs`, `crates/ui-egui/src/panels/all_tools_ui.rs`, `crates/ui-egui/src/panels/edit.rs`, `crates/ui-egui/src/panels/right.rs`, `crates/ui-egui/src/panels/pages.rs`, `crates/ui-egui/src/panels/organize_panel.rs`, `fork/changes/interface-parts-hidden.md` (new), `fork/changes/all-tools-hidden-groups.md`, `fork/index.md`, `fork/log.md`.
- Change record: [interface-parts-hidden](/changes/interface-parts-hidden.md). Not built or run (workers do not build).

## 2026-11-08 (integration of host API, offered commands, interface parts hidden)
- Agent: integrator. Task: review, check, build and test the three changes above; e2e tests for the seven items ("Save and return the bytes to the client", "Report editor state to the client", "Run commands from the client", "Hide upstream interface parts Workspace does not offer", "Go directly to a page", "Step to the next or previous page", "See the document's own page numbering").
- Sent back to the workers and fixed by them: commands.rs written with CRLF (worker-offered-commands); `cargo fmt --check` in editing.rs and lib.rs (worker-host-api) and panels/edit.rs (worker-hide-parts); clippy `items_after_test_module` in panels.rs (worker-hide-parts); host_api.rs not compiling for wasm32 (worker-host-api); the form tab-order commands missing from the list (worker-offered-commands). Integrator decisions given to the workers: `page.combine` and the Fill & Sign group are not offered (files and the signature come from the client, fork/hosting.md); the further stand-alone, network and hidden-panel commands named in offered-commands.md. worker-hide-parts' log entry names `state.rs`, which it changed and then restored; `state.rs` is unchanged.
- Build result:
  - pass: `cargo fmt --check`; `cargo clippy -p pdfcraft-ui-egui -p pdfcraft-web --all-targets -- -D warnings` (native); `cargo test -p pdfcraft-ui-egui -p pdfcraft-web` without craft-i18n (816 + 1489 + 6 + 7 + 4 + 1 passed, pdfcraft-web 8 passed, 0 failed); `cargo xtask layers`; `cargo xtask wasm`; `cargo clippy -p pdfcraft-web --target wasm32-unknown-unknown` (no finding in pdfcraft-web or in the files the workers touched).
  - fail, not a worker's: `cargo clippy ... --target wasm32-unknown-unknown -- -D warnings` stops on dead code in `crates/render` (`raster/pool.rs`, `pixels.rs`, `raster.rs`), as on 2026-11-07; `cargo xtask assets` fails only on the untracked `Screenshot.png` in the repository root (no ATTRIBUTION.toml entry).
  - The release WASM build with craft-i18n and craft-fonts succeeded on 2026-11-08 at 17:27 into the Mock/Stub's `pkg\`.
  - Mock/Stub e2e (37 tests): 36 passed, 1 failed intermittently (`test_the_keys_step_pages_after_a_click_on_the_pages`). worker-nav-inspect (Sonnet, investigation only, no repository change) found the cause: Playwright's `wait_for_function` does not await a Promise, so every wait on `docInspect()` / `uiInspect()` in the Mock/Stub tests returned at once (confirmed by the integrator), and the test sent its keys about 20 ms apart; then a Ctrl key-up can reach the editor in the same frame as the key, and `crates/ui-egui/src/canvas/navigation.rs:431` reads Ctrl from the frame's modifier state, so Control+PageDown is handled as a plain Page Down (a scroll). The integrator fixed the tests (a polling helper `wait_until` in `conftest.py`). After the fix, two full runs: 37 passed, 0 failed each. The editor's frame-end modifier read is upstream key handling, not changed (see `.claude/STATE.md`).
- Docs by the integrator: `fork/hosting.md` ("Today's build": the host functions and `pdfcraft:state`); one sentence each in `fork/changes/page-count-reported.md` and `fork/changes/document-info-reported.md` pointing to the host API.
- Mock/Stub by the integrator: the client uses `save()`, `runCommand`, `getState()` and `pdfcraft:state` without `?control` (Undo and Redo buttons, the unsaved indicator from `edit_generation` and `clean_generation`); tests `tests/pdfs.py`, `tests/test_host_api_e2e.py`, `tests/test_interface_parts_e2e.py`, `tests/test_navigation_e2e.py`; the Ctrl+S save tests moved to the client's Save button; `README.md`.
- Not committed.

## 2026-11-08 (criteria of the seven items in work)
- Agent: the product owner's session. Task: complete the acceptance criteria of the seven items in work, so that each states everything the item delivers and how it is tested.
- Items: "Save and return the bytes to the client", "Report editor state to the client", "Run commands from the client", "Hide upstream interface parts Workspace does not offer", "Go directly to a page", "Step to the next or previous page", "See the document's own page numbering".
- Files touched: `modifications.md` and `workspace-backlog.md` next to the fork's folder, their copies in `fork/sources/`, `fork/log.md`.

## 2026-11-08 (state and save criteria)
- Agent: worker-state-save. Task: the criteria "Save and return the bytes to the client" (one generation in the result, failure text) and "Report editor state to the client" (permissions for every document, no second way to read the values, one event after a save).
- Files touched: `crates/ui-egui/src/app/host_calls.rs`, `crates/ui-egui/src/control/host.rs`, `crates/ui-egui/tests/it/control.rs`, `apps/pdfcraft-web/src/api.rs`, `apps/pdfcraft-web/src/host_api.rs`, `fork/changes/host-api.md`, `fork/changes/page-count-reported.md`, `fork/changes/document-info-reported.md`, `fork/index.md`, `fork/log.md`.
- Change record: [host-api](/changes/host-api.md) (second revision). The records page-count-reported and document-info-reported are deprecated.
- Failure case: `Session::save_bytes` returns the original bytes, not an error, for a document opened without an editor (`editable()` false, `crates/engine/src/session/save.rs:16`); no hand-made PDF found that makes a save fail. Not built or run (workers do not build).

## 2026-11-08 (offered commands as an allowlist)
- Agent: worker-offered-list. Task: turn the web build's refusal list around: `NOT_OFFERED_ON_THE_WEB` becomes the allowlist `OFFERED_ON_THE_WEB`, each command mapped to the backlog story that needs it.
- Files touched: `crates/ui-egui/src/commands.rs`, `fork/changes/offered-commands.md`, `fork/index.md`, `fork/log.md`.
- Change record: [offered-commands](/changes/offered-commands.md) (allowlist). Not built or run (workers do not build).

## 2026-11-08 (surfaces)
- Agent: worker-surfaces. Task: backlog items "Hide upstream interface parts Workspace does not offer" and "Run commands from the client": every surface follows `offered()`, the Tools pane lists only groups with an offered tool, the web build never shows or enters the All tools catalogue and starts in Edit, and no message shows with no document open.
- Files touched: `crates/ui-egui/src/app.rs`, `app/commands.rs`, `app/control.rs`, `app/files.rs`, `app/prefs.rs`, `canvas.rs`, `canvas/organize.rs`, `chrome.rs`, `hotbar.rs`, `menus.rs`, `dialogs/palette.rs`, `panels.rs`, `panels/edit.rs`, `panels/tools.rs`, `panels/side_panels.rs`, `panels/right.rs`, `panels/pages.rs`, `panels/comments_panel.rs`, `panels/organize_panel.rs`, `tools/comments/menu.rs`, `tools/edit_text_ui/image.rs` (all under `crates/ui-egui/src/`), `fork/changes/interface-parts-hidden.md`, `fork/log.md`.
- Change record: [interface-parts-hidden](/changes/interface-parts-hidden.md). Not built or run (workers do not build).

## 2026-11-08 (integration: the seven items against their completed criteria)
- Agent: integrator. Task: on the product owner's instruction (relayed by the product owner's other session), finish "Save and return the bytes to the client", "Report editor state to the client", "Run commands from the client", "Hide upstream interface parts Workspace does not offer", "Go directly to a page", "Step to the next or previous page" and "See the document's own page numbering" against every acceptance criterion, each criterion with at least one e2e test.
- Workers (Sonnet, closed file lists): worker-state-save, worker-offered-list, worker-surfaces (their entries above).
- Integrator decisions given to the workers:
  - worker-offered-list's unsure commands: `comment.polyline`, `comment.cloud`, `edit.copy`, `view.tools_pane` and `view.side_panel` are offered; the rest of its list stays off ("no story names it").
  - worker-surfaces' question: the Tools pane's group list (`LeftPanel::AllTools`) is the All tools catalogue; the web build opens the Tools pane on the Edit panel.
- Sent back and fixed: `cargo fmt --check` in `app/host_calls.rs` (worker-state-save) and in `chrome.rs`, `panels.rs`, `panels/edit.rs`, `panels/right.rs` (worker-surfaces).
- Build result:
  - pass: `cargo fmt --check`; `cargo clippy -p pdfcraft-ui-egui -p pdfcraft-web --all-targets -- -D warnings`; `cargo test -p pdfcraft-ui-egui -p pdfcraft-web` (826 + 1489 + 6 + 7 + 4 + 1, pdfcraft-web 6, 0 failed); `cargo xtask layers`; `cargo xtask wasm`; `cargo clippy -p pdfcraft-ui-egui -p pdfcraft-web --target wasm32-unknown-unknown` (no finding in a file a worker touched).
  - fail, not a worker's: wasm32 clippy with `-D warnings` (dead code in `crates/render`); `cargo xtask assets` (the untracked `Screenshot.png`).
  - The release WASM build with craft-i18n and craft-fonts succeeded on 2026-11-08 at 19:10 into the Mock/Stub's `pkg\`.
  - Mock/Stub e2e: 58 tests. First run 53 passed, 5 failed (faults in the integrator's tests: a note placed where the comments panel had moved the page, a label typed as a page number, thumbnails read twice). After the test fixes, two full runs: 58 passed, 0 failed each (15 min each).
- Docs by the integrator: `fork/issues.md` Issue 4 (Control+Page Down read with the frame's modifiers, `canvas/navigation.rs:431`, upstream).
- Mock/Stub by the integrator: tests per criterion (`tests/test_item_*.py`, helpers `helpers.py`, `editor_ui.py`, `nav.py`; the PDFs gained a text field and selectable text); the old `test_host_api_e2e.py`, `test_interface_parts_e2e.py` and `test_navigation_e2e.py` replaced; the client on the new `save()` result and `getState().info`; README.
- Not committed.

## 2026-11-08 (criteria of the next batch)
- Agent: the product owner's session. Task: write the completed acceptance criteria of the next batch into the backlog, before dispatch.
- Items: "Place an image on a page", "Place the drawn signature image", "Expose insert pages from bytes", "Insert blank pages", "Add a bookmark", "Rename a bookmark", "Delete selected pages", "Print the document", "Follow a link to another place in the document", "Search the whole document as I type". 49 criteria added, each marked "(added; <source>)" and ending with where it is tested; the product owner's answers of 2026-11-08 on image placement, images that cannot be placed and password-protected insert inputs are among them. "Delete selected pages": "by click, Command-click or Shift-click" reworded to "by click and Command-click", since Shift-click selects a range.
- Files touched: `modifications.md` and `workspace-backlog.md` next to the fork's folder, their copies in `fork/sources/`, `fork/log.md`.

## 2026-11-08 (Save C7 and State C7 closed)
- Agent: integrator. Task: on the product owner's instruction (relayed by the product owner's other session), close the two open criteria of the seven items.
- "Report editor state to the client", the criterion "A further state value ... is added through an item with Component ...": closed by the product owner with no test.
- "Save and return the bytes to the client", the criterion "Given the editor cannot produce the bytes...": e2e test `workspace/tests/test_item_save.py::test_c7_when_the_editor_cannot_produce_the_bytes_the_failure_carries_its_text`.
  - How the failure is forced: upstream's test channel (`?control`, `ui.set quick add-text`) selects the Add text tool, which the web build does not offer. The test leaves an unfinished draft holding a character the PDF text fonts cannot draw (U+2192). The editor's save path refuses to save while it is open.
  - The test checks:
    - `save()` resolves `{ ok: false, error: "The PDF text fonts can't draw ... (U+2192). Choose supported text." }`, with no bytes;
    - the client's Save shows "the editor did not return the document" with that text;
    - no PUT reaches the API.
  - No fork code changed.
  - Result: passed; `tests/test_item_save.py` 10 passed, 0 failed (the WASM of 19:10).
- 2026-11-08, later: "Save and return the bytes to the client", criterion "The editor gives the client saved bytes only through this function": scoped to the saved document, so that it no longer contradicts the print fallback of "Print the document" (a download of the print-ready PDF when the browser refuses the print). Raised by the integrating session.

## 2026-11-08 (insert pages from bytes, image size rule)
- Agent: worker-insert-bytes. Task: "Expose insert pages from bytes" (engine and app side) and the image size rule as a public engine function for worker-image-place.
- Files touched: `crates/engine/src/edit.rs`, `crates/engine/src/edit/run.rs`, `crates/engine/src/lib.rs` (exports), `crates/engine/src/tests.rs`, `crates/automation/src/assemble.rs`, `crates/ui-egui/src/editing.rs`, `crates/ui-egui/src/workflows/files.rs`, `fork/changes/insert-pages-from-bytes.md`, `fork/index.md`, `fork/log.md`. `edit/labels.rs` and `session/editing.rs` needed no change (they match the edit with `..`).
- Change record: [insert-pages-from-bytes](/changes/insert-pages-from-bytes.md).
- Not built or run. Needed by the integrator: `InsertFromBytesError` is not yet exported from `crates/ui-egui/src/lib.rs` (line 71), and `crates/engine/src/error.rs` has no dedicated variant for a needed password (a file list matter, see the record).

## 2026-11-08 (image placement)
- Agent: worker-image-place. Task: "Place an image on a page" and the placement part of "Place the drawn signature image": an image follows the pointer and one click places it; `start_image_placement`; Add image on the web opens the browser's file chooser; the page context menu's Delete page follows the `page.delete` command.
- Files touched: `crates/ui-egui/src/canvas.rs`, `crates/ui-egui/src/canvas/image_place.rs` (new), `crates/ui-egui/src/tools/content_ui.rs`, `crates/ui-egui/tests/it/content.rs`, `fork/changes/image-placement.md` (new), `fork/index.md`, `fork/log.md`.
- Change record: [image-placement](/changes/image-placement.md). Not built or run (workers do not build).
- Revision (integrator): `insert_pages_from_bytes` takes the file name first (`name: String`, blank = `document.pdf`), used for the undo label, the bookmark title and error texts; tests for bookmarks, none, blank name added. Files: `crates/ui-egui/src/workflows/files.rs`, the change record, this log.

## 2026-11-08 (host functions: placeSignature, insertPages)
- Agent: worker-host-functions. Task: "Place the drawn signature image" and "Expose insert pages from bytes", the JS side: `window.pdfcraft.placeSignature(bytes)` and `insertPages(bytes, options)`.
- Files touched: `crates/ui-egui/src/app/host_calls.rs`, `apps/pdfcraft-web/src/host_api.rs`, `crates/ui-egui/src/lib.rs` (exports `InsertFromBytesError`, `HostPassword`), `crates/ui-egui/src/canvas/page_pointer/tests.rs` (`canvas/image_place.rs` in `PAGE_MODULES`), `fork/changes/host-api.md`, `fork/log.md`.
- Change record: [host-api](/changes/host-api.md), section "placeSignature and insertPages". Not built or run (workers do not build).

## 2026-11-08 (page, bookmark, print, search and link items)
- Agent: worker-page-items. Task: check every criterion of "Insert blank pages", "Add a bookmark", "Rename a bookmark", "Delete selected pages", "Print the document", "Search the whole document as I type" and "Follow a link to another place in the document" against the web build, and fix what does not hold.
- Files touched: `crates/ui-egui/src/commands.rs`, `crates/ui-egui/src/panels/bookmarks.rs`, `crates/ui-egui/src/panels/pages.rs`, `crates/ui-egui/src/panels/organize_panel.rs`, `crates/ui-egui/src/panels/search_ui.rs`, `crates/ui-egui/src/canvas/organize.rs`, `crates/ui-egui/src/canvas/find.rs`, `crates/ui-egui/src/canvas/navigation.rs`, `crates/ui-egui/src/workflows/print_ui.rs`, `crates/ui-egui/tests/it/bookmarks.rs`, `editing.rs`, `print.rs`, `viewer.rs`, `pdf_links.rs` (same folder), `fork/changes/page-items.md` (new), `fork/index.md`, `fork/log.md`.
- Change record: [page-items](/changes/page-items.md). Not built or run (workers do not build). The record lists the changes needed outside the file list (`editing.rs`, `canvas.rs`, `host_calls.rs`, `right.rs`).
- Follow-up (integrator's permission): also touched `crates/ui-egui/src/editing.rs` (`A::New` arm), `crates/ui-egui/src/canvas.rs` (page menu Delete page), `crates/ui-egui/src/app/host_calls.rs` (`check_host_command` reason). Record updated.

## 2026-11-09 (integration: the second batch of ten items)
- Agent: integrator. Task: on the product owner's instruction (relayed by the product owner's other session), implement and test every criterion of "Place an image on a page", "Place the drawn signature image", "Expose insert pages from bytes", "Insert blank pages", "Add a bookmark", "Rename a bookmark", "Delete selected pages", "Print the document", "Follow a link to another place in the document" and "Search the whole document as I type".
- Workers (Sonnet, closed file lists): worker-insert-bytes, worker-image-place, worker-page-items, worker-host-functions (their entries above).
- Sent back and fixed (2026-11-08): compile errors in `canvas/image_place.rs` (worker-image-place) and `panels/pages.rs` (worker-page-items); `cargo fmt --check` in `assemble.rs`, `workflows/files.rs` (worker-insert-bytes) and `app/host_calls.rs`, `commands.rs`, `canvas/organize.rs`, `tests/it/bookmarks.rs` (worker-page-items); 7 ui-egui test failures (rename selection and two `bookmark_position` tests, worker-page-items; three image placement tests, worker-image-place); a wasm32 deprecation warning in `image_place.rs`.
- Build result:
  - pass: `cargo fmt --check`; clippy `-D warnings` for engine, automation, ui-egui and web; `cargo test` (engine 352, ui-egui 845 + 1506, 0 failed); `cargo xtask layers`; `cargo xtask wasm`.
  - fail, not a worker's (unchanged from the first batch): wasm32 clippy with `-D warnings` (dead code in `crates/render`); `cargo xtask assets` (the untracked `Screenshot.png`).
  - The release WASM build with craft-i18n and craft-fonts succeeded on 2026-11-08 at 21:14 into the Mock/Stub's `pkg\`. A rebuild on 2026-11-09 compiled nothing.
  - Mock/Stub e2e: the first run of the new tests had 26 failures; all were faults in the integrator's tests, none in the product. On 2026-11-09 the original workers could no longer be reached; on the product owner's instruction the remaining test faults went to fresh workers: worker-print-test (the `print()` recorder now runs inside the editor's `about:srcdoc` print frame; sheets are Letter paper with the page fitted; Comments & Forms is a drop-down) and worker-e2e-three (Add image C4 and signature C3 sampled a screen point the page had moved away from, and C4 used the Select tool instead of the Edit tool; delete C7's Page 3 thumbnail was out of view). The integrator fixed the rest (search options closed with their button; the Pages panel's Page tools entry for insert blank; FitH is the editor's fit width with its 70 pt margins; insertion after the current page; non-ASCII queries inserted at once) and added the resize to Add image C4. Full run of both batches: 134 passed; the 3 failures were tests fixed during the run, which then passed.
- Read only: worker-print-dialog-inspect (Sonnet) found why the Print dialog does not fit a small editor area (upstream code); findings in `fork/issues.md` Issue 5.
- Docs by the integrator: `fork/issues.md` Issue 5 (the Print dialog does not fit a 760 x 596 editor area; confirmed by the product owner in Chrome, where printing works) and Issue 6 (characters inserted one at a time end up reversed in a text field; seen with Playwright only).
- Mock/Stub by the integrator: tests per criterion (`tests/test_item_place_image.py`, `test_item_signature.py`, `test_item_insert_bytes.py`, `test_item_insert_blank.py`, `test_item_bookmarks.py`, `test_item_delete_pages.py`, `test_item_print.py`, `test_item_links.py`, `test_item_search.py`; helpers `pixels.py`, `item_common.py`; new fixtures in `pdfs.py`; `editor_ui.click_at` holds modifiers over frames); the client's Insert pages and drawn signature through `insertPages` and `placeSignature`; README.
- Not committed.
- 2026-11-09: "Place an image on a page", added criterion on Add image in the web build: "the shortcut" removed; `edit.image` has no keyboard shortcut. Raised by the integrating session's report.
- Agent: worker-preview-formats (Sonnet). Task: the pointer preview of "Place an image on a page" and "Place the drawn signature image" shows the actual picture for every placeable format (JPEG 2000, TIFF, GIF, BMP, PCX and files over 4 MiB, not only PNG and JPEG). The pixels come from the engine's own reader (`Session::convert_to_pdf`) and the page renderer, decoded once at the start of the placement, at most 1024 pixels on a side, files up to 16 MiB. Files touched: `crates/ui-egui/src/canvas/image_place.rs`, `crates/ui-egui/tests/it/content.rs`, `fork/changes/image-placement.md`, `fork/log.md`. Change record: `fork/changes/image-placement.md`. Not built or run.

## 2026-11-09 (integration: open points of the second batch)
- Agent: integrator. Task: on the product owner's instruction (relayed by the product owner's other session), close the open points of the second batch: the place preview for every placeable format; Add image C6 against the corrected wording; the print tests C1 and C6 proving the page order; then one clean full run of both batches. Held on the product owner's correction: the Print dialog size (Issue 5) and Issue 6; their workers (worker-print-fit, worker-issue6-probe, Sonnet) were stopped before changing any file.
- Workers (Sonnet): worker-preview-formats (entry above); worker-print-order (Mock/Stub only: `tests/test_item_print.py`, `tests/pdfs.py` `build(marks=...)`, `tests/pixels.py` `count_dark_blobs`; C1 and C6 count solid marks on each recorded sheet picture, pages of the same size and orientation side by side; the 1700 x 1300 window stays).
- Sent back and fixed: `cargo fmt --check` in `canvas/image_place.rs` and `tests/it/content.rs` (worker-preview-formats).
- Fixed by the integrator: clippy `unused variable: x0` in `tests/it/content.rs` `a_corner_drag_of_a_placed_image_keeps_its_proportions`, a first-batch test whose worker (worker-image-place) can no longer be reached.
- Mock/Stub by the integrator: `tests/test_item_place_image.py` C6 docstring follows the corrected criterion (no shortcut).
- Build result: pass: `cargo fmt --check`; clippy `-p pdfcraft-ui-egui --all-targets -D warnings`; `cargo test -p pdfcraft-ui-egui` (845 + 1514, 0 failed); `cargo xtask layers`, `assets`, `wasm`. Release WASM with craft-i18n and craft-fonts built on 2026-11-09 at 14:42 into `pkg\`.
- Mock/Stub e2e, one full run of both batches on that WASM: 136 passed, 0 failed (19 min 17 s); the print tests in their 1700 x 1300 window.
- Committed on 2026-11-09 on the product owner's word: the code of both batches and the earlier changes in f706da14 ("Host Workspace's web editor: host API, offered commands, two backlog batches"); the fork/ documents in the commit that follows it. Not pushed.

## 2026-11-09 (issue workflow)
- Agent: the product owner's session. Task: write down how an issue moves (opened, confirmed or rejected by someone other than the opener, planned or declined, fixed, closed), approved by the product owner.
- Files touched: `fork/issues.md` (section "How an issue moves"), `fork/log.md`.

## 2026-11-09 (library build deferred)
- Agent: the product owner's session. Task: record the product owner's decision: "Build the fork as a WASM library" (cdylib, wasm-pack, the exported JS API) is deferred until Workspace's stack is decided (the spike "Workspace framework and language"). Today's web build already loads as a module and exposes one API; mounting on a canvas the client supplies can be added to it.
- Files touched: `fork/overview.md`, `fork/hosting.md`, `fork/log.md`, `.claude/STATE.md`.

## 2026-11-09 (PDF file comments and Workspace Document Comments)
- Agent: the product owner's session, from an edit list prepared by a Fable agent. Task: apply the product owner's rule to the backlog's fork items: a PDF file comment (an annotation in the file, the editor's) is not a Workspace Document Comment (managed by Workspace across a document's versions, not in the file, outside the fork's scope); the bare word "comment" is not used; "Document permissions" is renamed "PDF file permissions".
- 20 lines changed in `workspace-backlog.md` (Terms; the epic "Comment at a specific place"; "Reply to a comment" retitled "Reply to a PDF file comment", criteria kept; "document permissions" in eight fork items), 13 of them also in `modifications.md`; copies in `fork/sources/` synced. No Workspace items added.

## 2026-11-09 (criteria of batch 3)
- Agent: the product owner's session, from `batch-3/batch.md` (writer, reviewer and reviewer of the review agreed; record in `batch-3/discussion.md`). Task: write the completed criteria of batch 3 into the backlog before dispatch.
- 57 criteria added, marked "(added; <source>)", to: Mark a passage with text markup; Place a note at a point on a page; Write a remark in a text box on a page; Draw freehand on a page; Draw a shape around content; List all markup in the comments panel; Add or change the explanation of a markup item; Delete a markup item; Undo and redo markup changes; Print the document (Issue 5); Reply to a PDF file comment.
- Product owner's answers: replies on every markup item except a text box (as Acrobat); the Print dialog scrolls, its buttons reachable at any editor area size.
- Reworded: Terms "Markup", "PDF file comment", "Reply", "Comments panel"; the story text of "Reply to a PDF file comment"; "Draw a shape around content" (shapes drawn by dragging); "Delete a markup item" (an item that is not locked). Open points added (mixed Arabic and Latin text; LockedContents; reply to a reply). `fork/issues.md` Issue 5 planned.
- Files touched: `modifications.md`, `workspace-backlog.md`, their copies in `fork/sources/`, `fork/issues.md`, `fork/log.md`.

## 2026-11-09 (batch 3)
- Agent: worker-markup-tools (Sonnet). Task: tool and key side of the stories "Mark a passage with text markup", "Place a note at a point on a page", "Draw freehand on a page", "Draw a shape around content", "Delete a markup item", "Undo and redo markup changes". The selection bar follows `offered()`; Delete/Backspace asks the engine's delete rule (`keys` takes `deletable`; the call in `canvas.rs` passes `doc.annotation_deletable`); tests for slips, the key path, and redo after undo. Files touched: `crates/ui-egui/src/canvas/selection.rs`, `crates/ui-egui/src/canvas.rs` (one line), `crates/ui-egui/src/tools/comments/input.rs`, `crates/ui-egui/src/tools/comments/tests.rs`, `crates/ui-egui/tests/it/editing.rs`, `fork/changes/markup-tools.md`, `fork/index.md`, `fork/log.md`. Change record: `fork/changes/markup-tools.md`. Not built or run.
- Agent: worker-markup-forktests (Sonnet). Task: the two fork tests named by the story "Mark a passage with text markup" (author after set_author; disabled selection bar on an encrypted PDF with text). Files: `crates/ui-egui/tests/it/comments.rs` (`a_text_markup_records_the_author_set_by_the_host`, `the_selection_bar_is_disabled_when_the_pdf_forbids_annotating`), `fork/changes/markup-tools.md`, `fork/log.md`. Change record: `fork/changes/markup-tools.md`. Not built or run.
- Agent: worker-print-fit (Sonnet). Task: the "Print the document" criteria marked "Issue 5": the Print dialog never wider or taller than the editor area, body scrolls with the Print and Cancel row outside the scroll, preview under the settings below 860 points, opens at its final size, 1400 x 900 unchanged. Files touched: `crates/ui-egui/src/dialogs.rs` (the width of `Dialog::Print`), `crates/ui-egui/src/workflows/print_ui.rs`, `crates/ui-egui/src/workflows/print_ui/body.rs`, `crates/ui-egui/tests/it/print.rs`, `fork/changes/print-dialog-fit.md`, `fork/index.md`, `fork/log.md`. Change record: `fork/changes/print-dialog-fit.md`. Not built or run.
- Agent: worker-freetext-arabic (Sonnet). Task: Arabic in a text box (FreeText) drawn shaped, right to left, in the craft-fonts Arabic face; box sized and wrapped with the Arabic widths; Western European and no-face scripts unchanged. Files touched: `crates/annot/src/appearance/arabic.rs` (new), `crates/annot/src/appearance.rs`, `crates/annot/src/draw.rs`, `crates/annot/src/tests.rs`, `crates/annot/Cargo.toml` (unicode-bidi), `crates/ui-egui/tests/it/text_box_arabic.rs` (new), `crates/ui-egui/tests/it/main.rs` (mod line), `fork/changes/text-box-arabic.md`, `fork/index.md`, `fork/log.md`. Change record: `fork/changes/text-box-arabic.md`. Not built or run.
- Agent: worker-markup-menus (Sonnet). Task: panel and menu side of "Reply to a PDF file comment", "List all markup in the comments panel", "Add or change the explanation of a markup item", "Delete a markup item". No reply field or Reply entry on a text box; the card and context menus offer only Edit, Reply, Properties and Delete; no checkmark button and no status or checkmark filter; no Copy text on a reply; fork tests for the offers, locks, permissions and the author field. Files touched: `crates/ui-egui/src/panels/comments_panel.rs`, `crates/ui-egui/src/tools/comments/menu.rs`, `crates/ui-egui/src/tools/comments.rs` (`defaultable` removed, module doc), `crates/ui-egui/tests/it/comments.rs`, `crates/ui-egui/tests/it/control.rs` (one query), `crates/ui-egui/src/tools/comments/tests.rs` (the Colour submenu test now checks there is no Colour entry), `fork/changes/markup-menus.md`, `fork/index.md`, `fork/log.md`. Change record: `fork/changes/markup-menus.md`. Not built or run.
- Integration (integrator, 2026-11-09): phase 1 workers worker-print-fit, worker-markup-menus, worker-markup-tools, worker-freetext-arabic (Sonnet, closed lists; entries above). Grants during the work: `crates/ui-egui/src/canvas.rs` (the Delete key's call to `keys`, worker-markup-tools), `tools/comments/geometry.rs` (approved for worker-freetext-arabic, not needed), `tools/comments.rs` and `tests/it/control.rs` (worker-markup-menus), the colour submenu test in `tools/comments/tests.rs` (worker-markup-menus, after worker-markup-tools finished).
- Sent back and fixed: literal CR/LF inside string and char literals in `annot/src/appearance/arabic.rs` and `appearance.rs`, and CRLF line endings written by Python text-mode writes (worker-freetext-arabic; worker-markup-tools: `canvas.rs`, `canvas/selection.rs`, `tools/comments/input.rs`, `tools/comments/tests.rs`); clippy: unused import and a deprecated `Id::new` (worker-markup-menus), `manual_clamp` (worker-print-fit), `Context::run` in a test (worker-markup-tools), a type mismatch in `tests/it/comments.rs`; test failures: the Print dialog card wider than a 300 x 200 area (worker-print-fit: the body's scroll area scrolls both ways), a reply-menu test and the upstream colour submenu test (worker-markup-menus).
- Fixed by the integrator: `a_replys_menu_has_edit_and_delete_only` clicked a button below the 900 pt test window; it now scrolls the reply into view first.
- Noted: worker-print-fit and worker-freetext-arabic ran read-only git commands (`git status`, `git show`, `git diff`), which only the integrator uses; nothing was changed with them.
- Build result: pass: `cargo fmt --check`; clippy `-p pdfcraft-annot -p pdfcraft-ui-egui --all-targets -D warnings`; `cargo test -p pdfcraft-annot` (102) and `-p pdfcraft-ui-egui` (846 + 1530, 0 failed); with craft-i18n and craft-fonts the two known failures `comments::hotbar_picker_searches_translated_tool_labels` and `fonts::primary_ui_fonts_cover_latin_and_cyrillic_catalogs` (unchanged files, seen in earlier batches); `cargo xtask layers`, `assets`, `wasm`; `Cargo.lock` gains `unicode-bidi` for pdfcraft-annot. Release WASM with craft-i18n and craft-fonts built 2026-11-09 19:31 into `pkg\`.
- Mock/Stub by the integrator: `tests/annots.py` (annotations and text lines read with PDFium), `tests/pdfs.py` `passages()` and `with_markup()` (markup by another user, locked items, a reply on a text box, a callout, encryption of strings and streams).
- 2026-11-09, later (batch 3 criteria corrected, raised by the integrating session): "Mark a passage with text markup" failure criterion allows upstream's area highlight on a page with no text; "Place a note" permission criterion names the page context menu's "Add comment"; "Draw a shape around content" checks a cloud by its /Vertices (its rect holds the scallops); "Write a remark in a text box" French test types key presses.
- 2026-11-09, later (issue workflow): confirming and closing rely on evidence independent of the opener's; visual issues can be confirmed and closed by the product owner or the product owner's Cowork session looking at the Mock/Stub, input issues by the product owner's real input (product owner). Issue 5 closed, verified by the product owner.
- Phase 2 (2026-11-09): e2e tests by worker-e2e-markup (`workspace/tests/test_item_text_markup.py`, `test_item_notes.py`, `test_item_text_box.py`, `test_item_ink.py`, `test_item_shapes.py`), worker-e2e-panel (`test_item_comments_panel.py`, `test_item_explanation.py`, `test_item_delete_markup.py`, `test_item_reply.py`), worker-e2e-undo-print (`test_item_undo_markup.py`, `test_item_print.py` now at 1280 x 720, README). Three criteria corrected by the product owner's session after the tests showed the product differs (area highlight, cloud rect, the note menu's "Add comment"); the French text box test types key presses. Issue 7 opened. worker-markup-forktests added the two text markup fork tests the criteria name (`a_text_markup_records_the_author_set_by_the_host`, `the_selection_bar_is_disabled_when_the_pdf_forbids_annotating`); `cargo test -p pdfcraft-ui-egui --test it comments::` 130 passed.
- The product owner stopped a first full run and four stub servers left by workers' test runs (2026-11-09); from now on every stub server, browser and background command is stopped when its run or worker ends.
- Mock/Stub e2e, one full run of all three batches (2026-11-09 20:54-21:35, WASM of 19:31): 285 passed, 0 failed (40 min 19 s).
- Committed on 2026-11-09 on the product owner's word: the code and tests of batch 3 in 0456685e ("Review and markup flow for Workspace's web editor (batch 3)"); the fork/ documents (batch 3's records, the issue workflow and Issues 5-10, the library build deferral, the new terms and criteria) in the commit that follows it. Not pushed.

## 2026-11-10 (new backlog item)
- Agent: the product owner's session, on the product owner's approval. New item "Save without earlier revisions" (Workspace and PdfCraft fork), after "Save and return the bytes to the client": the user can choose, when saving, a file written in full without earlier revisions; off by default; the fork's save can return such bytes when the client asks.
- Files touched: `modifications.md`, `workspace-backlog.md`, their copies in `fork/sources/`, `fork/log.md`.
- 2026-11-10, later (backlog rewording, product owner's sign-off): "Move and resize a placed object" criterion 5 adds the locked-item and undrawable-text-box exceptions; "Add a link over an area of a page" names the fit settings (fit page, fit width, fit height); "Correct wording in a Latin-script paragraph" becomes "Correct wording in a paragraph": it covers English, French and Arabic, and the fallback font is no longer named. The records `fork/changes/interface-parts-hidden.md` and `offered-commands.md` still cite the old title.

## 2026-11-10 (criteria of batch 4)
- Agent: the product owner's session, from `batch-4/requirements.md` (written by an agent, reviewed against its purpose, the review reviewed, converged; every reviewer read the final file in full; the product owner's answers applied). 47 criteria added, marked "(added; <source>)", to: Move and resize a placed object; Placed objects and corrections are kept in the saved document; Markup stays on its page when pages are moved, inserted or deleted; Markup carries its appearance in the saved file; Correct wording in a paragraph; Add a link over an area of a page; Place a QR code or barcode on a page; Honour the restrictions set inside a PDF.
- Files touched: `modifications.md`, `workspace-backlog.md`, their copies in `fork/sources/`, `fork/log.md`.

## 2026-11-10 (batch 4)
- Product owner's answers (through the product owner's session and directly): the opened note is the on-page PDF pop-up, movable and resizable, its open state, position and size saved; to the user there is only Workspace, so the editor's existing password prompts are how Workspace asks (opening; "Unlock..." on the notice and on the Security tab), with no new password call; `placeImage(bytes, { name? })` with `placeSignature` kept as its alias; French and Arabic of new texts written by the fork's agents into the local craft-i18n checkout only; pages inserted from another PDF keep their markup; the notice sentence becomes "This document's security settings restrict some actions."; `pdfcraft:open-failed` carries a `cause` value, and the other open failures stay an open point under "Open a document from bytes"; no merge of upstream and no work on its pull requests until upstream has finished its own work (upstream fetched by the product owner to show that PdfCraft is alpha).
- Agent: worker-comments (Sonnet). Task: "Move and resize a placed object" for annotations, the note pop-up, "Markup carries its appearance in the saved file" (fork side). Files touched: `crates/ui-egui/src/tools/comments.rs`, `tools/comments/{geometry,input,menu,paint,state,tests}.rs`, `crates/annot/src/{create,info,lib,modify}.rs`, `crates/engine/src/{edit.rs,edit/run.rs,edit/labels.rs,history.rs,comments.rs}`, `crates/ui-egui/tests/it/comments.rs`; on grants `crates/render/src/inspect/{model,annotations,tests}.rs` and `crates/ui-egui/src/canvas/page_router.rs`. Change record: [note-popup-and-object-handles](/changes/note-popup-and-object-handles.md). Not built or run.
- Agent: worker-images (Sonnet). Task: "Move and resize a placed object" for added images, the QR code's proportions. Files touched: `crates/ui-egui/src/tools/edit_text_ui/image.rs`, `crates/ui-egui/tests/it/content.rs`. Change record: [added-image-handles](/changes/added-image-handles.md). Not built or run.
- Agent: worker-links (Sonnet). Task: "Add a link over an area of a page". Files touched: `crates/annot/src/{links,tests}.rs`, `crates/automation/src/links.rs`, `crates/engine/src/{lib,tests}.rs`, `crates/ui-egui/src/tools/link_ui.rs`, `crates/ui-egui/tests/it/{pdf_links,fill_sign}.rs`; on a grant `crates/automation/src/tools.rs`. Change record: [link-targets](/changes/link-targets.md). Not built or run.
- Agent: worker-restrictions (Sonnet). Task: "Honour the restrictions set inside a PDF" (fork side) and the blank page before the selection. Files touched: `crates/engine/src/{commands,protection}.rs`, `crates/ui-egui/src/{canvas.rs,canvas/bars.rs,app.rs,app/prefs.rs,app/control.rs,lib.rs,dialogs.rs,dialogs/preferences.rs,commands.rs,panels/organize_panel.rs,panels/pages.rs}`, `crates/ui-egui/tests/it/{protect,text_select,editing}.rs`; on grants `parity/acrobat-features.toml` and `fork/changes/offered-commands.md`. Change records: [restrictions-honoured](/changes/restrictions-honoured.md), [blank-page-before](/changes/blank-page-before.md). Not built or run.
- Agent: worker-host (Sonnet). Task: `placeImage`, the "password required" result for the host, a pending correction in `save()`. Files touched: `apps/pdfcraft-web/src/{host_api,main}.rs`, `crates/ui-egui/src/app/{host_calls,files}.rs`. Change records: [host-api](/changes/host-api.md) (revised), [open-password-result](/changes/open-password-result.md). Not built or run.
- Agent: worker-edit-text (Sonnet). Task: "Correct wording in a paragraph". Files touched: `crates/edit/src/text.rs`, `text/{blocks,rewrite,arabic}.rs`, `crates/edit/src/tests.rs`; on a grant `crates/ui-egui/src/canvas/tests.rs`. Change record: [paragraph-correction](/changes/paragraph-correction.md). Not built or run.
- Integration (2026-11-10): grants as listed above. Sent back to the worker that made them: a lost backslash in a char literal (worker-links), a wrong `Arc` type and a double borrow in tests (worker-links), lost `\n` escapes from a Python rewrite (worker-edit-text), missing `popup` fields and a bool assertion (worker-comments), `identity_op` and `sliced_string_as_bytes` (worker-edit-text); test failures: four from the rewrap rule (worker-edit-text: a wrap is now taken out only next to a corrected row), three from the pop-up (worker-comments: two test faults, and a `comment_at` call moved out of `paint_page`), two image-move tests (worker-images: the page-edge limit applied to existing page artwork; it now applies to added images only, and the existing test is unchanged), the Security tab test finding two "Unlock..." (worker-restrictions). The integrator ran `cargo fmt` (the workers run no cargo). Noted: worker-comments ran `git status` and worker-edit-text `git diff --stat`, read-only; nothing was changed with them. By the integrator: `fork/index.md`, `fork/hosting.md`, the renamed story title in `interface-parts-hidden.md` and `offered-commands.md`, the local-only translation notes in three records, eight texts in French and Arabic in the local craft-i18n checkout (`locales/fr/pdfcraft.tsv`, `locales/ar/pdfcraft.tsv`; `craft-i18n.rev` unchanged).
- Build result: pass: `cargo fmt --check`; clippy `-p pdfcraft-annot -p pdfcraft-automation -p pdfcraft-edit -p pdfcraft-engine -p pdfcraft-render -p pdfcraft-ui-egui --all-targets -D warnings`; `cargo xtask layers`, `assets`, `wasm`, `parity`. `cargo test` of those six crates with craft-i18n and craft-fonts: 4482 passed, 3 failed, reported as they stand: `comments::hotbar_picker_searches_translated_tool_labels` and `fonts::primary_ui_fonts_cover_latin_and_cyrillic_catalogs` ("fr: Inter lacks" the triangle U+25B8; both also fail on d255813c, run in a separate worktree), and `protect::the_security_tab_unlock_asks_for_the_password` (after the harness's click on the Security tab's "Unlock...", no prompt within four frames; whether the test, the harness or the code is at fault is not known; to be checked in the browser). `fill_sign::fill_sign_enter_adds_lines_outside_click_commits_and_escape_cancels` failed in one run and passed in the next ones; not explained. Release WASM with craft-i18n and craft-fonts built 2026-11-10 14:09 into `pkg\`.
- Product owner's session (2026-11-10): the criteria of batch 1 and of the five items before it were checked against `.claude/recovered-tests.md`; they are in the backlog, and the tests follow them. Product owner's approval: "Set the interface language of the editor", "Set the editor's theme" and "Set the author name for markup" each get a criterion that the client can also give the value when it starts the editor (the decision of 2026-11-07 in `fork/hosting.md`). Files touched: `modifications.md`, `workspace-backlog.md`, their copies in `fork/sources/`, `fork/log.md`.
- Product owner's session (2026-11-10): Issue 11 rejected by the product owner. Open point added to the backlog: which upstream interface parts work only in the stand-alone application, and which an item needs, cannot be judged while upstream's commands are still being added and discussed. Files touched: `workspace-backlog.md`, its copy in `fork/sources/`, `fork/issues.md`, `fork/log.md`.
- Product owner's session (2026-11-10): the open point on upstream interface parts became the spike "Upstream interface parts Workspace needs", after "Hide upstream interface parts Workspace does not offer" (product owner). Files touched: `modifications.md`, `workspace-backlog.md`, their copies in `fork/sources/`, `fork/log.md`.
- Phase 2 (2026-11-10): Mock/Stub e2e tests by four workers (Sonnet). worker-e2e-objects (`workspace/tests/test_item_move_resize.py`, `test_item_qr_code.py`, `test_item_place_image_bytes.py`, `fixtures_objects.py`): 73 passed. worker-e2e-kept (`test_item_kept_saved.py`, `test_item_markup_pages.py`, `test_item_appearance.py`, `fixtures_kept.py`, a raw reader appended to `annots.py`): 28 passed. worker-e2e-restrictions (`test_item_restrictions.py`, `test_item_open_password.py`, `fixtures_restrictions.py`, `conftest.py` records `pdfcraft:open-failed`, `editor/app.js` and `editor/index.html`): 36 passed; it ran `git diff --stat` once, read-only, nothing changed with it. worker-e2e-text-links (`test_item_correct_wording.py`, `test_item_link_area.py`, `fixtures_text.py`): 54 passed, 4 failed. Noted by the workers, not judged: a pop-up's corner dragged past the opposite one flips it; a 1 px drag may be taken as a click; an arrow's or cloud's BBox can extend past the page; a new note's pop-up rect lies partly below the page; "Honour the restrictions set inside a PDF" criterion "open with a password" is met through the prompt and `pdfcraft:open-failed`, not by an open function taking a password.
- The four failures, reported as they stand: `test_c2_a_corrected_paragraph_is_rewrapped` (en_standard, en_embedded, fr_embedded, shorter: the row count stays and rows are ragged) and `test_c10_arabic_..._a_font_that_draws_it_is_kept` (the Arabic font embedded in the fixture is not kept; Type 3 is used). worker-edit-text read the code (nothing changed): for the rewrap, `break_gap` (`crates/edit/src/text_regions/grouping.rs`) reads a greedy wrap with more than 0.75 em left as a typed break, and the fit estimate in `rewrite.rs` uses one row's width per character; for Arabic, `validate_contextual_rows` (`text_regions/styled.rs`) refuses a row whose characters come from different source lines, which the reflow produces. Its proposals (in `rewrite.rs`, and in `styled.rs` on a grant) wait for the product owner.
- Mock/Stub e2e, one full run of all batches (2026-11-10 15:08-16:16, WASM of 14:09): 476 passed, 4 failed (the four above), 1 h 7 min 33 s. No server, browser or background command of this session left running; a stub server on port 8055 started 2026-11-09 20:57 is not this session's and was left as it is.
- Product owner's session (2026-11-10): batch 4 checked against the backlog from a per-criterion report written by a read-only agent (Sonnet) from the criteria, the tests and the recorded runs (`batch-4/report.md`, not in the repository): 97 criteria; 94 with passing tests; 2 failing, both in "Correct wording in a paragraph" (a shorter correction gives no fewer rows; an embedded Arabic font that draws the letters is not kept); "Open a document from bytes with a password" met through the editor's own prompt and `pdfcraft:open-failed`, not by an open function taking a password, by the product owner's decision. Issues 13-17 opened from the integrating session's records. Files touched: `fork/issues.md`, `fork/log.md`.
- Product owner's session (2026-11-10): Workspace has two passwords, the PDF file protection password and the password of the private key (product owner). Workspace asks for both in one place and passes the PDF password to the editor, so "Open a document from bytes with a password" stays as written and is not met by batch 4 (criteria 1 and 2: the editor's own prompt is no function taking the bytes and a password); "Honour the restrictions set inside a PDF" stays open with it (its criterion on opening password-protected documents). Both go to the next batch.

## 2026-11-10 (proof of concept concluded)
- Product owner: the fork is concluded as a proof of concept of PdfCraft as Workspace's editor, after batch 4. The result of the work is the backlog (`fork/sources/`) and what was learned while building it; the code and tests belong to the upstream snapshot of merge base `9dbede87`. The fork is pushed (`main` mirrors upstream at `92b965dba`, `wasm-bundle` holds the fork). Next: an investigation of the conflict patterns of merging `main` into `wasm-bundle`, to learn whether keeping the fork's changes isolated, with minor changes to upstream files, would have avoided them.
