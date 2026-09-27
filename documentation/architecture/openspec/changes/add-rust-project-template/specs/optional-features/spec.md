## ADDED Requirements

### Requirement: Publication is optional and tokenless only after registration

Publication MUST default off. The template MUST reject publication enabled while CI is disabled. When CI is enabled, each selected cell MUST run its applicable release build, test, and package gates. Exactly one non-matrix publish job MUST publish after the entire matrix succeeds; that job MUST be rendered only when publication is enabled. Publishing MUST use OpenID Connect trusted publishing and MUST NOT store a long-lived crates.io token. The template MUST document trusted-publisher registration as an operator prerequisite. It MUST NOT promise that the first publication of a new crate succeeds without a registry onboarding step. Release attestation MUST NOT be part of this template.

#### Scenario: Publication left off

- **WHEN** a consumer accepts the publication default
- **THEN** the rendered workflows do not publish to crates.io

#### Scenario: Publication enabled

- **WHEN** a consumer enables publication with CI enabled and a release tag is pushed
- **THEN** each selected cell tests and packages first and exactly one publish job publishes with OpenID Connect and no long-lived token

#### Scenario: One matrix cell fails

- **WHEN** a consumer enables publication with CI enabled and one selected cell fails its release gate
- **THEN** no publish job publishes the crate

#### Scenario: Publication without CI

- **WHEN** a consumer enables publication and disables CI
- **THEN** Copier rejects the combination

### Requirement: Fuzz, Criterion, and proptest are separate opt-ins

Fuzz, Criterion, and proptest MUST each be a boolean and MUST default off. When proptest is enabled, those tests MUST run in the normal test suite and MAY run on the pre-commit test stage. When fuzz or Criterion is enabled, those commands MUST NOT run on the pre-commit fast path. A rendered fuzz crate MUST be excluded from the Cargo workspace.

#### Scenario: All three left off

- **WHEN** a consumer accepts the defaults
- **THEN** the rendered project contains no fuzz crate, no Criterion bench crate, and no proptest dependency

#### Scenario: Fuzz enabled

- **WHEN** a consumer enables fuzz
- **THEN** the fuzz crate is excluded from the workspace and the pre-commit stage does not run cargo-fuzz

#### Scenario: proptest enabled

- **WHEN** a consumer enables proptest
- **THEN** proptest runs as part of the normal test suite

#### Scenario: Criterion enabled

- **WHEN** a consumer enables Criterion
- **THEN** a Criterion bench target is rendered and the pre-commit stage does not run it

### Requirement: cargo-deny is not implied

The template MUST NOT render cargo-deny or cargo-audit configuration. A later deny baseline MUST be a separate change.

#### Scenario: Default project

- **WHEN** a consumer generates a project from this template
- **THEN** the rendered tree contains no `deny.toml` and no cargo-audit hook
