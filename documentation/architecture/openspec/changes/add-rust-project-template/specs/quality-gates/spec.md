## ADDED Requirements

### Requirement: Hooks use the pre-commit framework

The template MUST render `.auxiliary/configuration/pre-commit.yaml` for the pre-commit framework. It MUST NOT install hooks by copying shell scripts into `.git/hooks`. The pre-commit stage MUST consist of clippy with `-D warnings` on all targets, rustfmt `--check`, linecheck, and nextest.

#### Scenario: Hooks are installed

- **WHEN** a consumer installs hooks from the rendered configuration
- **THEN** the pre-commit framework runs that file and no template hook is a copied script under `.git/hooks`

### Requirement: CI runs the same config for one stage

When CI is enabled, the lint job MUST invoke the rendered pre-commit configuration and MUST run only the pre-commit stage. It MUST NOT run pre-merge, pre-push, or live hooks. The workflow MUST pin pre-commit, linecheck, and nextest, and MUST install project hook dependencies before the gate. The test runner in that shared config MUST be nextest. That stage is the mandatory gate on every selected OS by CPU pair, except a hook scoped out as genuinely inapplicable.

#### Scenario: CI defaults on

- **WHEN** a consumer accepts default answers
- **THEN** CI is enabled and the lint job runs the pre-commit stage of `.auxiliary/configuration/pre-commit.yaml`

#### Scenario: Lint job on pull request

- **WHEN** CI is enabled and a pull request runs the lint job
- **THEN** the job runs the pre-commit stage of `.auxiliary/configuration/pre-commit.yaml` and does not run pre-push hooks

#### Scenario: CI declined

- **WHEN** a consumer disables CI
- **THEN** the rendered project contains no GitHub Actions workflows from this template

### Requirement: Selected OS and CPU pairs are real jobs

Operating system and CPU architecture MUST be separate answers. Linux and macOS, and both x86-64 and arm64, MUST be selected by default. Windows MUST default off. CI MUST default on and MAY be declined. When CI is enabled, the workflow matrix MUST be the cartesian product of the selected values, the initializer MUST emit only selected pairs, and tester and releaser MUST consume those outputs. When CI is enabled, the template MUST render initializer, tester, and releaser workflows; the releaser MUST publish to crates.io only when publication is enabled. When CI is enabled, release build, test, and package gates MUST run in each selected cell; publication MUST be a single non-matrix publish job that depends on the success of the entire matrix and MUST be rendered only when publication is enabled. When Windows is selected and CI is enabled, a Windows cell MUST exist for each selected CPU architecture and for no other. When CI is enabled, each selected pair MUST run its applicable build, test, and mandatory quality gates; a platform-specific hook MAY be scoped out only where it is genuinely inapplicable; an unsupported mandatory step MUST fail visibly; and the template MUST NOT emit a pair and then exclude it from those gates.

#### Scenario: Default platforms

- **WHEN** a consumer accepts default platform answers with CI enabled
- **THEN** the matrix contains Linux and macOS on both x86-64 and arm64, and contains no Windows job

#### Scenario: Windows selected with both CPUs

- **WHEN** a consumer selects Windows and both x86-64 and arm64 with CI enabled
- **THEN** the matrix adds Windows x86-64 and Windows arm64 as jobs that run the mandatory gates

#### Scenario: Windows selected with x86-64 only

- **WHEN** a consumer selects Windows and only x86-64 with CI enabled
- **THEN** the matrix contains a Windows x86-64 job and no Windows arm64 job

#### Scenario: Mandatory gate has no equivalent

- **WHEN** CI is enabled and a selected OS and CPU pair has no equivalent for a mandatory gate
- **THEN** that gate fails the job and is not skipped

### Requirement: CI builds with the toolchain pin

When CI is enabled, the normal build MUST use the channel in root `rust-toolchain.toml`. The template MUST NOT add a second job that builds with a different compiler.

#### Scenario: CI enabled

- **WHEN** CI is enabled and `rust-version` is `1.98`
- **THEN** the build job uses toolchain `1.98` and no other compiler job is rendered

### Requirement: linecheck is always present

The template MUST always render the linecheck hook and config. It MUST NOT offer an answer to disable linecheck. When CI is enabled, the template MUST also render the linecheck CI install. The initial config MUST warn at 800 and error at 1000. Thresholds and exclusions MUST be project-owned after the first copy; the linecheck config file MUST use `_skip_if_exists` so a later update MUST NOT overwrite them.

#### Scenario: Project rendered with CI on

- **WHEN** a consumer generates a project with CI enabled
- **THEN** the linecheck hook, config, and CI install all exist, and the config starts at warn 800 and error 1000

#### Scenario: Project rendered with CI off

- **WHEN** a consumer generates a project with CI disabled
- **THEN** the linecheck hook and config exist and no linecheck CI install is rendered

#### Scenario: Consumer edits thresholds

- **WHEN** a consumer changes the linecheck warn or error numbers and then runs `copier update`
- **THEN** those numbers remain the consumer's values
