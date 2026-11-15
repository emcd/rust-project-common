## Why

The template renders no `.auxiliary/.gitignore`, so scratch work under the fleet's conventional locations (`.auxiliary/scribbles/`, `.auxiliary/temporary/`) shows up as untracked noise — or worse, gets committed — in every consumer. Python-project-common already ships one (with Python-specific entries that do not apply to Rust); the Rust template should ship the two fleet locations and nothing language-specific.

## What Changes

- Render a static `template/.auxiliary/.gitignore` ignoring `/scribbles/` and `/temporary/`, with a comment pointing at the agent instructions.
- The file is intentionally NOT in `_skip_if_exists`: ignore files are additive, and Copier's three-way merge carries consumer-added lines forward cleanly. Skip-if-exists would freeze the file forever; merge keeps it alive. Conflicts resolve keeping both sides.
- Mirror the same two lines in this repository's own `.auxiliary/.gitignore`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `template-layout`: the rendered tree gains the auxiliary gitignore.

## Impact

- `template/.auxiliary/.gitignore` (new static file).
- `.auxiliary/.gitignore` (this repo's own hygiene; same two lines).
- `openspec/specs/template-layout/spec.md` (rendered-tree requirement).
- Consumers: scratch dirs stop showing as untracked on next update; no conflicts possible (new file, previously unmanaged path).
