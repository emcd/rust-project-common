## Why

The installation stub assumes layouts a fresh copy cannot satisfy: a sibling-checkout path dependency that is not portable, and registry installs for crates that may never be published.

## What Changes

- Render `## Installation` only when publication is on: `cargo add` for libraries, `cargo install` for executables.
- Publication-off renders keep the Usage stub and no Installation section.
- Add an `Additional Badges` section: last-commit, Copier, and pre-commit badges always, plus a crates.io downloads badge when publication is on.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `template-layout`: installation section conditional on publication; badge set extended.

## Impact

- Rendered README only. No downstream edits. The `Other Projects by This Author` section waits on the operator's Rust project list.
