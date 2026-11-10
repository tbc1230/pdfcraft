---
type: Fork Change
title: Correcting wording in a paragraph rewraps it, and Arabic is set in the editor's face
description: A corrected word rewraps the paragraph to its width (longer text gives more lines, shorter fewer), a corrected Arabic paragraph keeps its right edge, and Arabic its own font cannot draw is shaped in the Arabic face the editor provides.
resource: uncommitted on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-sonnet-5-5, at: 2026-11-10T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# Correcting wording in a paragraph rewraps it, and Arabic is set in the editor's face

## Why
The story "Correct wording in a paragraph" promises that a reviewer can replace a word in an English, French or Arabic paragraph, that the paragraph is rewrapped, that a letter its font cannot draw is drawn in a font the editor provides, and that the replaced word is gone from the page text.[^mods] Reading the code found three places where that was not true:
- The inline editor shows a newline for every row of a paragraph, and the rewrite keeps every newline as a typed line break. A word corrected into a longer one left each row break where it was (the overflow of a row became a short row of its own), and a shorter one gave the same number of rows.
- An Arabic paragraph whose font cannot shape the new text (an embedded subset has no joining tables, or lacks a letter of the new word) was refused ("complex text needs the complete source TrueType/OpenType face"). The Arabic face of craft-fonts cannot be embedded as a font file (it is a variable font, and `pdfcraft_fonts::embedding::validate` refuses those), so the existing styled path cannot use it.
- Where an Arabic paragraph did go through the styled path, its rows were aligned left, not right.

## What it changes
- `crates/edit/src/text/blocks.rs`: `TextBlock::soft_breaks` reads from `reflow_text` which breaks between the paragraph's rows are only wraps and which were typed; `TextBlock::joined_draft(draft, join)` takes the marked breaks back out (joined with a space, or without the hyphen a wrap fell after).
- `crates/edit/src/text/rewrite.rs`, `wanted_joins`: for a draft with as many breaks as the paragraph has, and no width given, a wrap is taken out where the row before it was corrected and (estimated from the width it had per character) no longer fits the paragraph's width, or now leaves room for the first word of the next row; the rows after it then wrap again, up to the next typed break. Rows the correction does not reach keep their source operators. A draft with a break added or taken out, a paragraph with a width given (a width drag, `text_edit` with `width`), and a small correction that neither overflows nor frees a word, keep the breaks as typed.
- `crates/edit/src/text/rewrite.rs`, `rewrite_block_from`:
  - the draft goes through `wanted_joins` and `joined_draft` first, so Apply, the draft measure (`text_edit_layout`) and the tools agree;
  - on the styled path, a corrected paragraph that reads right to left and has no alignment of its own is aligned right (as the single-style path already did), so it keeps its right edge;
  - an Arabic correction (a draft with Arabic letters that differs from the paragraph, with no explicit font, family or ranges, with the craft-fonts Arabic face present, in upright text) first tries the styled path, on a copy of the document, so a font that can draw the text is kept (only for a paragraph of several rows); where that fails, or at once for a single row, the paragraph is written by the single-style path with the Arabic face.
- `crates/edit/src/text/arabic.rs` (new, a child module of `text`, declared in `crates/edit/src/text.rs`; `rewrite.rs` imports it with `use super::arabic`): `ArabicFace`, the Arabic face as a paragraph font. Each wrapped line is shaped in logical order, laid out in drawing order (UAX #9, the same split as added text in `crate::added::layout`), and shown cluster by cluster from Type 3 fonts drawn from the face's glyph outlines (as added text is, no font file is embedded), inside an `/ActualText` span holding the line as typed, so Find and copy read it back in typing order. Latin text and digits in such a line use the paragraph's standard font. Widths for wrapping come from the same shaping. A character neither the face nor the standard font can show refuses the correction before the page is touched, with the characters named.
- The paragraph keeps its first line's size, colour, line spacing and position as on the single-style path; a right-to-left single line keeps its right edge as it grows, and wrapped lines are aligned right in the paragraph's own width.

Grants from the integrator, for these changes only: (1) `arabic` is declared in `crates/edit/src/text.rs` as a normal child module, with no `#[path]`; `FontChoice` and `LineCursor` in `rewrite.rs` are `pub(super)`, with the fields `arabic.rs` uses (`show_font`, `encode`, `substituted`, `shown_font`) `pub(super)`. (2) The fork test for the forbidden-modification criterion is in `crates/ui-egui/src/canvas/tests.rs`.

Not changed: `crates/fonts/src/`, `crates/ui-egui/src/tools/edit_text_ui.rs` and its folder. The text the inline editor sends is unchanged; the engine now reads it as above.

## Already working (read in the code, not run)
- A letter the paragraph's font cannot draw but WinAnsi has (a French accent) is drawn in a standard font of the paragraph's family and weight (`choose_font`, the styled path's `substitute_font`) with no placeholder; a font that already draws it is kept (`source_encodes`).
- A character no font can draw (neither the standard font nor the craft-fonts faces) is refused before the page is touched (`choose_font`, `type3_font` read every outline first), with a message naming the text; the editor keeps the paragraph open with the user's text (`a_refused_paragraph_stays_open_with_its_text` in `edit_text_ui.rs`).
- The replaced word is removed from the content (the rewrite drops the paragraph's operators and any copy on the same baseline), so it is gone from the text layer Find reads.
- With modification forbidden, `page_input` in `canvas/page_input.rs` runs the edit-text input only when `can_modify`, and `check_host_command` in `app/host_calls.rs` refuses a command that is not enabled ("is not available now").
- The next paragraph is not touched (the paragraph marker and `splice` keep it); line spacing, size and colour are the first line's.

## Tests
In `crates/edit/src/tests.rs`, the section "Correcting wording in a paragraph" at the end:
- `a_corrected_word_shows_and_the_replaced_word_is_gone_from_the_page_text` (page text is read from the paragraphs, so a word drawn in two fonts is one word; standard font and subset-style font): the new word shows, the old one is in neither the page text nor the content.
- `a_correction_rewraps_the_paragraph_within_its_width_and_leaves_the_next_one_alone`: longer text gives more rows, none outside the original width; the next paragraph is equal in text, box, size, colour and rows; the corrected one keeps its left edge, size, colour and a 14.4 pt line spacing; shorter text, with every row break still in the draft, gives fewer rows. Fails without the rewrap.
- `a_break_that_was_typed_stays_a_break_when_a_word_is_corrected`: rows that stop short of the measure stay rows.
- `a_letter_the_paragraph_font_cannot_draw_is_drawn_as_typed_in_an_editor_font`: French accented letters in a subset without them (stand-in font, no `?`) and kept fonts (a subset with them, Courier).
- `a_character_no_font_can_draw_is_refused_and_the_paragraph_is_unchanged`: the content bytes and the paragraphs are as they were.
- `an_arabic_paragraph_is_corrected_shaped_right_to_left_and_keeps_its_right_edge`: one-row and two-row paragraphs in a synthetic Arabic font; the new word is in the page text, the replaced one is gone, its letters are shown last letter first from the Type 3 fonts, the paragraph is still right to left and every line ends at the old right edge. Skips without a craft-fonts Arabic face (`CRAFT_FONTS_DIR`). Fails without `arabic.rs`.

- `crates/ui-egui/src/canvas/tests.rs`: `with_modification_forbidden_edit_text_is_refused_and_a_click_on_a_paragraph_opens_no_editor`. An AES-256 document with `/P` bit 4 cleared, opened with the user password: `host_run_command("edit.edit_text")` fails with "is not available now", `execute` returns false, a click on the paragraph with the Edit text tool opens no editor, and the edit generation and undo state are unchanged. The same click on the unencrypted document opens the editor (control).

## Open points
- Not covered by a test here, because it needs the app: one correction as one undo step and one redo step (the editor's Apply goes through one `Edit::EditTextBlock`, which is one history entry; to be checked in the e2e test), the correction applied at save (the host worker's).
- A paragraph set in a font that can draw the Arabic text itself is kept only for several rows; a single Arabic row is always set in the editor's face (its right edge is kept only by the single-style path). A test for a full embedded Arabic font needs a fixture with a complete font program; none was written.
- A mixed Arabic and Latin paragraph is set with Latin text in the paragraph's standard font; Latin letters outside WinAnsi, and Hebrew inside an Arabic paragraph, refuse the correction.
- Justified Arabic is refused as before ("justification of mixed-style existing paragraphs is not supported" on the styled path); on the Arabic path a justified paragraph is shown right aligned without stretching.
- Rows are taken for wraps by `reflow_text`'s rule (a marker the editor wrote, or a row that reaches the paragraph's right edge); a paragraph the rule reads as wrapped is rewrapped on any correction that changes its text. Existing tests that edit such a paragraph with a changed length may need the new expectation.
- Other scripts, turned text and subset-font text are not promised by the story and are unchanged.

Not built or run.

[^mods]: [Fork items of the backlog](/sources/modifications.md).
