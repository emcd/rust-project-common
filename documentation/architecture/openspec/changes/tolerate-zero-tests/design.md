## Context

`cargo nextest run` exits 4 when no tests are discovered. The template scaffold ships no tests (deliberately bare), so the very first commits fail the test gate. The fix belongs in the invocation, not in a placeholder test: a smoke test would assert nothing and rot.

## Goals / Non-Goals

**Goals:**

- Fresh scaffolds pass the test gate with a visible "no tests" warning.
- Same behavior on pre-commit, merge-commit, and release-gates paths.

**Non-Goals:**

- Changing the pinned nextest version or the lockfile bootstrap.
- A scaffold smoke test (asserts nothing; rots).

## Decisions

### 1. `warn`, not `pass`

`pass` would silently green-light a suite with broken discovery. `warn` exits 0 but prints the state, keeping the nudge to write the first test.

### 2. Both invocations, not just the hook

The releaser's direct `cargo nextest run` hits the same wall on test-less crates at release time.

## Risks / Trade-offs

- None material: strictly loosens a false failure; real failures still fail.

## Migration Plan

Land, re-validate, tag. Consumers pick it up on next update.

## Open Questions

None.
