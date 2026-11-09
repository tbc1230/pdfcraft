---
okf_version: "0.2"
---

# Workspace fork of PdfCraft: knowledge

This folder is the fork's own knowledge, in Open Knowledge Format (OKF) v0.2. Everything outside it that upstream wrote (README.md, ROADMAP.md, docs/ and the rest) describes upstream PdfCraft and is never edited here. Start with the overview.

## Start here
- [Overview](/overview.md) - what the fork is, what Workspace needs from it, branches, pull requests, testing.
- [How to read the backlog](/backlog.md) - acceptance criteria are the target behaviour; components; terms.
- [How Workspace hosts the editor](/hosting.md) - the editor as Workspace uses it, from load to save; every change to the editor's interface with the client is designed against it.
- [Issues found in the editor](/issues.md) - problems found in the Mock/Stub, with cause and backlog; open ones are not yet tasks.

## Decisions
- [Branches and syncing](/decisions/branches.md)
- [Pull request descriptions](/decisions/pull-requests.md)
- [Upstream documents stay unchanged](/decisions/upstream-documents.md)
- [How work on the fork is divided](/decisions/work-architecture.md)

## Procedures
- [Sync the fork branch with upstream](/procedures/upstream-sync.md)
- [Integrate worker changes](/procedures/integrate.md)

## Fork changes
- [All tools sidebar hidden by default in the web build](/changes/all-tools-sidebar-hidden.md)
- [Six tool groups hidden from All tools in the web build](/changes/all-tools-hidden-groups.md)
- [Author name set by the host for markup](/changes/author-name-set.md)
- [Commands offered in the web build](/changes/offered-commands.md)
- [Document information reported to the client](/changes/document-info-reported.md) (deprecated, replaced by the host API)
- [Document opened by the host is shown](/changes/host-open-shown.md)
- [Save, commands and editor state for the client](/changes/host-api.md)
- [Language, theme and author name given by the host at init](/changes/host-settings-at-init.md)
- [Image placed where the reader clicks](/changes/image-placement.md)
- [Insert pages from bytes, and the image size rule as an engine function](/changes/insert-pages-from-bytes.md)
- [Interface parts Workspace does not offer hidden in the web build](/changes/interface-parts-hidden.md)
- [Page, bookmark, print, search and link items checked against the code](/changes/page-items.md)
- [Interface language set by the host](/changes/language-set.md)
- [Page count reported to the client](/changes/page-count-reported.md) (deprecated, replaced by the host API)
- [Preferences hidden in the web build](/changes/preferences-hidden.md)
- [Theme set by the host](/changes/theme-set.md)
- [Tab strip hidden in the web build](/changes/tab-strip-hidden.md)
- [Page navigation works without the canvas focus in the web build](/changes/wheel-without-canvas-focus.md)
- [Upstream AGENTS.md and CLAUDE.md removed](/changes/upstream-agent-files-removed.md)

## Upstream documents
- [README.md](/upstream/readme.md)
- [ROADMAP.md](/upstream/roadmap.md)
- [docs/architecture.md](/upstream/architecture.md)
- [docs/development.md](/upstream/development.md)

## Sources
- [Workspace backlog](/sources/workspace-backlog.md)
- [Fork items of the backlog](/sources/modifications.md)
- [Workspace plan](/sources/workspace-plan.md)

## History
- [Log](/log.md)
