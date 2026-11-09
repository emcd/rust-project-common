## 1. Starter

- [x] 1.1 Add static `template/CHANGELOG.md` (fleet header plus empty Unreleased, no version entries)
- [x] 1.2 List `CHANGELOG.md` in `copier.yaml` `_skip_if_exists`

## 2. Validate

- [x] 2.1 `openspec validate --all --strict` passes
- [x] 2.2 `copiertv validate` passes for default, maximum, and executable-only variants
- [x] 2.3 Every rendered tree contains byte-identical `CHANGELOG.md`; a pre-existing consumer file survives `copier update` unchanged
