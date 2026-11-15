# quality-gates

## Purpose

Pre-commit hooks, CI parity, platform matrix, and line budgets.

## Requirements

### Requirement: Hooks use the pre-commit framework
The template MUST render `.auxiliary/configuration/pre-commit.yaml` for the pre-commit framework. It MUST NOT install hooks by copying shell scripts into `.git/hooks`. The pre-commit stage MUST consist of clippy with `-D warnings` and no default features on all targets, rustfmt `--check`, linecheck, nextest with zero tests tolerated as a warning, an explicit `--config-file .auxiliary/configuration/nextest.toml`, and no default features, and, when the crate shape includes library, a doctest gate running `bash -c 'test -f Cargo.lock || cargo generate-lockfile; exec cargo test --doc --locked'`, and the standard file-hygiene hooks: large-file, filename-case-conflict, merge-conflict-marker, symlink, VCS-permalink, destroyed-symlink, private-key, mixed-line-ending, JSON, TOML, and YAML checks, plus end-of-file and trailing-whitespace fixers. The doctest gate MUST be rendered only when the crate shape includes library. The same fast gates MUST also run at the `pre-merge-commit` stage, so merge commits cannot bypass them. The pre-push stage MUST consist of release build with no default features (`cargo build --release --locked --all-targets --no-default-features`), release nextest with zero tests tolerated as a warning, an explicit `--config-file .auxiliary/configuration/nextest.toml`, and no default features (`cargo nextest run --no-default-features --release --locked --no-tests=warn --config-file .auxiliary/configuration/nextest.toml`), and, when the crate shape includes library, a release doctest gate (`cargo test --doc --release --locked`), each with the lockfile bootstrap. When fuzzing is enabled, a bounded `cargo fuzz` run MUST be a pre-push hook. When benchmarking is enabled, a `cargo bench` run MUST be a pre-push hook. The mandatory merge-time gates are exactly the fast gates listed for the pre-commit stage; hygiene hooks run at merge per their upstream stage pins, which the template does not override. Merge-stage gates behave exactly as on ordinary commits, including lockfile bootstrap; if a hook modifies tracked files, pre-commit fails the merge visibly rather than completing it silently.

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

### Requirement: CI runs the same config for one stage
The gates job MUST invoke the rendered pre-commit configuration with an explicit `--config` path and MUST run only the pre-commit stage. It MUST NOT run pre-merge, pre-push, or live hooks. The workflow MUST pin pre-commit, linecheck, and nextest, and MUST install project hook dependencies before the gate. The suite runner in that shared config MUST be nextest; a doctest-only runner MAY accompany it. That stage is the mandatory gate on every selected OS by CPU pair, except a hook scoped out as genuinely inapplicable. When the consumer enables custom CI, the tester MUST render exactly one additional `custom` job that calls the consumer-authored `./.github/workflows/local--tester.yaml` reusable workflow at job level with explicit `contents: read` permissions and no secrets, and the tester MUST render nothing else for it. The custom job MUST be omitted byte-entirely when custom CI is disabled, MUST NOT be hidden behind a runtime condition, and MUST NOT be part of the mandatory same-config gate.

#### Scenario: Gates job on pull request

- **WHEN** a pull request runs the gates job
- **THEN** the job runs the pre-commit stage of `.auxiliary/configuration/pre-commit.yaml` with an explicit `--config` argument and does not run pre-push hooks

#### Scenario: Rendered workflows pass the template's own gates

- **WHEN** a consumer renders the template
- **THEN** every rendered file already satisfies the hygiene hooks with no fixer rewrites

#### Scenario: Custom CI disabled

- **WHEN** a consumer renders the template with custom CI off
- **THEN** the tester contains no custom job and no reference to any local workflow file

#### Scenario: Custom CI enabled

- **WHEN** a consumer enables custom CI and provides `local--tester.yaml`
- **THEN** the tester runs the consumer workflow as an independent gate alongside the template gates

#### Scenario: Custom CI enabled without the file

- **WHEN** a consumer enables custom CI but does not provide `local--tester.yaml`
- **THEN** the tester run fails at workflow resolution rather than skipping

### Requirement: Selected OS and CPU pairs are real jobs

Operating system and CPU architecture MUST be separate answers. Linux and macOS, and both x86-64 and arm64, MUST be selected by default. Windows MUST default off. The workflow matrix MUST be the cartesian product of the selected values, the initializer MUST emit only selected pairs, and tester and releaser MUST consume those outputs. The template MUST render initializer, tester, and releaser workflows; the releaser MUST publish to crates.io only when publication is enabled. Release build, test, and package gates MUST run in each selected cell; publication MUST be a single non-matrix publish job that depends on the success of the entire matrix and MUST be rendered only when publication is enabled. When Windows is selected, a Windows cell MUST exist for each selected CPU architecture and for no other. Each selected pair MUST run its applicable build, test, and mandatory quality gates; a platform-specific hook MAY be scoped out only where it is genuinely inapplicable; an unsupported mandatory step MUST fail visibly; and the template MUST NOT emit a pair and then exclude it from those gates.

#### Scenario: Default platforms

- **WHEN** a consumer accepts default platform answers
- **THEN** the matrix contains Linux and macOS on both x86-64 and arm64, and contains no Windows job

#### Scenario: Windows selected with both CPUs

- **WHEN** a consumer selects Windows and both x86-64 and arm64
- **THEN** the matrix adds Windows x86-64 and Windows arm64 as jobs that run the mandatory gates

#### Scenario: Windows selected with x86-64 only

- **WHEN** a consumer selects Windows and only x86-64
- **THEN** the matrix contains a Windows x86-64 job and no Windows arm64 job

#### Scenario: Mandatory gate has no equivalent

- **WHEN** a selected OS and CPU pair has no equivalent for a mandatory gate
- **THEN** that gate fails the job and is not skipped

### Requirement: CI builds with the toolchain pin

The normal build MUST use the channel in root `rust-toolchain.toml`. The template MUST NOT add a second job that builds with a different compiler.

#### Scenario: Default toolchain

- **WHEN** `rust-version` is `1.98`
- **THEN** the build job uses toolchain `1.98` and no other compiler job is rendered

### Requirement: linecheck is always present

The template MUST always render the linecheck hook, config, and CI install. It MUST NOT offer an answer to disable linecheck. The initial config MUST warn at 800 and error at 1000. Thresholds and exclusions MUST be project-owned after the first copy; the linecheck config file MUST use `_skip_if_exists` so a later update MUST NOT overwrite them.

#### Scenario: Project is rendered

- **WHEN** a consumer generates a project
- **THEN** the linecheck hook, config, and CI install all exist, and the config starts at warn 800 and error 1000

#### Scenario: Consumer edits thresholds

- **WHEN** a consumer changes the linecheck warn or error numbers and then runs `copier update`
- **THEN** those numbers remain the consumer's values

### Requirement: Nextest config is rendered
The template MUST render a default `.auxiliary/configuration/nextest.toml` carrying no behavioral tuning, and MUST list it in `_skip_if_exists` so later copies and updates never overwrite consumer tuning. Every template nextest invocation MUST pass `--config-file` naming that path.

#### Scenario: Project is rendered

- **WHEN** a consumer generates a project
- **THEN** the config file exists with no tuning, and every nextest entry names it

#### Scenario: Consumer tunes nextest

- **WHEN** a consumer edits the config and then runs `copier update`
- **THEN** those edits remain the consumer's values
