## ADDED Requirements

### Requirement: Hooks use the pre-commit framework

The template MUST render `.auxiliary/configuration/pre-commit.yaml` for the pre-commit framework. It MUST NOT install hooks by copying shell scripts into `.git/hooks`. The pre-commit stage MUST consist of clippy with `-D warnings` on all targets, rustfmt `--check`, linecheck, nextest, and the standard file-hygiene hooks: large-file, filename-case-conflict, merge-conflict-marker, symlink, VCS-permalink, destroyed-symlink, private-key, mixed-line-ending, JSON, TOML, and YAML checks, plus end-of-file and trailing-whitespace fixers. When fuzzing is enabled, a bounded `cargo fuzz` run MUST be a pre-push hook. When benchmarking is enabled, a `cargo bench` run MUST be a pre-push hook.

#### Scenario: Hooks are installed

- **WHEN** a consumer installs hooks from the rendered configuration
- **THEN** the pre-commit framework runs that file and no template hook is a copied script under `.git/hooks`

### Requirement: CI runs the same config for one stage

The lint job MUST invoke the rendered pre-commit configuration and MUST run only the pre-commit stage. It MUST NOT run pre-merge, pre-push, or live hooks. The workflow MUST pin pre-commit, linecheck, and nextest, and MUST install project hook dependencies before the gate. The test runner in that shared config MUST be nextest. That stage is the mandatory gate on every selected OS by CPU pair, except a hook scoped out as genuinely inapplicable.

#### Scenario: Lint job on pull request

- **WHEN** a pull request runs the lint job
- **THEN** the job runs the pre-commit stage of `.auxiliary/configuration/pre-commit.yaml` and does not run pre-push hooks

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
