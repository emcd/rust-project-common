## 1. Releaser rewrite

- [x] 1.1 Retitle trigger to `v[0-9]+.*`, add `dry-run` dispatch input (default true), add top-level `CARGO_TERM_COLOR`
- [x] 1.2 Rename `publish` to `publish-crates-io`, declare `environment: crates-io`, gate on tag ref with dry-run disabled, bump checkout to v6 and auth action to v1.0.4
- [x] 1.3 Add `RUST_BACKTRACE: '1'` to the release test step; keep `cargo publish --locked`, lockfile bootstrap, and `contents: read`
- [x] 1.4 Bump checkout to v6 in `tester.yaml.jinja`

## 2. Validate

- [x] 2.1 `openspec validate --all --strict` passes
- [x] 2.2 `copiertv validate` passes for default, maximum, and executable-only variants
- [x] 2.3 Diff rendered `releaser.yaml` (publication enabled) against the reference `publish-crates-io` job: same identity, environment, permissions baseline, trigger, and dry-run semantics
- [x] 2.4 Confirm no other in-repo reader of the old `publish` job id
