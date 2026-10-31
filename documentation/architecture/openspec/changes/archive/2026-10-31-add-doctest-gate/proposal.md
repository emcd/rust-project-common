## Why

`cargo nextest` does not execute doctests (verified by Praetor Owner with a probe crate: `cargo test` runs unit plus doctest, `cargo nextest` runs only the unit test). The template's fast gates are clippy, fmt, linecheck, and nextest — so no gate anywhere executes documentation examples, even though published crates render them on docs.rs. Merge-time coverage inherits the same hole since the merge gate is nextest.

## What Changes

- Add a narrow `cargo-doctest` hook running `cargo test --doc --locked` on the pre-commit and pre-merge-commit stages. It runs doctests only, so the full suite is not duplicated.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `quality-gates`: doctests run as a fast gate alongside nextest.

## Impact

- One additional hook in the rendered pre-commit config; one extra test-binary compile per commit. Held for implementation until the in-review stack clears, so as not to stale Tier-1's packet.
