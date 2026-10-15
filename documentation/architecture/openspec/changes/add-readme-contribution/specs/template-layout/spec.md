## MODIFIED Requirements

### Requirement: Root README is rendered

The template MUST render a root `README.md` in every project with a title, CI status and license badges, a usage stub, and a license line. The template MUST ask for a GitHub repository owner defaulting to `emcd`. The CI status badge MUST target `https://github.com/<owner>/<project>/actions/workflows/tester.yaml` and the license badge MUST derive from the answered license. When publication is enabled, crates.io version and docs.rs badges MUST also render. The `## Installation` section MUST render only when publication is enabled: registry install notes for the selected components. An `Additional Badges` section MUST carry last-commit, Copier, and pre-commit badges, plus a crates.io downloads badge when publication is enabled. A `Contribution` section MUST link the code of conduct and the contribution guide on the floating documentation-common `v1` line, never bare versions.

#### Scenario: Default project README

- **WHEN** a consumer generates a project with default answers
- **THEN** root `README.md` exists with title, CI status and license badges, usage stub, additional badges, contribution links, and license line, and no Installation section or crates.io badges

#### Scenario: Custom GitHub owner

- **WHEN** a consumer answers a GitHub owner other than the default
- **THEN** the CI status badge targets that owner's repository URL

#### Scenario: Publication enabled README

- **WHEN** a consumer enables publication
- **THEN** root `README.md` additionally carries crates.io version and docs.rs badges plus an Installation section with registry install notes
