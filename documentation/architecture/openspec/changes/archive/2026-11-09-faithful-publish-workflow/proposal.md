## Why

The template's release workflow is not a faithful port of the consumer workflows it was supposed to follow (agentmux, nb-mcp-server), and the divergence is load-bearing: the publish job omits `environment: crates-io`, so OIDC trusted publishing fails against a publisher record that names the environment. Approots hit this attempting publication. Further gaps in the same job: the tag trigger `v[1-9]+.*` silently ignores pre-1.0 releases, there is no dry-run rehearsal path, and action pins lag the established precedent. The releaser was authored fresh in-template rather than ported, and no process step diffs template output against the reference workflows — so nothing could catch the drift.

## What Changes

- Rename the publish job `publish` to `publish-crates-io` and declare `environment: crates-io`, matching both reference workflows exactly.
- Widen the release tag trigger from `v[1-9]+.*` to `v[0-9]+.*` (operator decision: follow precedent).
- Add a `dry-run` boolean dispatch input (default true); the publish job runs only when `!inputs.dry-run` on a tag ref. Release gates always run, so dry-run still exercises the full build/test/package path.
- Follow precedent pins in rendered workflows: `actions/checkout@v6`, `rust-lang/crates-io-auth-action@v1.0.4`.
- Add top-level `env: CARGO_TERM_COLOR: always` and `RUST_BACKTRACE: '1'` on the release test step, per agentmux.
- **BREAKING**: the publish job id changes (`publish` to `publish-crates-io`); required checks referencing it must migrate on re-pin. Widening the tag trigger means pushing a `v0.x` tag now runs the release pipeline (and publishes, when enabled) instead of being ignored.
- Deliberate deviations kept (each with reason in design): `contents: read` retained alongside `id-token: write` (consumers are public repos; the template must also serve private ones where checkout needs the scope); `cargo publish --locked` retained (template-wide lockfile enforcement); no binary-artifact or GitHub-release jobs (lib crates have no binaries to attach); `release-gates` keeps its name (it merges per-cell build, test, and package — no counterpart job exists under the `test` name).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `optional-features`: the publication requirement now pins the release trigger, dry-run behavior, publish job identity (`publish-crates-io`), and the `crates-io` environment binding.

## Impact

- `template/.github/workflows/releaser.yaml.jinja` (trigger, dispatch, env, job identity, pins, test-step env).
- `template/.github/workflows/tester.yaml.jinja` (checkout pin only).
- `openspec/specs/optional-features/spec.md` (publication requirement wording plus two scenarios).
- Consumers: publish-job check names change on re-pin; `v0.x` tags become live.
