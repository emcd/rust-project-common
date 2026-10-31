## Why

A freshly scaffolded crate has zero tests, so the nextest gate fails every commit (exit 4, "no tests to run") until someone writes the first test. Approots hit this on a fresh populate: clippy and fmt pass, only nextest trips, on every commit including pure doc/config ones. Verified directly: default exits 4, `--no-tests=warn` exits 0 with a visible warning.

## What Changes

- Add `--no-tests=warn` to both nextest invocations: the pre-commit hook entry and the releaser's release-gates step.
- Amend the test-runner sentence to require zero-test tolerance.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `quality-gates`: nextest tolerates zero tests with a warning instead of failing.

## Impact

- `warn` (not `pass`): the gate stays green but the empty suite stays visible, so broken test discovery cannot hide behind silence. No downstream edits.
