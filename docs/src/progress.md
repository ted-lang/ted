# Progress

This page tracks implementation status and planned work. Update it alongside changes to the language semantics or tooling.

## Status Snapshot

- Parser: stubbed; lexer and diagnostics are scaffolded
- Core syntax: documented; implementation pending
- Timed runtime: not implemented yet
- Cranelift backend: prototype emits a `main` that only supports literal `print(...)` calls
- CLI: compile/check are wired; compile uses the prototype backend

## Near-Term TODOs

- TODO: Implement the parser for module items, statements, and time operators.
- TODO: Define explicit temporal storage syntax and history bounds.
- TODO: Add a standalone delay statement (`@ +delta;`) for timed code without assignment.
- TODO: Specify `timed fn` tasks and where they can appear.
- TODO: Define standard event sources (timeouts, intervals, channels) and ordering rules.
- TODO: Specify the core standard library surface (including I/O and `print`).
- TODO: Lower MIR into Cranelift and emit real program semantics.

## Longer-Term Goals

- Lower timed code to explicit state machines and scheduled tasks.
- Build a deterministic scheduler with a fast event queue.
- Provide a standard library for timeouts, intervals, and waveform output.

## Updating This Page

- Add links to issues/PRs once issue tracking is in place.
- Keep status short and factual; move detailed design notes into the language docs.
