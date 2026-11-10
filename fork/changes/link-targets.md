---
type: Fork Change
title: A link's page target has a fit, and its web address is checked
description: A link to a page of the document is written with fit page, fit width or fit height; the Create Link dialog offers the fit, accepts only an http or https address with a host and a page number of the document, and a drag under 2 points draws no link.
resource: uncommitted on wasm-bundle (no pull request yet)
status: draft
generated: { by: claude-sonnet-5-5, at: 2026-11-10T00:00:00Z }
sources:
  - id: mods
    resource: /sources/modifications.md
---

# A link's page target has a fit, and its web address is checked

## Why
The story "Add a link over an area of a page" asks that a link's target is a web address or a page of the same document with a fit setting (fit page, fit width or fit height), that only a web address is accepted as an address, that a page number outside the document cannot be accepted, and that a slip draws nothing.[^mods] Before, a page target was always written `/Dest [page /Fit]`, the dialog accepted any text of more than three characters, the page was a drag value, and a drag of 4 x 4 screen pixels opened the dialog.

## What it changes
- `crates/annot/src/links.rs`:
  - new `LinkFit` (`Page`, `Width`, `Height`; `ALL`; `label()` gives the View menu texts "Fit to page", "Fit to width", "Fit to height"); `LinkAction::Page(usize)` becomes `LinkAction::Page { page, fit }`.
  - Writing: fit page is `[page /Fit]`, fit width `[page /FitH null]`, fit height `[page /FitV null]`. The top or left operand is null (no position asked for: the page's own edge); the design left it open. The viewer already follows `/FitH` and `/FitV` with a null operand (`DocView::go_to_dest`): it switches to fit width or fit height and shows the page, so no viewer code changed.
  - Reading: `/Fit` and `/FitB` are fit page, `/FitH` and `/FitBH` fit width, `/FitV` and `/FitBV` fit height; every other destination type (`/XYZ`, `/FitR`, missing type) is listed as fit page, as the page alone was before. A change that does not set the action (`SetLink` with `action: None`, the appearance edit) leaves the destination byte for byte.
  - new `web_address(text)`: trimmed text that begins with `http://` or `https://` (the scheme in any letter case, an addition the design asked to be decided) and has a host (not empty after a `user@` and a `:port` are taken off; no space in the authority; no control character). Anything else is an `AnnotError::Invalid` that says why. `links::add` and `links::set` apply it to a `Uri` action, so the engine edits `AddLink` and `SetLink` refuse any other address. `add_many` (links made from addresses found in the text) does not, so `mailto:` links found in text are still made.
- `crates/engine/src/lib.rs`: re-exports `LinkFit` and `web_address` (as `link_web_address`).
- `crates/automation/src/links.rs`: `link_add` and `link_edit` take an optional `fit` (`page`, `width`, `height`; only with `to_page`), refuse an address that is not a web address, and `link_list` reports `fit` for a page link that has a target.
- `crates/automation/src/tools.rs` (granted by the integrator for this change only): the `link_add` and `link_edit` schemas take `fit` (`page`, `width`, `height`) and their descriptions say so. A missing `fit` is fit page; `fit` without `to_page`, or with `url`, is refused.
- `crates/ui-egui/src/tools/link_ui.rs`:
  - `LinkDraft.target` is the typed text (a number entry, `widgets::line`) and `LinkDraft.fit` is the fit; a combo box offers "Fit to page", "Fit to width" and "Fit to height" (new existing texts only). The dialog still offers two target kinds only ("Open a web page", "Go to a page view").
  - OK is enabled by `target_ok`: a kept action, a web address by `web_address`, or a page number from 1 to the page count (`typed_page`: empty, 0, above the count, negative, decimal or text names none). The message says why: "Type the web address." (empty or "https://"), "Type a web address that begins with http:// or https://.", "Type a page number from 1 to {n}.".
  - An existing link whose action is `Other`, or a web address that is not http or https (a `mailto:` link of another program), has `preserve_action`: its appearance can be edited and its action is not rewritten.
  - A drag draws a link only where `drawn_link_rect` finds both sides at least 2 points in page space (was 4 screen pixels), so a slip opens no dialog. Cancel adds nothing; OK is one `Edit::AddLink`, one undo step (label "Add link").
- Other criteria that needed no code: the permission refusal (`Edit::AddLink` is in the modify group of `check_permission`, and the Link tool needs the modification permission); deleting a target page drops the destination of the links to it and keeps the others (`pdfcraft_organize::drop_destinations_to`); a link holds the page object, so a moved page is opened where it is now.

## New texts (in the local craft-i18n checkout only)
| English | French | Arabic |
|---|---|---|
| Type a web address that begins with http:// or https://. | Saisissez une adresse web commençant par http:// ou https://. | اكتب عنوان ويب يبدأ بـ http:// أو https://. |
| Type a page number from 1 to {n}. | Saisissez un numéro de page de 1 à {n}. | اكتب رقم صفحة من 1 إلى {n}. |

The three fit labels already exist in both catalogs ("Ajuster à la page", "Ajuster à la largeur", "Ajuster à la hauteur"; the Arabic "ملاءمة الصفحة", "ملاءمة العرض", "ملاءمة الارتفاع"). The French and Arabic wording of the two new texts was written by the worker, as the product owner decided (2026-11-10).

## Tests
- `crates/annot/src/tests.rs`: `a_page_target_keeps_its_fit_through_a_save_and_a_reopen`, `destinations_written_by_other_programs_read_as_their_fit_and_stay_when_the_action_is_not_set`, `a_link_accepts_only_a_web_address_with_a_host`; `links_are_added_edited_listed_and_removed` updated.
- `crates/engine/src/tests.rs`: `a_page_link_keeps_its_area_and_its_fit_when_saved_and_opened_again` (also: one undo step, the viewer's `DestView` of each), `a_page_link_follows_its_page_when_pages_move_and_loses_it_when_it_is_deleted` (four links to pages 3, 2, 4, 3), `adding_a_link_is_refused_when_the_document_forbids_modification`, `a_link_address_that_is_not_a_web_address_is_refused`.
- `crates/ui-egui/src/tools/link_ui.rs`: `only_a_web_address_with_a_host_is_accepted`, `a_page_number_must_name_a_page_of_the_document`, `a_page_target_carries_its_fit_into_the_edit`, `a_drag_under_two_points_on_a_side_draws_no_link`.
- `crates/ui-egui/tests/it/pdf_links.rs`: `a_slip_opens_no_dialog_cancel_adds_nothing_and_ok_adds_one_undo_step`; `fill_sign.rs` and `pdf_links.rs` adapted to `LinkAction::Page { .. }` and the typed page.
- The e2e criteria (four-page file, one link per fit, typed texts, no-modify sample, file with four links, French and Arabic labels) are for the integrator.

## Open points
- The `fit` argument is in the tool schemas (see above).
- The two new texts are added by the integrator to the local craft-i18n checkout only (`locales/fr/pdfcraft.tsv`, `locales/ar/pdfcraft.tsv`); they live only there until upstream craft-i18n has them, and `craft-i18n.rev` does not move (product owner's decision, 2026-11-10).
- A page link made with fit width or height has a null top or left, so following it keeps the reader's place on that axis as `go_to_dest` does today; a position was not asked for.
- A web address with a space after the host (`http://example.org/a b`) is accepted and written percent-encoded, as before.
- `Edit::AddLink` with a `Page` whose page does not exist still returns `NoSuchPage`; nothing new.

Not built or run.

[^mods]: [Fork items of the backlog](/sources/modifications.md).
