## MODIFIED Requirements

### Requirement: CI runs the same config for one stage

The gates job MUST invoke the rendered pre-commit configuration with an explicit `--config` path and MUST run only the pre-commit stage. It MUST NOT run pre-merge, pre-push, or live hooks. The workflow MUST pin pre-commit, linecheck, and nextest, and MUST install project hook dependencies before the gate. The suite runner in that shared config MUST be nextest; a doctest-only runner MAY accompany it. That stage is the mandatory gate on every selected OS by CPU pair, except a hook scoped out as genuinely inapplicable.

#### Scenario: Gates job on pull request

- **WHEN** a pull request runs the gates job
- **THEN** the job runs the pre-commit stage of `.auxiliary/configuration/pre-commit.yaml` with an explicit `--config` argument and does not run pre-push hooks

#### Scenario: Rendered workflows pass the template's own gates

- **WHEN** a consumer renders the template
- **THEN** every rendered file already satisfies the hygiene hooks with no fixer rewrites
