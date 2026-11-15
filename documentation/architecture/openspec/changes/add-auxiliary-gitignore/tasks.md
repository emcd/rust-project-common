## 1. Ignore file

- [x] 1.1 Add static `template/.auxiliary/.gitignore` (`/scribbles/`, `/temporary/`, pointer comment)
- [x] 1.2 Mirror the same two lines in this repository's own `.auxiliary/.gitignore`

## 2. Validate

- [x] 2.1 `openspec validate --all --strict` passes
- [x] 2.2 `copiertv validate` passes for default, maximum, and executable-only variants
- [x] 2.3 Every rendered tree contains the file with both entries; no other `.gitignore` gains them
