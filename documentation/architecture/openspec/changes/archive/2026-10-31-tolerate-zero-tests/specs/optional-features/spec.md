## MODIFIED Requirements

### Requirement: Publication is optional and tokenless only after registration

Publication MUST default off. Each selected cell MUST run its applicable release build, test, and package gates. The release test gate MUST tolerate a suite with zero tests by warning rather than failing. Exactly one non-matrix publish job MUST publish after the entire matrix succeeds; that job MUST be rendered only when publication is enabled. Publishing MUST use OpenID Connect trusted publishing and MUST NOT store a long-lived crates.io token. The template MUST document trusted-publisher registration as an operator prerequisite. It MUST NOT promise that the first publication of a new crate succeeds without a registry onboarding step. Release attestation MUST NOT be part of this template.

#### Scenario: Publication left off

- **WHEN** a consumer accepts the publication default
- **THEN** the rendered workflows do not publish to crates.io

#### Scenario: Publication enabled

- **WHEN** a consumer enables publication and a release tag is pushed
- **THEN** each selected cell tests and packages first and exactly one publish job publishes with OpenID Connect and no long-lived token

#### Scenario: One matrix cell fails

- **WHEN** a consumer enables publication and one selected cell fails its release gate
- **THEN** no publish job publishes the crate

#### Scenario: Release test gate with no tests

- **WHEN** a release build runs against a crate with no tests yet
- **THEN** the release test gate warns about zero tests and passes instead of failing
