---
type: Decision
title: How work on the fork is divided
description: An integrating session builds, tests and commits; worker agents only change code and record their change; the log links each change to the agent that made it.
status: draft
generated: { by: claude-opus-5-5, at: 2026-11-07T00:00:00Z }
---

# How work on the fork is divided

## Roles
- **Integrator**: the main session, usually the conversation with the product owner. It dispatches worker agents, and it alone uses git, builds (fmt, clippy, the tests, the WASM build), tests the WASM in the Mock/Stub, commits and opens pull requests.
- **Workers**: agents dispatched by the integrator, several at a time in the same working tree. A worker changes code, records its change in this bundle, adds its entry to `log.md`, and reports the files it touched. A worker does not use git and does not build.

## Why workers do not build
Workers share one working tree and one `target/` folder. Any cargo command, `cargo check` included, compiles the code and takes the build directory lock:
- a second cargo command waits until the first one ends;
- a worker's build compiles other workers' unfinished edits and reports errors that are not its own.

## Keeping workers reachable
A worker that has finished can be resumed with its conversation intact, by the session that dispatched it. When a build fails, the integrator sends the failure back to the worker that made the change. This works only within the dispatching session; after that, the change record and the log are what remain.

## The log as changelog
Each change made by a worker gets an entry in `log.md`, under the date:
- the agent: the label the integrator gave it when dispatching;
- the task it was given;
- the files it touched;
- the change record under `changes/`;
- the build result, added by the integrator after it builds.

## Integration failures
When changes that each looked right fail together, it is handled as in any team: the integrator finds the cause, using the log to narrow it down, and fixes it or sends it back to the worker concerned.
