## Why

Adopting 0.3.0 showed `src/bin/<crate>.rs` still overwrites an already-implemented entry point on copy and update. An implemented `main` is consumer code and must never be overwritten.

## What Changes

- Add the rendered executable path to `_skip_if_exists` so an existing entry point survives; absent files still render the stub.
- Amend the Extra binaries requirement to state the mechanism.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `cargo-manifest`: template-owned executable target uses `_skip_if_exists`.

## Impact

- Template config only. Fresh renders unchanged. No downstream edits.
