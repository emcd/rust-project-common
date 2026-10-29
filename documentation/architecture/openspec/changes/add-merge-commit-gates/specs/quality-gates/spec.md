## MODIFIED Requirements

### Requirement: Hooks use the pre-commit framework

The template MUST render `.auxiliary/configuration/pre-commit.yaml` for the pre-commit framework. It MUST NOT install hooks by copying shell scripts into `.git/hooks`. The pre-commit stage MUST consist of clippy with `-D warnings` on all targets, rustfmt `--check`, linecheck, nextest, and the standard file-hygiene hooks: large-file, filename-case-conflict, merge-conflict-marker, symlink, VCS-permalink, destroyed-symlink, private-key, mixed-line-ending, JSON, TOML, and YAML checks, plus end-of-file and trailing-whitespace fixers. The same fast gates MUST also run at the `pre-merge-commit` stage, so merge commits cannot bypass them. When fuzzing is enabled, a bounded `cargo fuzz` run MUST be a pre-push hook. When benchmarking is enabled, a `cargo bench` run MUST be a pre-push hook. The mandatory merge-time gates are exactly those four fast gates; hygiene hooks run at merge per their upstream stage pins, which the template does not override.

#### Scenario: Hooks are installed

- **WHEN** a consumer installs hooks from the rendered configuration
- **THEN** the pre-commit framework runs that file and no template hook is a copied script under `.git/hooks`

#### Scenario: Merge commit runs fast gates

- **WHEN** a consumer creates a merge commit with hooks installed
- **THEN** the clippy, fmt, linecheck, and nextest hooks run at `pre-merge-commit` and a violation blocks the merge
