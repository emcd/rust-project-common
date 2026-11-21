## ADDED Requirements

### Requirement: Apply guide states the hermetic shared-baseline rule

The application guide MUST document that shared gates are hermetic: consumer suites MUST pass under the shared gates with no ambient tools or services, and suites needing environment MUST move under an explicit non-default Cargo feature exercised in the consumer-owned custom gates, which provision the pinned dependency and fail if it is absent or the expected suite did not run. This covers the with-defaults shared doctest gates as well as the no-default-features nextest gates. The guide MUST record that the feature is opt-in for developer commands but not optional for merge/release where it proves a supported contract, that defaults need explicit validation where the minimal-feature jobs leave coverage gaps, and that the live feature MUST NOT be transitively enabled by ordinary product features.

#### Scenario: Consumer with an environmental suite reads the guide

- **WHEN** a consumer with tests needing ambient tools or services follows the application guide
- **THEN** the guide directs them to gate those tests behind an explicit non-default feature run in their custom gates, not in the shared gates

#### Scenario: Guide lists no exemption path

- **WHEN** a consumer looks for a way to excuse an environmental suite from the shared gates
- **THEN** the guide offers none: the suite is either hermetic or feature-gated into custom gates

#### Scenario: With-defaults doctests are in scope

- **WHEN** a consumer's doctests need ambient tools or services
- **THEN** the guide directs them to the same feature-gate treatment even though doctest gates run with defaults
