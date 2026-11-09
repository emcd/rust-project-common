## Why

The template ships no changelog, so every consumer invents or forgoes one. The sibling projects (agentmux, nb-mcp-server, cistella) all keep a Keep-a-Changelog file with a byte-identical header and an `Unreleased` section, and Approots needs one for its 0.1.1 release. Standardize the starter in-template so new crates start with the fleet convention and release history is project-owned from the first render.

## What Changes

- Render a root `CHANGELOG.md` starter: the shared Keep-a-Changelog header plus an empty `## [Unreleased]` section. No version entries are seeded; entries are written at release time. The file is static (no template variables).
- List `CHANGELOG.md` in `_skip_if_exists`: the starter renders only when absent, and later updates never touch consumer release history.
- No changelog pipeline (towncrier or otherwise) is implied; the file is hand-edited per Keep a Changelog.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `template-layout`: the rendered tree gains the changelog starter and its update protection.

## Impact

- `template/CHANGELOG.md` (new static file).
- `copier.yaml` (`_skip_if_exists` gains one entry).
- `openspec/specs/template-layout/spec.md` (new requirement plus scenarios).
- Consumers: first re-pin after this lands renders the starter (if absent); existing changelogs are never overwritten or merged.
