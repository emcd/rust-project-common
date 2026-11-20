## 1. Reusable tester

- [x] 1.1 Add root `.github/workflows/xrepo--tester.yaml` with `platforms`, validated-string `cargo-profile`, and `has-library` inputs plus a leading validation step; lint, build, and test jobs chained by needs with no `if:` on any mandatory gate job
- [x] 1.2 Rewrite tester callers: `initialize`, parallel dev and release calls, single-runner `hygiene` job with the explicit hook-ID list, `custom` job needing only `initialize`, and a `final-gate` umbrella needing all of them with `if: always()`
- [x] 1.3 Rewrite releaser release-gates as a release-profile caller; keep a local per-platform package job unconditional on custom/dry-run/publication flags; wire `publish-crates-io` and `build-binaries` to wait on release gates, package, and `custom-release` when rendered while preserving publisher tag/dry-run/publication predicates and credential protections; `publish-github` via `build-binaries`
- [x] 1.4 Add the single `enable_custom_ci` answer covering both lanes; render failing-loudly `local--tester.yaml` / `local--releaser.yaml` stubs (declaring `platforms` and `cargo-profile` only) whenever absent (regardless of the enabled answer), with an explicit `permissions: contents: read` block, both `_skip_if_exists`-protected
- [x] 1.5 Versioning note: floating major default in rendered callers; per-change immutable tags cut at release time; initial tag ships before any consumer renders a caller; fixture validation of each candidate at its commit before any floating-major move
- [x] 1.6 Rename the initializer's `cells` output to `platforms` in `common-initializer.yaml.jinja` and rebind every caller to `needs.initialize.outputs.platforms` (plus a hand-migration note for consumer-owned `cells` readers Copier cannot rewrite); the initializer itself stays local and answers-rendered

## 2. Validate

- [x] 2.1 `openspec validate --all --strict` passes
- [x] 2.2 `copiertv validate` passes for default, maximum, and executable-only variants
- [x] 2.3 Rendered callers reference the floating major; no inline gate steps remain in the shared Rust gate bodies (tester hygiene excepted); no `if:` on any mandatory gate job except the umbrella's `always()`; publication predicates keep theirs
- [x] 2.4 Byte-identical hook-to-CI command map with a single documented normalization (strip the lockfile-bootstrap wrapper); setup, package, upload, and publish commands on an explicit no-twin allowlist; local-only fuzz/bench hooks exempt from the reverse direction; negative drift fixtures (each side changed alone) fail validation
- [x] 2.5 Enabled renders carry the `custom` and `custom-release` jobs with the specified needs and only the `platforms` + `cargo-profile` inputs; disabled renders omit them while stub files may exist; absent consumer files get failing stubs that are never overwritten
- [x] 2.6 Umbrella fails closed on any non-success including skipped dependencies; releaser needs graph verified stage by stage including dry-run and publication-off runs; red-package (publish blocked when package fails) and red-custom-release controls asserted structurally (live red runs are promotion-time work, tracked outside this proposal)

- [x] 2.7 Least privilege asserted: explicit `permissions` on all shared/custom/package/hygiene jobs (`contents: read` or less), no secrets to test workflows, `persist-credentials: false` on non-push checkouts, no `pull_request_target` execution of PR code; initializer mapping vs callee allowlist agreement; invalid runner/target entries rejected before fan-out

Promotion-time obligations (fixture branches, hosted red controls, first tag, ref protections, freshness recheck, bounded recovery) live outside the proposal task list by rule: proposals carry implementation tasks only. They are tracked as project todos until promotion.
