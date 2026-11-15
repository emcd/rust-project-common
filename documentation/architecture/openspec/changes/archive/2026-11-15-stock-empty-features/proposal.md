## Why

Cargo builds and tests with default features on unless told otherwise, so a crate that later gains a `default` feature silently changes what CI lints and tests — the suite drifts without any invocation changing. Passing an empty feature set explicitly pins the tested configuration the way `--locked` pins dependencies: nothing about the feature set changes unless the invocation text changes. The proposal's own fleet check confirms the blast radius is zero today for the proposer and that the two feature-bearing consumers already carry explicit `--all-features` overrides, which merge keeps.

## What Changes

- Template clippy entries pass `--no-default-features` (kept alongside `--all-targets` and `-D warnings`).
- Template nextest entries (dev, merge-commit, and release) pass `--no-default-features` (kept alongside `--locked` and the config-file flag).
- Release build entries (pre-push hook and CI release-gates) pass `--no-default-features`.
- The tester dev smoke build, doctest gates, `cargo package`, and fuzz/bench entries are untouched: the smoke build keeps manifest defaults (not `--all-features`), doctests are out of scope per the proposer, packaging has no feature flags, and fuzz/bench have their own feature story.
- No new answers; no conditional rendering. Consumers with explicit feature flags keep them on merge (project-owned overrides win the conflict hunks).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `quality-gates`: clippy and nextest invocations carry an explicit empty feature set.

## Impact

- `template/.auxiliary/configuration/pre-commit.yaml.jinja` (clippy, three nextest entries).
- `template/.github/workflows/releaser.yaml.jinja` (release test command).
- `openspec/specs/quality-gates/spec.md` and `openspec/specs/optional-features/spec.md` (invocation wording).
- Consumers with a `[features]` table and no explicit override start testing with default features off; the two known such consumers already override explicitly and are unaffected.
