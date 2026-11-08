## 1. Rename

- [x] 1.1 Rename job `lint` to `gates` in `template/.github/workflows/tester.yaml.jinja` (id only; no step, matrix, or dependency changes)
- [x] 1.2 Update the copiertv `general.toml` bootstrap comment from (lint, build) to (gates, build)

## 2. Validate

- [x] 2.1 `openspec validate --all --strict` passes
- [x] 2.2 `copiertv validate` passes for default, maximum, and executable-only variants
- [x] 2.3 Rendered `tester.yaml` carries the `gates` job id and is otherwise byte-identical apart from the rename
- [x] 2.4 Confirm no other in-repo reader of the old job name (clippy hook display name, review prose, and archive history stay)
