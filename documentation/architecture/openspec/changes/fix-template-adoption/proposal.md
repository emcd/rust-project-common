## Why

The Agentmux retrofit surfaced three template defects: no answers file is written on copy, a `[]` authors value crashes updates before validators run, and `src/main.rs` fights multi-bin layouts on every update.

## What Changes

- Ship the `{{ _copier_conf.answers_file }}.jinja` answers template so copies record answers for updates.
- Keep `authors` as `str`; machine-written files always serialize correctly.
- Render the executable component as `src/bin/{{ crate_name }}.rs`, never `src/main.rs`, assuming multiple executables from the beginning.
- Document the `src/bin/` convention in the `crate_shape` help text.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `template-layout`: answers file recorded on copy.
- `cargo-manifest`: executable target lives in `src/bin/`.

## Impact

- Template output only. Existing renders are unaffected until their next update; multi-bin projects stop getting a default-bin hijacker reintroduced.
