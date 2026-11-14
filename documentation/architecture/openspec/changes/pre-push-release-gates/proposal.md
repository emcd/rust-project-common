## Why

Release-only breakage (optimizer-sensitive code, release-only cfgs, packaging faults) currently reaches CI before any local gate sees it: pre-commit hooks run the dev profile, and the template's pre-push stage carries only opt-in fuzz and bench. Every consumer re-discovers this at release time. A pre-push release twin (build, test, doctest in release) catches it at push time, extending the parity principle per stage instead of breaking it.

## What Changes

- Render three always-on pre-push hooks: release build (`cargo build --release --locked --all-targets`), release nextest (`cargo nextest run --release --locked --no-tests=warn`, verbose so the zero-test warning stays visible), and release doctests (`cargo test --doc --release --locked`, library shape only), each with the lockfile bootstrap.
- Add the release doctest step to CI release-gates (library shape only), so hook and CI run the identical release suite — parity holds verbatim, not by analogy.
- Pre-commit stays exactly as is (dev profile, fast feedback). No hook type is added; pre-push is already an installed stage.
- Pushes get slower (full release compile plus suite per push). That is the accepted, explicit cost: slower pushes for fewer red CIs. `--no-verify` remains the escape hatch.
- No publish credentials, publishing, or tag mutation in hooks, ever.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `quality-gates`: the Hooks requirement gains the pre-push release twins; the pre-push stage is no longer fuzz-and-bench-only.

## Impact

- `template/.auxiliary/configuration/pre-commit.yaml.jinja` (three new hooks).
- `openspec/specs/quality-gates/spec.md` (Hooks requirement wording plus release-gate-parity scenario).
- Consumers: pushes run release gates locally from the next update; no configuration needed, no opt-out besides the standard bypass.
