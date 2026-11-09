## Context

`template/.github/workflows/releaser.yaml.jinja` renders `initialize`, per-cell `release-gates` (build, test, package), and an optional single-cell `publish` job using OIDC trusted publishing. The reference workflows (`agentmux`, `nb-mcp-server` `releaser.yaml`) structure the same outcome differently (separate `test`/`build-binaries` jobs, crates-io plus GitHub-release publish jobs) but agree exactly on the crates-io publish contract: job name `publish-crates-io`, `environment: crates-io`, `id-token: write`, `v[0-9]+.*` tags, dry-run dispatch defaulting true with publish jobs gated on `!inputs.dry-run`. The template matches none of the publish-job identity. The tester/initializer pair was audited in the same pass (see Decisions 5–6).

## Goals / Non-Goals

**Goals:**

- The template's crates-io publish job is identical to the reference contract where a counterpart exists.
- Rehearsal (dry-run) and pre-1.0 releases work the way they do in the references.

**Non-Goals:**

- No binary-artifact or GitHub-release pipeline in the template.
- No tester/initializer restructuring in this change.
- No new Copier answers: both references hardcode `crates-io`, so the template does too.

## Decisions

### 1. Fixed `crates-io` environment, not an answer

Both references hardcode the same name. A new answer would buy variance nobody has demonstrated, at the cost of another matrix dimension to validate. If a consumer ever needs a different environment name, that is a new change with evidence.

### 2. `publish` becomes `publish-crates-io`

An exact counterpart exists, so the name is identical per operator direction. The old id disappears; the breaking note and tag announcement carry the migration.

### 3. Tag trigger `v[0-9]+.*`, dry-run default true

Operator decision: follow precedent. Consequence is explicit — `v0.x` tags become live releases. The dry-run input defaults true so a manual dispatch can never publish by accident; only a tag push (or an explicit dry-run: false) reaches the publish job.

### 4. Deviations kept, with reasons

- `contents: read` stays beside `id-token: write`. The references declare `id-token` only and work because those repositories are public (checkout of a public repo needs no token scope). The template must also serve private repositories, where checkout without `contents: read` fails. Strictly safer, zero cost.
- `cargo publish --locked` stays. The template enforces `--locked` on every other Cargo invocation and bootstraps the lockfile for exactly this; dropping it only at publish time would be incoherent.
- No `publish-github` / binary jobs. A generic crate template cannot assume binaries exist (library crates have none); GitHub releases are distribution policy, not template material. `release-gates` already runs `cargo package` per cell, which is the portable packaging check.
- `release-gates` keeps its name. It merges per-cell build, test, and package — there is no counterpart under the `test` name, and relabeling it `test` would misdescribe the packaging step. Same for `initialize` (different output contract: `cells` vs `platforms`/`exclusions`/`rust-targets`) and tester `gates`/`build` (different architecture, approved by both tiers).

### 5. Tester audit verdict: architecture differs deliberately, two narrow findings

Both references run an ubuntu-only `lint` job then a matrix `test` job (`needs: [initialize, lint]`); the template runs matrix `gates` plus `build` in parallel off one pre-commit config by spec design (local/CI parity). That is a deliberate architectural difference, not a port failure, and the `gates` rename just passed both tiers. Portable items: `RUST_BACKTRACE=1` on test execution — adopted here only for the release-gates step (agentmux precedent; the pre-commit hooks are untouched to preserve hook parity and because only one of two references sets it). `cargo package` verification already runs per-cell at release time; moving it earlier would tax every commit for a release-time concern. Per-repo `nextest.toml` tuning (e.g. slow-timeout) is consumer-owned. Sequencing (`lint` before `test` in the references) stays out per operator decision, tracked in `todos/5`.

### 6. Initializer audit verdict: no port possible

The output contracts differ (`cells` vs `platforms`/`matrix-exclusions`/`rust-targets`) because the matrix models differ (cartesian OS-by-CPU cells vs platform lists with exclusions). The template's contract is spec'd behavior. No change.

### 7. Pins follow the references

`checkout@v6` and `crates-io-auth-action@v1.0.4` in rendered workflows. Pre-commit/linecheck/nextest pins are template-owned policy and untouched.

## Risks / Trade-offs

- [Publish-job rename breaks required checks] → Mitigation: breaking note plus tag announcement; direct notice to Approots, LitRPG, Praetor. No consumer uses required checks today, but the note stands.
- [`v0.x` tags become live] → Mitigation: called out in proposal and announcement; any consumer pushing `v0` tags casually must stop. This is the precedent behavior, not a new invention.
- [References drift again] → Accepted: no automated consumer-conformance check exists. A standing diff-against-references validation is future work, not this change.
