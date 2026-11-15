## 1. Empty feature set

- [x] 1.1 Pass `--no-default-features` in the template clippy entry (kept with `--all-targets` and `-D warnings`)
- [x] 1.2 Pass `--no-default-features` in all three template nextest entries (dev, merge-commit, release; doctest entries untouched)
- [x] 1.3 Pass `--no-default-features` in the CI release-gates nextest step and release build step
- [x] 1.4 Note the sync-order dependency (merges with the config-file wording at sync, verified clause-by-clause)
- [x] 1.5 Correct to `--no-default-features` after Tier-1 sentinel refutation; prove with sentinel crate plus control

## 2. Validate

- [x] 2.1 `openspec validate --all --strict` passes
- [x] 2.2 `copiertv validate` passes for default, maximum, and executable-only variants
- [x] 2.3 Rendered entries carry the flag in every nextest and clippy invocation; doctest entries do not
- [x] 2.4 Agentmux and nb-api manifests confirm the blast-radius claim (both define `[features]`, both already override explicitly)
