## Why

The LitRPG dry render proved the empty `src/lib.rs` stub would overwrite an implemented 43-line library root — the exact revisit trigger named when the entry-point guard scoped `lib.rs` out.

## What Changes

- Add the rendered library path to `_skip_if_exists` so an implemented `src/lib.rs` is never overwritten; the stub renders only when absent.
- Amend the Extra binaries requirement (now covering both template-owned stubs).

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `cargo-manifest`: template-owned stubs use `_skip_if_exists`.

## Impact

- Template config only. Fresh renders unchanged. LitRPG answers library-only: no bin stub rendered, implemented lib preserved, ten bins project-owned.
