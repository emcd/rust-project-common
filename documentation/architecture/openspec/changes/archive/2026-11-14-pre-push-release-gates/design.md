## Context

The Hooks requirement currently gates two stages: fast dev gates at pre-commit and pre-merge-commit, and opt-in fuzz/bench at pre-push. CI mirrors this with a dev-profile gates job and a release-profile release-gates job — but locally, the release profile has no hook twin, so release-only faults first appear in CI.

## Goals / Non-Goals

**Goals:**

- Every CI release gate has a local pre-push twin running the same command.
- Semantic diversity: dev hooks keep assertion-enabled diagnostics; release hooks prove optimized behavior.

**Non-Goals:**

- No change to pre-commit speed or contents.
- No release doctest omission: doctests compile and run code, so they twin too (library shape only, mirroring the existing doctest gate).
- No publishing, credentials, or tag operations in hooks.

## Decisions

### 1. Always on, not opt-in

The value is catching faults before CI on every push; an opt-in flag would reproduce the current gap for everyone who never flips it. The cost (slower pushes) is stated in the proposal and accepted by the operator.

### 2. Same commands as CI release-gates, verbatim

Build, nextest (with `--no-tests=warn` plus `verbose: true` so the warning stays visible, matching the pre-commit twin), doctest with `--release` added. Lockfile bootstrap prefix identical to the existing hooks. Any future change to the CI release commands must update the twins in the same commit — the parity rule now covers both stages.

### 3. CI release-gates gains the doctest step too

Leaving release doctests hook-only would create exactly the asymmetry this change removes. The CI step costs little (tags only) and keeps the parity claim verbatim: every release-suite command exists identically in both lanes.

### 4. No new hook types or install flow

Pre-push is already in `default_install_hook_types`; hook IDs are read from config at run time, so no reinstall ceremony beyond the guide's standard re-run. Consumers with fuzz/bench enabled keep those hooks alongside the twins, ordered after (slowest last is incidental, not specified).

## Risks / Trade-offs

- [Slower pushes, including WIP pushes to feature branches] → Accepted explicitly: the operator's "slower pushes, fewer red CIs". Escape hatch is the standard bypass, not a template option.
- [Doctest --release doubles doctest cost] → Accepted: doctests are compiled code; a release-only doctest fault is exactly the class being hunted.
- [Twins can drift from CI commands] → Mitigation: parity rule stated in design; copiertv asserts the command strings match between hook entries and release-gates steps.
