## Why

Fleet Rust projects have drifted on Cargo layout, Git hooks, and CI. New projects need one Copier template for those surfaces, and later retrofits need answers rather than a forced rewrite. The plan is ready for review before any template files are written.

## What Changes

- Add a native-Rust Copier template: root `copier.yaml`, rendered project under `template/`.
- Generate Cargo manifests, rustfmt config, pre-commit hooks, and GitHub Actions from project answers.
- Leave documentation, PyO3 extensions, cargo-deny policy, release attestation, and the path-resolution crate out of this change.
- Dogfood this repository as a consumer of its own template, on a single template-tag line.

## Capabilities

### New Capabilities

- `template-layout`: Copier root layout, dogfood update rules, and exclusions (no docs tree, no PyO3).
- `cargo-manifest`: Edition, MSRV, toolchain file, crate shape, workspace mode, extra binaries, and rustfmt pins.
- `quality-gates`: pre-commit configuration, stage-scoped CI parity, platform selection, and linecheck.
- `optional-features`: Publication, fuzz, Criterion, proptest, and the decision to leave cargo-deny off.

### Modified Capabilities

- None. This repository has no existing specs.

## Impact

- New template source in this repository. No downstream repository is edited by this change.
- LitRPG can select edition 2021 and decline CI. Arbor is not a retrofit target.
- Operator review, 2026-09-26: toolchain file always matches `rust-version` and stays at the root; platforms are an OS by CPU matrix; linecheck cannot be disabled; `max_width` 79 is the default; fuzz, proptest, and Criterion default off.
- Trusted-publisher registration for crates.io stays an operator setup step, outside the template.
