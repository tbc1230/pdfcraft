---
type: Source
title: Fork items of the backlog
description: The backlog items with Component PdfCraft fork or Workspace and PdfCraft fork, copied from the backlog. Copied unchanged below this header.
status: stable
resource: modifications.md
---

# PdfCraft fork - modifications

Extracted from workspace-backlog.md: every item whose Component is "PdfCraft fork" or "Workspace and PdfCraft fork", copied unchanged under its epic and feature.

## Epic: Combine files into one document

### Feature: Combining files with the editor's Combine

#### Technical story: Expose Combine to Workspace

Component: PdfCraft fork

The fork exposes its Combine (the pdfcraft-organize combine that upstream provides) in the exported JS API, taking the files' bytes in the order Workspace passes them, so that Workspace can run "Combine several files in chosen order" on files the user chose.
Acceptance criteria:
* The exported JS API has a function that accepts an ordered list of byte arrays and opens the combined document in the editor.
* The order of pages in the opened document equals the order of the byte arrays passed.
* Given an input could not be read, then the function reports which input could not be read and why, so that Workspace can show the reason under "Report why a document could not be opened", and opens no document.
* The function produces the per-file bookmarks and carries over links, named destinations, form fields, layers and attachments as the upstream Combine does; no fork change alters that behaviour.

## Epic: Understand what a document is made of

### Feature: Document composition

#### Story: See the document's properties

Component: PdfCraft fork

As a user, I want to see the properties the editor knows about the open document, so that I understand what it contains.
Acceptance criteria:
* Given a document is open, When the user views its properties, Then the editor shows its Document Properties: title, author, subject and keywords; the fonts used and whether each is embedded; PDF version, page size, tags, fields, layers and attachments; encryption method, which password opened it, and each document permission.
* Title, author, subject and keywords are shown read-only; they are separate from Workspace's metadata ("See a document's metadata") and are not synchronised with it.

## Epic: Read the document

### Feature: Display of every page

#### Story: Read every page of the document

Component: PdfCraft fork

As a user, I want every page of the open document displayed in the document view, so that I can read the whole document in Workspace.
Acceptance criteria:
* Given a document is open in the editor, when the user scrolls from the first page to the last page, then each page of the document is displayed in document order.
* Given a 500-page document is open in the editor, when the user scrolls to the last page, then the last page is displayed.
* Given a document with one page that cannot be drawn, when the document is open in the editor, then every other page of the document is displayed.
* Given a damaged document that the editor repairs when opening it, when it is open in the editor, then its pages are displayed. Documents that cannot be opened are handled by "Report why a document could not be opened".

### Feature: Page layouts

#### Story: Choose how pages are arranged

Component: PdfCraft fork

As a user, I want to choose a continuous, single page or two-up page layout, so that I can read the document in the arrangement that suits it.
Acceptance criteria:
* Given the continuous layout is chosen, when the user scrolls, then each page follows the previous one without a break.
* Given the single page layout is chosen, when a page is in view, then only that page is displayed.
* Given the two-up layout is chosen, when a page is in view, then two pages are displayed side by side.
* Given a document is open, when the user changes the page layout, then the document is not changed.

### Feature: View rotation

#### Story: Turn the displayed pages

Component: PdfCraft fork

As a user, I want to turn the displayed pages, so that I can read pages that are set sideways without changing the document.
Acceptance criteria:
* Given a document is open, when the user applies view rotation, then the pages in the document view are displayed turned.
* Given a document is open, when the user applies view rotation, then the document is not changed.

### Feature: Read mode

#### Story: Read without distractions

Component: PdfCraft fork

As a user, I want a Read mode that shows only the document, so that I can read long documents without other content in view.
Acceptance criteria:
* Given a document is open, when the user turns on Read mode, then the editor shows its focused document view without its panels.
* Given Read mode is on, when the user turns it off, then the editor shows the view it showed before Read mode was turned on.

### Feature: Layers

#### Story: Show or hide layers

Component: PdfCraft fork

As a user, I want to switch a document's layers on and off, so that I can read the page with or without that content.
Acceptance criteria:
* Given a document with layers is open, when the user switches a layer off, then the pages are displayed without that layer's content.
* Given a layer is switched off, when the user switches it on, then the pages are displayed with that layer's content.

## Epic: Read at a comfortable size

### Feature: Zoom

#### Story: Make pages larger or smaller

Component: PdfCraft fork

As a user, I want to zoom in and out on the pages, so that I can read the text at a size that suits me.
Acceptance criteria:
* Given a document is open, when the user zooms in, then the pages are displayed larger.
* Given a document is open, when the user zooms out, then the pages are displayed smaller.
* Given a page is displayed, when the user zooms in, then the text is shown sharp at the new magnification.
* Given a document is open, when the user zooms, then the document is not changed.

### Feature: Fit to the document view

#### Story: Fit pages to the document view

Component: PdfCraft fork

As a user, I want to fit the page, the page width or the page height to the document view, so that I can see the page at a size that matches my screen with one action.
Acceptance criteria:
* Given a document is open, when the user chooses fit page, then the whole page is displayed inside the document view.
* Given a document is open, when the user chooses fit width, then the page width matches the width of the document view.
* Given a document is open, when the user chooses fit height, then the page height matches the height of the document view.

## Epic: Move through the document

### Feature: Go to page

#### Story: Go directly to a page

Component: PdfCraft fork

As a user, I want to go straight to a page I name, so that I can reach it without scrolling through the pages before it.
Acceptance criteria:
* Given a document of N pages is open, when the user goes to page k (k from 1 to N), then page k is brought into view.
* Given page k is wider than the document view, when the user goes to page k, then page k is also brought into view horizontally.

### Feature: Page stepping

#### Story: Step to the next or previous page

Component: PdfCraft fork

As a user, I want to step one page forward or back, so that I can move through the document page by page.
Acceptance criteria:
* Given page k is in view and is not the last page, when the user steps forward, then page k+1 is brought into view.
* Given page k is in view and is not the first page, when the user steps back, then page k-1 is brought into view.

### Feature: Page labels

#### Story: See the document's own page numbering

Component: PdfCraft fork

As a user, I want pages identified by the document's own page labels, so that the numbers I see match the numbers printed in the document.
Acceptance criteria:
* Given a document whose page labels are i and ii for the first two pages and 1 for the third page, when the editor shows the third page's number, then it shows 1.
* Given a document without page labels, when the editor shows a page's number, then it shows the page's position in the document.

#### Story: See the page in view

Component: PdfCraft fork

As a user, I want to see which page is in view and how many pages the document has while I read, so that I know where I am in the document.
Acceptance criteria:
* Given a document with page labels is open, when a page is in view, then the editor shows the page label of that page, its position and the page count (for example "iii (3 of 120)").
* Given a document without page labels is open, when a page is in view, then the editor shows the position of that page and the page count.
* The fork's own controls ("Replace upstream menus and toolbars with the fork's own") keep this indicator.

### Feature: Links inside the document

#### Story: Follow a link to another place in the document

Component: PdfCraft fork

As a user, I want links that point inside the document to take me to their destination, so that I can follow references between pages.
Acceptance criteria:
* Given a link whose destination is a position and zoom on a page, when the user follows it, then that page is displayed with the destination position at the top-left of the document view and the destination zoom applied.
* Given a link whose destination is fit page, fit width or fit height, when the user follows it, then the destination page is displayed with that fit setting.
* Bookmarks are handled by "Jump to a bookmark".

## Epic: Present and print

### Feature: Full screen

#### Story: Present the document on the full screen

Component: PdfCraft fork

As a user, I want to show the document on the full screen, so that I can present it to others without other content in view.
Acceptance criteria:
* Given a document is open, when the user turns on full screen, then the document view fills the screen.
* Given the document is on the full screen, when the user leaves full screen, then Workspace shows the layout it showed before full screen was turned on.

### Feature: Print

#### Story: Print the document

Component: PdfCraft fork

As a user, I want to print the open document, so that I can get a paper copy.
Acceptance criteria:
* Given a document is open, when the user prints it, then the browser's print dialog opens with the pages of the document in document order.
* The paper copy shows the document as it is in the editor at the moment of printing, including markup, stamps, watermarks, drawn signatures and changes that are not saved yet.
* Markup prints by default; where the editor offers its comments and forms print option, the user can switch markup off with it.
* Pages reach the browser's print dialog as images at the dpi steps and caps PdfCraft uses; this is accepted for review copies, and users who need higher quality take a copy away with "Download as PDF" and print elsewhere.
* Given the browser refuses the print, then the user receives the print-ready PDF as a download instead.
* Given the user has selected pages, when the user prints, then only the selected pages are printed.

## Epic: Find words quickly

### Feature: Find as you type

#### Story: Search the whole document as I type

Component: PdfCraft fork

As a user, I want the whole document searched while I type, so that I can find words without waiting for a separate search step.
Acceptance criteria:
* Given a document is open, when the user types text to find, then the matches on all pages are found while the user types.
* Given the search has matches, when the matches are shown, then the editor shows the number of matches and which match is current (for example match 10 of 16).
* Given a page with text in columns, right-to-left text or CJK text, when the user searches for a word from that text, then the word is found.
* The search is case-insensitive by default; the user can switch on a case-sensitive option, and then only matches with the same case are found.
* The user can switch on a whole-word option, and then only matches that are whole words are found.
* No other find option is offered.

#### Story: Step through the matches

Component: PdfCraft fork

As a user, I want to step from one match to the next, so that I can check each place where the word occurs.
Acceptance criteria:
* Given the search has more than one match, when the user steps to the next match, then the next match in document order becomes current and its page is brought into view.
* Given the search has more than one match, when the user steps back, then the previous match in document order becomes current and its page is brought into view. The browser behaviour of the find-previous shortcut is covered by the spike "Event handling in the WASM editor".

#### Story: List all matches

Component: PdfCraft fork

As a user, I want a list of all matches, so that I can go straight to the one I need.
Acceptance criteria:
* Given the search has matches, when the user opens the results list, then every match is listed.
* Given the results list is shown, when the user chooses a match, then that match becomes current and its page is brought into view.

## Epic: Work faster on frequent actions

### Feature: Command search

#### Story: Find and run an action by name

Component: PdfCraft fork

As a user, I want to search for an editor tool or command by name and run it, so that I can reach it quickly without knowing where it is.
Acceptance criteria:
* Given a document is open, when the user opens command search and types part of an action's name, then every matching tool and command that an item of this backlog defines is listed with the tool it belongs to.
* Given command search lists matches, when the user chooses one, then that tool or command runs.
* Upstream functions that overlap Workspace concerns are hidden from command search, as "Hide upstream functions that overlap Workspace concerns" defines.
* The criteria of this story that rest on browser behaviour are provisional until the spike "Event handling in the WASM editor" reports; whether command search also finds Workspace actions outside the editor is decided by the spike "Event handling in the WASM editor".

### Feature: Keyboard shortcuts

#### Story: Run frequent editor actions from the keyboard

Component: PdfCraft fork

As a user, I want to use the editor's keyboard shortcuts, so that I can run frequent actions without the pointer. Examples are Command-K for command search and Command-G for the next match on macOS, and Page Down to step forward a page.
Acceptance criteria:
* Given the editor has keyboard focus and a shortcut is not also a browser shortcut, when the user presses the editor's shortcut for an action that an item of this backlog defines, then that action runs.
* Upstream functions that overlap Workspace concerns are hidden from keyboard shortcuts, as "Hide upstream functions that overlap Workspace concerns" defines.
* The criteria of this story that rest on browser behaviour are provisional until the spike "Event handling in the WASM editor" reports.

#### Spike: Event handling in the WASM editor

Component: Workspace and PdfCraft fork

Question: how do keyboard events reach the editor, which is Rust running as WASM, and the browser, given that event handling differs from normal HTML pages; and which shortcuts do browsers hand over to the page?
Acceptance criteria:
* Delivers how keyboard events reach the editor and the browser, and in which order.
* Delivers which shortcuts browsers hand over to the page and which they keep, including the find-next and find-previous shortcuts of "Step through the matches".
* Delivers which action runs in Workspace when an editor shortcut is also a browser shortcut (for example find next).
* Delivers which Workspace actions outside the editor, such as saving, have keyboard shortcuts and which keys they use.
* Delivers how users find out which keyboard shortcuts exist.
* Delivers whether command search also finds Workspace actions outside the editor, such as saving, versions and signatures.

## Epic: Mark passages of text

### Feature: Text markup tools

#### Story: Mark a passage with text markup

Component: PdfCraft fork

As a reviewer, I want to highlight, underline, squiggly-underline or strike out a passage I select, so that each passage is flagged in a way that shows what I mean.
Acceptance criteria:
* Given a page with selectable text, when the reviewer selects a passage and applies highlight, then the selected passage shows a highlight.
* Given four passages on one page, when the reviewer applies highlight to the first, underline to the second, squiggly underline to the third and strike out to the fourth, then the document holds four text markup annotations. Their subtypes are Highlight, Underline, Squiggly and StrikeOut, and each covers its own passage.
* Given a selected passage that runs over two lines, when the reviewer highlights it, then the highlight covers the selected text on both lines and no text outside the selection.
* Text markup is kept as stated in "Placed objects and corrections are kept in the saved document".

## Epic: Comment at a specific place

### Feature: Notes

#### Story: Place a note at a point on a page

Component: PdfCraft fork

As a reviewer, I want to attach a comment to a point on a page, so that readers know exactly what my comment refers to.
Acceptance criteria:
* Given an open document, when the reviewer chooses the note tool and clicks a point on a page, then a note is placed at that point and the reviewer can type its explanation.
* Given a placed note with an explanation, when the reviewer opens the note, then it shows the explanation the reviewer typed.
* The author recorded on the note is the logged-in user's display name, set through "Set the author name for markup"; the reviewer cannot change it in the editor.
* Notes are kept as stated in "Placed objects and corrections are kept in the saved document".

#### Story: Reply to a comment

Component: PdfCraft fork

As a reviewer, I want to reply to a comment, so that the discussion about one place stays together.
Acceptance criteria:
* Given a note with an explanation, when the reviewer adds a reply, then the comments panel shows the reply in the thread of that note.
* Given a thread with two replies, when the document is saved and opened again, then the thread shows both replies in the order they were added.
* The author recorded on a reply is the logged-in user's display name, set through "Set the author name for markup".

## Epic: Add visible remarks

### Feature: Text boxes

#### Story: Write a remark in a text box on a page

Component: PdfCraft fork

As a reviewer, I want to type a remark directly onto a page, so that it can be read on the page without opening a comment.
Acceptance criteria:
* Given an open document, when the reviewer chooses the text box tool, clicks a place on a page and types text, then the typed text appears on the page at that place.
* Given an existing text box, when the reviewer edits its text, then the page shows the edited text in place of the earlier text.
* Text boxes are kept as stated in "Placed objects and corrections are kept in the saved document".

## Epic: Sketch and frame content

### Feature: Ink drawing

#### Story: Draw freehand on a page

Component: PdfCraft fork

As a reviewer, I want to draw freehand on a page, so that I can point at content in my own way.
Acceptance criteria:
* Given an open document, when the reviewer chooses the ink tool and drags the pointer across a page, then a stroke appears along the path of the pointer.
* Given a drawn stroke, when the reviewer releases the pointer, then the stroke stays on the page.
* Ink drawings are kept as stated in "Placed objects and corrections are kept in the saved document".

### Feature: Shapes

#### Story: Draw a shape around content

Component: PdfCraft fork

As a reviewer, I want to draw shapes on a page, so that I can frame the content I am talking about.
Acceptance criteria:
* Given an open document, when the reviewer chooses the rectangle tool and drags from one point to another on a page, then a rectangle appears that spans the dragged area.
* Given each shape tool the editor provides, when the reviewer chooses it and drags on a page, then the shape appears across the dragged area.
* Shapes are kept as stated in "Placed objects and corrections are kept in the saved document".

## Epic: Correct wording in place

### Feature: Editing existing text

#### Story: Correct wording in a Latin-script paragraph

Component: PdfCraft fork

As a reviewer, I want to replace words in an existing paragraph, so that the document says the corrected text. The criteria of this story are limited to Latin-script paragraphs: no in-place correction is promised for CJK, right-to-left, rotated or subset-font text; for such text reviewers use text markup with the corrected wording in the explanation ("Mark a passage with text markup", "Add or change the explanation of a markup item") or a text box ("Write a remark in a text box on a page").
Acceptance criteria:
* Given a page with a Latin-script paragraph that uses an embedded or standard font, when the reviewer replaces a word in it, then the paragraph shows the corrected word and the replaced word no longer appears on the page.
* Given a corrected paragraph, when the corrected text is longer or shorter than the original, then the paragraph is rewrapped.
* Given a paragraph whose font cannot be reused for the corrected text, when the reviewer corrects it, then the corrected text is set in Helvetica.
* Corrections are kept as stated in "Placed objects and corrections are kept in the saved document".

## Epic: Insert images and custom marks

### Feature: Added images

#### Story: Place an image on a page

Component: PdfCraft fork

As a reviewer, I want to place an image such as a logo on a page, so that the page carries the mark I need.
Acceptance criteria:
* Given an image the reviewer has chosen, when the reviewer places it on a page, then the image appears on that page at the place the reviewer chose.
* The image comes from the reviewer's own computer; images from the fileserver are not offered; the accepted image formats are those the editor can place.
* An added image is page content and has no explanation of its own; the reviewer explains it by placing a note ("Place a note at a point on a page") on or next to the image, which carries the explanation and appears in the comments panel.
* Given a placed image, when the reviewer selects it later, then it can still be edited as stated in "Move and resize a placed object".
* Added images are kept as stated in "Placed objects and corrections are kept in the saved document".

## Epic: Link to related resources

### Feature: Links

#### Story: Add a link over an area of a page

Component: PdfCraft fork

As a reviewer, I want to turn an area of a page into a link with a target, so that readers can follow it to the related resource.
Acceptance criteria:
* Given an open document, when the reviewer marks an area of a page as a link and gives it a target, then the document holds a link over that area with that target.
* A link's target is one of two kinds: a web address, or a page of the same document with a fit setting; links to other files, to stored documents and other action types are not offered.
* Given a saved document with a link, when it is opened again, then the link is over the same area and has the same target.
* Following a link inside the document is defined by "Follow a link to another place in the document".
* Links are kept as stated in "Placed objects and corrections are kept in the saved document".

## Epic: Carry machine-readable codes

### Feature: QR codes and barcodes

#### Story: Place a QR code or barcode on a page

Component: Workspace and PdfCraft fork

As a reviewer, I want to place a QR code or a barcode on a page, so that readers can scan it.
Acceptance criteria:
* Given an open document, when the reviewer chooses to place a QR code or a barcode, then the code appears on the page as an added image.
* Given a placed code, then it can be moved and resized as "Move and resize a placed object" defines and is kept as "Placed objects and corrections are kept in the saved document" defines.

## Epic: Position placed objects precisely

### Feature: Moving and resizing

#### Story: Move and resize a placed object

Component: PdfCraft fork

As a reviewer, I want to move and resize a placed object, so that it sits exactly where and how I want it.
Acceptance criteria:
* All placed objects (notes, text boxes, ink drawings, shapes and added images) can be moved and resized; lines, arrows, polygons and clouds can also be reshaped by their handles; text markup is not moved or resized and stays on its text passage.
* Given a placed object on a page, when the reviewer selects it and drags it to another place on the same page, then it appears at the new place.
* Given a placed object, when the reviewer resizes it, then it appears with the new size.
* Given a moved and resized placed object, when the document is saved and opened again, then the object has the position and size the reviewer left it with.
* This applies to placed objects made by other users: any reviewer who may edit the document can move and resize any placed object, whoever made it; no ownership rule is enforced.

### Feature: Kept markup

#### Technical story: Placed objects and corrections are kept in the saved document

Component: PdfCraft fork

The fork's save ("Save and return the bytes to the client") writes every placed object, text markup, explanation, reply, link and text correction into the returned bytes, so that Workspace stores them through "Store saved bytes on the fileserver" and they are present and editable when the document is reopened.
Acceptance criteria:
* Given a document with a note with a reply, one text markup of each type, a text box, an ink drawing, a shape, an added image, a link and a corrected paragraph, when the fork's save returns the bytes and those bytes are opened in the editor again, then each of them is present on the same page with the same position, size, text and target.
* Given the reopened bytes, when the reviewer selects a placed object, then it can be moved and resized as stated in "Move and resize a placed object"; the text markup stays on its text passage.
* Given the reopened bytes, when the reviewer selects a markup item, then its explanation can be added or changed as stated in "Add or change the explanation of a markup item"; an added image has no explanation of its own and is explained by a note on or next to it, as "Place an image on a page" defines.

## Epic: Keep markup with its page

### Feature: Markup follows page operations

#### Story: Markup stays on its page when pages are moved, inserted or deleted

Component: PdfCraft fork

As a reviewer, I want my markup to move, stay or go with the page it is on, so that it always refers to the content I marked.
Acceptance criteria:
* Given a three-page document with a markup item on page 2, when the reviewer moves page 2 to the end ("Move pages to a new position"), then the markup item is on the moved page, now page 3, at the same place on that page.
* Given markup items on pages 1 and 2, when the reviewer deletes page 1 ("Delete selected pages"), then the markup items of page 1 are gone from the document and from the comments panel. The markup items of page 2 remain on that page, now page 1.
* Given a markup item on page 1, when the reviewer inserts a blank page before page 1 ("Insert blank pages"), then the markup item is on its original page, now page 2, and the inserted page has no markup.
* Given a deleted page that had markup, when the reviewer undoes the deletion ("Undo and redo page changes"), then the page returns with its markup items at their earlier places.

## Epic: Review all markup in one place

### Feature: Comments panel

#### Story: List all markup in the comments panel

Component: PdfCraft fork

As a reviewer, I want one list of all markup in the document, so that I can review it without searching page by page.
Acceptance criteria:
* Given a document with markup items on three different pages, when the reviewer opens the comments panel, then it lists one entry for each markup item.
* Given a note with replies, when the reviewer opens the comments panel, then the replies appear in the thread of that note.
* Given the comments panel is open, when the reviewer adds a markup item, then the panel gains an entry for it. When the reviewer deletes a markup item, the panel loses its entry.
* The author shown for each entry is the logged-in user's display name of the user who made it, set through "Set the author name for markup".
* Given the reviewer chooses an entry in the comments panel, then the markup item's page comes into view, the item is scrolled into the document view and it is selected.

## Epic: Explain markup

### Feature: Markup explanations

#### Story: Add or change the explanation of a markup item

Component: PdfCraft fork

As a reviewer, I want to add an explanation to a markup item and change it later, so that readers understand why I marked something.
Acceptance criteria:
* Given a highlight without an explanation, when the reviewer adds an explanation to it, then the comments panel entry for the highlight shows that explanation.
* Given a markup item with an explanation, when the reviewer changes the explanation, then the comments panel entry shows the new explanation and not the earlier one.
* Given one markup item of each kind (note, text markup, text box, ink drawing, shape), when the reviewer adds an explanation to each, then each one keeps its own explanation.
* This applies to markup made by other users: any reviewer who may edit the document can change the explanation of any markup item, whoever made it.

## Epic: Remove mistaken markup

### Feature: Deleting markup

#### Story: Delete a markup item

Component: PdfCraft fork

As a reviewer, I want to delete a markup item, so that markup I no longer want does not stay in the document.
Acceptance criteria:
* Given a selected markup item, when the reviewer deletes it, then it disappears from the page and from the comments panel.
* Given a deleted markup item, when the document is saved and opened again, then the markup item is not present.
* This applies to markup made by other users: any reviewer who may edit the document can delete any markup item, whoever made it; mistakes are recovered through "Undo and redo markup changes" and "Open an earlier version".

#### Story: Undo and redo markup changes

Component: PdfCraft fork

As a reviewer, I want to undo and redo my markup changes, so that a mistaken addition, change or deletion can be taken back.
Acceptance criteria:
* Given the reviewer deleted a markup item that had an explanation and replies, when the reviewer undoes, then the markup item returns with its explanation and replies.
* Given an undone deletion, when the reviewer redoes, then the markup item is removed again.
* Given the reviewer added a highlight, when the reviewer undoes, then the highlight is removed.

## Epic: Remove confidential content permanently

### Feature: Redaction with the editor

#### Technical story: Expose the editor's redaction in the WASM library build

Component: PdfCraft fork

The fork makes the editor's existing redaction available to Workspace through the exported JS API, so that Workspace users can redact content in the editor.
Acceptance criteria:
* Given a document is open in the editor through the WASM library build, when Workspace runs the editor's redaction through the exported JS API, then the redaction is applied to that document.
* Each mark type of "Redact confidential content from the result" (selected text, a dragged area, an image) is confirmed to work in the WASM library build.
* Given a redaction has been applied through the WASM library build, when Workspace calls "Save and return the bytes to the client", then the returned bytes do not contain the redacted content.
* Given a saved document with a redaction, when the text of the saved file is extracted with a tool other than the editor, then the redacted text is not found.

## Epic: See the document structure

### Feature: Page thumbnails

#### Story: See all pages in the Organize pages view

Component: PdfCraft fork

As a Workspace user, I want to see every page of the document at once as thumbnails, so that I understand its structure before I change it.
Acceptance criteria:
* Given a document of N pages is open in the editor, when the user opens the Organize pages view, then it shows N thumbnails in document order.
* Given a document with portrait and landscape pages, when the Organize pages view shows them, then each thumbnail keeps the aspect ratio of its page.
* Given the Organize pages view is open, when the user zooms the grid, then the zoom goes from 50% to 300% and the user can reset it.

#### Story: Keep the current page in sight in the page thumbnails panel

Component: PdfCraft fork

As a Workspace user, I want the page thumbnails next to the document view to follow the page I am reading, so that I always see where I am in the document.
Acceptance criteria:
* Given a document is open, when the page thumbnails panel is shown, then it shows one thumbnail for each page of the document.
* Given the thumbnail of the target page is out of sight in the page thumbnails panel, when the user goes to that page (by scrolling, "Go directly to a page" or "Jump to a bookmark"), then the panel scrolls until that thumbnail is visible.

## Epic: Arrange pages

### Feature: Move pages

#### Story: Move pages to a new position

Component: PdfCraft fork
As a Workspace user, I want to move one or several pages to another position, so that the document is in the order I need.
Acceptance criteria:
* Given the Organize pages view of a 5-page document, when the user drags the thumbnail of page 3 before page 1, then the former page 3 becomes page 1 and the former pages 1 and 2 become pages 2 and 3.
* Given the user has selected several pages by click, Command-click or Shift-click in the Organize pages view, when the user moves the selection earlier or later, then all selected pages move together and the page count does not change.
* Given a page that carries markup, when the page is moved, then the markup stays on that page as defined in "Markup stays on its page when pages are moved, inserted or deleted".
* Given a bookmark that opens page 3, when page 3 is moved to position 1, then the bookmark opens the moved page at position 1.

## Epic: Remove unwanted pages

### Feature: Delete pages

#### Story: Delete selected pages

Component: PdfCraft fork

As a Workspace user, I want to delete one page or a selection of pages, so that the document contains only the pages I need.
Acceptance criteria:
* Given the Organize pages view of a 5-page document, when the user deletes page 2, then the document has 4 pages and the former pages 3 to 5 become pages 2 to 4.
* Given the user has selected pages 2 and 4 by click, Command-click or Shift-click, when the user deletes the selection, then the document has 3 pages and pages 2 and 4 are no longer in it.
* Given a bookmark that opens page 4, when page 4 is deleted, then the bookmark no longer has a destination that points at the deleted page.
* Given a deleted page carried markup, when the page is deleted, then its markup is handled as defined in "Markup stays on its page when pages are moved, inserted or deleted".
* No confirmation is asked before pages are deleted; "Undo and redo page changes" and "Open an earlier version" are the protection.
* Given the selection includes every page of the document, when the user tries to delete it, then delete is unavailable and the user is told that at least one page must remain; a document with no pages is not allowed.
* Pages picked in the Pages panel stay selected after the panel is closed.

### Feature: Undo page changes

#### Story: Undo and redo page changes

Component: PdfCraft fork

As a Workspace user, I want to undo and redo page changes step by step, so that a page deleted by mistake is not lost.
Acceptance criteria:
* Given the user has deleted pages 2 and 4 in one change, when the user undoes once, then pages 2 and 4 come back at their former positions with their content, markup and bookmark destinations.
* Given a page change has been undone, when the user redoes, then the change is applied again.
* Given the user has made three page changes, when the user undoes three times, then each undo reverses one change, newest first.
* Given the user has deleted a page and the document has then been saved, when the user undoes, then the deleted page comes back in the editor.

## Epic: Extend the document

### Feature: Insert pages

#### Story: Insert blank pages

Component: PdfCraft fork

As a Workspace user, I want to insert a blank page, so that I have room in the document for new content.
Acceptance criteria:
* Given a document of N pages, when the user inserts one blank page, then the document has N+1 pages and the new page has no content.
* Given a blank page has been inserted, when the user undoes, then the document has N pages again as defined in "Undo and redo page changes".

#### Story: Insert pages from another PDF

Component: PdfCraft fork

As a Workspace user, I want to insert the pages of another PDF into my document, so that I can extend it with existing material.
Acceptance criteria:
* Given a document of N pages and another PDF of M pages chosen by the user, when the user inserts that PDF's pages, then the document has N+M pages and the inserted pages show the content of the other PDF.
* The other PDF comes from one of three sources: a file uploaded from the user's computer, converted first to PDF as "Image handling approach" decides; a PDF file stored on the backend, for which the backend checks read permission ("Read only with read permission") before delivering its bytes; and a file from the scanner, a third component outside the scope of this backlog. The document inherits no metadata and no versions from the other PDF.
* Given the other PDF has bookmarks, when its pages are inserted, then its bookmarks appear collapsed under one new bookmark titled with the other file's name without its extension, and that bookmark opens the first inserted page, as PdfCraft's insert from another file does (https://github.com/storytold/pdfcraft/pull/1947).
* Given the other PDF has no bookmarks, when its pages are inserted, then no bookmark is added, as PdfCraft's insert from another file does (https://github.com/storytold/pdfcraft/pull/1947).
* Given the other PDF has form fields, when its pages are inserted, then the fields on the inserted pages stay interactive.
* The inserted pages are recorded in the source record as "Keep the source record of a document" defines.

## Epic: Return to important pages

### Feature: Bookmarks

#### Story: Add a bookmark

Component: PdfCraft fork

As a Workspace user, I want to add a bookmark at the place I am reading, so that I can come back to it later.
Acceptance criteria:
* Given no text is selected, when the user adds a bookmark, then a bookmark titled "Untitled" is added, and it opens the current page at the position at the top of the document view.
* Given text is selected on a page, when the user adds a bookmark, then the bookmark is titled with the selected text on one line, at most 128 characters, and it opens the page at the top-left of the selection.
* Given a bookmark has just been added, when the Bookmarks panel shows it, then its title is open for renaming as defined in "Rename a bookmark".
* Given the document permissions do not allow assembly or the document is read-only, when the user tries to add a bookmark, then the action is unavailable.

#### Story: Rename a bookmark

Component: PdfCraft fork

As a Workspace user, I want to give a bookmark a name of my own, so that I recognise what it points to.
Acceptance criteria:
* Given a bookmark in the Bookmarks panel, when the user starts renaming it, then its whole current title is selected, so typing replaces it.
* Given the user has typed a new title and confirmed it, when the Bookmarks panel is shown, then the bookmark has the new title and still opens the same page.

#### Story: Jump to a bookmark

Component: PdfCraft fork

As a Workspace user, I want to choose a bookmark and go straight to its page, so that I return to important pages quickly.
Acceptance criteria:
* Given a document with bookmarks is opened, when the Bookmarks panel is shown, then it lists the document's bookmarks with their nesting.
* Given a bookmark that opens a position on page 7, when the user chooses it, then the document view shows page 7 at that position.
* Given a nested bookmark under a collapsed parent, when the user searches bookmark titles for its title, then the bookmark is listed together with its parents.

## Epic: Sign by hand

### Feature: Signature drawing canvas

#### Story: Draw a signature in Workspace

Component: Workspace and PdfCraft fork

As a user, I want to draw my signature on a canvas in Workspace, so that the editor can place it in the document I am working on.
Acceptance criteria:
* Given a document is open in the editor, when the user draws on the canvas with a pointer, then the strokes are shown on the canvas while they are drawn.
* When the user confirms the drawing, then the drawing becomes one image that the user places in the document.
* After the drawing is confirmed, the editor starts the interactive placement that "Place the drawn signature image" defines: the image follows the pointer at its natural size and the user clicks the spot on any page; cancelling places nothing.
* A drawn signature does not involve the backend's digital signing and does not make the document a signed document; given the open document is a signed document, then the drawn signature cannot be saved to it, as "Save changes to the fileserver" and "Show whether the document is signed" define.

## Epic: Share results that look the same anywhere

### Feature: Self-contained PDF output

#### Technical story: Markup carries its appearance in the saved file

Component: PdfCraft fork

The fork's save returns bytes in which every annotation of the kinds the editor builds (notes, text markup, shapes, ink and text boxes) carries its appearance stream, so that PDF readers other than Workspace show the markup without rebuilding it.
Acceptance criteria:
* Given a document containing one annotation of each kind the editor builds (a note, a text markup, a shape, an ink drawing and a text box), when the bytes are obtained through "Save and return the bytes to the client", then each of these annotations in the bytes has an appearance stream.
* Given those bytes are opened in a PDF reader that is not Workspace, then each annotation is shown on the same page and at the same position as in the editor.

## Epic: Control who may read and change documents

### Feature: Restrictions set inside a PDF

#### Story: Honour the restrictions set inside a PDF

Component: Workspace and PdfCraft fork

As a user, I want Workspace to respect the restrictions an author set inside a PDF, so that I am not offered actions the document forbids.
Acceptance criteria:
* Given a document whose author restricted it, when it is opened in Workspace, then the editor shows its notice about the restrictions and honours the document permissions, as the editor already does.
* Workspace does not offer an action that the document permissions forbid.
* Given an encrypted document is open in the editor and has been edited, when "Save and return the bytes to the client" returns the bytes, then the saved document stays encrypted as the document was.
* Workspace opens password-protected documents as "Enter a password for a protected document" defines.

## Epic: Keep a maintained editor

### Feature: Upstream sync of the fork

#### Technical story: Pull upstream updates into the fork

Component: PdfCraft fork

The fork pulls upstream updates and fixes regularly, so that the editor keeps receiving upstream improvements while the organisation uses its own editor. No cadence is a requirement: when to sync depends on the situation, and syncing at upstream releases is at most a recommendation.
Acceptance criteria:
* Given upstream has commits the fork does not contain, When a sync is performed, Then the fork contains those commits and keeps every change permitted by "Keep fork changes to what Workspace needs".
* After a sync, the editor builds as defined in "Build the fork as a WASM library" and every function of the exported JS API behaves as it did before the sync.
* Conflicts between upstream changes and the fork's changes are resolved at sync time; a sync is complete only when no unresolved conflict remains.
* After a sync, the rename and trademark removal of "Rename PdfCraft to Workspace and remove upstream trademarks" still hold.

#### Technical story: Remove upstream CLAUDE.md and AGENTS.md

Component: PdfCraft fork

The fork removes upstream CLAUDE.md and AGENTS.md, so that upstream agent instructions do not apply to the organisation's project.
Acceptance criteria:
* The fork contains neither CLAUDE.md nor AGENTS.md.
* Given upstream changes CLAUDE.md or AGENTS.md, When a sync is performed, Then the conflict is resolved at sync time and both files stay absent from the fork.

## Epic: Use the editor as a part of Workspace

### Feature: WASM library build of the fork

#### Technical story: Build the fork as a WASM library

Component: PdfCraft fork

The fork adds a library crate (cdylib, wasm-bindgen) built with wasm-pack into a package, so that Workspace imports the editor directly.
Acceptance criteria:
* The fork contains a library crate of type cdylib that uses wasm-bindgen.
* wasm-pack builds the crate into a package that a CSR web application imports directly as a module.
* The package exposes the exported JS API defined by the feature "Exported JS API".
* The editor runs inside the client that imports it: there is no iframe and no postMessage bridge between Workspace and the editor.

## Epic: Drive the editor from Workspace

### Feature: Exported JS API

#### Technical story: Mount the editor on a canvas supplied by the client

Component: PdfCraft fork

The fork exports a function that mounts the editor on a canvas element the client supplies, so that Workspace places the editor where its layout needs it.
Acceptance criteria:
* The exported JS API has a function that takes a canvas element supplied by the client and mounts the editor on it.
* After the call, the editing canvas renders into the supplied canvas element and receives the user's pointer and keyboard input there.
* Given the editor cannot be mounted, then the call returns a failure result to the client, which reports it through "Report that the editor could not be loaded".

#### Technical story: Open a document from bytes

Component: PdfCraft fork

The fork exports a function that opens a document from bytes the client passes, so that Workspace decides where documents come from.
Acceptance criteria:
* The exported JS API has a function that takes the bytes of a PDF document and opens it in the mounted editor.
* After a successful call, the document's pages are shown in the editing canvas.
* The call result tells the client whether the document was opened; a failure is returned to the client, which reports it through "Report why a document could not be opened".
* The editor never reads the fileserver itself; the client passes the bytes it obtained through "Deliver fileserver bytes to the editor" or "Upload a PDF or image file".

#### Technical story: Open a document from bytes with a password

Component: Workspace and PdfCraft fork

The fork exports the opening of a document from bytes with a password, so that Workspace can open password-protected documents through "Enter a password for a protected document". Whether the WASM library build already exposes password entry is to be verified in the fork.
Acceptance criteria:
* The exported JS API has a function that takes the bytes of a PDF document and a password and opens the document in the mounted editor.
* Given the bytes need a password and none or a wrong one is given, then the call returns a "password required" result that is distinct from every other failure result of "Open a document from bytes".
* The password is used only by the editor; the editor sends it nowhere.

#### Technical story: Expose insert pages from bytes

Component: Workspace and PdfCraft fork

The fork exports a function that inserts the pages of another PDF passed as bytes, so that "Insert pages from another PDF" works with the three sources it names. This item applies only if the WASM build takes the other PDF only through a file picker; this is to be verified in the fork before the item is built.
Acceptance criteria:
* The exported JS API has a function that takes the bytes of another PDF and inserts its pages into the open document as "Insert pages from another PDF" defines.
* The function produces the bookmark behaviour of "Insert pages from another PDF"; no fork change alters that behaviour.

#### Technical story: Save and return the bytes to the client

Component: PdfCraft fork

The fork exports a function that saves the open document and returns its bytes to the client, so that Workspace sends them to its backend.
Acceptance criteria:
* The exported JS API has a function that saves the open document and returns its bytes to the client.
* The returned bytes are a PDF document that contains every edit made in the editor up to the call.
* The editor sends the bytes nowhere; the client sends them to the backend as "Store saved bytes on the fileserver" defines.

#### Technical story: Run commands from the client

Component: PdfCraft fork

The fork exports a function that runs an editor command, backed by PdfCraftApp::run_command, so that Workspace triggers editing actions from outside the editor.
Acceptance criteria:
* The exported JS API has a function that runs an editor command identified by the client, backed by PdfCraftApp::run_command.
* Every editing action that an item of this backlog requires Workspace to trigger from outside the editor is runnable through this function.
* A command run through this function changes the document and the editor state exactly as the same action taken inside the editor does.

#### Technical story: Place the drawn signature image

Component: PdfCraft fork

The fork exports a function that takes a drawn signature image and starts its interactive placement in the document open in the editor, so that Workspace can hand over the signature the user drew in "Draw a signature in Workspace".
Acceptance criteria:
* The exported JS API has a function whose input is the drawn signature image.
* Given a document is open in the editor, when Workspace calls the function with an image, then the editor shows the image at the pointer at its natural size and the user clicks the spot on any page.
* Given the user clicks a spot, then the image is visible on that page at that spot in the editing canvas without reopening the document.
* The placed image is contained in the bytes returned by "Save and return the bytes to the client", and a PDF reader shows the image on that page as "Results show the same in other PDF readers" defines.
* The placement is one step in the editor's undo history: after the click, can_undo is true, and undo removes the placed image from the page.
* The placed image can be moved and resized as "Move and resize a placed object" defines, with corner handles that keep its proportions.
* Given the user cancels before clicking, then nothing is placed and the document is unchanged.

#### Technical story: Place an image from bytes

Component: Workspace and PdfCraft fork

The fork exports a function that takes image bytes and places the image in the open document, as a generalisation of "Place the drawn signature image", so that Workspace can place images it generates, such as the codes of "Place a QR code or barcode on a page".
Acceptance criteria:
* The exported JS API has a function whose input is an image as bytes.
* Given a document is open in the editor, when Workspace calls the function with an image, then the placement, visibility, saved bytes, undo and move and resize behaviour are those of "Place the drawn signature image".

#### Technical story: Place a stamp or watermark received from the client

Component: Workspace and PdfCraft fork

The fork lets the editor receive a stamp or a watermark from the client and place or apply it, so that "Apply an organisation stamp to a document" and "Apply an organisation watermark to pages" work without the editor contacting the existing APIs.
Acceptance criteria:
* Given the client passes a stamp's content, then the editor places the stamp as "Apply an organisation stamp to a document" defines.
* Given the client passes a watermark's content and the chosen pages, then the editor applies the watermark to those pages as "Apply an organisation watermark to pages" defines.
* The editor makes no request to the existing APIs; the backend obtains each stamp and watermark ("Obtain the organisation's stamps from the existing APIs", "Obtain the organisation's watermarks from the existing APIs") and the client passes its content to the editor.

#### Technical story: Report the page count to the client

Component: Workspace and PdfCraft fork

The fork exports the page count of the open document as an editor state value, so that "See the page count and the source file count" can show it.
Acceptance criteria:
* The exported JS API exposes the page count of the open document.
* Given a page change ("Move pages to a new position", "Delete selected pages", "Insert blank pages", "Insert pages from another PDF"), then the value reported after the change is the new page count.
* When a different document is opened in the editor, then the value returned is that of the newly opened document.

#### Technical story: Report the document information to the client

Component: Workspace and PdfCraft fork

The fork exports the document information stored inside the open PDF, so that Workspace can use the PDF's title as the suggested name in "Save as new Item".
Acceptance criteria:
* The exported JS API has a function that returns the title, author, subject and keywords stored inside the open document.
* The function changes nothing in the document; the document is not changed.

#### Technical story: Set the interface language of the editor

Component: Workspace and PdfCraft fork

The fork exports a function that sets the editor's interface language, so that Workspace sets the language instead of the editor's Preferences, which are not offered ("Hide upstream interface parts Workspace does not offer").
Acceptance criteria:
* The exported JS API has a function that takes a language and sets the editor's interface language to it.
* After the call, every text the editor shows to the user is shown in that language where the editor has a translation for it.

#### Technical story: Set the editor's theme

Component: Workspace and PdfCraft fork

The fork exports a function that sets the editor's theme, so that the editor follows the theme chosen in "Choose a reading theme". Whether the WASM editor already follows the browser's system setting is to be verified; if it does, the follow-the-system choice needs no call.
Acceptance criteria:
* The exported JS API has a function that takes one of light, dark and follow the system and sets the editor's theme to it.
* After the call, the editing canvas is shown in that theme.

#### Technical story: Set the author name for markup

Component: Workspace and PdfCraft fork

The fork exports a function that sets the author name, so that markup items and replies carry the logged-in user's display name as "Place a note at a point on a page", "Reply to a comment" and "List all markup in the comments panel" require.
Acceptance criteria:
* The exported JS API has a function that takes a display name and sets it as the author name.
* After the call, every markup item and reply the user creates records that name as its author.
* The author name cannot be changed inside the editor.

#### Technical story: Report editor state to the client

Component: PdfCraft fork

The fork exports the editor state the client's controls need, so that controls outside the editor behave correctly.
Acceptance criteria:
* The exported JS API exposes can_undo, can_redo, permissions, is_signed and edit_generation, taken from the engine.
* can_undo is true exactly when the editor has an edit to undo; can_redo is true exactly when the editor has an undone edit to redo.
* edit_generation has a different value after each edit of the open document than before it.
* permissions reports the open document's document permissions as the engine exposes them.
* is_signed reports whether the open document is signed: true for a document carrying a digital signature, false for a document carrying no signature.
* When a different document is opened in the editor, then the values returned are those of the newly opened document.
* A further state value that an item of this backlog needs for a Workspace control is added through an item with Component "Workspace and PdfCraft fork".

## Epic: Edit on the proven editing surface

### Feature: PdfCraft editing canvas kept in the fork

#### Story: Edit on the PdfCraft editing canvas

Component: PdfCraft fork

As a user, I want the editor's page view, selection, annotations and text editing to behave as in PdfCraft, so that I work on a proven editing surface.
Acceptance criteria:
* The editing canvas of the editor is upstream's editing canvas (page view, selection, annotations, text editing); the fork changes it only through items permitted by "Keep fork changes to what Workspace needs".
* Given a document opened through "Open a document from bytes", When the user views pages, selects, annotates or edits text on the editing canvas, Then the result equals the result of the same action in the upstream PdfCraft web build of the upstream version the fork is synced to.
* Upstream fixes to the editing canvas reach the editor through "Pull upstream updates into the fork".

## Epic: Present the editor under the organisation's own name and controls

### Feature: Renaming to Workspace

#### Technical story: Rename PdfCraft to Workspace and remove upstream trademarks

Component: PdfCraft fork

The fork renames PdfCraft to Workspace and removes upstream trademarks, so that the editor appears as the organisation's own.
Acceptance criteria:
* The fork contains none of the ArtCraft name, wordmark and logo files from docs/brand/, which docs/brand/LICENSE-brand.txt requires forks to remove; the fork's README carries no upstream trademarks.
* Every text the editor shows to the user names the editor "Workspace" where upstream shows "PdfCraft"; no text the editor shows to the user contains "PdfCraft" or "ArtCraft".
* The editor shows no link to getartcraft.com or discord.gg/artcraft.
* The rename covers what users see: editor texts including their translations, window and page titles, icons and images, links, the README and the docs/brand/ files.
* Crate names, Rust identifiers, command ids and upstream file names stay as upstream has them; code the fork adds may carry the Workspace name.
* LICENSE-MIT, LICENSE-APACHE, NOTICE and ATTRIBUTION.md stay in the fork, and the required licence and copyright notices stay.

### Feature: Own menus and toolbars in the fork

#### Technical story: Replace upstream menus and toolbars with the fork's own

Component: PdfCraft fork

The fork replaces upstream menus and toolbars with its own, so that the editor presents the organisation's controls.
Acceptance criteria:
* No upstream menu or toolbar is shown in the editor.
* The fork's own menus and toolbars offer only actions that the items of this backlog define; an upstream action that no item defines is not offered in them.
* Which other upstream interface parts are shown is defined by "Hide upstream interface parts Workspace does not offer".

#### Technical story: Hide upstream interface parts Workspace does not offer

Component: PdfCraft fork

The fork hides the upstream interface parts that no item of this backlog uses, so that users meet only the parts Workspace offers.
Acceptance criteria:
* The editor offers the parts a backlog item uses: the editing canvas, the comments panel, the Bookmarks panel, the page thumbnails panel, the Organize pages view, the find bar and its results list, the Layers panel, Document Properties and command search.
* The editor does not show the home screen, the tab strip, the All tools catalogue or Preferences.
* The Tools and Inspector panels are shown only as far as actions that items of this backlog define need them.
* The Fields panel is not shown until an item of this backlog defines a form-filling capability; form fields stay interactive on the editing canvas.
* The interface language is set by Workspace through "Set the interface language of the editor".

## Epic: Change the editor only where Workspace needs it

### Feature: Fork change policy

#### Technical story: Keep fork changes to what Workspace needs

Component: PdfCraft fork

The fork contains only changes that a Workspace feature, the plan or the rename requires, so that upstream sync stays feasible and application concerns stay in Workspace.
Acceptance criteria:
* Every difference between the fork and upstream belongs to one of: (a) a backlog item with Component "PdfCraft fork" or "Workspace and PdfCraft fork"; (b) the plan's fork work: "Pull upstream updates into the fork", "Remove upstream CLAUDE.md and AGENTS.md", "Build the fork as a WASM library", "Replace upstream menus and toolbars with the fork's own"; (c) "Rename PdfCraft to Workspace and remove upstream trademarks".
* The fork adds no implementation of document metadata, file versioning, login and permissions, fileserver access, stamps and watermarks from the existing APIs, digital signatures or the web application itself.
* The editor makes no request to the fileserver or to the existing APIs.

#### Technical story: Hide upstream functions that overlap Workspace concerns

Component: PdfCraft fork

The fork applies one list of offered commands, so that upstream functions that overlap Workspace concerns are hidden everywhere at once.
Acceptance criteria:
* Menus, toolbars, command search and keyboard shortcuts offer the same set of commands.
* The following upstream functions are not on the list and are not reachable through menus, toolbars, command search or keyboard shortcuts: editing the PDF's document information; upstream digital signing; adding or removing passwords; autosave and crash recovery; adding or extracting attachments.
* Attachments stay in the document and are carried over by "Combine several files in chosen order".
* Document Properties stays visible and read-only, as "See the document's properties" defines.
