## MODIFIED Requirements

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
