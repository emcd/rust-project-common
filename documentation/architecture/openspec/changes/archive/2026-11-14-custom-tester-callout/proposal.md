## Why

Consumers with project-specific CI gates (conformance oracles, provisioned integration tests) currently merge custom jobs into the rendered tester, which guarantees merge conflicts on every template update. A render-time optional final job calling a consumer-authored reusable workflow gives those gates a protected home that never conflicts, because the template never renders the consumer file.

## What Changes

- Rename `template/.github/workflows/core--initializer.yaml.jinja` to `common-initializer.yaml.jinja` and update both caller references. No behavior change; the `core--` prefix is retired so `--` uniformly means never-rendered.
- Add an `enable_custom_ci` boolean answer (default false). When true, the tester renders a final `custom` job that runs the consumer-authored `./.github/workflows/local--tester.yaml` reusable workflow with `needs: [initialize]` (parallel, independent verdict).
- Render-time omission: when false, the job is byte-absent from the render. There is no runtime `if:false` hiding, no auto-generated stub, and no fallback.
- The custom job is explicitly outside the mandatory gate: branch protection decides its weight, exactly like every other gate. A missing `local--tester.yaml` fails the run at workflow resolution (platform behavior); a failing custom job fails the tester run.
- Consumer code in the custom workflow runs with ordinary test permissions; the template grants it no tokens beyond checkout.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `quality-gates`: the CI requirement gains the optional custom job, its render-time contract, and its explicit exclusion from the mandatory same-config gate.

## Impact

- `template/.github/workflows/tester.yaml.jinja` (conditional custom job).
- `template/.github/workflows/{core--initializer.yaml.jinja → common-initializer.yaml.jinja}`, plus both caller references.
- `copier.yaml` (new answer).
- `.auxiliary/configuration/copiertv/general.toml` (naming-inventory assertion).
- `openspec/specs/quality-gates/spec.md` (custom-job clause plus scenario).
- Consumers: opt-in only; unanswered default renders byte-identical workflows apart from the initializer rename, which re-pin carries automatically.
