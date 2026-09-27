## MODIFIED Requirements

### Requirement: Documentation and PyO3 are absent

The rendered project MUST NOT contain a documentation tree from this template. The template MUST NOT offer a PyO3 or Maturin extension switch. A root `README.md` is permitted and is not a documentation tree.

#### Scenario: New project is rendered

- **WHEN** a consumer generates a project with default answers
- **THEN** the rendered tree contains no documentation directory and no PyO3 or Maturin manifest entries

## ADDED Requirements

### Requirement: Root README is rendered

The template MUST render a root `README.md` in every project with a title, CI status and license badges, an install and usage stub, and a license line. The template MUST ask for a GitHub repository owner defaulting to `emcd`. The CI status badge MUST target `https://github.com/<owner>/<project>/actions/workflows/tester.yaml` and the license badge MUST derive from the answered license. When publication is enabled, crates.io version and docs.rs badges MUST also render.

#### Scenario: Default project README

- **WHEN** a consumer generates a project with default answers
- **THEN** root `README.md` exists with title, CI status and license badges, install and usage stub, and license line, and no crates.io badges

#### Scenario: Custom GitHub owner

- **WHEN** a consumer answers a GitHub owner other than the default
- **THEN** the CI status badge targets that owner's repository URL

#### Scenario: Publication enabled README

- **WHEN** a consumer enables publication
- **THEN** root `README.md` additionally carries crates.io version and docs.rs badges
