---
type: Source
title: Workspace backlog
description: The full backlog of Workspace and the fork: terms, epics, features, items with acceptance criteria, open points. Copied unchanged below this header.
status: stable
resource: workspace-backlog.md
---

# Workspace - backlog

This backlog covers Workspace, the organisation's web application for opening, reading, reviewing, marking up, organising, stamping, signing and saving PDF and image documents, and the fork of PdfCraft from which its editor component is built. Its sources are the product owner decisions, the Workspace plan and the PdfCraft README and repository documentation; where these are silent, the backlog records an open point instead of a design. The old viewer's backlog was used only to identify capabilities: none of its features, values, limits, UI mechanics or technical details is a Workspace requirement.

Epics are capabilities: what Workspace enables users or the organisation to accomplish. Each epic holds Features; each Feature holds items of type Story, Technical story or Spike, with acceptance criteria and sources. Every item has a Component: "Workspace" (the web application, client and backend), "PdfCraft fork" (the fork from which the editor is built) or "Workspace and PdfCraft fork" (a Workspace feature that needs a named change in the fork). Open points at the end are product questions that no source answers; no item settles what an open point leaves open.

## Terms

* Workspace: the organisation's web application with client-side rendering (CSR) and a backend. It uses the editor as its PDF editor component and owns login, permissions, document metadata, versioning, the fileserver connection, stamps and watermarks from the existing APIs, digital signatures and all messages to the user.
* The client: Workspace's client-side code that runs in the browser and hosts the editor (the plan's "page").
* The backend: the server part of Workspace.
* PdfCraft: the upstream open-source PDF reader and editor at github.com/storytold/pdfcraft.
* Upstream: the unmodified PdfCraft repository from which the fork receives updates and fixes.
* The fork: the organisation's fork of PdfCraft, renamed to Workspace, kept in sync with upstream, from which the editor is built.
* The editor: the WASM package built from the fork; it shows pages and performs editing on the open document.
* WASM library build: the library crate in the fork, built with wasm-pack into the package Workspace imports.
* Exported JS API: the functions the WASM library build exposes to the client through wasm-bindgen.
* Editor state: the values the editor reports to the client: can_undo, can_redo, permissions, is_signed and edit_generation.
* edit_generation: the engine's count of edits to the open document; it has a different value after each edit.
* The editing canvas: PdfCraft's page view, selection, annotation and text editing surface, kept in the fork.
* Document view: the part of the editing canvas where document pages are displayed.
* Upstream trademarks: the ArtCraft name, wordmark and logos in docs/brand/ of the upstream repository, and the PdfCraft name.
* The fileserver: the organisation's document store from which Workspace reads documents and to which it saves them.
* The existing APIs: the organisation's APIs that provide its stamps, watermarks and certificates.
* Document: the PDF open in the editor.
* Stored document: a document held on the fileserver that Workspace can open.
* Own file: a PDF or image file a user brings in from their computer.
* Draft: unsaved work on any document: a document the user has changed in the editor and not yet saved, as well as an uploaded own file, a document combined from several files or a file from the scanner that the user has not yet stored with Save as new Item; only the second group has no fileserver identity, no metadata and no versions. See "Draft Management".
* Image file: a raster image file; TIFF is one accepted format, the others are decided by the spike "Image handling approach".
* Source file: one of the files from which a document was made, whether by opening a single file, by combining, or by inserting the pages of another PDF; Workspace records each source file in the source record of "Keep the source record of a document".
* Per-file bookmark: the bookmark that the editor's Combine creates for each source file, with that file's own bookmarks nested under it.
* Version: one stored state of a document that Workspace keeps and can tell apart from the other stored states of the same document.
* Metadata: the descriptive information Workspace keeps about a document.
* Read permission, save permission: the organisation's decision whether a logged-in user may read a document, or may save changes to it, enforced by Workspace.
* PDF file permissions: the restrictions a PDF's author set inside the file, which the editor honours and reports as permissions.
* Save: one of the three save actions, Save, Save as a Version and Save as new Item. In Save, the changes are stored to the open stored document without creating a version ("Save changes to the fileserver"); Save as a Version creates a version on request ("Save as a Version"); Save as new Item stores a draft that has no fileserver identity as a new stored document with its first version ("Save as new Item"). Saving does not end the work: the user saves and continues.
* Markup: the annotations users add to pages: notes, text markup, text boxes, ink drawings and shapes; every markup item other than a text box can carry replies. Text corrections, added images and links are named by their own terms and are not markup.
* Markup item: one annotation the editor builds: a note, a text markup, a text box, an ink drawing or a shape.
* Text markup: the PDF text markup annotation types highlight, underline, squiggly underline and strike out, applied to a passage of selected text.
* Note: a markup item type: a PDF file comment attached to a point on a page.
* Explanation: the text a reviewer attaches to a markup item to say why it was made.
* PDF file comment: a comment annotation in a PDF file: a note, or a reply to a markup item. It is part of the file and changes with the file's version; the editor makes it, and the comments panel lists it with the markup item it belongs to.
* Workspace Document Comment: a comment that Workspace manages through its own API and attaches to a document across all its versions; it is not in the PDF file and is outside the fork's scope.
* The bare word "comment" is not used in this backlog; every item says PDF file comment or Workspace Document Comment.
* Text box: a remark written directly onto a page.
* Reply: an answer to a markup item other than a text box, made in the editor as a reply annotation in the file; a markup item and its replies form a thread.
* Added image: an image that a reviewer places on a page as page content.
* Link: an area of a page that readers can follow to a target.
* Placed object: an added image, a note, a text box, an ink drawing or a shape. Text markup is not a placed object: it stays on its text passage.
* Comments panel: the editor's panel that lists a document's markup items, with the replies to each as a thread.
* Reviewer: a Workspace user who reviews and marks up a document.
* Stamp: one of the organisation's marks, provided by the existing APIs, that states a document's status.
* Watermark: one of the organisation's marks, provided by the existing APIs, that shows status or confidentiality across a page.
* Redaction: removing content from a document so that it cannot be seen, selected, copied or found in the saved result.
* Scripts embedded in PDF forms: scripts inside PDF files.
* Drawn signature: the image produced when a user draws a signature on a canvas in Workspace; the editor places it in the document. It is a picture of a signature, not a digital signature.
* Digital signature: a cryptographic signature applied to the PDF bytes on the backend with a certificate.
* Certificate: a signing certificate the organisation makes available through the existing APIs.
* Signed document: a document that Workspace reports as carrying a digital signature. A drawn signature does not make a document a signed document.
* Organize pages view: the editor's grid that shows every page of a document as a thumbnail and provides the page changes.
* Page thumbnails panel: the editor's panel of page thumbnails, shown next to the document view.
* Bookmarks panel: the editor's panel that lists a document's bookmarks as a nested tree.
* Bookmark: a named entry in the document's outline that opens a page at a position on that page.
* Page change: moving, deleting or inserting pages in a document.
* Page layout: how the document view arranges pages. The options are continuous, single page and two-up.
* View rotation: turning the displayed pages in the document view without changing the document.
* Fit page, fit width, fit height: zoom settings that size the page so that the whole page, its width or its height fits the document view.
* Page label: the document's own page numbering stored in the PDF (for example i, ii, 1, 2).
* Read mode: the editor's distraction-free reading mode.
* Full screen: the editor's layout in which the document fills the screen.
* Layer: optional content in a PDF that can be switched on and off.
* Match: one place where the searched text occurs in the document.
* Command search: the editor's search over every tool and command by name (the README calls it the command palette).

## Epic: Open stored documents

Capability: Users open documents held on the fileserver and start working on them in Workspace without first downloading them to their computer.

### Feature: Opening a stored document from the fileserver

#### Story: Choose a stored document

Component: Workspace

As a user, I want to find and choose the stored document to open, from a list or through a link, so that I reach the document I need.
Acceptance criteria:
* Given the user is logged in, when the user browses the list of stored documents, then the list shows only documents the user has read permission for ("Read only with read permission") and the user can filter it.
* Given the user chooses a document in the list, then it opens as "Open a stored document in the editor" defines.
* A link opens Workspace on one stored document and may name a version ("Open an earlier version").
* Given a link is opened by a user who is not logged in, then login comes first, as "Log in to Workspace" defines.
* Given a link is opened, then the backend checks read permission before delivering the document; a failed check is reported through "Report why a document could not be opened".
* Given a link that names no version, when it is opened, then the current version opens.

#### Story: Open a stored document in the editor

Component: Workspace

As a user, I want to open a stored document directly in Workspace, so that I can start reviewing it without downloading it to my computer first.
Acceptance criteria:
* Given the user is logged in and has read permission for the document (see "Read only with read permission"), When the user opens a stored document, Then Workspace reads it from the fileserver and the editor shows its first page.
* No file is written to the user's computer while a stored document is opened.
* Given the stored document has more than one version (see "See the versions of a document"), When the user opens it without choosing a version, Then the current version opens.
* Given the stored document is a PDF, Then the document opened is identical to the document held on the fileserver.

#### Technical story: Deliver fileserver bytes to the editor

Component: Workspace

Workspace reads the document bytes from the fileserver and hands them to the editor through "Open a document from bytes", so that the editor works on the stored document without a download step in the browser.
Acceptance criteria:
* Workspace obtains the document bytes from the fileserver; the editor never contacts the fileserver itself.
* Workspace calls "Open a document from bytes" with the complete byte array of the document; the editor receives no file path and no URL.
* Given the fileserver returns the current version of the document, When "Open a document from bytes" succeeds, Then Workspace records the fileserver identity of the document so that "Store saved bytes on the fileserver" can write to the same stored document.
* Given the fileserver does not return the document, Then "Report why a document could not be opened" applies.

## Epic: Bring own files in

Capability: Users bring PDF and image files from their own computer into Workspace for review and markup.

### Feature: Own file intake

#### Story: Upload a PDF or image file

Component: Workspace

As a user, I want to bring a PDF or image file from my computer into Workspace, so that I can review and mark it up there.
Acceptance criteria:
* Workspace accepts PDF files and image files from the user's computer; TIFF is accepted; the other accepted image formats are those decided by "Image handling approach".
* Given the user selects a PDF file, When the upload completes, Then the file opens in the editor with its pages shown.
* Given the user selects an image file in an accepted format, When the upload completes, Then "Review and mark up an image like a PDF page" applies and the image is shown and can be marked up; whether this happens in the editor or in another tool follows the spike "Image handling approach".
* Given the user selects a file that is neither a PDF nor an image file in an accepted format, Then Workspace does not open it and "Report why a document could not be opened" applies.
* Given the upload does not complete, Then "Report why a document could not be opened" applies.
* An uploaded file is a draft: it is not written to the fileserver until the user stores it with "Save as new Item", as "Draft Management" defines.

## Epic: Work on images as document pages

Capability: Users review, mark up and save image files, including TIFF, with the same result as PDF pages.

### Feature: Images as pages

#### Story: Review and mark up an image like a PDF page

Component: Workspace

As a user, I want an image file I brought in to appear as document pages, so that I can mark it up and save it exactly as I would a PDF.
Acceptance criteria:
* Given an uploaded image file in an accepted format, When intake completes, Then the image is shown and can be marked up; whether this happens in the editor or in another tool follows the spike "Image handling approach".
* Given the image is shown, When the user places markup on it, Then the markup is kept with the image.
* A document made from an uploaded image file is a draft; the user stores it with "Save as new Item".
* The number of pages produced from a multi-page TIFF follows the rule delivered by "Image handling approach".

### Feature: Image preparation for the editor

#### Spike: Image handling approach

Component: Workspace

Question: how does Workspace turn an uploaded image file, including TIFF, into pages the editor can work on? Options named in Decision 3: (a) convert image files to PDF on the backend after upload, so that the editor opens the result as an ordinary PDF; (b) use another tool for images. Input to be weighed: the PdfCraft repository documents that PdfCraft itself creates PDFs from images including multi-page TIFF (docs/target-app-parity.md, area "J Create and export"), that in its Combine list PNG, JPEG, TIFF, GIF, BMP and JPEG 2000 files become PDFs as they are added (ROADMAP.md, Progress log), and that its desktop builds create a PDF from image files (.png .jpg .jpeg .tif .tiff .gif .bmp .jp2 .j2k .jpx); no source states that this is available in the WASM build. Further input: an uploaded file is converted to PDF before it is combined ("Combine several files in chosen order"), and under option (a) converting an uploaded image does not make it a stored document ("Draft Management").
Acceptance criteria:
* The spike delivers the chosen option, (a) or (b), and names the component that performs the conversion (Workspace or the fork). For option (a): the backend conversion step and the PDF it produces, which the client opens through "Open a document from bytes". For option (b): which tool handles images and whether the editor is involved; if it is, the resulting items get Component "Workspace and PdfCraft fork".
* The spike verifies whether PdfCraft's creation of a PDF from image files is available in the WASM build before relying on it.
* The spike delivers the list of accepted image formats; TIFF is on the list.
* The spike delivers the rule for a TIFF file with more than one page: how many document pages it produces and in which order.
* The spike delivers the page size rule for a converted image (how the pixel dimensions of an image map to the page dimensions).
* The spike demonstrates with a TIFF file and with a PDF file that review, markup and save give the same result for both, as "Review and mark up an image like a PDF page" requires.
* Given a file cannot be converted to PDF, then the combine does not take place and the user is told which file could not be read or converted to PDF and why, and "Report why a document could not be opened" applies; no document is produced from that file.

## Epic: Combine files into one document

Capability: Users work on several files as a single document, with the files in the order they chose.

### Feature: Combining files with the editor's Combine

#### Story: Combine several files in chosen order

Component: Workspace

As a user, I want to pick several files, put them in the order I want, and have Workspace combine them into one document, so that I can review and mark them up as one.
Acceptance criteria:
* Given the user has chosen two or more files and set their order, When the user combines them, Then one document opens in the editor whose pages are all pages of the first file, then all pages of the second file, and so on, in the chosen order of files and the original page order inside each file.
* The files a user can combine come from three sources: a file uploaded from the user's computer, converted first to PDF as "Image handling approach" decides; a PDF file stored on the backend, for which the backend checks read permission ("Read only with read permission") before delivering its bytes; and a file from the scanner, a third component outside the scope of this backlog. The combined document inherits no metadata and no versions from a source.
* Given a chosen file is encrypted, then it is unlocked first as "Enter a password for a protected document" defines.
* Each source file gets a per-file bookmark in the combined document, and the source file's own bookmarks are nested under it, as the PdfCraft README states for Combine (section "Combine and split without losing a thing").
* Links and named destinations in the source files point to the copied pages; form fields stay interactive; layers keep their on/off defaults; attachments are carried over.
* Each page of the combined document renders identical to the same page in its source file.
* Image files among the chosen files become pages as decided by "Image handling approach" before they are combined.
* Given one of the chosen files cannot be read or cannot be converted to PDF, When the user combines, Then the combine does not take place and the user is told which file and why, as "Report why a document could not be opened" defines.
* The combined document is a single document for markup and page changes; it is a draft, and the user stores it with "Save as new Item".

#### Technical story: Expose Combine to Workspace

Component: PdfCraft fork

The fork exposes its Combine (the pdfcraft-organize combine that upstream provides) in the exported JS API, taking the files' bytes in the order Workspace passes them, so that Workspace can run "Combine several files in chosen order" on files the user chose.
Acceptance criteria:
* The exported JS API has a function that accepts an ordered list of byte arrays and opens the combined document in the editor.
* The order of pages in the opened document equals the order of the byte arrays passed.
* Given an input could not be read, then the function reports which input could not be read and why, so that Workspace can show the reason under "Report why a document could not be opened", and opens no document.
* The function produces the per-file bookmarks and carries over links, named destinations, form fields, layers and attachments as the upstream Combine does; no fork change alters that behaviour.

## Epic: Understand what a document is made of

Capability: Users see how many files and pages make up a document and which pages came from which file.

### Feature: Document composition

#### Story: See which pages came from which file

Component: Workspace

As a user, I want to see which pages of a document came from which source file, so that I know what I am reviewing and where it came from.
Acceptance criteria:
* Given a document made by "Combine several files in chosen order", When the user looks at its bookmarks (see "Jump to a bookmark"), Then one per-file bookmark per source file is present, named after the source file, and leads to the first page that came from that file.
* Given the user follows a per-file bookmark, Then the editor shows the first page of that source file.
* Given a document opened from one file, Then no per-file bookmarks are added.
* Workspace shows the source record of "Keep the source record of a document": for each source file its name, its origin and its page count at the time of the combine or insert.
* Given the user renames or deletes a per-file bookmark ("Rename a bookmark", "Delete selected pages" does not apply), Then the source record shown does not change; the per-file bookmark stays an ordinary editable bookmark.

#### Story: See the page count and the source file count

Component: Workspace

As a user, I want to see how many pages a document has and how many source files it was made from, so that I know the size and composition of what I am reviewing.
Acceptance criteria:
* Given a document is open, Then Workspace shows its page count as a number, and the number changes after each page change.
* Given the document has source files in the source record of "Keep the source record of a document", Then Workspace shows the number of source files as a number taken from that record.
* Given a document opened from one file, Then Workspace shows one source file.

#### Technical story: Keep the source record of a document

Component: Workspace

Workspace keeps a source record of each document outside the PDF, so that the composition of a document is known even after its bookmarks are edited.
Acceptance criteria:
* For each source file the record holds its name, its origin (an own file, a PDF file stored on the backend with its identity and version, or a file from the scanner) and its page count at the time of the combine or insert.
* Given pages are inserted from another PDF ("Insert pages from another PDF"), Then the record holds that file's name, origin and the number of inserted pages.
* The record describes the composition at combine or insert time; it is not changed by bookmark edits and is not updated after page changes.
* Given a document opened from one file, Then the record holds that one source file.

#### Story: See the document's properties

Component: PdfCraft fork

As a user, I want to see the properties the editor knows about the open document, so that I understand what it contains.
Acceptance criteria:
* Given a document is open, When the user views its properties, Then the editor shows its Document Properties: title, author, subject and keywords; the fonts used and whether each is embedded; PDF version, page size, tags, fields, layers and attachments; encryption method, which password opened it, and each PDF file permission.
* Title, author, subject and keywords are shown read-only; they are separate from Workspace's metadata ("See a document's metadata") and are not synchronised with it.

## Epic: Know when a document cannot be opened

Capability: Users are told why nothing is shown when a document fails to load or none is available.

### Feature: Load failure and empty state messages

#### Story: Report why a document could not be opened

Component: Workspace

As a user, I want Workspace to tell me why no document is shown when opening fails, so that I know what to do next.
Acceptance criteria:
* Given opening fails, Then Workspace shows a message instead of document pages; the message names the document (its stored name or the uploaded file name) and states one of the following causes: the document was not found on the fileserver; the user is not permitted to read it; the fileserver did not respond; the upload did not complete; the file type is not accepted; the file could not be converted to PDF; the editor could not open the file; a file to be combined could not be read or converted to PDF; password required or wrong.
* Given the cause is "the file type is not accepted", Then the message lists the accepted types: PDF and the image formats decided by "Image handling approach".
* Given the cause is "the editor could not open the file", Then the message includes the error text returned by the editor unchanged.
* Given the cause is "a file to be combined could not be read or converted to PDF", Then no document is shown; the message names that file and includes the reason reported by "Expose Combine to Workspace" or by the conversion of "Image handling approach".
* Given the cause is "password required or wrong", Then the message states which of the two it is and contains no password; the password is asked for as "Enter a password for a protected document" defines.
* Given the cause is "the user is not permitted to read it", Then the message contains no content of the document.
* Given a document is already open and opening another one fails, Then the already open document stays open and unchanged.

#### Story: Report that the editor could not be loaded

Component: Workspace

As a user, I want Workspace to tell me when the editor itself could not be loaded, so that I know the problem is not the document.
Acceptance criteria:
* Given the editor cannot be loaded or mounted, Then Workspace shows a message that the editor could not be loaded, with the browser's or the editor's error text unchanged, and a way to try again.
* The message is distinct from every cause of "Report why a document could not be opened".
* Given the browser does not support WebAssembly, Then the message names the browser as unsupported.
* While the editor is not loaded, the parts of Workspace that do not need the editor stay usable and the actions that depend on the editor are unavailable.
* Given the editor fails while a document is open, Then the failure is reported with the same message.
* Whether the message says that unsaved changes may be lost depends on "Draft Management".

## Epic: Continue earlier work

Capability: Users reopen a document saved from Workspace and find its markup, bookmarks and source information still editable.

### Feature: Reopening a saved document

#### Story: Reopen a saved document with editable markup, bookmarks and source information

Component: Workspace

As a user, I want to reopen a document I saved from Workspace and find my markup, my bookmarks and the per-file bookmarks still there and editable, so that I can continue where I stopped.
Acceptance criteria:
* Given a document saved under "Save changes to the fileserver" that contains markup, When the user reopens it from the fileserver, Then every piece of markup is present on the same page at the same position as when saved, as "Placed objects and corrections are kept in the saved document" defines.
* Given the reopened document, Then each markup item is listed under "List all markup in the comments panel", can be changed under "Add or change the explanation of a markup item" and can be removed under "Delete a markup item"; each placed object can be moved and resized under "Move and resize a placed object"; text markup stays on its text passage.
* Given the saved document contains bookmarks made by the user, When reopened, Then they are present as "Bookmarks are kept after saving and reopening" defines.
* Given the saved document contains page changes, When reopened, Then they are present as "Page changes are kept in the saved document" defines.
* Given a saved document made by "Combine several files in chosen order" and stored with "Save as new Item", When reopened, Then the per-file bookmarks and the source files' nested bookmarks are present as saved, and the source record is present as "Keep the source record of a document" defines.
* Given the saved document contains stamps, watermarks or signatures, When reopened, Then they are present as "Results show the same in other PDF readers" defines.
* The reopened document is the saved file as stored on the fileserver; changes discarded before the save are not restored.

## Epic: Read the document

Capability: Users read every page of a document on screen.

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

### Feature: Reading themes

#### Story: Choose a reading theme

Component: Workspace

As a user, I want to choose a light theme, a dark theme or the theme my system uses, so that I can read in the appearance that suits me.
Acceptance criteria:
* Workspace offers three theme choices: light, dark and follow the system.
* Given the user chooses a theme, then the whole Workspace layout uses it and the editor uses it.
* Given the user chooses follow the system, when the system setting changes, then Workspace and the editor follow the new setting.

## Epic: Read at a comfortable size

Capability: Users adjust how large pages appear and fit them to their screen.

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

Capability: Users go to any page directly and step between pages.

### Feature: Go to page

#### Story: Go directly to a page

Component: PdfCraft fork

As a user, I want to go straight to a page I name, so that I can reach it without scrolling through the pages before it.
Acceptance criteria:
* Given a document of N pages is open, when the user goes to page k (k from 1 to N), then page k is brought into view.
* Given page k is wider than the document view, when the user goes to page k, then page k is also brought into view horizontally.
* Every criterion of this item is tested end to end in the Mock/Stub, with a document of several pages, one of them wider than the document view.

### Feature: Page stepping

#### Story: Step to the next or previous page

Component: PdfCraft fork

As a user, I want to step one page forward or back, so that I can move through the document page by page.
Acceptance criteria:
* Given page k is in view and is not the last page, when the user steps forward, then page k+1 is brought into view.
* Given page k is in view and is not the first page, when the user steps back, then page k-1 is brought into view.
* Stepping works with the editor's page controls, and with the keyboard after the user has clicked the pages.
* Every criterion of this item is tested end to end in the Mock/Stub, with a document of several pages.

### Feature: Page labels

#### Story: See the document's own page numbering

Component: PdfCraft fork

As a user, I want pages identified by the document's own page labels, so that the numbers I see match the numbers printed in the document.
Acceptance criteria:
* Given a document whose page labels are i and ii for the first two pages and 1 for the third page, when the editor shows the third page's number, then it shows 1.
* Given a document without page labels, when the editor shows a page's number, then it shows the page's position in the document.
* The page label is shown wherever the editor shows a page's number: the page box, the page thumbnails and the Organize pages view.
* Every criterion of this item is tested end to end in the Mock/Stub, with a document labelled i, ii, 1 and a document without page labels.

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
* (added; hosting.md "State the client reads") Following a link does not change the document: `edit_generation` and `can_undo` are as before. Test: e2e.

## Epic: Present and print

Capability: Users present a document on the full screen and obtain a paper copy.

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
* (added; mods:216 and "Honour the restrictions set inside a PDF"; `chrome.rs:526-535`, engine `commands.rs:195`) Given the permissions do not allow printing the Print button is disabled with the editor's hover text, `runCommand("print.dialog")` returns `{ ok: false, error: "command print.dialog is not available now" }` and nothing prints. Test: e2e with the encrypted fixture (print bit cleared).
* (added; hosting.md) A print that succeeds does not change the document: `edit_generation` and `can_undo` are as before; the fallback download changes neither and sends no `pdfcraft:save` event. Test: e2e.
* (added; Issue 5) In a 1280 x 720 browser window, with the editor area the Mock/Stub gives (760 x 596 points), the whole Print dialog lies inside the editor area: its title, its settings and its Print and Cancel buttons have rectangles with 0 <= x, y and x1 <= 760, y1 <= 596. Test: e2e at `set_viewport_size(1280, 720)`: `runCommand("print.dialog")`, `uiInspect` rects of the button with role Button and label "Print", "Cancel" and the title (role Label, value "Print"); fork test in `tests/it/print.rs` with `Harness::builder().with_size(egui::vec2(760.0, 596.0)).build_eframe(...)` and `get_by_role_and_label(Role::Button, "Print")`.
* (added; Issue 5) At that size the Print button can be clicked and the browser's print dialog opens with the document's pages in order, and Cancel closes the dialog with nothing printed. Test: e2e at 1280 x 720 with `RECORD_PRINT` of `test_item_print.py`: `__printed` has one entry of 4 sheets for a 4-page document; Cancel leaves `__printed` unchanged and `docInspect().dialog` null. `start` moves to 1280 x 720, and `print_now` scrolls the dialog's settings before it clicks the "Comments & Forms" combo; the existing tests C1-C8 of that file are re-run at 1280 x 720 as part of this item.
* (added; Issue 5, the Preferences precedent `dialogs.rs:1371-1405`) When the settings are taller than the editor area they scroll inside the dialog and the Print and Cancel row stays in view below them; every setting can be reached by scrolling. Test: fork test at 760 x 596 and at the sample sizes below: the "Comments & Forms" control brought into view with `scroll_to_me`, and the Print button's rectangle unchanged by that.
* (added; Issue 5, the design above) The dialog is never wider than the editor area: below 860 points the preview moves under the settings, no control lies outside the area horizontally and no horizontal scroll is needed to reach a button. At 1400 x 900 the dialog keeps today's size (820 wide, two columns, no scroll). Test: fork test at 760 x 596 and at the narrower sample sizes below; at 1400 x 900 the existing tests `print_dialog_lays_out_sheets_and_saves_a_pdf` and the others in `tests/it/print.rs` pass unchanged.
* (added; Issue 5, `preferences_layout.rs:51-80`) The dialog opens at its final size and does not move or grow in the frames after it opens. Test: fork test: the rectangles of the title and the Print button are equal after 6 and 14 frames at 760 x 596 and at 1400 x 900.
* (added; Workspace supports French and Arabic, `fork/hosting.md:31`) The same holds with the interface in French and in Arabic (right to left): Print and Cancel inside the editor area at 760 x 596. Test: e2e; `setLanguage("fr")` and `("ar")`, same rectangle check (needs the build with craft-i18n; with Arabic, craft-fonts, Issue 3).
* (added; Issue 5, product owner 2026-11-09: the dialog scrolls, its Print and Cancel buttons stay reachable at any editor area size, no lower bound; the Preferences precedent, `dialogs.rs:1371-1405`, `preferences_layout.rs:28`) At any editor area size the body of the dialog scrolls and the Print and Cancel buttons stay inside the area and reachable, with no horizontal scroll needed to reach them; no smallest size is set. Test: fork test loops over sample sizes as `preferences_layout.rs:28` loops over heights: 760 x 596 (required, Issue 5), 1400 x 900, 520 x 400, 400 x 300 and 300 x 200; the list is test data, not a bound; e2e at 1280 x 720 and at 1024 x 600 windows.

## Epic: Find words quickly

Capability: Users search the text of a document and move between the matches.

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
* (added; "List all matches") The number of results `edit.advanced_search` lists equals "n" of the find bar. Test: e2e, the "N instances" label and the rows.
* (added; "Run commands from the client") The find bar opens with `runCommand("edit.find")` and typing works as by keyboard. Test: e2e.
* (added; hosting.md) Searching does not change the document: `edit_generation` and `can_undo` as before. Test: e2e.
* (added; `find.rs:120-135`) A page whose text arrives late is added to the count; the status ends without "+" and equals the number of pages that hold the word. Test: e2e, 100-page document.

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

Capability: Users perform frequent actions quickly and can discover how.

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

Capability: Reviewers flag passages of text in distinct ways: highlighted, underlined, squiggly-underlined or struck out.

### Feature: Text markup tools

#### Story: Mark a passage with text markup

Component: PdfCraft fork

As a reviewer, I want to highlight, underline, squiggly-underline or strike out a passage I select, so that each passage is flagged in a way that shows what I mean.
Acceptance criteria:
* Given a page with selectable text, when the reviewer selects a passage and applies highlight, then the selected passage shows a highlight.
* Given four passages on one page, when the reviewer applies highlight to the first, underline to the second, squiggly underline to the third and strike out to the fourth, then the document holds four text markup annotations. Their subtypes are Highlight, Underline, Squiggly and StrikeOut, and each covers its own passage.
* Given a selected passage that runs over two lines, when the reviewer highlights it, then the highlight covers the selected text on both lines and no text outside the selection.
* Text markup is kept as stated in "Placed objects and corrections are kept in the saved document".
* (added; "Run commands from the client", `offered-commands.md`: each tool is offered and reachable) Each of the four types is applied by a selection then `runCommand`; its command then a drag; the hotbar's Highlight flyout (Squiggly is on it); and, for highlight, underline and strike out, the bar next to a selection. Test: e2e; one annotation of the right subtype per type and path (the bar by `uiInspect` labels "Highlight", "Underline", "Strikethrough" and a click).
* (added; "Set the author name for markup") After `setAuthor("Alice Reviewer")` a new text markup records /T "Alice Reviewer", an empty /Contents and the page it is on. Test: e2e; `annots.py`; fork test: `set_author` then a markup edit, `comments(&h)[0].author`.
* (added; "Hide upstream functions that overlap Workspace concerns", `offered-commands.md` "one list that every surface follows"; the product owner's criterion "No hidden part can be opened from anywhere in the editor: not from its menus, toolbars, hotbar, ... context menus, command search or keyboard shortcuts", mods:1036) Replace text and Insert text are hidden: `runCommand("comment.replace")` and `("comment.caret")` return "is not offered in this editor"; the bar next to a selection offers "Add comment", Highlight, Underline, Strikethrough and Copy and no "Replace text"; the hotbar's Highlight flyout and command search list neither. Test: e2e (`uiInspect` labels after a selection, flyout labels, `palette_labels`); not a native test (`offered` is true natively). Code change: `selection.rs:211-221` filters by `offered()`.
* (added; failure; inference from `selection_quads`; area highlight: upstream, `canvas.rs` drag box with the Highlight tool, as PDF editors offer for scanned pages) A drag with the underline, squiggly underline or strike out tool over a page with no text (`pdfs.build(1)`) creates nothing: `edit_generation` and `can_undo` as before. With the highlight tool the same drag highlights the dragged area: one Highlight whose quad is the dragged box. `runCommand("comment.highlight")` with no selection returns `{ ok: true }`, adds nothing and leaves the highlight tool chosen with the Comments panel open. Test: e2e.
* (added; the selection has one page, `selection.rs:4-8`; the backlog speaks of "a passage") A drag that starts on one page and ends on the next adds markup on one page only: no annotation has quads on two pages. Test: e2e; `passages(pages=2)` in continuous layout, drag from page 1 over the page break into page 2; `annots.py` shows annotations on one page only. A passage over two pages needs two markup items.
* (added; "Honour the restrictions set inside a PDF") Given the PDF file permissions do not allow annotating, `runCommand` of each of the four commands returns `{ ok: false, error: "command comment.highlight is not available now: ..." }`, nothing is added, and the bar next to a selection is disabled (`add_enabled(can_comment, ...)`, `selection.rs:221`). Test: e2e `runCommand` refusals with the encrypted fixture; fork test of the disabled bar with an encrypted PDF that has text.

## Epic: Comment at a specific place

Capability: Reviewers leave PDF file comments attached to a point on a page and discuss them in threads.

### Feature: Notes

#### Story: Place a note at a point on a page

Component: PdfCraft fork

As a reviewer, I want to attach a PDF file comment to a point on a page, so that readers know exactly what it refers to.
Acceptance criteria:
* Given an open document, when the reviewer chooses the note tool and clicks a point on a page, then a note is placed at that point and the reviewer can type its explanation.
* Given a placed note with an explanation, when the reviewer opens the note, then it shows the explanation the reviewer typed.
* The author recorded on the note is the logged-in user's display name, set through "Set the author name for markup"; the reviewer cannot change it in the editor.
* Notes are kept as stated in "Placed objects and corrections are kept in the saved document".
* (added; the composer asks for the explanation first, `composer.rs:88`; sibling "Place an image on a page": a note "carries the explanation") A note exists when it is posted with a text: Cancel, Escape, or Post with no text places no note, and `edit_generation` and `can_undo` are as before. Test: e2e; click with the note tool, press Escape; again with Cancel.
* (added; "a note is placed at that point") The note is on the page that was clicked, with its icon's top-left at the point, also when the page is not the first and when the view is scrolled. Test: e2e; `passages(pages=3)`, click on page 2; annotation on page 2 only.
* (added; "Honour the restrictions set inside a PDF") Given the PDF file permissions do not allow annotating, `runCommand("comment.note")` is refused as in item 1, the page context menu's "Add comment" is disabled, and the panel has no "Add a comment" box. Test: e2e with the encrypted fixture.

#### Story: Reply to a PDF file comment

Component: PdfCraft fork

As a reviewer, I want to reply to a markup item other than a text box, so that the discussion about one place stays together in the file.
Acceptance criteria:
* Given a note with an explanation, when the reviewer adds a reply, then the comments panel shows the reply in the thread of that note.
* Given a thread with two replies, when the document is saved and opened again, then the thread shows both replies in the order they were added.
* The author recorded on a reply is the logged-in user's display name, set through "Set the author name for markup".
* (added; code, the story's "adds a reply") A reply is posted from the panel card's field with the Post button, plain Enter or Ctrl+Enter; Shift+Enter adds a line; the context menu's "Reply" opens the panel with the item's card selected and the field ready. Test: e2e; each way once; fork tests: `ctrl_enter_posts_a_reply_and_shift_enter_adds_a_line` (`comments.rs:466`) and a new one for plain Enter on the reply field.
* (added; "Reply: made in the editor as a reply annotation in the file", wb:54; `modify.rs:262-266`) A reply is a Text annotation in the file on the note's page, with /IRT naming the note, the typed /Contents and the author in /T; it is no card of its own and not counted by the header's "Comments N", only by "N replies". Test: e2e; `annots.py` on `save()` bytes; the header texts (`uiInspect`).
* (added; product owner 2026-11-09, following Acrobat; `modify.rs:236-260` makes the reply, no kind check) A reply can be added to a note, a text markup (highlight, underline, squiggly underline, strike out), a shape (rectangle, oval, line, arrow, polygon, connected lines, cloud) and a freehand drawing, also when the item was made by another user: the card shows the "Add a reply" field and the item's context menu offers "Reply". Test: e2e; `with_markup()` items by "Bob Reviewer" (note, highlight, Square, Ink) plus a polygon made in the editor: one reply each, one thread each.
* (added; product owner 2026-11-09) Reply is not offered on a text box, on any surface: the selected text box's card shows no "Add a reply" field, its context menu has no "Reply" entry, and the same for a text callout (a FreeText with intent FreeTextCallout in a file made elsewhere). A reply already in a file on a text box is still shown in its thread. Test: e2e; `with_markup()` FreeText selected on the page and from the panel: `uiInspect` has no "Add a reply" field and the `context_menu_labels` (`editor_ui.py`) have no "Reply"; a fixture FreeText with an /IRT reply shows the reply; a FreeText with /IT /FreeTextCallout likewise; fork test on the card and the menu for the FreeText case. Code change in `comments_panel.rs:518` and `menu.rs:91`.
* (added; `comments_panel.rs:530`) A reply with no text, or only spaces, cannot be posted: Post is disabled and `edit_generation` is unchanged. Test: e2e.
* (added; `comments_panel.rs:669-699`) A reply can be edited and deleted by its own "..." menu; deleting one reply leaves the note and the other replies, editing keeps its place in the thread and its author. Test: e2e; three replies, delete the second, the first and third stay in order; edit the first: `annots.py` /Contents changed, /T unchanged.
* (added; `modify.rs:236-270` has no lock check; `modify.rs:301-302`: a locked comment's "text and replies stay editable"; `modify.rs:90-99` for deletion) On a locked note a reply can be added and a reply can be edited; a locked reply cannot be deleted on its own (its menu entry is disabled) and keeps its note undeletable (item 8). Test: e2e; `with_markup()` locked note: add a reply; locked reply: its Delete is disabled.
* (added; "Honour the restrictions set inside a PDF") Given the PDF file permissions do not allow annotating, the panel offers no "Add a reply" field and the context menu's "Reply" is disabled; the engine refuses `ReplyToAnnotation` (`check_permission`). Test: e2e with the encrypted fixture holding a note: the note selected, no reply field; fork test of the refusal.

## Epic: Add visible remarks

Capability: Reviewers write remarks directly onto a page where every reader sees them.

### Feature: Text boxes

#### Story: Write a remark in a text box on a page

Component: PdfCraft fork

As a reviewer, I want to type a remark directly onto a page, so that it can be read on the page without opening a PDF file comment.
Acceptance criteria:
* Given an open document, when the reviewer chooses the text box tool, clicks a place on a page and types text, then the typed text appears on the page at that place.
* Given an existing text box, when the reviewer edits its text, then the page shows the edited text in place of the earlier text.
* Text boxes are kept as stated in "Placed objects and corrections are kept in the saved document".
* (added; mods:389, `fork/hosting.md:31`, `crates/fonts/src/craft.rs:54-56`) Given text in Arabic (right to left), typed into a text box in a build with craft-fonts, the text appears on the page in the Arabic face, shaped (letters joined) and read right to left, the box is sized and wrapped with the Arabic text's own widths, and the annotation is not marked /PCUndrawn. Test: fork test in `crates/annot/src/tests.rs` (skipped when `document_arabic_font()` is none, as `crates/edit/src/tests.rs:1282` does): the appearance of a FreeText with Arabic text embeds a font and `undrawable_text(d)` is none, joining and order checked against the shaper's output; fork test in `tests/it/text_box_arabic.rs`: the box rect measured at creation is wider than the same number of Latin characters would give only where the Arabic widths say so (rect equals the box computed from the Arabic widths within 1 pt); e2e: type one Arabic word as one insertion (Issue 6 was rejected: `keyboard.type` inserts per character), save, open the bytes in a new document, render the page with pypdfium2 (`pixels.py`) and compare the box region with the render of the same box holding "?" characters and with a render of the same word in a reference PDF drawn with the Arabic face: it differs from the first and matches the second within a tolerance; /PCUndrawn is absent in the new document's annotation (`FPDFAnnot_HasKey`).
* (added; `crates/annot/src/draw.rs:29-34`, the project's rule "never lose work silently") Given text in a script for which the build has no face (a script other than Western European and Arabic, or Arabic in a build without craft-fonts), the text is kept as typed in /Contents, the page shows `?` for those characters, the annotation is marked /PCUndrawn and the editor shows its notice (`docInspect().notice` is set). Test: e2e with a Cyrillic word in a new document; fork test `crates/annot/src/tests.rs:1774-1851` (existing pattern).
* (added; `composer.rs:88`) Cancel, Escape or Post with no text places nothing. Test: e2e, as item 2.
* (added; Workspace supports French, `fork/hosting.md:31`) Text with Western European characters, "Revision a faire" written with U+00E9, U+00E8, U+00E7 and U+0153, appears on the page as typed (no `?`), /Contents holds it, and the annotation is not marked /PCUndrawn. Test: e2e, typed as key presses (one key event per character, as a keyboard sends them), a new document per `/PCUndrawn` check; `FPDFAnnot_HasKey`.
* (added; `modify.rs:200-206`) Editing the text keeps the box's top-left and width; its height follows the lines. Test: e2e; the rect's top-left is the same before and after within 1 pt.
* (added; "Add or change the explanation of a markup item"; `set_contents`) The text of a text box is its explanation: the panel entry shows it, and changing the explanation from the panel changes the text on the page. Test: e2e; edit through the panel card, screenshot region changes.
* (added; "Hide upstream functions that overlap Workspace concerns", mods:1036) The Callout and Attach tools are hidden: `runCommand("comment.callout")` and `("comment.attach")` return "is not offered in this editor", and the note group's hotbar flyout and command search list neither. Test: e2e: one test opens each hotbar flyout (note, highlight, draw groups) and compares its labels with the allowlist of `commands.rs:72-92`.
* (added; "Honour the restrictions set inside a PDF") Given the PDF file permissions do not allow annotating, `runCommand("comment.freetext")` is refused as in item 1. Test: e2e, encrypted fixture.

## Epic: Sketch and frame content

Capability: Reviewers draw freehand and draw shapes to point at or frame content.

### Feature: Ink drawing

#### Story: Draw freehand on a page

Component: PdfCraft fork

As a reviewer, I want to draw freehand on a page, so that I can point at content in my own way.
Acceptance criteria:
* Given an open document, when the reviewer chooses the ink tool and drags the pointer across a page, then a stroke appears along the path of the pointer.
* Given a drawn stroke, when the reviewer releases the pointer, then the stroke stays on the page.
* Ink drawings are kept as stated in "Placed objects and corrections are kept in the saved document".
* (added; "Hide upstream functions that overlap Workspace concerns", `offered-commands.md`) The editor offers no Freehand-Highlight and no Eraser: `runCommand` of `comment.freehand_highlight` and `comment.eraser` returns "is not offered in this editor", and the quick bar's flyout and command search list no such entry. Test: e2e (`uiInspect` labels, `palette_labels`; the hotbar flyout test of item 3).
* (added; "Honour the restrictions set inside a PDF") Given the PDF file permissions do not allow annotating, `runCommand("comment.ink")` is refused as in item 1. Test: e2e, encrypted fixture.

### Feature: Shapes

#### Story: Draw a shape around content

Component: PdfCraft fork

As a reviewer, I want to draw shapes on a page, so that I can frame the content I am talking about.
Acceptance criteria:
* Given an open document, when the reviewer chooses the rectangle tool and drags from one point to another on a page, then a rectangle appears that spans the dragged area.
* Given each shape tool the editor provides that is drawn by dragging (rectangle, oval, line, arrow), when the reviewer chooses it and drags on a page, then the shape appears across the dragged area.
* Shapes are kept as stated in "Placed objects and corrections are kept in the saved document".
* (added; `input.rs:486-575`; `offered-commands.md` offers them as "each shape tool the editor provides"; completes the backlog wording) With the polygon, connected lines and cloud tools, clicks place the vertices (a preview follows), and a double-click, Enter, the menu's Complete, or for a polygon or a cloud of at least three points a click on its first point, finishes the shape across the clicked points. Test: e2e; click four points around a paragraph, double-click the last; one Polygon whose rect bounds the four points within 4 pt; the same for connected lines (three points); for the cloud, its /Vertices lie within 4 pt of the clicked points and its rect contains them (a cloud's rect also holds its scallops); the first-point close for polygon and cloud.
* (added; failure; `paint.rs:288-310`, `create.rs` `rect_for`) A polygon or cloud with fewer than three clicked points, connected lines with fewer than two, a rectangle or oval dragged with a side under 2 pt (a flat drag), a line or arrow shorter than 2 pt, and a click without a drag with the rectangle, oval, line or arrow tool, place nothing and change nothing. Test: e2e; `edit_generation` as before; a flat horizontal drag with the rectangle tool; fork test `drawn_shapes_ignore_slips` (`tools/comments/tests.rs:491-499`) extended.
* (added; "Honour the restrictions set inside a PDF") Given the PDF file permissions do not allow annotating, each of the seven commands is refused as in item 1. Test: e2e, encrypted fixture.

## Epic: Correct wording in place

Capability: Reviewers replace text in a document with corrected text.

### Feature: Editing existing text

#### Story: Correct wording in a paragraph

Component: PdfCraft fork

As a reviewer, I want to replace words in an existing paragraph, so that the document says the corrected text. The criteria of this story cover paragraphs in the supported languages, English, French and Arabic: no in-place correction is promised for other scripts (such as CJK), rotated or subset-font text; for such text reviewers use text markup with the corrected wording in the explanation ("Mark a passage with text markup", "Add or change the explanation of a markup item") or a text box ("Write a remark in a text box on a page").
Acceptance criteria:
* Given a page with a paragraph in English, French or Arabic that uses an embedded or standard font, when the reviewer replaces a word in it, then the paragraph shows the corrected word and the replaced word no longer appears on the page.
* Given a corrected paragraph, when the corrected text is longer or shorter than the original, then the paragraph is rewrapped.
* Given a paragraph whose font cannot be reused for the corrected text, when the reviewer corrects it, then the corrected text is set in a font the editor provides that draws it correctly.
* Corrections are kept as stated in "Placed objects and corrections are kept in the saved document".
* (added; "Honour the restrictions set inside a PDF") With modification forbidden, the edit-text command is refused ("is not available now"), a click on a paragraph opens no editor and the document is unchanged. Test: e2e, paragraph sample with no-modify protection; fork test.
* (added; "Undo and redo markup changes"; hosting "State the client reads") A correction is one undo step and one redo step; undo restores the original text. Test: e2e, paragraph sample.
* (added; backlog criterion 1 ("no longer appears")) After the correction the replaced word is gone from the page's text and from Find, not only from view. Test: e2e, paragraph sample.
* (added; "Save and return the bytes to the client") A correction typed but not yet applied when the client saves is in the returned bytes; if it cannot be applied the save fails with the editor's text and no bytes. Test: e2e.
* (added; project rule "never lose work silently" (as the text box story)) A character no available font can draw is refused: the paragraph is unchanged and the user is told why. Test: e2e, paragraph sample.
* (added; backlog criterion 3; hosting (English, French, Arabic)) A letter the paragraph's own font cannot draw (an accented French letter, an Arabic letter) is shown as typed, with no placeholder mark, in a font the editor provides; a font that already draws it is kept. Test: e2e, French and Arabic paragraphs, with and without the letters in the font.
* (added; backlog criterion 1 (paragraph in English, French or Arabic); hosting (Arabic, right to left)) An Arabic paragraph can be corrected: the corrected word shows right to left and shaped, the paragraph keeps its direction and alignment, and the replaced word is gone. Test: e2e, Arabic paragraph sample.
* (added; backlog criterion 2; practice) A correction affects only its paragraph: the next paragraph is unchanged and the corrected one keeps its edge, line spacing, size and colour; longer text gives more lines within the original width, shorter fewer. Test: e2e, two-paragraph sample.

## Epic: Insert images and custom marks

Capability: Reviewers place images, logos and custom marks on a page, with an explanation where useful.

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
* (added; code `content_ui.rs:915`; "Run commands from the client": the editor offers commands that apply to working in Workspace) In the web build, Add image (the Edit panel row, command search) opens the browser's file chooser for an image and, once a file is chosen, places it; the message at `content_ui.rs:915` is shown nowhere. Test: e2e as above; `runCommand("edit.image")` is accepted but cannot open a chooser without a user gesture (Playwright clicks carry one, `page.evaluate` does not, inference), so only acceptance is checked for that path. The placement itself: fork test `tests/it/content.rs`, `add_image("p.png", bytes)` on a one-page document gives one added image on page 0 and `can_undo` true.
* (added; "Honour the restrictions set inside a PDF", mods:707-716) Given the PDF file permissions do not allow modification, Add image is unavailable and `runCommand("edit.image")` returns `{ ok: false, error: "command edit.image is not available now" }`. Test: e2e with `pdfs.build(encrypt=(owner, user, P))` with the modify bit cleared.
* (added; hosting.md, "State the client reads") After the placement `getState().can_undo` is true and `edit_generation` differs; one `runCommand("edit.undo")` removes the image. Test: e2e.
* (added; product owner, 2026-11-08; as "Place the drawn signature image") After a file is chosen, the editor shows the image at the pointer at its natural size, and the reviewer's click on a page places it at that spot; given the reviewer cancels before clicking, nothing is placed and the document is unchanged. Test: e2e; screenshot region check at the click point; Esc before the click leaves `edit_generation` unchanged.
* (added; product owner, 2026-11-08; backlog "the accepted image formats are those the editor can place") The file chooser offers only the image formats the editor can place; a file in another format, such as DICOM, cannot be chosen. Test: e2e; the accept list of the chooser's input.
* (added; failure) A file that is not an image or cannot be read changes nothing (`edit_generation`, `can_undo` as before). The editor reports the failure as it reports any other failed edit (product owner, 2026-11-08). Test: e2e with a text file named `x.png`.

### Feature: Custom marks

#### Technical story: Provide the custom marks from the backend

Component: Workspace

The backend provides the custom marks that reviewers can place, so that which marks exist and where they come from stay a Workspace concern.
Acceptance criteria:
* Given the backend provides custom marks, when the reviewer starts to place a custom mark, then Workspace offers every mark the backend provides and no other.
* A custom mark reaches the editor in the way decided for "Place a stamp or watermark received from the client".

## Epic: Link to related resources

Capability: Reviewers add links that readers can follow to related resources.

### Feature: Links

#### Story: Add a link over an area of a page

Component: PdfCraft fork

As a reviewer, I want to turn an area of a page into a link with a target, so that readers can follow it to the related resource.
Acceptance criteria:
* Given an open document, when the reviewer marks an area of a page as a link and gives it a target, then the document holds a link over that area with that target.
* A link's target is one of two kinds: a web address, or a page of the same document with a fit setting (fit page, fit width or fit height); links to other files, to stored documents and other action types are not offered.
* Given a saved document with a link, when it is opened again, then the link is over the same area and has the same target.
* Following a link inside the document is defined by "Follow a link to another place in the document".
* Links are kept as stated in "Placed objects and corrections are kept in the saved document".
* (added; Terms "Fit page, fit width, fit height"; "Follow a link to another place in the document") The page target of a link has a fit the reviewer chooses: fit page (default), fit width or fit height; following the link shows the page with that fit. (changes upstream behaviour: only fit page today) Test: e2e, four-page file, one link per fit; fork test.
* (added; backlog criterion 2 (no other action types, no other files); practice) Only text that is a web address (begins http:// or https://, with a host) is accepted as a link's address; for other text the reviewer cannot accept and is told why. (changes upstream behaviour) Test: e2e, typed texts: no scheme, file:, javascript:, mailto:, ftp:.
* (added; backlog criterion 2) The dialog offers only the two target kinds; no file, stored document or other action. Test: e2e.
* (added; backlog criterion 1 ("a page of the same document"); practice) A page number outside the document (below 1, above the count, empty) cannot be accepted. Test: e2e, four-page file.
* (added; "Honour the restrictions set inside a PDF") With modification forbidden the link command is refused and a drag adds no link. Test: e2e, no-modify sample; fork test.
* (added; slip rule of "Draw a shape around content"; hosting "State the client reads") A drag below the slip threshold opens no dialog, Cancel adds nothing, OK adds one link as one undo step. Test: e2e.
* (added; "Delete selected pages" (a bookmark no longer has a destination that points at the deleted page)) When the target page is deleted the link no longer has a destination on it and the other links keep theirs. Test: e2e, file with four links.
* (added; "Move pages to a new position" (bookmarks)) When the target page is moved the link opens the moved page. Test: e2e, same file.
* (added; hosting (three languages)) The fit labels appear in French and Arabic when the language is set; item 6 owns these catalogue entries. Test: e2e. Open: who supplies the French and Arabic wording.

## Epic: Carry machine-readable codes

Capability: Reviewers add codes to a page that readers can scan.

### Feature: QR codes and barcodes

#### Story: Place a QR code or barcode on a page

Component: Workspace and PdfCraft fork

As a reviewer, I want to place a QR code or a barcode on a page, so that readers can scan it.
Acceptance criteria:
* Given an open document, when the reviewer chooses to place a QR code or a barcode, then the code appears on the page as an added image.
* Given a placed code, then it can be moved and resized as "Move and resize a placed object" defines and is kept as "Placed objects and corrections are kept in the saved document" defines.
* (added; "Place an image from bytes"; "Place the drawn signature image") Workspace hands a generated code to the editor as image bytes through a function of the exported API; the code follows the pointer at natural size, a click places it, cancel places nothing, one undo removes it, and refusals match the drawn-signature function (no document, no modification permission, not an image). Test: e2e, stand-in image of known module grid. The function's name is decided in the plan and `hosting.md`.
* (added; story purpose (readers can scan it); "Results show the same in other PDF readers") Module colours are kept at the placed size and after enlarging with a corner handle; the code stays square (proportions rule of item 1). Test: e2e, PDFium render at two sizes.
* (added; that criterion) Backlog criterion 2 (move, resize, kept after save) is tested on a placed code. Test: e2e, saved and reopened.

## Epic: Position placed objects precisely

Capability: Reviewers move and resize what they placed, and the result is kept in the document.

### Feature: Moving and resizing

#### Story: Move and resize a placed object

Component: PdfCraft fork

As a reviewer, I want to move and resize a placed object, so that it sits exactly where and how I want it.
Acceptance criteria:
* All placed objects (notes, text boxes, ink drawings, shapes and added images) can be moved and resized; lines, arrows, polygons and clouds can also be reshaped by their handles; text markup is not moved or resized and stays on its text passage.
* Given a placed object on a page, when the reviewer selects it and drags it to another place on the same page, then it appears at the new place.
* Given a placed object, when the reviewer resizes it, then it appears with the new size.
* Given a moved and resized placed object, when the document is saved and opened again, then the object has the position and size the reviewer left it with.
* This applies to placed objects made by other users: any reviewer who may edit the document can move and resize any placed object, whoever made it; no ownership rule is enforced. A locked item is not moved or resized, and a text box whose text the editor cannot redraw is not resized.
* (added; backlog criteria 1 and 3 as written; "Place a note at a point on a page") A note has two states, closed (the icon) and opened (its pop-up). The closed icon can be moved; an opened note can be moved and resized. Test: e2e, note sample in both states; fork test.
* (added; backlog criterion 1 (negative)) Text markup dragged across is not moved or resized and stays on its passage. Test: e2e, highlight sample.
* (added; "Honour the restrictions set inside a PDF") With annotating forbidden by the PDF, no handle is offered and no move, resize or reshape starts on an annotation; with modification forbidden an added image is not moved or resized; nothing changes. (changes upstream behaviour: resize starts today without the check) Test: e2e, markup sample with no-annotating protection; fork test for images with no-modify protection.
* (added; "Delete a markup item" (locked item)) A locked placed object shows no handle and is not moved or resized; nothing changes and no notice is shown. (changes upstream behaviour) Test: e2e, locked note and locked square; fork test.
* (added; backlog criterion 5) Moving or resizing another user's item keeps its author. Test: e2e, other user's square.
* (added; that sibling's slip rule) A click, or a drag below the slip threshold of "Draw a shape around content" (a side under 2 pt), only selects the object; nothing changes. Test: e2e, square.
* (added; "Undo and redo markup changes"; hosting "State the client reads") A move, resize or reshape of an annotation, and a move or resize of an added image, is one undo step; redo repeats it. Test: e2e, one of each type incl. line.
* (added; backlog criteria 2, 3) On a page with a rotated view an object ends where dropped, with the dragged size. Test: e2e, rotated one-page sample, saved file rendered by PDFium.
* (added; practice (Acrobat keeps an annotation within its page)) A moved or resized object stays on its page: a drag stops at the page edge. (changes upstream behaviour) Test: e2e, square, ink and an added image dragged past the edge.
* (added; project rule "never lose work silently") Resizing a text box whose text the editor cannot redraw is refused with the editor's text and nothing changes; moving it works. (a stated exception of backlog criterion 5) Test: fork test, text box from another program with text no standard font draws.

### Feature: Kept markup

#### Technical story: Placed objects and corrections are kept in the saved document

Component: PdfCraft fork

The fork's save ("Save and return the bytes to the client") writes every placed object, text markup, explanation, reply, link and text correction into the returned bytes, so that Workspace stores them through "Store saved bytes on the fileserver" and they are present and editable when the document is reopened.
Acceptance criteria:
* Given a document with a note with a reply, one text markup of each type, a text box, an ink drawing, a shape, an added image, a link and a corrected paragraph, when the fork's save returns the bytes and those bytes are opened in the editor again, then each of them is present on the same page with the same position, size, text and target.
* Given the reopened bytes, when the reviewer selects a placed object, then it can be moved and resized as stated in "Move and resize a placed object"; the text markup stays on its text passage.
* Given the reopened bytes, when the reviewer selects a markup item, then its explanation can be added or changed as stated in "Add or change the explanation of a markup item"; an added image has no explanation of its own and is explained by a note on or next to it, as "Place an image on a page" defines.
* (added; backlog criteria 1 and 3) A second save with no edit between returns the same objects (none twice); a save after one more edit holds all earlier objects and the new one once. Test: e2e, the full document of criterion 1.
* (added; those criteria) Backlog criteria 2 and 3 are tested on the reopened bytes: one object of each placed type is moved and one resized, text markup stays on its passage, and an explanation is added to one item and changed on another. Test: e2e, saved then reopened full document.

## Epic: Keep markup with its page

Capability: Markup stays on the right page when pages are added, removed or rearranged.

### Feature: Markup follows page operations

#### Story: Markup stays on its page when pages are moved, inserted or deleted

Component: PdfCraft fork

As a reviewer, I want my markup to move, stay or go with the page it is on, so that it always refers to the content I marked.
Acceptance criteria:
* Given a three-page document with a markup item on page 2, when the reviewer moves page 2 to the end ("Move pages to a new position"), then the markup item is on the moved page, now page 3, at the same place on that page.
* Given markup items on pages 1 and 2, when the reviewer deletes page 1 ("Delete selected pages"), then the markup items of page 1 are gone from the document and from the comments panel. The markup items of page 2 remain on that page, now page 1.
* Given a markup item on page 1, when the reviewer inserts a blank page before page 1 ("Insert blank pages"), then the markup item is on its original page, now page 2, and the inserted page has no markup.
* Given a deleted page that had markup, when the reviewer undoes the deletion ("Undo and redo page changes"), then the page returns with its markup items at their earlier places.
* (added; practice (Acrobat keeps comments of inserted pages); sibling "Insert pages from another PDF" is silent on markup (the editor already does this). An addition for the owner to confirm) Pages inserted from another PDF keep their markup at the same place with author and text; the document's own markup stays put. Test: e2e, two-page file plus the markup sample.
* (added; "Undo and redo page changes") Undo of a page move, of a blank-page insertion and of an insertion from another PDF puts the markup back with its page, and redo applies the change again. Test: e2e, three-page markup sample.
* (added; "List all markup in the comments panel") After each page change the comments panel has no entry for markup of a deleted page and none for an inserted blank page, and choosing an entry brings the page that now carries it into view. Test: e2e, three-page markup sample.
* (added; backlog criterion 3 as written (not bounded by what the editor does today)) A blank page can be inserted before page 1 and the markup then sits on its original page, now page 2. Test: e2e, markup sample.

## Epic: Review all markup in one place

Capability: Reviewers see every piece of markup in a document in one list.

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
* (added; "Markup item" in the Terms, mods:490-504) One entry per markup item for each of the five kinds (note, text markup, text box, ink, shape): a highlight with two lines is one entry, a note's Popup annotation is no entry, a reply is shown in its note's thread and is no entry of its own, and each entry names its kind (`kind_label`, `:41-49`). Test: e2e; `with_markup()` plus a two-line highlight: card count equals the item count; the labels are those.
* (added; "List all markup in the comments panel": "lists one entry for each markup item"; links and form fields are no markup, `info.rs:61-63`) Links and form fields are not entries. Test: e2e; `linked()` of `pdfs.py` (four links): "No comments yet."; a `text_field=True` fixture: none.
* (added; "Honour the restrictions set inside a PDF") Given the PDF file permissions do not allow annotating, the panel lists the markup items but has no "Add a comment" box and offers no edit or delete (`allowed`, `comments_panel.rs:132, 420-430`). Test: e2e with the encrypted fixture holding a Square: one card, no "Add a comment" field, no "More" button on the selected card.

## Epic: Explain markup

Capability: Reviewers add and change an explanation on each piece of markup.

### Feature: Markup explanations

#### Story: Add or change the explanation of a markup item

Component: PdfCraft fork

As a reviewer, I want to add an explanation to a markup item and change it later, so that readers understand why I marked something.
Acceptance criteria:
* Given a highlight without an explanation, when the reviewer adds an explanation to it, then the comments panel entry for the highlight shows that explanation.
* Given a markup item with an explanation, when the reviewer changes the explanation, then the comments panel entry shows the new explanation and not the earlier one.
* Given one markup item of each kind (note, text markup, text box, ink drawing, shape), when the reviewer adds an explanation to each, then each one keeps its own explanation.
* This applies to markup made by other users: any reviewer who may edit the document can change the explanation of any markup item, whoever made it.
* (added; the four ways of Today, "Add or change the explanation") The explanation can be added or changed from the panel card (double-click and "More" > Edit), the item's context menu ("Edit text...") and a double-click on the item; Cancel and Escape leave it unchanged (`edit_generation` as before). Test: e2e; each way once; then Cancel and Escape.
* (added; "any reviewer who may edit the document can change the explanation of any markup item"; `modify.rs:164-206, 301-302`) A locked item (flag Locked) can still have its explanation changed; the flag LockedContents is not honoured by the editor, recorded here as an implementing decision. Test: e2e; `with_markup()` locked note: the change succeeds.
* (added; `composer.rs:88`) Saving an empty explanation clears it: the card shows the kind in italics and /Contents is empty. Test: e2e.
* (added; Workspace's languages, `fork/hosting.md:31`; `set_contents`) An explanation in any script is kept exactly as typed in /Contents: "Revision a faire" with U+00E9 and an Arabic word (typed as one insertion) round-trip through `save()` and reopen, and the card shows the text. How the panel draws Arabic is the interface font's matter (Issue 3, fixed by craft-fonts). Test: e2e; /Contents compared with the typed string.
* (added; mods:1033, 1036; implementing decision) The card menu and the item's context menu do not offer "Set status", "Mark with checkmark", "Colour", "Copy text" and "Make Current Properties Default"; they offer "Edit text...", "Reply" (not on a text box, item 11), "Delete" and "Properties...". Test: e2e; select an item, open the card's "More" menu and the item's context menu, `menu_labels`/`context_menu_labels` (`editor_ui.py`) contain the four wanted entries and none of the five. Code change: `comments_panel.rs:588-640`, `menu.rs`.
* (added; "the reviewer cannot change it in the editor", "Set the author name for markup") The Properties dialog of a markup item shows its author and the field is read-only after `setAuthor` (`dialogs/comment_props.rs:477`). Test: e2e; fork test `tests/it/comments.rs` (the field is disabled when `author_locked`).
* (added; "Honour the restrictions set inside a PDF") Given the PDF file permissions do not allow annotating, the panel's Edit, the context menu's "Edit text..." and a double-click offer no editing; the engine refuses `SetAnnotationContents` (`check_permission`, `session/editing.rs`). Test: fork test with an encrypted PDF holding a note (the refusal, the disabled entries); e2e with the encrypted fixture: no Edit entry, no editor opens.

## Epic: Remove mistaken markup

Capability: Reviewers delete markup they no longer want, and can take back a mistaken change.

### Feature: Deleting markup

#### Story: Delete a markup item

Component: PdfCraft fork

As a reviewer, I want to delete a markup item, so that markup I no longer want does not stay in the document.
Acceptance criteria:
* Given a selected markup item, when the reviewer deletes it, then it disappears from the page and from the comments panel.
* Given a deleted markup item, when the document is saved and opened again, then the markup item is not present.
* This applies to markup made by other users: any reviewer who may edit the document can delete any markup item that is not locked, whoever made it; mistakes are recovered through "Undo and redo markup changes" and "Open an earlier version".
* (added; the Locked flag the author set inside the file, "Honour the restrictions set inside a PDF"; `modify.rs:82-100`) A locked item cannot be deleted by the key, the context menu or the card menu: the menu entries are disabled, the Delete key does nothing (the selection stays and no notice is shown), and nothing changes (`edit_generation`, `can_undo` as before). Test: e2e; `with_markup()` locked note: `annots.py` still lists it, `selected_comment` still names it; fork test of the key path. Code change: `keys` asks `deletable` before building the edit.
* (added; `modify.rs:90-99`) A note with a locked reply cannot be deleted either: the note's Delete is disabled in both menus and the Delete key does nothing. Test: e2e; `with_markup()` note with a locked reply.
* (added; "Delete a markup item"; `modify.rs:72-77`) Deleting a note that has replies deletes the replies with it and leaves no Popup: the card and its thread are gone, and the page lists no annotation of the note. One `edit.undo` brings the note and its replies back. Test: e2e; the note with two replies; `annots_all` (with Popup) lists none of them.
* (added; "Honour the restrictions set inside a PDF") Given the PDF file permissions do not allow annotating, the Delete key does nothing and the menus' Delete is disabled. Test: e2e with the encrypted fixture holding a Square: it stays.
* (added; `input.rs:694`) Backspace or Delete typed in a text field (the explanation editor, the composer, the find bar) edits the text and does not delete the selected item. Test: e2e; open the editor on a selected item, press Backspace: the item stays.

#### Story: Undo and redo markup changes

Component: PdfCraft fork

As a reviewer, I want to undo and redo my markup changes, so that a mistaken addition, change or deletion can be taken back.
Acceptance criteria:
* Given the reviewer deleted a markup item that had an explanation and replies, when the reviewer undoes, then the markup item returns with its explanation and replies.
* Given an undone deletion, when the reviewer redoes, then the markup item is removed again.
* Given the reviewer added a highlight, when the reviewer undoes, then the highlight is removed.
* (added; hosting.md, "State the client reads"; the one place the criterion is stated for all items) After each markup change of items 1 to 5, 7, 8 and 11 `getState().can_undo` is true, `edit_generation` differs from before and one `pdfcraft:state` was sent for it; `can_undo` is true exactly when an edit can be undone and `can_redo` exactly when an undone edit can be redone; after each undo or redo `edit_generation` differs from before and one `pdfcraft:state` is sent. Test: e2e; the sequence add, undo, redo with `getState()` and the event log (`window.__pdfcraft`) at each step, for a table of the operations of those items, each undone and redone and compared by screenshot region and `annots.py`; each undo takes back exactly one operation (one step per change).
* (added; `session/editing.rs:192`) A new markup change after an undo removes what could be redone: `can_redo` is false and `runCommand("edit.redo")` is refused with `{ ok: false, error: "command edit.redo is not available now" ... }`; with nothing to undo `edit.undo` is refused likewise. Test: e2e; fork test `tests/it/editing.rs`.
* (added; `commands.rs:110-111`, the client's buttons, hosting.md) Undo and redo run from `runCommand`, from Ctrl+Z and Ctrl+Shift+Z with the canvas focused, and from the client's Undo and Redo buttons, with the same result; the Stub's buttons are enabled exactly while `can_undo` / `can_redo`. Test: e2e; each way once on the same operation.
* (added; `editing.rs:383-386`) After an undo or redo no markup item is selected (`selected_comment` null) and the panel's editor is closed. Test: e2e.
* (added; hosting.md "Save", `session/save.rs:66-92`) After `save()` the markup can still be undone; the undo makes `getState().edit_generation` differ from the `edit_generation` of the save result (so the client shows unsaved changes), and a second `save()` returns bytes whose last revision has no such markup (`annots.py` reads the page's annotation list as PDFium loads it; the earlier revision in the incremental update still holds it). Test: e2e.

## Epic: Mark document status with stamps

Capability: Users apply the organisation's stamps, provided by the existing APIs, to mark a document's status.

### Feature: Organisation stamps from the existing APIs

#### Technical story: Obtain the organisation's stamps from the existing APIs

Component: Workspace

Workspace gets the organisation's stamps from the existing APIs and offers them to the user, so that users apply only the organisation's own stamps.
Acceptance criteria:
* Given the existing APIs return a list of stamps, when the user starts to apply a stamp, then Workspace offers every stamp in that list.
* Given the existing APIs return a list of stamps, when the user starts to apply a stamp, then Workspace offers no stamp that is not in that list.

#### Story: Apply an organisation stamp to a document

Component: Workspace

As a user, I want to apply one of the organisation's stamps to the document I have open, so that the document's status is marked.
Acceptance criteria:
* Given a document is open in the editor and a stamp is offered by "Obtain the organisation's stamps from the existing APIs", when the user chooses the stamp, then the editor shows it at the pointer at actual size and the user clicks the spot on any page; one click places one stamp as one undo step.
* Given the existing APIs prescribe a fixed position for the stamp, then the stamp is placed at that position without the click.
* Given a placed stamp, then the user can move and resize it.
* Given a stamp has been applied, when the document is saved as described in "Save changes to the fileserver", then the saved PDF contains the stamp.
* An applied stamp can be moved, resized or removed, and these changes undone, before a save, after a save in the same session without reopening, and after the document is reopened by a user with save permission; earlier versions keep the stamp as stored.
* Given the open document is a signed document, then no change to its stamps can be saved to it, as "Save changes to the fileserver" and "Show whether the document is signed" define.

## Epic: Show status or confidentiality across a page

Capability: Users apply the organisation's watermarks, provided by the existing APIs, to a page.

### Feature: Organisation watermarks from the existing APIs

#### Technical story: Obtain the organisation's watermarks from the existing APIs

Component: Workspace

Workspace gets the organisation's watermarks from the existing APIs and offers them to the user, so that users apply only the organisation's own watermarks.
Acceptance criteria:
* Given the existing APIs return a list of watermarks, when the user starts to apply a watermark, then Workspace offers every watermark in that list.
* Given the existing APIs return a list of watermarks, when the user starts to apply a watermark, then Workspace offers no watermark that is not in that list.

#### Story: Apply an organisation watermark to pages

Component: Workspace

As a user, I want to apply one of the organisation's watermarks to pages of the document I have open, so that the pages show their status or confidentiality.
Acceptance criteria:
* Given a document is open in the editor and a watermark is offered by "Obtain the organisation's watermarks from the existing APIs", when the user applies the watermark, then the user chooses all pages (the default), the current page or a typed page range, and the editor shows the watermark on each chosen page.
* The watermark is centred on each chosen page, with an offset only if the existing APIs define one.
* Applying a watermark is one undo step.
* Given a watermark has been applied, when the document is saved as described in "Save changes to the fileserver", then the saved PDF contains the watermark on each page it was applied to.
* An applied watermark can be updated or removed, and these changes undone, before a save, after a save in the same session without reopening, and after the document is reopened by a user with save permission; earlier versions keep the watermark as stored.
* Given the open document is a signed document, then no change to its watermarks can be saved to it, as "Save changes to the fileserver" and "Show whether the document is signed" define.

## Epic: Remove confidential content permanently

Capability: Users redact confidential content so that it cannot be seen, selected, copied or found in the saved result.

### Feature: Redaction with the editor

#### Technical story: Expose the editor's redaction in the WASM library build

Component: PdfCraft fork

The fork makes the editor's existing redaction available to Workspace through the exported JS API, so that Workspace users can redact content in the editor.
Acceptance criteria:
* Given a document is open in the editor through the WASM library build, when Workspace runs the editor's redaction through the exported JS API, then the redaction is applied to that document.
* Each mark type of "Redact confidential content from the result" (selected text, a dragged area, an image) is confirmed to work in the WASM library build.
* Given a redaction has been applied through the WASM library build, when Workspace calls "Save and return the bytes to the client", then the returned bytes do not contain the redacted content.
* Given a saved document with a redaction, when the text of the saved file is extracted with a tool other than the editor, then the redacted text is not found.

#### Story: Redact confidential content from the result

Component: Workspace

As a user, I want to redact confidential content in a document, so that nobody can see, select, copy or find it in the saved result.
Acceptance criteria:
* Workspace offers redaction marks over selected text, over a dragged area and over an image; a form field value is redacted by marking its area; search-and-redact is not offered.
* Given marks are applied, then the glyphs, pixels, vectors, embedded content, annotations, form fields and tags under the marks are removed.
* Known limits: partial words cannot be redacted, and vertical text is refused.
* Given the user has redacted a piece of text and saved the document, when the saved document is opened in the editor, then the text is not shown on the page.
* Given the same saved document, when the user selects the area where the text was, then the selection contains none of the redacted text.
* Given the same saved document, when the user copies the text of the page, then the copied text does not contain the redacted text.
* Given the same saved document, when the user searches for the redacted text, then the search finds no match.
* Given a saved document with a redaction, when the text of the saved file is extracted with a tool other than the editor, then the redacted text is not found.
* Given a saved document with a redaction, then no part of the redacted content remains anywhere in the saved file.

## Epic: See the document structure

Capability: Users get an overview of all pages of a document.

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

Capability: Users change the order of pages, and markup and bookmarks follow the pages they belong to.

### Feature: Move pages

#### Story: Move pages to a new position

Component: PdfCraft fork
As a Workspace user, I want to move one or several pages to another position, so that the document is in the order I need.
Acceptance criteria:
* Given the Organize pages view of a 5-page document, when the user drags the thumbnail of page 3 before page 1, then the former page 3 becomes page 1 and the former pages 1 and 2 become pages 2 and 3.
* Given the user has selected several pages by click, Command-click or Shift-click in the Organize pages view, when the user moves the selection earlier or later, then all selected pages move together and the page count does not change.
* Given a page that carries markup, when the page is moved, then the markup stays on that page as defined in "Markup stays on its page when pages are moved, inserted or deleted".
* Given a bookmark that opens page 3, when page 3 is moved to position 1, then the bookmark opens the moved page at position 1.

### Feature: Keep page changes

#### Technical story: Page changes are kept in the saved document

Component: Workspace

Workspace keeps every page change in the document it saves and reopens, so that the stored document has the pages and order the user created in the editor.
Acceptance criteria:
* Given the user has moved, deleted or inserted pages, when the document is saved as defined in "Save changes to the fileserver" and then reopened as defined in "Open a stored document in the editor", then the document has the same page count and page order as before the save.

## Epic: Remove unwanted pages

Capability: Users delete pages they do not need and can recover them if they deleted them by mistake.

### Feature: Delete pages

#### Story: Delete selected pages

Component: PdfCraft fork

As a Workspace user, I want to delete one page or a selection of pages, so that the document contains only the pages I need.
Acceptance criteria:
* Given the Organize pages view of a 5-page document, when the user deletes page 2, then the document has 4 pages and the former pages 3 to 5 become pages 2 to 4.
* Given the user has selected pages 2 and 4 by click and Command-click, when the user deletes the selection, then the document has 3 pages and pages 2 and 4 are no longer in it.
* Given a bookmark that opens page 4, when page 4 is deleted, then the bookmark no longer has a destination that points at the deleted page.
* Given a deleted page carried markup, when the page is deleted, then its markup is handled as defined in "Markup stays on its page when pages are moved, inserted or deleted".
* No confirmation is asked before pages are deleted; "Undo and redo page changes" and "Open an earlier version" are the protection.
* Given the selection includes every page of the document, when the user tries to delete it, then delete is unavailable and the user is told that at least one page must remain; a document with no pages is not allowed.
* Pages picked in the Pages panel stay selected after the panel is closed.
* (added; "Run commands from the client": a command not available in the current state is refused with its reason; `commands.rs:180-203`) `runCommand("page.delete")` with every page selected, and with no selection in a one-page document, returns `{ ok: false, error }` with a reason and the document is unchanged. Test: e2e both cases. One arm in `command_enabled` serves every surface.
* (added; `canvas.rs:905`) In a one-page document the page context menu entry is disabled; in a document of several pages it deletes the clicked page. Test: e2e.
* (added; "Report the page count to the client") After a delete `getState().page_count` is the new count and one `pdfcraft:state` was sent. Test: e2e.
* (added; "Undo and redo page changes": deleting pages 2 and 4 in one change, one undo brings both back) Deleting a selection is one undo step. Test: e2e, undo once gives 5 pages with the original ratios.
* (added; "Honour the restrictions set inside a PDF") Given the permissions do not allow assembly, delete is unavailable on every surface and the command is refused with a reason. Test: e2e with the encrypted fixture.
* (added; "Hide upstream functions that overlap Workspace concerns") Delete is enabled and disabled in the same states on the Pages panel, the Organize grid, the Organize pages panel, the context menu, command search and the keys. Test: e2e, one page selected and all pages selected.

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

Capability: Users add new pages or pages taken from another PDF to a document.

### Feature: Insert pages

#### Story: Insert blank pages

Component: PdfCraft fork

As a Workspace user, I want to insert a blank page, so that I have room in the document for new content.
Acceptance criteria:
* Given a document of N pages, when the user inserts one blank page, then the document has N+1 pages and the new page has no content.
* Given a blank page has been inserted, when the user undoes, then the document has N pages again as defined in "Undo and redo page changes".
* (added; code `commands.rs:503-510`) The new page follows the last selected page, or the current page when none is selected, and has that page's size. Test: e2e with pages of distinct ratios and page 2 current.
* (added; "Run commands from the client": a command run through the function changes the document exactly as the same action inside the editor does) `runCommand("page.insert_blank")` and the editor's own entries give the same result. Test: e2e, both paths.
* (added; "Report the page count to the client") `getState().page_count` is N+1 and one `pdfcraft:state` was sent. Test: e2e.
* (added; "Honour the restrictions set inside a PDF") Given the permissions do not allow assembly, the command is refused with its reason and the entries are unavailable. Test: e2e with the encrypted fixture.
* (added; "Save and return the bytes to the client") The bytes returned by `save()` reopen with N+1 pages. Test: e2e, `pages_on_screen` after reopening.

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

Capability: Users bookmark pages, name the bookmarks and jump to them, and the bookmarks are kept after saving and reopening.

### Feature: Bookmarks

#### Story: Add a bookmark

Component: PdfCraft fork

As a Workspace user, I want to add a bookmark at the place I am reading, so that I can come back to it later.
Acceptance criteria:
* Given no text is selected, when the user adds a bookmark, then a bookmark titled "Untitled" is added, and it opens the current page at the position at the top of the document view.
* Given text is selected on a page, when the user adds a bookmark, then the bookmark is titled with the selected text on one line, at most 128 characters, and it opens the page at the top-left of the selection.
* Given a bookmark has just been added, when the Bookmarks panel shows it, then its title is open for renaming as defined in "Rename a bookmark".
* Given the PDF file permissions do not allow assembly or the document is read-only, when the user tries to add a bookmark, then the action is unavailable.
* (added; "Run commands from the client") `runCommand("bookmark.add")`, the panel button and Ctrl+B make the same bookmark (title and destination). Test: e2e.
* (added; "Save and return the bytes to the client", mods:`Save and return`: every edit made up to the call) `save()` bytes contain the outline entry with the title; reopening them lists it. Test: e2e.
* (added; hosting.md "State the client reads") After the add `edit_generation` differs, `can_undo` is true and one undo removes the bookmark. Test: e2e.
* (added; "Hide upstream functions that overlap Workspace concerns": the same commands on every surface) New bookmark is available on the menu, the panel, the page context menu, command search and the shortcut, and on none of them when unavailable. Test: e2e, `palette_labels` (`workspace/tests/helpers.py`) and the buttons.

#### Story: Rename a bookmark

Component: PdfCraft fork

As a Workspace user, I want to give a bookmark a name of my own, so that I recognise what it points to.
Acceptance criteria:
* Given a bookmark in the Bookmarks panel, when the user starts renaming it, then its whole current title is selected, so typing replaces it.
* Given the user has typed a new title and confirmed it, when the Bookmarks panel is shown, then the bookmark has the new title and still opens the same page.
* (added; criterion 1 and item 5) The selection rule holds for all three starts: after an add, from the row's menu, by double-click. Test: fork test, three starts.
* (added; mods:707-716; `panels/bookmarks.rs:213, 216`) Given the permissions do not allow assembly, the menu entry and the double-click do not start a rename. Test: fork test with an outlined PDF protected with the assembly bit cleared (pattern `crates/engine/src/tests.rs:2375`, built in the test); no e2e, because the encrypted fixture cannot hold an outline title (`workspace/tests/pdfs.py:45-51`).
* (added; code `panels/bookmarks.rs:156-169`, current behaviour kept) Escape cancels and the title is unchanged; an empty title leaves the old one; the confirmed title is trimmed. Test: `tests/it/bookmarks.rs:123` and cases for empty and padded titles.
* (added; hosting.md) The rename is one undo step; `save()` bytes hold the new title. Test: e2e.

#### Story: Jump to a bookmark

Component: PdfCraft fork

As a Workspace user, I want to choose a bookmark and go straight to its page, so that I return to important pages quickly.
Acceptance criteria:
* Given a document with bookmarks is opened, when the Bookmarks panel is shown, then it lists the document's bookmarks with their nesting.
* Given a bookmark that opens a position on page 7, when the user chooses it, then the document view shows page 7 at that position.
* Given a nested bookmark under a collapsed parent, when the user searches bookmark titles for its title, then the bookmark is listed together with its parents.

#### Technical story: Bookmarks are kept after saving and reopening

Component: Workspace

Workspace keeps the document's bookmarks in the document it saves and reopens, so that bookmarks made in Workspace are there the next time the document is opened.
Acceptance criteria:
* Given the user has added a bookmark titled "Contract terms" that opens page 5 and renamed another bookmark, when the document is saved as defined in "Save changes to the fileserver" and reopened as defined in "Open a stored document in the editor", then the Bookmarks panel lists "Contract terms", it opens page 5 at the same position, and the renamed bookmark has its new title.

## Epic: Sign by hand

Capability: Users draw their signature and have it placed in the document they are working on.

### Feature: Signature drawing canvas

#### Story: Draw a signature in Workspace

Component: Workspace and PdfCraft fork

As a user, I want to draw my signature on a canvas in Workspace, so that the editor can place it in the document I am working on.
Acceptance criteria:
* Given a document is open in the editor, when the user draws on the canvas with a pointer, then the strokes are shown on the canvas while they are drawn.
* When the user confirms the drawing, then the drawing becomes one image that the user places in the document.
* After the drawing is confirmed, the editor starts the interactive placement that "Place the drawn signature image" defines: the image follows the pointer at its natural size and the user clicks the spot on any page; cancelling places nothing.
* A drawn signature does not involve the backend's digital signing and does not make the document a signed document; given the open document is a signed document, then the drawn signature cannot be saved to it, as "Save changes to the fileserver" and "Show whether the document is signed" define.

## Epic: Sign digitally with organisation certificates

Capability: Users apply a digital signature to a document using certificates the organisation makes available through the existing APIs.

### Feature: Digital signature action

#### Story: Apply a digital signature

Component: Workspace

As a user, I want to apply a digital signature with one of the organisation's certificates, so that the document carries a signature that other readers can verify.
Acceptance criteria:
* When the user triggers the action, then the document carries a digital signature.
* The certificate applied is one that the existing APIs make available.
* Given the open document is a signed document, then it cannot be updated, as "Save changes to the fileserver" and "Show whether the document is signed" define.

#### Technical story: Sign the PDF on the backend

Component: Workspace

The backend applies a digital signature to the PDF bytes with a certificate obtained through the existing APIs, so that signing does not depend on the browser or on the editor.
Acceptance criteria:
* The signing operation, including every use of the certificate, runs on the backend; no part of it runs in the browser or in the editor.
* The backend obtains the certificate through the existing APIs.
* The editor's own signing tools are not used for the digital signature.
* The output is a PDF in which a PDF reader that validates signatures finds one signature made with the certificate used.
* Given the existing APIs return no certificate, then the backend produces no signed document and reports the failure to the client.

#### Spike: Digital signature appearance

Component: Workspace

Question: does the digital signature have a visible appearance on a page, on which page and with what content, or is it invisible? Input to be weighed: gaps.md lists signature graphics as missing in PdfCraft (https://github.com/storytold/pdfcraft/blob/main/docs/gaps.md, gap 10), and digital signing is a backend action.
Acceptance criteria:
* Delivers whether the digital signature appears on a page.
* Given it appears, delivers on which page and with what content it appears and which component produces the appearance.
* Delivers the decision.

## Epic: Know a document's signature state

Capability: Users see whether a document is signed and what that means for further changes.

### Feature: Signature state in the layout

#### Story: Show whether the document is signed

Component: Workspace

As a user, I want to see in Workspace whether the open document is signed, so that I know it carries a signature before I change it.
Acceptance criteria:
* Workspace shows the signed state outside the editor as "Work in the editor as one part of Workspace" defines.
* Given the backend reports the document as a signed document, then Workspace shows it as signed with an icon that it is digitally marked; given the backend reports it as not signed, then Workspace shows it as not signed.
* For each digital signature, Workspace shows the signer, the signing time and the certificate (holder, issuer, validity period), taken from the backend.
* Workspace shows a validity verdict only where one is available.
* The editor's own Signatures panel is not offered.
* A signed document cannot be updated: "Save changes to the fileserver" is unavailable for it, and "Save as a Version" produces a new version from which the digital signature is removed.
* When the user opens another document, then the state shown is that of the newly opened document.

## Epic: Save work back to the fileserver

Capability: Users save their changes to the organisation's fileserver so that the stored document is up to date.

### Feature: Saving to the fileserver

#### Story: Save changes to the fileserver

Component: Workspace

As a user, I want to save the changes I made in the editor to the fileserver with Save, so that the stored document is up to date.
Acceptance criteria:
* Save is available only to a user with save permission ("Save only with save permission") and only for a document that is a stored document; an uploaded own file, a combined document or a file from the scanner not yet stored with Save as new Item has none ("Draft Management").
* Given a document opened from the fileserver at its current version that has changes in the editor, when the user chooses Save, then the changes are stored to the open stored document.
* Save creates no version; when versions are created is defined in "Keep versions of documents".
* Given the open document is a signed document, then Save is unavailable.
* Given a save succeeded, when the same document is opened again from the fileserver as defined in "Open a stored document in the editor", then every change that was in the editor at the moment the bytes were obtained is present.
* Given a save succeeded, when the user makes further changes and saves again, then the second save contains every change made since the document was opened, including those of the first save.
* Given a save succeeded, then "Show whether the document has unsaved changes" shows no unsaved changes until the next edit.
* Saving does not end the work: after a save the document stays open and the user continues working on it.
* The outcome of each save is shown as defined in "Report the outcome of a save".

#### Story: Save as a Version

Component: Workspace

As a user, I want to store the state of the document I have open as a version on request, so that this state is kept apart from the other stored states of the document.
Acceptance criteria:
* Save as a Version is available only for a document that has a fileserver identity, as "Save changes to the fileserver" defines for Save, and only to a user who passes "Save only with save permission".
* Given the user chooses Save as a Version, then the state of the document is stored as a new version of the document, as "Keep versions of documents" defines.
* Given the open document is a signed document, when the user chooses Save as a Version, then the new version carries no digital signature.
* After the save the document stays open and the user continues working on it; "Show whether the document has unsaved changes" shows no unsaved changes until the next edit, and the outcome is shown as defined in "Report the outcome of a save".

#### Story: Save as new Item

Component: Workspace

As a user, I want to store a draft (an uploaded own file, a document combined from several files or a file from the scanner) as a new stored document, so that it is kept on the fileserver with metadata and versions like any other stored document.
Acceptance criteria:
* Given such a draft is open, when the user chooses Save as new Item, then the user gives the document's name, its location on the fileserver and the metadata the backend requires.
* Workspace may offer the PDF's title as the suggested name.
* The draft is stored as a new stored document with its first version.
* Save as new Item needs a permission to create documents at the chosen location.
* Given the new stored document has been written, then it is an ordinary stored document: Workspace records its fileserver identity, and "Save changes to the fileserver", "Save as a Version", "Keep versions of documents" and "See a document's metadata" apply to it.
* Given the permission check fails or the fileserver does not accept the write, then "Report the outcome of a save" applies and the draft stays open with its changes.
* After the save the document stays open and the user continues working on it.

#### Technical story: Draft Management

Component: Workspace

Workspace's backend manages drafts, so that unsaved work, on a stored document or on a file that is not yet a stored document, is handled by Workspace and not by the editor's own autosave and crash recovery, which are hidden by "Hide upstream functions that overlap Workspace concerns".
Acceptance criteria:
* Given a draft of an uploaded own file, a document combined from several files or a file from the scanner, then nothing of it is written to the fileserver and it has no metadata and no versions until the user stores it with "Save as new Item" or takes it away with "Download as PDF".
* Draft Management keeps the user's draft so that unsaved work is not lost when the browser page is closed, reloaded or crashes, or when the user leaves the document; the user can return to the draft.
* Under option (a) of "Image handling approach", converting an uploaded image does not make it a stored document.

#### Technical story: Store saved bytes on the fileserver

Component: Workspace

The backend receives the document bytes from the client and writes them to the fileserver, so that the stored document reflects the user's saved changes through Save, Save as a Version and Save as new Item.
Acceptance criteria:
* Given the client sends the bytes of Save or of Save as a Version for a document opened from the fileserver, when the check of "Save only with save permission" has passed, then the backend writes the bytes to the fileserver: for Save to the open stored document without creating a version, for Save as a Version as a new version of the document.
* Given the client sends the bytes of Save as new Item, when the check of the create permission of "Save as new Item" has passed, then the backend writes the bytes as a new stored document with its first version at the location the user gave.
* The backend reports success to the client only after the fileserver has accepted the write; it never reports success before.
* Given the fileserver does not accept the write, then the backend reports failure to the client with the reason "the fileserver did not accept the write".
* The write to the fileserver is made by the backend; the client never writes to the fileserver directly.
* Given the bytes are encrypted, then they are stored unchanged.

#### Story: Show whether the document has unsaved changes

Component: Workspace

As a user, I want to see whether the document has changes that are not yet saved, so that I know whether a save is needed.
Acceptance criteria:
* Given a document was just opened in the editor, then Workspace shows no unsaved changes.
* Given the user makes a change after the document was opened or after the last successful save, then Workspace shows the document as having unsaved changes.
* Given a save succeeds, then only changes made after the saved state was taken are shown as unsaved changes.
* Given a save fails, then the document stays shown as having unsaved changes.

## Epic: Keep a document's history

Capability: Users and the organisation keep earlier versions of a document and can tell versions apart.

### Feature: Document versions

#### Technical story: Keep versions of documents

Component: Workspace

Workspace keeps versions of documents, so that earlier states of a document are available to users.
Acceptance criteria:
* Given a document with more than one stored version, then each version can be listed (see "See the versions of a document").
* A version is created when the user chooses "Save as a Version"; the first version of a new stored document is created by "Save as new Item"; Save ("Save changes to the fileserver") creates no version.

#### Story: See the versions of a document

Component: Workspace

As a user, I want to see the versions of the document I have open, so that I can tell them apart and find an earlier state.
Acceptance criteria:
* Given a document opened from the fileserver, then Workspace lists its versions outside the editor, as "Work in the editor as one part of Workspace" defines.
* Every version stored for the document appears exactly once in the list.
* The version that is currently open is marked as such in the list.
* No two versions in the list are presented identically.

#### Story: Open an earlier version

Component: Workspace

As a user, I want to open an earlier version of a document, so that I can see what the document looked like at that time.
Acceptance criteria:
* Given the list of "See the versions of a document", when the user chooses an earlier version, then the editor shows that version with exactly the bytes stored for it.
* Given a link of "Choose a stored document" that names a version, when the user opens the link, then that version opens.
* Opening an earlier version does not change the document's current version on the fileserver.
* While an earlier version is open, Workspace shows which version is open.
* Opening an earlier version requires the same read permission as the document; see "Read only with read permission".

## Epic: Describe documents with metadata

Capability: Users and the organisation keep descriptive information about each document.

### Feature: Document metadata

#### Story: See a document's metadata

Component: Workspace

As a user, I want to see the descriptive information kept about a document, so that I know what the document is.
Acceptance criteria:
* Given a document opened from the fileserver, then Workspace shows the metadata it keeps for the document.
* Who may see metadata is defined in "Read only with read permission".

#### Story: Change a document's metadata

Component: Workspace

As a user, I want to change the descriptive information kept about a document, so that it stays correct.
Acceptance criteria:
* Given the user changes a metadata value and the change is stored, when the document is opened again, then the changed value is shown.
* Given the change is not stored, then the metadata shown on reopening is the metadata before the change, and the user is told that the change was not stored and why; the message takes the same form as a failure of "Report the outcome of a save".

## Epic: Take a copy away

Capability: Users obtain the document with all their changes as a PDF file.

### Feature: Download

#### Story: Download as PDF

Component: Workspace

As a user, I want to download the document I have open as a PDF file on my computer, so that I have a copy with all my changes.
Acceptance criteria:
* Given a document is open, when the user chooses Download as PDF, then the user receives the document as a PDF file on their computer with every change in the editor, saved or not.
* Download as PDF writes nothing to the fileserver and is not a save: unsaved changes stay shown.
* The file name is the stored name for a stored document; the uploaded name with its extension changed to ".pdf" for an uploaded own file; and the first source file's name for a combined draft that has not been given a name through "Save as new Item".
* Read permission ("Read only with read permission") is enough for Download as PDF; save permission is not needed, and a user who may not save downloads the result instead of saving.
* Given no PDF file could be produced, then the user is told so, with the document named and the editor's error text unchanged, in the same form as a failure of "Report the outcome of a save"; the document stays open with its changes.

## Epic: Share results that look the same anywhere

Capability: Recipients see all markup, watermarks, stamps and signatures in a PDF reader other than Workspace.

### Feature: Self-contained PDF output

#### Technical story: Markup carries its appearance in the saved file

Component: PdfCraft fork

The fork's save returns bytes in which every annotation of the kinds the editor builds (notes, text markup, shapes, ink and text boxes) carries its appearance stream, so that PDF readers other than Workspace show the markup without rebuilding it.
Acceptance criteria:
* Given a document containing one annotation of each kind the editor builds (a note, a text markup, a shape, an ink drawing and a text box), when the bytes are obtained through "Save and return the bytes to the client", then each of these annotations in the bytes has an appearance stream.
* Given those bytes are opened in a PDF reader that is not Workspace, then each annotation is shown on the same page and at the same position as in the editor.
* (added; backlog criterion 1) Every annotation made with each tool and command of the offered markup list has an appearance in the saved file. Test: e2e, one of each.
* (added; "Reply to a PDF file comment" (no card, no icon); "Results show the same in other PDF readers") A reply shows no second note icon in another reader. Test: e2e, note with two replies.
* (added; backlog criterion 2) An annotation moved, resized or reshaped, or whose text or colour was changed after creation, shows in another reader as in the editor and not as before. Test: e2e, square, ink, text box, line, polygon, note.
* (added; practice (highlights are see-through); "Results show the same in other PDF readers") A saved highlight does not hide its text; underline, squiggly and strike out cover none of it. Test: e2e, text sample.
* (added; backlog criterion 2) A deleted markup item leaves nothing visible in another reader. Test: e2e, any item deleted.

#### Story: Results show the same in other PDF readers

Component: Workspace

As a user who shares a document, I want recipients to see all markup, stamps, watermarks and signatures in their own PDF reader, so that the result I reviewed is the result they see.
Acceptance criteria:
* Given a document saved to the fileserver that contains markup from "Mark a passage with text markup", "Place a note at a point on a page", "Write a remark in a text box on a page", "Draw freehand on a page" and "Draw a shape around content", a stamp from "Apply an organisation stamp to a document", a watermark from "Apply an organisation watermark to pages" and a drawn signature from "Place the drawn signature image", when the file is opened in a PDF reader that is not Workspace, then each piece of markup, the stamp, the watermark and the drawn signature are shown on the same page and at the same position as in Workspace.

## Epic: Control who may read and change documents

Capability: The organisation decides who can log in, read documents and save changes.

### Feature: Login

#### Story: Log in to Workspace

Component: Workspace

As a user, I want to log in to Workspace, so that Workspace knows who I am and can apply the organisation's permissions to me.
Acceptance criteria:
* Given a user who is not logged in, when they open Workspace, then no document from the fileserver is shown, no version list and no metadata are shown, and no save is possible until login succeeds.
* Given a user who is not logged in opens a link of "Choose a stored document", then login comes first and, when login succeeds, Workspace opens the linked document as "Choose a stored document" defines.
* Given login succeeded, then every read and save of a document, a version or metadata is made as the logged-in user.
* Given login fails, then the user is told that the login failed and no document from the fileserver is shown.

### Feature: Permissions on documents

#### Story: Read only with read permission

Component: Workspace

As the organisation, I want only users with read permission for a document to read it, so that documents are seen only by those allowed to see them.
Acceptance criteria:
* Given a logged-in user without read permission for a document, when they try to open it, then the backend does not deliver the document's bytes, and the user is told as defined in "Report why a document could not be opened".
* Given a logged-in user without read permission for a document, then the backend delivers none of the document's versions and none of its metadata.
* Given a logged-in user with read permission for a document, when they open it, then the backend delivers the bytes of the requested version.

#### Story: Save only with save permission

Component: Workspace

As the organisation, I want only users with save permission for a document to store changes to it, so that stored documents change only through those allowed to change them.
Acceptance criteria:
* Given a logged-in user without save permission for a document, when a save is requested, then the backend refuses it before writing to the fileserver, and the user is told the reason "the user lacks save permission" as defined in "Report the outcome of a save".
* Given a logged-in user with save permission for a document, when a save is requested, then "Store saved bytes on the fileserver" proceeds.
* Editing in the editor does not depend on save permission: every user who can open a document can make page changes, bookmarks, markup and every other edit; the check happens when the user wants to save, and a user who may not save takes the result away with "Download as PDF" instead.

#### Technical story: Enforce permissions in the backend

Component: Workspace

The backend checks the login session and the permission on every request that reads or writes a document, a version or metadata, so that permissions cannot be bypassed by a client.
Acceptance criteria:
* Given a request to read or save a document, a version or metadata that carries no valid login session, then the backend refuses it and nothing is read from or written to the fileserver for that request.
* Given a request from a logged-in user who lacks the permission the request needs, sent directly to the backend without the Workspace client, then the backend refuses it and nothing is read from or written to the fileserver for that request.
* Permission checks run in the backend; a check in the client alone is not sufficient for a request to pass.

### Feature: Restrictions set inside a PDF

#### Story: Honour the restrictions set inside a PDF

Component: Workspace and PdfCraft fork

As a user, I want Workspace to respect the restrictions an author set inside a PDF, so that I am not offered actions the document forbids.
Acceptance criteria:
* Given a document whose author restricted it, when it is opened in Workspace, then the editor shows its notice about the restrictions and honours the PDF file permissions, as the editor already does.
* Workspace does not offer an action that the PDF file permissions forbid.
* Given an encrypted document is open in the editor and has been edited, when "Save and return the bytes to the client" returns the bytes, then the saved document stays encrypted as the document was.
* Workspace opens password-protected documents as "Enter a password for a protected document" defines.
* (added; backlog criterion 1 ("a document whose author restricted it"); the permissions list of `fork/changes/host-api.md`) The notice about restrictions is shown for every document opened without the owner password that restricts any right the state reports (printing, modifying, copying, annotating, form filling, accessibility extraction, assembly, high-quality printing); not for an unrestricted document, nor after the owner password was entered. (changes upstream behaviour: today only modification or assembly) Test: e2e, samples with one right cleared each, and unrestricted ones; fork test.
* (added; backlog criterion 1; "Hide upstream interface parts Workspace does not offer" (no Preferences)) The notice cannot be turned off for good: no "Don't show again", and it shows each time a restricted document is opened. (changes upstream behaviour) Test: e2e, two restricted documents.
* (added; backlog criterion 1; rule of earlier items) Every offered command that needs a right the document does not give is refused with "is not available now" and changes nothing. Test: e2e, one sweep, samples with each right cleared.
* (added; backlog criterion 1) With copying forbidden, copying selected text puts nothing on the clipboard and shows the refusal; with it allowed it copies. Test: e2e, text sample protected twice.
* (added; backlog criterion 3) An encrypted document saved after an edit stays encrypted as it was, with the same permissions, and opens with the same passwords in another named reader (PDFium);  Test: e2e, encrypted markup sample.
* (added; hosting "State the client reads"; backlog criterion 2) A client reading the state can tell each right; after upload of a restricted document the state reports exactly the cleared right as false. Test: e2e, four single-right samples (kept only where "Report editor state to the client" tests lack them); the stand-in client's buttons follow it (Workspace's real buttons are Workspace's).

#### Story: Enter a password for a protected document

Component: Workspace

As a user, I want to enter the password of a protected stored document or own file in Workspace, so that I can open it and, with the owner password, lift its restrictions.
Acceptance criteria:
* Given a document requires a password, then Workspace asks the user for the password and passes it only to the editor; the password never goes to the backend, to storage or to logs.
* Given an encrypted file is among the inputs of "Combine several files in chosen order", then it is unlocked first in the same way.
* Given the user enters the owner password, then the editor lifts the PDF file permissions as it already does, and Workspace records that this happened: the user, the document, the version and the time; the password itself is never recorded.
* Lifting a PDF's restrictions grants no read permission and no save permission of the organisation.
* Given the user enters no password or a wrong one, then "Report why a document could not be opened" applies with the cause "password required or wrong".

## Epic: Know whether a save succeeded

Capability: Users are told when a save succeeded or why it failed.

### Feature: Save outcome

#### Story: Report the outcome of a save

Component: Workspace

As a user, I want to be told whether my save succeeded and, if not, why, so that I know whether the stored document is up to date and what to do next. Guidance, not a criterion: success is shown as a short message that closes by itself while "Show whether the document has unsaved changes" switches to no unsaved changes; failure is shown as a message that stays until dismissed, states the reason and offers to save again; the same form is used for a metadata change that is not stored ("Change a document's metadata") and for a failure of "Download as PDF".
Acceptance criteria:
* Given a save, when the backend confirms that the fileserver accepted the write ("Store saved bytes on the fileserver"), then Workspace shows the user that the save succeeded. Success is never shown before that confirmation.
* Given a save failed, then Workspace shows the user that the save failed and names the reason. The reason is one of: "the user lacks save permission" (from "Save only with save permission"), "the user is not logged in or the session is not valid" (from "Enforce permissions in the backend"), "the fileserver did not accept the write" (from "Store saved bytes on the fileserver"), or "the editor did not return the document" ("Save and return the bytes to the client" returned no bytes).
* Each of the four reasons is distinguishable from the other three in what the user sees.
* Given a save failed, then the user's changes remain in the editor and the user can save again.

## Epic: Keep a maintained editor

Capability: The organisation keeps receiving upstream PdfCraft updates and fixes while using its own editor.

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

Capability: The organisation runs the editor as one component inside Workspace rather than as a separate application.

### Feature: WASM library build of the fork

#### Technical story: Build the fork as a WASM library

Component: PdfCraft fork

The fork adds a library crate (cdylib, wasm-bindgen) built with wasm-pack into a package, so that Workspace imports the editor directly.
Acceptance criteria:
* The fork contains a library crate of type cdylib that uses wasm-bindgen.
* wasm-pack builds the crate into a package that a CSR web application imports directly as a module.
* The package exposes the exported JS API defined by the feature "Exported JS API".
* The editor runs inside the client that imports it: there is no iframe and no postMessage bridge between Workspace and the editor.

#### Technical story: Workspace imports the editor package

Component: Workspace

Workspace imports the fork's WASM package directly and supplies the canvas the editor is mounted on, so that the editor is one component of Workspace.
Acceptance criteria:
* The client imports the package built by "Build the fork as a WASM library" as a direct module import, in the way "Build the fork as a WASM library" defines (no iframe, no postMessage bridge).
* The client supplies the canvas element and mounts the editor on it through "Mount the editor on a canvas supplied by the client".
* Workspace communicates with the editor only through the exported JS API.

#### Spike: Workspace framework and language

Component: Workspace

Question: which framework and language does Workspace use for its CSR web application and its backend? Options named by the decisions: React, Next.js, vinext or another.
Acceptance criteria:
* Delivers the framework and language of Workspace's client and backend.
* Delivers whether Workspace is one application or several projects.
* Demonstrates that the chosen framework imports the package of "Build the fork as a WASM library" directly and mounts the editing canvas on a canvas the client supplies, with a document opened from bytes and shown.
* Delivers the routing for the links of "Choose a stored document".

### Feature: Editor in the Workspace layout

#### Story: Work in the editor as one part of Workspace

Component: Workspace

As a user, I want the editor to be one part of the Workspace layout, so that I work on documents inside Workspace instead of in a separate application.
Acceptance criteria:
* The editor occupies one part of the Workspace layout; the rest of the layout belongs to Workspace.
* Actions, versions and signatures are in the layout outside the editor; their content is defined by "See the versions of a document", "Show whether the document is signed" and the items with Component "Workspace" that define actions.
* The user stays in Workspace while using the editor: no other application and no separate browser window is opened to edit.

## Epic: Drive the editor from Workspace

Capability: The organisation controls the editor from Workspace, so that Workspace's actions and the editor's state stay consistent.

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
* (added; host-api.md pattern; hosting.md "Open a document") The function returns a Promise of `{ ok: true }` or `{ ok: false, error }` with the editor's error text unchanged, never rejects; the number of inserted pages is obtainable by the client (wb:242), as a field of the result or as the difference of `page_count`; the implementer chooses and records it. Test: e2e.
* (added; "Save and return the bytes to the client") Given no document is open the result is a failure that says so. Test: e2e.
* (added; `files.rs:635`) Given the bytes are not a PDF or cannot be read (empty, random or truncated), the result is a failure with the error text and the document is unchanged (`page_count`, `edit_generation`, `can_undo`). Test: e2e.
* (added; product owner, 2026-11-08; "Report why a document could not be opened", "Enter a password for a protected document") The function takes an optional password for the other PDF. Given the other PDF needs a password and none or a wrong one is given, the result is a "password required" failure, distinct from every other failure, and the document is unchanged; Workspace asks the user for the password and calls the function again. With the right password the pages are inserted. The password is used only by the editor and appears in no request, result, console message or state event. Test: e2e with an other PDF that has a user password: without it, with a wrong one, with the right one.
* (added; "Honour the restrictions set inside a PDF"; `edit/run.rs:13-36`) Given the PDF file permissions do not allow assembly the result is a failure with the reason and the document is unchanged. Test: e2e with the encrypted fixture (assemble bit cleared).
* (added; "Report the page count to the client") After the call `getState().page_count` is N+M, `pdfcraft:state` was sent once, and `getState().info` (title, author, subject, keywords) is the open document's, not the other PDF's. Test: e2e; the other PDF has a title (`build(title=...)`).
* (added; "Undo and redo page changes") The insertion is one undo step: one `runCommand("edit.undo")` returns N pages. Test: e2e.
* (added; `files.rs:660-664`) After the call the inserted pages are the selected pages, as after the editor's own insert. Test: e2e, `docInspect().active.selected_pages`.
* (added; "Hide upstream interface parts") The call opens no file chooser and no dialog. Test: e2e, `page.on("filechooser")` records none.
* (added; "Keep fork changes to what Workspace needs") The editor makes no request to the fileserver during the call. Test: e2e, the call record.

#### Technical story: Save and return the bytes to the client

Component: PdfCraft fork

The fork exports a function that saves the open document and returns its bytes to the client, so that Workspace sends them to its backend.
Acceptance criteria:
* The exported JS API has a function that saves the open document and returns its bytes to the client.
* The returned bytes are a PDF document that contains every edit made in the editor up to the call.
* The editor sends the bytes nowhere; the client sends them to the backend as "Store saved bytes on the fileserver" defines.
* The editor offers no Save and no Save As command: neither is shown in its menus, toolbars or command search, nor run by a keyboard shortcut. Saving is done with Workspace's buttons outside the editor: Save, which overwrites the current version ("Save changes to the fileserver"), Save as a Version ("Save as a Version") and Download ("Download as PDF"); each obtains the bytes through this function.
* The function is available to a page that does not opt in to the page command API (`?control`).
* Given no document is open, the call returns a failure result that says so, and no bytes.
* Given the editor cannot produce the bytes, the call returns a failure result with the editor's error text, so that the client reports "the editor did not return the document" ("Report the outcome of a save").
* The result carries the edit_generation value the returned bytes correspond to, so that the client shows unsaved changes as "Show whether the document has unsaved changes" defines, also when the backend does not accept the write.
* The editor gives the client saved bytes only through this function: it starts no download of the saved document and sends no other save message (`pdfcraft:save`). The print-ready PDF that "Print the document" downloads when the browser refuses the print is not the saved document.
* Every criterion of this item is tested end to end in the Mock/Stub, with the function called on a page opened without `?control`.

#### Story: Save without earlier revisions

Component: Workspace and PdfCraft fork

As a user, I want to choose, when saving, that the PDF file is written in full without its earlier revisions, so that text I replaced and pages I deleted cannot be recovered from the saved file.
Acceptance criteria:
* When the user saves (Save, Save as a Version, Save as new Item, Download as PDF), Workspace offers an option to write the file in full; it is off by default.
* Given the option is off, the file is saved incrementally, as today.
* Given the option is on, the saved PDF file holds no earlier revisions: text replaced and pages deleted before the save cannot be recovered from it.
* Given the option is on, the saved PDF file shows the same pages, markup and content as an incremental save of the same document.
* The fork's save ("Save and return the bytes to the client") can return the bytes written in full, without earlier revisions, when the client asks for it.

#### Technical story: Run commands from the client

Component: PdfCraft fork

The fork exports a function that runs an editor command, backed by PdfCraftApp::run_command, so that Workspace triggers editing actions from outside the editor.
Acceptance criteria:
* The exported JS API has a function that runs an editor command identified by the client, backed by PdfCraftApp::run_command.
* Every editing action that an item of this backlog requires Workspace to trigger from outside the editor is runnable through this function.
* A command run through this function changes the document and the editor state exactly as the same action taken inside the editor does.
* The editor offers only commands that apply to working in Workspace. Commands that belong to the stand-alone application are not shown in command search ("Find tools and commands") or anywhere else in the editor, enabled or disabled; among them: New blank PDF, Create PDF from file, Create PDF from Multiple Files, Open Recent, Clear Recent Files, Pin Folder to Home and Tab Order.
* The function is available to a page that does not opt in to the page command API (`?control`).
* The result tells the client whether the command ran; an unknown command, a command the editor does not offer and a command not available in the current state are refused, each with its reason.
* The editing actions Workspace triggers through this function are undo and redo ("Use the editor state for controls outside the editor"); the other actions Workspace triggers from outside the editor have their own functions in the feature "Exported JS API".
* A command the editor does not offer is shown nowhere in it: not in its menus, toolbars, hotbar, context menus or command search, and no keyboard shortcut runs it. The commands that change what Workspace sets, the interface language and the theme, are not offered either.
* Every criterion of this item is tested end to end in the Mock/Stub; each command not offered is checked in command search, the menus, the hotbar and the context menus.

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
* (added; the pattern of `fork/changes/host-api.md`) The function returns a Promise of `{ ok: true }` or `{ ok: false, error }`, never rejects; `ok` means the placement started. Test: e2e.
* (added; mods:855, `edit/run.rs:766-777`) Natural size is pixels * 72 / dpi (72 dpi without one), within 80 percent of the page; the preview and the placed image have the same size. Test: e2e with the 480 x 180 px PNG; fork test for the size rule.
* (added; mods:431, 444, 855) The placed image is an added image: it has no comments panel entry and is moved and resized as an added image. Test: e2e, comments panel rows unchanged after the placement.
* (added; "Save and return the bytes to the client": no document open) Given no document is open the result is a failure that says so and nothing starts. Test: e2e before any document is opened.
* (added; "Honour the restrictions set inside a PDF") Given the PDF file permissions do not allow the change (the modify bit for `Edit::AddImage`), the result is a failure with the reason. Test: e2e with the encrypted fixture.
* (added; failure pattern) Bytes that are not an image, an empty array or an oversized image give `{ ok: false, error }` with the editor's text, nothing starts, `edit_generation` is unchanged. Test: e2e.
* (added; hosting.md, "State the client reads") After the click `edit_generation` differs and `pdfcraft:state` was sent once for it. Test: e2e, event count.

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
* (added; product owner, 2026-11-07, `fork/hosting.md`) The client can also give the language when it starts the editor; the editor shows it from the first thing it shows, without a call. Test: e2e, the language given at start, no call.

#### Technical story: Set the editor's theme

Component: Workspace and PdfCraft fork

The fork exports a function that sets the editor's theme, so that the editor follows the theme chosen in "Choose a reading theme". Whether the WASM editor already follows the browser's system setting is to be verified; if it does, the follow-the-system choice needs no call.
Acceptance criteria:
* The exported JS API has a function that takes one of light, dark and follow the system and sets the editor's theme to it.
* After the call, the editing canvas is shown in that theme.
* (added; product owner, 2026-11-07, `fork/hosting.md`) The client can also give the theme when it starts the editor; the editor shows it from the first thing it shows, without a call. Test: e2e, the theme given at start, no call.

#### Technical story: Set the author name for markup

Component: Workspace and PdfCraft fork

The fork exports a function that sets the author name, so that markup items and replies carry the logged-in user's display name as "Place a note at a point on a page", "Reply to a PDF file comment" and "List all markup in the comments panel" require.
Acceptance criteria:
* The exported JS API has a function that takes a display name and sets it as the author name.
* After the call, every markup item and reply the user creates records that name as its author.
* The author name cannot be changed inside the editor.
* (added; product owner, 2026-11-07, `fork/hosting.md`) The client can also give the author name when it starts the editor; the editor uses it from the first markup item, without a call. Test: e2e, the name given at start, no call, then a note.

#### Technical story: Report editor state to the client

Component: PdfCraft fork

The fork exports the editor state the client's controls need, so that controls outside the editor behave correctly.
Acceptance criteria:
* The exported JS API exposes can_undo, can_redo, permissions, is_signed and edit_generation, taken from the engine.
* can_undo is true exactly when the editor has an edit to undo; can_redo is true exactly when the editor has an undone edit to redo.
* edit_generation has a different value after each edit of the open document than before it.
* permissions reports the open document's PDF file permissions as the engine exposes them.
* is_signed reports whether the open document is signed: true for a document carrying a digital signature, false for a document carrying no signature.
* When a different document is opened in the editor, then the values returned are those of the newly opened document.
* A further state value that an item of this backlog needs for a Workspace control is added through an item with Component "Workspace and PdfCraft fork".
* These values, the page count of "Report the page count to the client" and the document information of "Report the document information to the client" are available to the client in one call, to a page that does not opt in to the page command API (`?control`).
* The editor tells the client each time one of these values changes, so that the client needs no polling; when no value changed, the editor tells the client nothing.
* The fork adds no second way to read these values: the page count and the document information the fork added to the `?control` functions (`docInspect`, `documentInfo`) are removed.
* permissions has a value for every open document, also for a document that sets no restrictions.
* Every criterion of this item is tested end to end in the Mock/Stub, on a page opened without `?control`.

### Feature: Workspace controls driven by editor state

#### Technical story: Use the editor state for controls outside the editor

Component: Workspace

Workspace reads the editor state through "Report editor state to the client" and uses it for its controls outside the editor, so that those controls reflect the editor.
Acceptance criteria:
* Given the editor reports can_undo false, When Workspace offers its undo action, Then the action is unavailable; the same holds for redo and can_redo.
* The signed state shown outside the editor comes from the backend, as "Show whether the document is signed" defines; the is_signed value the editor reports does not decide it. Given the backend reports a signed document, Then "Save changes to the fileserver" is unavailable, as that story defines.
* Given the editor reports permissions, Then Workspace behaves as "Honour the restrictions set inside a PDF" defines.
* Workspace uses edit_generation to provide "Show whether the document has unsaved changes".

## Epic: Edit on the proven editing surface

Capability: Users keep PdfCraft's page view, selection, annotation and text editing behaviour inside Workspace.

### Feature: PdfCraft editing canvas kept in the fork

#### Story: Edit on the PdfCraft editing canvas

Component: PdfCraft fork

As a user, I want the editor's page view, selection, annotations and text editing to behave as in PdfCraft, so that I work on a proven editing surface.
Acceptance criteria:
* The editing canvas of the editor is upstream's editing canvas (page view, selection, annotations, text editing); the fork changes it only through items permitted by "Keep fork changes to what Workspace needs".
* Given a document opened through "Open a document from bytes", When the user views pages, selects, annotates or edits text on the editing canvas, Then the result equals the result of the same action in the upstream PdfCraft web build of the upstream version the fork is synced to.
* Upstream fixes to the editing canvas reach the editor through "Pull upstream updates into the fork".

## Epic: Present the editor under the organisation's own name and controls

Capability: Users meet the editor as part of Workspace, under the organisation's own name, without upstream branding.

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
* No hidden part can be opened from anywhere in the editor: not from its menus, toolbars, hotbar, tabs, context menus, command search or keyboard shortcuts.
* The editor never shows or enters the All tools catalogue, also not when it starts.
* Given no document is open, the editor shows an empty page area with no actions and no messages; what is opened, and every message about it, is Workspace's.
* Each tool group the Tools panel shows is used by an action an item of this backlog defines; the change record names that item for each group.
* Every criterion of this item is tested end to end in the Mock/Stub.

#### Spike: Upstream interface parts Workspace needs

Component: PdfCraft fork

Question: which upstream interface parts (commands, tools, panels) work only in the stand-alone application, and which of the others an item of this backlog needs? It cannot be answered yet: upstream's commands and tools are still being added and discussed. So far the parts that work only in the stand-alone application were hidden; a part still shown is not evidence that an item needs it, and a hidden one is not evidence that none does.
Acceptance criteria:
* Delivers, for each upstream command, tool and panel, whether it works only in the stand-alone application.
* Delivers, for each of the others, the backlog item that needs it, or that no item does.
* Delivers the parts hidden so far that an item needs, and the parts shown that no item needs.

## Epic: Change the editor only where Workspace needs it

Capability: The organisation limits its changes to the fork to what Workspace features, the plan or the rename require, and keeps all application concerns in Workspace.

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

#### Technical story: Workspace owns all application concerns

Component: Workspace

Workspace implements every application concern itself, so that the fork stays limited to the editor.
Acceptance criteria:
* Document metadata, file versioning, login and permissions, fileserver access, stamps and watermarks from the existing APIs, digital signatures and the web application itself are implemented in Workspace through the epics "Describe documents with metadata", "Keep a document's history", "Control who may read and change documents", "Open stored documents", "Save work back to the fileserver", "Mark document status with stamps", "Show status or confidentiality across a page" and "Sign digitally with organisation certificates".
* Where one of these concerns needs something from the editor, the need is expressed as an item with Component "Workspace and PdfCraft fork" that names the change to the exported JS API or, where the change is not yet known, an open point that names the need.

## Open points

* Mixed Arabic and Latin text in a text box: its alignment and wrapping ("Write a remark in a text box on a page").
* The PDF LockedContents flag is not honoured by the editor ("Add or change the explanation of a markup item").
* A reply to a reply is not offered ("Reply to a PDF file comment").