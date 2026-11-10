---
type: Fork Change
title: Insert a blank page before the selection
description: A command, an Organize pages row and a Pages panel action insert a blank page before the first selected page, so a blank page can go before page 1; tests show markup stays with its page.
resource: uncommitted on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-sonnet-5-5, at: 2026-11-10T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Insert a blank page before the selection

## Why
The story "Markup stays on its page when pages are moved, inserted or deleted" has the criterion that a blank page can be inserted before page 1 and the markup then sits on its original page, now page 2. The editor offered only "after the selection" (`Edit::InsertBlankPage { at }` takes any position; the interface always passed the last selected page plus one).[^mods]

## What it changes
- New registered command `page.insert_blank_before` ("Insert blank page before", `crates/engine/src/commands.rs`): needs Assembly like `page.insert_blank`, icon `file-plus`, no menu entry and no shortcut (the Pages menu list is in `menus.rs`; the command is run from the places below, command search and the host API).
- `crates/ui-egui/src/commands.rs`: the command runs `Edit::InsertBlankPage { at: first selected page (or the current page), width, height }` with the size rule of the "after" variant (that page's crop box); the page that was first and its markup move one place later; the inserted page is selected (`editing.rs` already selects page `at`). `page.insert_blank_before` is added to `OFFERED_ON_THE_WEB`, in the group "Insert blank pages".
- Organize pages panel (`panels/organize_panel.rs`): the Insert menu gets "Insert a blank page before the selection", queued as the same edit.
- Pages panel menu (`panels/pages.rs`): "Insert a blank page before the selection", running the command.
- Undo takes the blank page out in one step, as for the "after" variant.

## Tests
In `crates/ui-egui/tests/it/editing.rs`: `markup_moves_with_its_page_when_the_page_is_moved`, `markup_goes_with_a_deleted_page_and_comes_back_on_undo`, `a_blank_page_before_page_one_leaves_the_markup_on_its_page` (by the command and by the engine's edit, with undo and redo), `a_blank_page_before_the_selection_takes_the_size_of_the_first_selected_page`, `pages_inserted_from_another_pdf_keep_their_markup` (author, text and place kept; undo and redo). The registry test `every_registered_command_is_implemented` in `tests/it/commands.rs` and the refusal sweep in `tests/it/protect.rs` cover the new command without change. Not yet built or run.

## Open points
- `fork/changes/offered-commands.md` is revised on the integrator's grant: the count is 47 and the table has the row `page.insert_blank_before | Insert blank page before | Insert blank pages`.
- The comments panel criterion ("no entry for markup of a deleted page, none for an inserted blank page") is tested on the document's markup list that the panel shows (`info.annotations`), not on the panel's drawn rows.
- The texts "Insert blank page before" (the command's label) and "Insert a blank page before the selection", with French and Arabic written by the worker, are added by the integrator to the local craft-i18n checkout only (`locales/fr/pdfcraft.tsv`, `locales/ar/pdfcraft.tsv`); they live only there until upstream craft-i18n has them, and `craft-i18n.rev` does not move (product owner's decision, 2026-11-10).
- The page move test uses the engine edit; dragging in the Organize pages view queues the same `MovePages` edit.

Not built or run.

[^mods]: [Fork items of the backlog](/sources/modifications.md), story "Markup stays on its page when pages are moved, inserted or deleted".
