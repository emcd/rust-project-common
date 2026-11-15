## MODIFIED Requirements

### Requirement: Hooks use the pre-commit framework

The template MUST render `.auxiliary/configuration/pre-commit.yaml` for the pre-commit framework. It MUST NOT install hooks by copying shell scripts into `.git/hooks`. The pre-commit stage MUST consist of clippy with `-D warnings` on all targets, rustfmt `--check`, linecheck, nextest with zero tests tolerated as a warning and an explicit `--config-file .auxiliary/configuration/nextest.toml`, and, when the crate shape includes library, a doctest gate running `bash -c 'test -f Cargo.lock || cargo generate-lockfile; exec cargo test --doc --locked'`, and the standard file-hygiene hooks: large-file, filename-case-conflict, merge-conflict-marker, symlink, VCS-permalink, destroyed-symlink, private-key, mixed-line-ending, JSON, TOML, and YAML checks, plus end-of-file and trailing-whitespace fixers. The doctest gate MUST be rendered only when the crate shape includes library. The same fast gates MUST also run at the `pre-merge-commit` stage, so merge commits cannot bypass them. The pre-push stage MUST consist of release build (`cargo build --release --locked --all-targets`), release nextest with zero tests tolerated as a warning and an explicit `--config-file .auxiliary/configuration/nextest.toml` (`cargo nextest run --release --locked --no-tests=warn`), and, when the crate shape includes library, a release doctest gate (`cargo test --doc --release --locked`), each with the lockfile bootstrap. When fuzzing is enabled, a bounded `cargo fuzz` run MUST be a pre-push hook. When benchmarking is enabled, a `cargo bench` run MUST be a pre-push hook. The mandatory merge-time gates are exactly the fast gates listed for the pre-commit stage; hygiene hooks run at merge per their upstream stage pins, which the template does not override. Merge-stage gates behave exactly as on ordinary commits, including lockfile bootstrap; if a hook modifies tracked files, pre-commit fails the merge visibly rather than completing it silently.

#### Scenario: Hooks are installed

- **WHEN** a consumer installs hooks from the rendered configuration
- **THEN** the pre-commit framework runs that file and no template hook is a copied script under `.git/hooks`

#### Scenario: Merge commit runs fast gates

- **WHEN** a consumer creates a merge commit with hooks installed
- **THEN** the clippy, fmt, linecheck, nextest, and (for library crates) doctest hooks run at `pre-merge-commit` and a violation blocks the merge

#### Scenario: Push runs release gates

- **WHEN** a consumer pushes with hooks installed
- **THEN** the release build, release nextest, and (for library crates) release doctest hooks run at `pre-push` and a violation blocks the push

#### Scenario: Doctests execute

- **WHEN** a consumer commits a doc comment containing a failing code example
- **THEN** the doctest gate fails before the commit completes

#### Scenario: Fresh scaffold with no tests

- **WHEN** a consumer commits to a rendered project that has no tests yet
- **THEN** the nextest gate warns about zero tests and passes instead of failing

## ADDED Requirements

### Requirement: Nextest config is rendered

The template MUST render a default `.auxiliary/configuration/nextest.toml` carrying no behavioral tuning, and MUST list it in `_skip_if_exists` so later copies and updates never overwrite consumer tuning. Every template nextest invocation MUST pass `--config-file` naming that path.

#### Scenario: Project is rendered

- **WHEN** a consumer generates a project
- **THEN** the config file exists with no tuning, and every nextest entry names it

#### Scenario: Consumer tunes nextest

- **WHEN** a consumer edits the config and then runs `copier update`
- **THEN** those edits remain the consumer's values
