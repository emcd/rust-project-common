## ADDED Requirements

### Requirement: Root CHANGELOG starter is rendered

The template MUST render a root `CHANGELOG.md` in every project with the Keep-a-Changelog header and an empty `## [Unreleased]` section, and MUST list `CHANGELOG.md` in `_skip_if_exists` so later copies and updates never overwrite or merge consumer release history. The starter MUST NOT seed any version entry.

#### Scenario: Project is rendered

- **WHEN** a consumer generates a project
- **THEN** root `CHANGELOG.md` exists with the Keep-a-Changelog header and an `Unreleased` section and no version entries

#### Scenario: Consumer keeps release history

- **WHEN** a consumer writes release entries and then runs `copier update`
- **THEN** the consumer's `CHANGELOG.md` is preserved byte-identical
