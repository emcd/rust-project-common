## ADDED Requirements

### Requirement: EditorConfig is rendered

The template MUST render a root `.editorconfig` in every project with fleet whitespace rules: utf-8, lf endings, 4-space indent, final newline, trimmed trailing whitespace, 79-col limit for Markdown and Rust sources, indent 2 for TOML and YAML.

#### Scenario: Project is rendered

- **WHEN** a consumer generates a project
- **THEN** root `.editorconfig` exists with the fleet rules
