## 1. Copier root

- [x] 1.1 Add root `copier.yaml` with `_subdirectory: template`, `_templates_suffix: .jinja`, and the answers in the cargo-manifest, quality-gates, and optional-features specs
- [x] 1.2 Reject real-workspace combined with an isolated `[workspace]` table
- [x] 1.3 Add this repository's answers file and dogfood instructions that require a pushed template tag and `--vcs-ref`

## 2. Cargo skeleton

- [x] 2.1 Render `Cargo.toml` for library, binary, and both, with required `rust-version` defaulting to `1.98` and edition defaulting to 2024
- [x] 2.2 Always render root `rust-toolchain.toml` with channel equal to `rust-version`
- [x] 2.3 Render real-workspace inheritance and the isolated `[workspace]` table as mutually exclusive outputs
- [x] 2.4 Render template bins from answers in a stable manifest location and rely on Copier's 3-way update merge to carry consumer-added `[[bin]]` entries forward with conflict markers on overlap
- [x] 2.5 Render root `rustfmt.toml` with `edition`, `style_edition`, `max_width = 79`, and `match_block_trailing_comma = true`, and no nightly-only options

## 3. Quality gates

- [x] 3.1 Render `.auxiliary/configuration/pre-commit.yaml` whose pre-commit stage runs clippy with `-D warnings` on all targets, rustfmt `--check`, linecheck, and nextest
- [x] 3.2 Always render the linecheck hook and config at warn 800 / error 1000 with `_skip_if_exists`, and render the linecheck CI install only when CI is on
- [x] 3.3 When CI is on, render `core--initializer.yaml` as the cartesian product of selected OS and CPU, defaulting to Linux and macOS on x86-64 and arm64, with a Windows cell for each selected CPU only, and with the tester matrix and the releaser consuming its outputs
- [x] 3.4 When CI is on, render `tester.yaml` so the lint job runs only the pre-commit stage of the same config as the mandatory gate on every selected pair, pins pre-commit, linecheck, and nextest, and builds with the toolchain pin only
- [x] 3.5 Fail a selected platform visibly when a mandatory gate has no equivalent, and emit no job for an unselected platform

## 4. Optional features

- [x] 4.1 Render publication off by default, reject publication enabled while CI is disabled, render the releaser workflow whenever CI is on with per-cell release gates plus exactly one non-matrix publish job that depends on the full matrix and renders only when publication is on, and when on, test and package before OIDC trusted publishing with no long-lived token
- [x] 4.2 Document trusted-publisher registration as an operator prerequisite and do not promise tokenless first publication
- [x] 4.3 Render fuzz, Criterion, and proptest only when each answer is on; exclude the fuzz crate from the workspace; keep fuzz and Criterion off the pre-commit stage
- [x] 4.4 Leave cargo-deny, cargo-audit, attestation, documentation, and PyO3 out of the rendered tree

## 5. Dogfood proof

- [ ] 5.1 Tag the template, push the tag, and update this repository with `--vcs-ref`
- [ ] 5.2 Confirm the rendered tree matches the specs for the answers this repository selects
