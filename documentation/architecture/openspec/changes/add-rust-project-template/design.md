## Context

Fleet Rust projects share a pre-commit layout and, where CI exists, a three-workflow shape, but they disagree on MSRV, toolchain files, platform matrices, and whether CI runs the hook config or a mirrored subset. Interviews are recorded in `rust-project-common:coordination/general/1` through `6`. Advisor accepted the plan in message `f7463fb0-2f50-48b3-9058-011b3cfc4111`, with the platform wording pin in `9707941f-ea0e-48d7-adf7-7c64813aace2`.

The operator required the Python template's top-level shape: root `copier.yaml`, rendered tree under `template/`. Documentation stays out. This repository dogfoods the template. No downstream repository is edited by this change.

## Goals / Non-Goals

**Goals:**

- One Copier template for native Rust libraries and binaries.
- Project answers for the forks that interviews did not settle.
- Hook and CI gates that cannot drift apart by stage.
- A dogfood loop that pins an explicit template tag.

**Non-Goals:**

- Documentation trees, Sphinx, rustdoc publishing, or copying `documentation/common`.
- PyO3 or Maturin extension support.
- The path-resolution crate in `home:ideas/projects/4`.
- A reviewed cargo-deny or cargo-audit baseline.
- Release attestation.
- Generating LitRPG's ten feature-gated binaries, or retrofitting Arbor.
- Editing any downstream repository.

## Decisions

### 1. Python-shaped Copier root

Root `copier.yaml` sets `_subdirectory: template` and `_templates_suffix: .jinja`. Rendered project files live under `template/`. This repository is also the consumer. Updates use `copier update --vcs-ref <template-tag>` from a clean tree after the tag is pushed. `_commit` in the answers file is that tag. There is one template-tag line in this repo.

Alternative: hand-mirror `template/` edits into the working tree. Rejected. That is how consumer copies drift.

Alternative: omit `--vcs-ref` and take Copier's latest PEP 440 tag. Rejected. A second tag namespace would silently win.

### 2. One compiler, pinned at the root

`rust-version` is a required answer and defaults to `1.98`. The template always renders `rust-toolchain.toml` at the repository root, and its channel equals `rust-version`. rustup discovers only `rust-toolchain` or `rust-toolchain.toml` by walking ancestors. A file under `.auxiliary/configuration` is not a toolchain file.

The normal CI build uses that pin. There is no separate MSRV job, and no stable-channel job. Developing on a newer compiler than the declared floor is not a template mode.

Alternative: optional toolchain file, pin greater than or equal to `rust-version`, stable CI plus a separate MSRV check. Rejected by operator review. This fleet wants one compiler story.

### 3. Crate shape is a question

Answers are library, binary, or both. Default is a single crate. A real workspace is default off. If on, the template emits members, a root resolver, and `[workspace.package]` inheritance. An isolated `[workspace]` table, used only to escape an ancestor workspace, is a separate boolean and cannot combine with real-workspace mode.

Additional `[[bin]]` entries after the first copy are project-owned. The template renders its own bin entries from answers in a stable manifest location, and Copier's 3-way update merge carries consumer-added entries forward, surfacing conflict markers on overlap rather than silently overwriting them.

Alternative: make Arbor's workspace the default. Rejected. The retrofit candidates are single crates.

### 4. Stage-scoped hook parity

Hooks use the pre-commit framework at `.auxiliary/configuration/pre-commit.yaml`. When CI is on, CI invokes that same file. The lint job runs the pre-commit stage only. It does not run pre-merge, pre-push, or live hooks. That stage consists of clippy with `-D warnings` on all targets, rustfmt `--check`, linecheck, and nextest. pre-commit, linecheck, and nextest are pinned. Project hook dependencies are installed before the gate.

A platform-specific hook may be scoped out where it is genuinely inapplicable. Each selected platform still runs its applicable build, test, and mandatory quality gates. An entirely excluded platform job is not coverage. An unsupported mandatory step fails visibly.

CI is default on for new projects and may be declined.

Alternative: one `pre-commit run --all-files` with every stage. Rejected. That pulls release builds and live suites into the lint job.

Alternative: mirror selected commands in workflow steps. Rejected. Agentmux and the notebook repos already drifted that way.

### 5. Platforms are an OS by CPU matrix

Operating system and CPU architecture are separate answers. The matrix is their cartesian product. Defaults are Linux and macOS, and both x86-64 and arm64. Windows is opt-in and, when selected, adds a cell for each selected CPU architecture and no other. The initializer emits only selected cells. When CI is on, the template renders initializer, tester, and releaser workflows. Each cell maps to one hosted runner, for example `ubuntu-24.04`, `ubuntu-24.04-arm`, `macos-26-intel`, `macos-26`, `windows-2025`, and `windows-11-arm`. Tester and releaser consume those outputs. An unselected cell is absent, not a skipped job.

Alternative: one list of runner labels such as `ubuntu-latest` plus `macos-latest`. Rejected. That conflates kernel and CPU.

### 6. linecheck always, deny off

linecheck is always rendered, with warn 800 and error 1000. There is no answer to disable it. The hook and config render in every project; the CI install renders only when CI is on. Thresholds and exclusions are project-owned after the first copy, kept by `_skip_if_exists` on the linecheck config file. cargo-deny stays off until a reviewed baseline exists. cargo-audit is not implied by that switch.

### 7. rustfmt is stable-only and lives at the root

`rustfmt.toml` stays at the repository root. rustfmt discovers it by walking ancestors of the file being formatted. A path under `.auxiliary/configuration` is invisible unless every caller, including rust-analyzer, passes `--config-path`.

The file pins `edition` and `style_edition` to the answered edition, which defaults to 2024. It sets `max_width = 79` and `match_block_trailing_comma = true`. It does not set `unstable_features`. A retrofit reviews the formatting diff before accepting it.

The Python extension `rustfmt.toml` was reviewed against current rustfmt. Uncommented options that are still nightly-only (`control_brace_style`, `fn_single_line`, `format_code_in_doc_comments`, `group_imports`, `imports_layout`) are not copied. `space_around_ranges` and `doc_comment_block_width` are gone. `version` is deprecated in favor of `style_edition`. Commented lines that only restate current defaults are not copied.

Alternative: enable those nightly options and pin a nightly toolchain. Rejected. The compiler pin is stable `1.98`.

### 8. Publication is optional and does not bootstrap crates.io

Publication defaults off. Copier rejects publication enabled while CI is disabled. When CI is on, each selected cell runs its release build, test, and package gates. Exactly one non-matrix publish job publishes after the whole matrix succeeds, with OIDC trusted publishing and no long-lived token; that job renders only when publication is on. The releaser workflow renders whenever CI is on. Trusted-publisher registration is an operator prerequisite. The template does not promise tokenless first publication. Attestation is later.

### 9. Fuzz, proptest, and Criterion are opt-in

Fuzz, proptest, and Criterion are separate booleans, each defaulting off. The template is the vehicle for adding them to existing projects later. Enabled proptest runs in the normal test suite and may sit on the pre-commit test stage. Enabled fuzz renders a crate excluded from the workspace and does not run on the pre-commit path. Enabled Criterion does not run on that fast path. Arbor itself is not retrofitted.

## Risks / Trade-offs

- [Width 79 and edition pins produce a formatting diff on every retrofit except LitRPG] → Retrofit reviews that diff before accepting the update.
- [One compiler pin hides breakage on newer stable] → Accepted. A newer-stable job is a later change, not this template.
- [nextest is not what every current repo runs] → The template standardizes on nextest so the hook and CI share one runner. Retrofit replaces `cargo test` hook entries on purpose.
- [First crates.io publish may still need a token or a manual publish] → Document the operator prerequisite. Do not encode a long-lived token.
- [Project-owned binaries and linecheck thresholds drift after copy] → That drift is accepted. Consumer-added `[[bin]]` entries survive through Copier's 3-way update merge, and linecheck thresholds survive through `_skip_if_exists`.
- [Windows is opt-in, so notebook coverage is not the default] → Selecting Windows adds a cell for each selected CPU as real jobs. Missing equivalents fail those jobs.

## Migration Plan

1. Land the template source and dogfood this repository with an explicit template tag.
2. New projects copy from that tag.
3. Retrofit is a later per-repository `copier update`, not part of this change. LitRPG may decline CI and keep edition 2021. Arbor is out.

Rollback is deleting the template tag and the consumer update commit. Downstream repos are untouched, so there is nothing to roll back there.

## Open Questions

None for implementation to start, pending operator review of this change. Review comments are the remaining input.
