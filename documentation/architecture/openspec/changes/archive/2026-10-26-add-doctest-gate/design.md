## Context

Nextest documents doctest support as out of scope; the template standardizes on nextest, which leaves documentation examples unexecuted everywhere — pre-commit, CI lint, and merge. The gap is template-level, not consumer-specific: any library published to docs.rs renders examples no gate validates.

## Goals / Non-Goals

**Goals:**

- Doctests run on every commit and every merge, via one narrow hook.

**Non-Goals:**

- Replacing nextest or re-adding a full `cargo test` (that would duplicate the suite the no-duplication decision removed).

## Decisions

### 1. Scope to --doc, both fast stages

`cargo test --doc --locked` covers exactly the hole: unit and integration tests stay with nextest. Pre-commit and pre-merge-commit mirror the nextest hook's stages so merges get the same coverage as ordinary commits. The hook entry carries the same lockfile bootstrap as the nextest hook, and the hook renders only when the crate shape includes library (there are no doctests without a lib target).

## Risks / Trade-offs

- [Extra doctest-binary compile per commit] → Accepted; doctest suites are typically small, and unvalidated published examples are worse.

## Migration Plan

Implement after the in-review stack (fix-consumer-render-defects) clears Tier-1/Tier-2, then validate, then tag. No downstream edits.

## Open Questions

None.
