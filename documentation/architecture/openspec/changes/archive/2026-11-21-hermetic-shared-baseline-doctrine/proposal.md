## Why

The nbspec retrofit exposed a contract gap: shared xrepo test gates run selected tests, but a suite with an ambient environmental dependency (a live `nb` binary) fails there — and the shared lane does not provision consumer-specific dependencies by design. A four-way with the NbApi Owner, Nbspec Owner, and Quasar settled the doctrine (hermetic shared baseline, explicitly selected live suites in project-owned gates). Codify it in the consumer guide now so future retrofits do not rediscover it one red matrix at a time.

## What Changes

- New Principle in `documentation/applying-template-updates.md`: shared gates are hermetic. Consumer suites MUST pass under the shared gates with no ambient tools or services; suites needing environment move under an explicit non-default Cargo feature exercised in consumer-owned custom gates (`local--tester.yaml` / `local--releaser.yaml`), which provision the pinned dependency and fail if it is absent or the expected suite did not run.
- Mechanism as built (the guide states this plainly, not as a full-suite-or-suppress-defaults story): shared nextest gates run with no default features; shared library doctest gates (dev and release) run WITH defaults, as does local parity — the rule covers those doctests too. The guide calls for explicit additional defaults validation for coverage the minimal-feature jobs do not provide.
- The Principle records the settled qualifications: a feature makes a suite opt-in for a developer command, not optional for merge/release when it proves a supported contract; bare `cargo test` stays fast and hermetic; one documented maintainer command runs full validation on the same invocation as the project gates; defaults are validated explicitly where they differ from the `--no-default-features` shared leg; live suites are gated as a module/unit with visible rosters (split mixed modules); the live feature MUST NOT be transitively enabled by ordinary product features.
- Verification expectation: both legs (hermetic shared matrix, live suite with pinned dependency) on the same SHA before the retrofit is called done.
- No template rendering changes, no workflow changes, no new shared inputs, no exemptions. Guidance only; nothing breaking.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `template-layout`: one ADDED docs-content requirement pinning the guide text (the guide MUST state the hermetic rule with its qualifications). No behavior changes; `quality-gates` requirements are unchanged.

## Impact

- `documentation/applying-template-updates.md` (new Principle) plus one `template-layout` spec delta pinning the guide text.
- Readers: nbspec (the live instance, already implementing) and all future consumers.
- No CI, workflow, template-render, or gate-behavior changes.
