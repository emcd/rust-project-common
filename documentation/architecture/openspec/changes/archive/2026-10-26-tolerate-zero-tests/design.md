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

### 3. Show the warning, don't just emit it

pre-commit suppresses successful hook output, which would hide the very warning `--no-tests=warn` produces. The hook carries `verbose: true` so the warning is visible in ordinary (non-verbose) runs.

## Risks / Trade-offs

- An empty suite is accepted, whether it is a fresh scaffold or accidental broken discovery. The visible warning is observability, not an enforced minimum-test guarantee: it keeps the nudge to write tests without inventing a test-count policy.
- Real failures still fail: only the zero-test exit path changes.

## Migration Plan

Land, re-validate, tag. Consumers pick it up on next update.

## Open Questions

None.
