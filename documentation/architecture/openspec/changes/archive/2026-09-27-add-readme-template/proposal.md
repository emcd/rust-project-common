## Why

Rendered projects have no README, unlike every Python-template consumer. The template-layout ban on documentation was meant to stop `documentation/common` duplication, not to forbid a root README.

## What Changes

- Add `template/README.md.jinja`: title, badges (CI status, license, plus crates.io version and docs.rs links when publication is on), install and usage stub, license line. Markdown, modeled on the Python `README.rst`.
- Amend template-layout to explicitly allow a root `README.md` while keeping the `documentation/` tree ban.

## Capabilities

### New Capabilities

- None. The README is rendered-project content covered by the amended capability below.

### Modified Capabilities

- `template-layout`: permit a root `README.md` with required sections; the `documentation/` tree stays banned.

## Impact

- New rendered file in every consumer project. No downstream repository is edited by this change.
- Follow-ups live in the notebook todos: README-embedded API docs (Phase 2) and Pages rustdoc plus release index (Phase 3).
