## Why

The Python template ships an `.editorconfig`; the Rust template does not. Editor-level whitespace and line-length consistency should not depend on pre-commit hooks alone.

## What Changes

- Add static `template/.editorconfig`, modeled on the Python template minus its Python/RST sections.
- Amend template-layout with an ADDED EditorConfig requirement.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- None. New ADDED requirement in existing `template-layout` spec.

## Impact

- One new rendered file in every consumer project. Consistent with the existing `rustfmt.toml` (79 cols) and pre-commit fixers. No downstream edits.
