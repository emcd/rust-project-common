## MODIFIED Requirements

### Requirement: Hooks use the pre-commit framework

The template MUST render `.auxiliary/configuration/pre-commit.yaml` for the pre-commit framework. It MUST NOT install hooks by copying shell scripts into `.git/hooks`. The pre-commit stage MUST consist of clippy with `-D warnings` on all targets, rustfmt `--check`, linecheck, nextest with zero tests tolerated as a warning, and, when the crate shape includes library, a doctest gate running `bash -c 'test -f Cargo.lock || cargo generate-lockfile; exec cargo test --doc --locked'`, and the standard file-hygiene hooks: large-file, filename-case-conflict, merge-conflict-marker, symlink, VCS-permalink, destroyed-symlink, private-key, mixed-line-ending, JSON, TOML, and YAML checks, plus end-of-file and trailing-whitespace fixers. The doctest gate MUST be rendered only when the crate shape includes library. The same fast gates MUST also run at the `pre-merge-commit` stage, so merge commits cannot bypass them. When fuzzing is enabled, a bounded `cargo fuzz` run MUST be a pre-push hook. When benchmarking is enabled, a `cargo bench` run MUST be a pre-push hook. The mandatory merge-time gates are exactly the fast gates listed for the pre-commit stage; hygiene hooks run at merge per their upstream stage pins, which the template does not override. Merge-stage gates behave exactly as on ordinary commits, including lockfile bootstrap; if a hook modifies tracked files, pre-commit fails the merge visibly rather than completing it silently.

#### Scenario: Hooks are installed

- **WHEN** a consumer installs hooks from the rendered configuration
- **THEN** the pre-commit framework runs that file and no template hook is a copied script under `.git/hooks`

#### Scenario: Merge commit runs fast gates

- **WHEN** a consumer creates a merge commit with hooks installed
- **THEN** the clippy, fmt, linecheck, nextest, and (for library crates) doctest hooks run at `pre-merge-commit` and a violation blocks the merge

#### Scenario: Doctests execute

- **WHEN** a consumer commits a doc comment containing a failing code example
- **THEN** the doctest gate fails before the commit completes

#### Scenario: Fresh scaffold with no tests

- **WHEN** a consumer commits to a rendered project that has no tests yet
- **THEN** the nextest gate warns about zero tests and passes instead of failing

### Requirement: CI runs the same config for one stage

The lint job MUST invoke the rendered pre-commit configuration with an explicit `--config` path and MUST run only the pre-commit stage. It MUST NOT run pre-merge, pre-push, or live hooks. The workflow MUST pin pre-commit, linecheck, and nextest, and MUST install project hook dependencies before the gate. The suite runner in that shared config MUST be nextest; a doctest-only runner MAY accompany it. That stage is the mandatory gate on every selected OS by CPU pair, except a hook scoped out as genuinely inapplicable.

#### Scenario: Lint job on pull request

- **WHEN** a pull request runs the lint job
- **THEN** the job runs the pre-commit stage of `.auxiliary/configuration/pre-commit.yaml` with an explicit `--config` argument and does not run pre-push hooks

#### Scenario: Rendered workflows pass the template's own gates

- **WHEN** a consumer renders the template
- **THEN** every rendered file already satisfies the hygiene hooks with no fixer rewrites
