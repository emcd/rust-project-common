# template-layout

## Purpose

Copier repository layout, render scope, exclusions, and validation.

## Requirements

### Requirement: Copier root matches the Python template shape
The template repository MUST keep `copier.yaml` at the repository root. That file MUST set `_subdirectory` to `template` and `_templates_suffix` to `.jinja`. Rendered project files MUST live under `template/`. The template MUST ship the answers-file template so copies record answers for updates.

#### Scenario: Template is copied

- **WHEN** a consumer runs Copier against this repository
- **THEN** Copier reads root `copier.yaml` and renders only the `template/` tree into the consumer root

#### Scenario: Answers are recorded

- **WHEN** a consumer copies the template
- **THEN** the answers file is written into the rendered tree with all answers serialized

### Requirement: Documentation and PyO3 are absent

The rendered project MUST NOT contain a documentation tree from this template. The template MUST NOT offer a PyO3 or Maturin extension switch. A root `README.md` is permitted and is not a documentation tree.

#### Scenario: New project is rendered

- **WHEN** a consumer generates a project with default answers
- **THEN** the rendered tree contains no documentation directory and no PyO3 or Maturin manifest entries

### Requirement: Validation renders an answer matrix instead of dogfooding

This repository MUST NOT commit a rendered dummy project. The template MUST be validated with `copiertv validate` over a fixed answer matrix: default answers, all-opt-in answers, and executable-only shape. Every render MUST satisfy the specs in this change. Each variant file MUST set every answer explicitly, because the validator renders the template directory directly and question defaults do not apply.

#### Scenario: Answer matrix renders cleanly

- **WHEN** a maintainer renders the fixed answer matrix against the working tree
- **THEN** every render completes and each rendered tree satisfies these specs

#### Scenario: No dummy project is committed

- **WHEN** a maintainer validates the template
- **THEN** no rendered project is committed to this repository

### Requirement: Root README is rendered
The template MUST render a root `README.md` in every project with a title, CI status and license badges, a usage stub, and a license line. The template MUST ask for a GitHub repository owner defaulting to `emcd`. The CI status badge MUST target `https://github.com/<owner>/<project>/actions/workflows/tester.yaml` and the license badge MUST derive from the answered license. When publication is enabled, crates.io version and docs.rs badges MUST also render. The `## Installation` section MUST render only when publication is enabled: registry install notes for the selected components. An `Additional Badges` section MUST carry last-commit, Copier, and pre-commit badges, plus a crates.io downloads badge when publication is enabled. A `Contribution` section MUST link the code of conduct and the contribution guide on the floating documentation-common `v1` line, never bare versions, plus the issue tracker and pull requests derived from the answered GitHub owner and project name.

#### Scenario: Default project README

- **WHEN** a consumer generates a project with default answers
- **THEN** root `README.md` exists with title, CI status and license badges, usage stub, additional badges, contribution links with issue tracker and pull requests, and license line, and no Installation section or crates.io badges

#### Scenario: Custom GitHub owner

- **WHEN** a consumer answers a GitHub owner other than the default
- **THEN** the CI status badge targets that owner's repository URL

#### Scenario: Publication enabled README

- **WHEN** a consumer enables publication
- **THEN** root `README.md` additionally carries crates.io version and docs.rs badges plus an Installation section with registry install notes

### Requirement: EditorConfig is rendered
The template MUST render a root `.editorconfig` in every project with fleet whitespace rules: utf-8, lf endings, 4-space indent, final newline, trimmed trailing whitespace, 79-col limit for Markdown and Rust sources, indent 2 for TOML and YAML.

#### Scenario: Project is rendered

- **WHEN** a consumer generates a project
- **THEN** root `.editorconfig` exists with the fleet rules
