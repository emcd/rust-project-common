# optional-features

## Purpose

Publication, fuzzing, benchmarking, property testing, and deny policy.

## Requirements

### Requirement: Publication is optional and tokenless only after registration
Publication MUST default off. The release pipeline MUST trigger on tags matching `v[0-9]+.*`, including pre-1.0 releases, and MUST offer a `dry-run` manual dispatch input defaulting to true that runs the release gates without publishing. Each selected cell MUST run its applicable release build, test, and package gates. The release test gate MUST tolerate a suite with zero tests by warning rather than failing. Exactly one non-matrix publish job MUST publish after the entire matrix succeeds; that job MUST be named `publish-crates-io`, MUST declare the `crates-io` environment, and MUST be rendered only when publication is enabled. The publish job MUST run only on tag refs with dry-run disabled. When the crate shape includes `executable` and publication is enabled, each selected cell MUST additionally build the release binary, name it deterministically, and upload it as an artifact; and exactly one non-matrix `publish-github` job MUST download all binaries, checksum them, attest their provenance, attach the binaries, the checksums, and the attestation bundle to the tag's GitHub release. On repository plans without attestation support, the binary release gate MUST fail visibly at the attest step rather than skip provenance. Publishing MUST use OpenID Connect trusted publishing and MUST NOT store a long-lived crates.io token. The template MUST document trusted-publisher registration as an operator prerequisite. It MUST NOT promise that the first publication of a new crate succeeds without a registry onboarding step. Release attestation MUST be part of the GitHub release for binary crates.

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

#### Scenario: Dry-run dispatch

- **WHEN** a consumer runs the release workflow manually with dry-run enabled
- **THEN** the release gates run and the publish job does not publish

#### Scenario: Pre-1.0 tag

- **WHEN** a consumer pushes a `v0.x` tag
- **THEN** the release pipeline runs, and publishes when publication is enabled

#### Scenario: Binary crate publishes to GitHub releases

- **WHEN** a consumer with the executable shape and publication enabled pushes a release tag
- **THEN** every selected cell uploads its release binary and the publish-github job attaches all binaries, checksums, and provenance attestation to the tag's GitHub release

#### Scenario: Repository plan without attestation support

- **WHEN** a binary release runs on a repository plan without attestation support
- **THEN** the publish-github job fails visibly at the attest step and creates no release; attestation is never silently skipped

#### Scenario: Library-only crate has no GitHub release job

- **WHEN** a consumer with only the library shape enables publication
- **THEN** the rendered releaser contains no binary-build and no publish-github jobs

### Requirement: Fuzzing, property testing, and benchmarking are separate opt-ins

Fuzzing, property-based testing, and benchmarking MUST each be a boolean and MUST default off. When property-based testing is enabled, those tests MUST run in the normal test suite and MAY run on the pre-commit test stage. When fuzzing or benchmarking is enabled, those commands MUST NOT run on the pre-commit fast path; they MUST run as pre-push hooks instead. A rendered fuzz crate MUST be excluded from the Cargo workspace.

#### Scenario: All three left off

- **WHEN** a consumer accepts the defaults
- **THEN** the rendered project contains no fuzz crate, no Criterion bench target, and no proptest dependency

#### Scenario: Fuzzing enabled

- **WHEN** a consumer enables fuzzing
- **THEN** the fuzz crate is excluded from the workspace, the pre-commit stage does not run cargo-fuzz, and a pre-push hook runs a bounded fuzz target

#### Scenario: Fuzzing without the library component

- **WHEN** a consumer enables fuzzing with executable-only shape
- **THEN** Copier rejects the combination

#### Scenario: Property testing enabled

- **WHEN** a consumer enables property-based testing
- **THEN** proptest runs as part of the normal test suite

#### Scenario: Benchmarking enabled

- **WHEN** a consumer enables benchmarking
- **THEN** a Criterion bench target is rendered, the pre-commit stage does not run it, and a pre-push hook runs it

### Requirement: cargo-deny is not implied

The template MUST NOT render cargo-deny or cargo-audit configuration. A later deny baseline MUST be a separate change.

#### Scenario: Default project

- **WHEN** a consumer generates a project from this template
- **THEN** the rendered tree contains no `deny.toml` and no cargo-audit hook
