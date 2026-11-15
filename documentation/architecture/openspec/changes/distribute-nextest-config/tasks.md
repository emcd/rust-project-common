## 1. Config distribution

- [x] 1.1 Add static `template/.auxiliary/configuration/nextest.toml` (header plus docs pointer, no tuning)
- [x] 1.2 List it in `copier.yaml` `_skip_if_exists`
- [x] 1.3 Pass `--config-file .auxiliary/configuration/nextest.toml` in all three template nextest entries (pre-commit, pre-merge-commit, pre-push release) and the CI release-gates test step
- [x] 1.4 Extend the copiertv bootstrap file with the config-file presence assertion (file rendered; every nextest entry names it)

## 2. Validate

- [x] 2.1 `openspec validate --all --strict` passes
- [x] 2.2 `copiertv validate` passes for default, maximum, and executable-only variants
- [x] 2.3 Rendered tree contains the config file; a pre-existing consumer file survives copy-over-existing byte-identical
- [x] 2.4 Full delta carries both ADDED and MODIFIED sections intact to sync
- [x] 2.5 Every prose line commented (valid TOML); execution proven with a real nextest run against the template file
