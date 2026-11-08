## Why

The tester workflow's gate job is named `lint`, but it executes the full pre-commit stage, which includes the test suite (nextest, and doctests for library crates). Tests are not lints: the name mislabels the job's function and invites misreading of CI results. The misnaming surfaced during the Approots 0.5.0 adoption.

## What Changes

- Rename the tester job `lint` to `gates` in `template/.github/workflows/tester.yaml.jinja`. No step, matrix, dependency, or pin changes.
- Update the `quality-gates` spec wording accordingly: "lint job" becomes "gates job", and the scenario "Lint job on pull request" becomes "Gates job on pull request". Requirement semantics are unchanged.
- Update the copiertv `general.toml` bootstrap comment, which names the tester jobs as (lint, build).
- **BREAKING** for consumers that pin the job name: branch protection rules and required status checks referencing the `lint` job must be updated to `gates` when re-pinning. The rename ships with an announcement, not silently.
- Explicitly out of scope: reordering `build` and `gates` (they stay parallel; sequencing is tracked separately).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `quality-gates`: the "CI runs the same config for one stage" requirement and its pull-request scenario refer to the gate job by its new name. Behavior (same config, same stage, same matrix) is unchanged.

## Impact

- `template/.github/workflows/tester.yaml.jinja` (job id only).
- `openspec/specs/quality-gates/spec.md` (wording only; no semantic change).
- `.auxiliary/configuration/copiertv/general.toml` (comment only).
- Consumers: required-check names change on next `copier update`; announced alongside the release tag.
