## MODIFIED Requirements

### Requirement: Copier root matches the Python template shape

The template repository MUST keep `copier.yaml` at the repository root. That file MUST set `_subdirectory` to `template` and `_templates_suffix` to `.jinja`. Rendered project files MUST live under `template/`. The template MUST ship the answers-file template so copies record answers for updates.

#### Scenario: Template is copied

- **WHEN** a consumer runs Copier against this repository
- **THEN** Copier reads root `copier.yaml` and renders only the `template/` tree into the consumer root

#### Scenario: Answers are recorded

- **WHEN** a consumer copies the template
- **THEN** the answers file is written into the rendered tree with all answers serialized
