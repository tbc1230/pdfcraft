---
type: Fork Change
title: The restrictions set inside a PDF are honoured
description: The notice about a document's restrictions shows for every restricted right and cannot be turned off for good; the Security tab offers Unlock only where Workspace owns the passwords; tests cover refusals, copying, saving and Unlock.
resource: uncommitted on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-sonnet-5-5, at: 2026-11-10T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# The restrictions set inside a PDF are honoured

## Why
The story "Honour the restrictions set inside a PDF" asks that Workspace respect what an author set inside a PDF: the notice names the restriction for every right the state reports, it cannot be turned off for good, an action the permissions forbid is refused, and an encrypted document stays encrypted when saved.[^mods] The product owner decided that to the user there is only Workspace, so the editor's existing password prompts are how Workspace asks: opening an encrypted file raises the editor's password prompt, and lifting the restrictions stays with Unlock (the notice's button, the Security tab and the command `protect.unlock`), which calls `Session::unlock`. No new password entry was built.

## What it changes
- The rule for the notice is `SecuritySummary::restricts_any_right` (`crates/engine/src/protection.rs`): the document was opened without the owner password and its permissions withhold printing, modifying, copying, annotating, form filling, accessibility extraction, assembly or high-quality printing. `canvas.rs` uses it in place of the old test of modification and assembly alone. It is false for an unrestricted document, for a document that is not encrypted, after the owner password was entered and after Unlock succeeded.
- The notice cannot be turned off for good:
  - "Don't show again" and `Notice::HideSecured` are removed (`canvas/bars.rs`, `canvas.rs`).
  - `show_secured_notice` is removed from the app (`lib.rs`, `app.rs`), from the settings written by `persist` and read by `restore` (`app/prefs.rs`: an old stored value is ignored), from Preferences (`dialogs/preferences.rs`: "Show a notice when a document is secured") and from the options of `set_option` (`app/control.rs`: `secured-notice` is now an unknown option).
  - Dismiss stays: it hides the notice for the current document only.
- The Security tab of Document properties (`dialogs.rs`) lists the rights as before. "Protect using password...", "Change settings..." and "Remove security" are drawn only where `commands::offered` offers `protect.password` and `protect.remove`, that is not in the web build; "Unlock..." is drawn wherever the document can be unlocked. The Unlock button now calls `request_unlock` directly, as the notice's button does, because `protect.unlock` is not on the web build's list of offered commands (`finish_close` in `dialogs.rs`). The desktop build is unchanged.
- `protect.password` and `protect.remove` (`commands.rs`, the lines that ask for the permissions password first) are not offered in the web build, so no offered surface reaches them.
- Offered commands that need a right: `engine/src/commands.rs` `is_enabled` already maps Assembly, Modification, Annotate, Copying, Printing and FillForms to the document's rights, and `execute` and the host API refuse with "is not available now". Each of the 46 offered commands was checked against the eight rights and none lacked a gap; the engine's edits are gated in `edit/run.rs` as a second line. No right is needed by an offered command for accessibility extraction or high-quality printing (the print dialog prints at low resolution when only that is withheld, `session/files.rs`).
- Copying selected text with copying forbidden already put nothing on the clipboard and showed `copy_refused` (`app/clipboard.rs`: `may_copy`); no change, a test was added.
- An encrypted document saved after an edit already keeps its security (`Session::save_bytes`, `Session::unlock` reopens as owner); no change, a test was added.

## Tests
- `crates/ui-egui/tests/it/protect.rs`: `the_notice_shows_for_each_withheld_right_and_only_then` (eight single-right documents, an unrestricted encrypted one, an unencrypted one, the owner password), `the_notice_cannot_be_turned_off_for_good` (an old stored `show_secured_notice: false`, no "Don't show again", Dismiss for one document, a second restricted document, no Preferences switch, no option), `the_security_tab_unlock_asks_for_the_password`, `a_command_that_needs_a_withheld_right_is_refused_and_changes_nothing` (one sweep over every registered command, each right withheld in turn), `unlocking_lifts_the_rights_and_keeps_the_edits` (wrong password and user password change nothing and the prompt says so; the owner password lifts the rights, the notice goes, the earlier edit stays, the edit generation does not change, the saved file keeps both passwords), `an_encrypted_document_saved_after_an_edit_keeps_its_security`.
- `crates/ui-egui/tests/it/text_select.rs`: `copying_forbidden_puts_nothing_on_the_clipboard_and_shows_the_refusal`.
- `crates/ui-egui/tests/it/editing.rs`: the test `the_security_notice_can_be_turned_off_for_good` is removed with the behaviour it tested.
- Not covered here: the e2e criteria (PDFium opening the saved file; the four single-right state samples) belong to the Mock/Stub.

## Open points
- `parity/acrobat-features.toml`, feature `view.restriction-notice`, is corrected on the integrator's grant: its note no longer names "Don't show again" or `--secured-notice`, and its evidence names `protect.rs::the_notice_cannot_be_turned_off_for_good` in place of the removed editing test.
- The craft-i18n catalogs still hold "Don't show again" and "Show a notice when a document is secured"; no source uses them now.
- Decision (product owner, 2026-11-10): the notice's second sentence, "Some changes are restricted by its security settings.", becomes "This document's security settings restrict some actions.", because the notice now also shows for a document that withholds only printing or copying. The first sentence, "This document is secured.", stays; the text is one string in `canvas/bars.rs`, and `editing.rs` (the secured-form notice test) looks it up. The French and Arabic texts are the integrator's.
- `protect.unlock` is not on the web build's list of offered commands; command search in the web build does not show it. The notice and the Security tab reach the same prompt.

Not built or run.

[^mods]: [Fork items of the backlog](/sources/modifications.md), story "Honour the restrictions set inside a PDF".
