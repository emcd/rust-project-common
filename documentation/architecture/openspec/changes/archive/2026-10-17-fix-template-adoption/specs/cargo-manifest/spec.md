## MODIFIED Requirements

### Requirement: Extra binaries stay project-owned

The template MUST NOT synthesize an unbounded `[[bin]]` list. The template MUST render its executable target as `src/bin/<crate-name>.rs`, never `src/main.rs`, assuming multiple executables from the beginning. The template MUST render its own bin entries from answers in a stable manifest location. Binaries added after the first copy MUST be project-owned; on update, Copier's 3-way merge MUST carry those entries forward, surfacing conflict markers on overlap rather than silently overwriting them.

#### Scenario: Consumer adds a second binary

- **WHEN** a consumer adds a `[[bin]]` entry that the template did not render and then runs `copier update`
- **THEN** that entry remains in the manifest

#### Scenario: Multi-bin project updates

- **WHEN** a consumer with additional binaries under `src/bin/` runs `copier update`
- **THEN** no `src/main.rs` appears and the default `cargo run` target is unchanged
