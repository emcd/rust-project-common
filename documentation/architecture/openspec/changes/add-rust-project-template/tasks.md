## 1. Copier root

- [x] 1.1 Root `copier.yaml` with `_subdirectory: template`, `_templates_suffix: .jinja`, and the answers in the cargo-manifest, quality-gates, and optional-features specs
- [x] 1.2 `crate_shape` is a multi-choice of library and executable defaulting to library with at least one required; `workspace_mode` is one choice of none, real, or isolated
- [x] 1.3 `copiertv` config plus default, all-opt-in, and executable-only answer variants; no committed dummy project and no dogfood answers file

## 2. Cargo skeleton

- [x] 2.1 Render `Cargo.toml` for library, executable, and both, with required `rust-version` defaulting to `1.98` and edition fixed at 2024
- [x] 2.2 Always render root `rust-toolchain.toml` with channel equal to `rust-version`
- [x] 2.3 Render real-workspace inheritance for mode real and the isolated `[workspace]` table for mode isolated
- [x] 2.4 Render template bins from answers in a stable manifest location and rely on Copier's 3-way update merge to carry consumer-added `[[bin]]` entries forward with conflict markers on overlap
- [x] 2.5 Render root `rustfmt.toml` with `edition` and `style_edition` fixed at 2024, `max_width = 79`, and `match_block_trailing_comma = true`, and no nightly-only options
- [x] 2.6 Render a bare skeleton: empty library root and empty main function

## 3. Quality gates

- [x] 3.1 Render `.auxiliary/configuration/pre-commit.yaml` whose pre-commit stage runs clippy with `-D warnings` on all targets, rustfmt `--check`, linecheck, nextest, and the file-hygiene hooks, with bounded fuzz and bench runs as pre-push hooks when enabled
- [x] 3.2 Always render the linecheck hook, config, and CI install at warn 800 / error 1000 with `_skip_if_exists`
- [x] 3.3 Render `core--initializer.yaml` as the cartesian product of selected OS and CPU, defaulting to Linux and macOS on x86-64 and arm64, with a Windows cell for each selected CPU only, and with the tester matrix and the releaser consuming its outputs
- [x] 3.4 Render `tester.yaml` so the lint job runs only the pre-commit stage of the same config as the mandatory gate on every selected pair, pins pre-commit, linecheck, and nextest, and builds with the toolchain pin only
- [x] 3.5 Fail a selected platform visibly when a mandatory gate has no equivalent, and emit no job for an unselected platform

## 4. Optional features

- [x] 4.1 Render publication off by default, always render the releaser workflow with per-cell release gates plus exactly one non-matrix publish job that depends on the full matrix and renders only when publication is on, and when on, test and package before OIDC trusted publishing with no long-lived token
- [x] 4.2 Document trusted-publisher registration as an operator prerequisite and do not promise tokenless first publication
- [x] 4.3 Render fuzzing, benchmarking, and property testing only when each answer is on; exclude the fuzz crate from the workspace; keep fuzz and benchmarking off the pre-commit stage and on the pre-push stage
- [x] 4.4 Leave cargo-deny, cargo-audit, attestation, documentation, and PyO3 out of the rendered tree

## 5. Validation proof

- [ ] 5.1 Tag the template and push the tag for consumers
- [x] 5.2 Confirm every answer-matrix render matches the specs
