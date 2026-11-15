## ADDED Requirements

### Requirement: Auxiliary gitignore is rendered

The template MUST render a root-relative `.auxiliary/.gitignore` in every project ignoring `/scribbles/` and `/temporary/`, with a comment pointing at the agent instructions for the current conventions.

#### Scenario: Project is rendered

- **WHEN** a consumer generates a project
- **THEN** `.auxiliary/.gitignore` exists and ignores the scratch directories
