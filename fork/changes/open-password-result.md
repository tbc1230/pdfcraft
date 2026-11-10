---
type: Fork Change
title: The host is told when a password prompt is cancelled
description: When the user cancels the editor's password prompt for a file the host sent with pdfcraft:open, the editor sends pdfcraft:open-failed with cause "password-required"; nothing opens and the password stays inside the editor.
resource: uncommitted on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-sonnet-5-5, at: 2026-11-10T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# The host is told when a password prompt is cancelled

## Why
The story "Open a document from bytes with a password" requires that when the password is missing or wrong the client gets a "password required" result, distinct from every other failure of "Open a document from bytes", and that the password is used only by the editor.[^mods] The product owner decided that to the user there is only Workspace and the editor is part of it, so the editor's existing password prompt is how Workspace asks: no password parameter and no new open function.

Before this change the host learned the outcome of a `pdfcraft:open` only through the state snapshot (`open`, `doc` and `page_count` change when a document opens). A file that needed a password raised the prompt and a cancel changed nothing the host could see; a failure was a toast in the editor and a console message.

## What it changes
- `crates/ui-egui/src/app/host_calls.rs`: `HostCalls` is a struct that holds the call queue and the names of files the host sent to open whose outcome is not known (`expect_open`, `settle_open`, at most 64 names). `HostEvent` (`State(snapshot)`, `PasswordRequired { name }`) is what the app hands the host; `HostStateFn` takes it. `PdfCraftApp::report_host_event` hands one over.
- `crates/ui-egui/src/app/files.rs`: `try_open` and `open_bytes_with_focus` settle the name when the file opened, failed or was converted; a password prompt keeps it. `submit_password(None)` (cancel) settles it and, when the host had sent it, reports `PasswordRequired { name }`. A wrong password keeps the prompt as before ("Incorrect password. Try again.") and reports nothing; the right password opens the document as before, with the owner or user rights the password gives.
- `apps/pdfcraft-web/src/main.rs`: `queue_open` calls `expect_open(name)` before it queues the file.
- `apps/pdfcraft-web/src/host_api.rs`: dispatches `pdfcraft:open-failed` on the canvas, `detail` = `{ name, cause: "password-required", error: "the PDF needs a password" }`, built by `HostEvent::detail` (with `HostEvent::event_name`) so that it is testable natively. `cause` is a value rather than a boolean so that other causes can join later without changing the shape; `HostEvent::PasswordRequired` stays the one variant for now.
- Nothing opens, and an already open document stays open and unchanged (the cancel only drops the prompt). The event is sent once per file. No password appears in the event, a result, a console message or the state; `HostEvent` has no field for one.

## Tests
In `host_calls::tests`: `cancelling_the_password_prompt_of_a_host_file_tells_the_host_once`, `a_wrong_password_keeps_the_prompt_and_tells_the_host_nothing`, `the_right_password_opens_the_host_file_and_tells_nothing`, `password_required_is_distinct_from_every_other_failure_and_from_the_editors_own_prompts`, `the_host_events_and_calls_show_no_password`, `the_owner_password_gives_owner_and_every_right_the_user_password_does_not`. `the_host_events_and_calls_show_no_password` also asserts the exact detail (cause "password-required", no `password_required` field). The dispatch itself runs only in the browser and is for the e2e tests.

## Open points
- The other causes are not built and stay open under "Open a document from bytes" (deferred): not a PDF or unreadable, over 512 MiB, a file that came as a `File` instead of bytes, and the rest. They are reported only as a toast or console message; the host gets no event for them. They would join as further `cause` values of the same event.
- Files are matched to their prompt by name; two files sent with the same name while the first one waits in the prompt are told apart by order only.
- The event is a `CustomEvent` on the canvas (host mode `data-host="page"`); like `pdfcraft:state` it is not relayed through the `?host=parent` bridge.
- `HostEvent` must be exported from the crate root (`crates/ui-egui/src/lib.rs`, the `pub use app::host_calls::{...}` line), which this change does not touch. Required: the web crate does not build without it.

Not built or run.

[^mods]: [Fork items of the backlog](/sources/modifications.md).
