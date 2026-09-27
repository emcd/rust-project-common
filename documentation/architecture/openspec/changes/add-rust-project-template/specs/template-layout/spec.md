## ADDED Requirements

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

### Requirement: Dogfood uses an explicit template tag

This repository MUST be a consumer of its own template. The answers file MUST record `_commit` as an explicit template tag. A dogfood update MUST run `copier update` with `--vcs-ref` set to that template tag, from a clean consumer tree, after the tag is pushed. The repository MUST NOT add a second tag namespace that Copier could select when `--vcs-ref` is omitted. Template edits MUST NOT be hand-mirrored into the rendered tree.

#### Scenario: Maintainer updates this repository

- **WHEN** a maintainer changes files under `template/` and dogfoods this repository
- **THEN** the update uses the pushed template tag as `--vcs-ref` and Copier rewrites `_commit` to that tag

#### Scenario: Tag is not pushed

- **WHEN** a maintainer runs `copier update` before the template tag is pushed
- **THEN** the update does not see the unpushed template commit and MUST NOT be treated as a successful dogfood
