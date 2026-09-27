# template-layout

## Purpose

Copier repository layout, render scope, exclusions, and validation.

## Requirements

### Requirement: Copier root matches the Python template shape

The template repository MUST keep `copier.yaml` at the repository root. That file MUST set `_subdirectory` to `template` and `_templates_suffix` to `.jinja`. Rendered project files MUST live under `template/`.

#### Scenario: Template is copied

- **WHEN** a consumer runs Copier against this repository
- **THEN** Copier reads root `copier.yaml` and renders only the `template/` tree into the consumer root

### Requirement: Documentation and PyO3 are absent

The rendered project MUST NOT contain a documentation tree from this template. The template MUST NOT offer a PyO3 or Maturin extension switch.

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
