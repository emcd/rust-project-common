## ADDED Requirements

### Requirement: Identity and edition are answers

The template MUST ask for project name, crate name, description, authors, and license. License MUST default to Apache-2.0. Crate name MUST default to the project name with hyphens preserved. Edition MUST be 2024; the template MUST NOT offer an edition question.

#### Scenario: New project accepts defaults

- **WHEN** a consumer accepts default answers
- **THEN** the rendered manifest uses edition 2024 and license Apache-2.0

#### Scenario: Hyphenated project name

- **WHEN** a consumer names the project `my-crate` and accepts the crate-name default
- **THEN** the rendered manifest names the package and binary `my-crate` and the `[lib]` target `my_crate`, because library target names cannot contain hyphens

### Requirement: MSRV and toolchain file are the same pin

The template MUST require a `rust-version` answer and MUST default it to `1.98`. It MUST always render `rust-toolchain.toml` at the repository root. The toolchain channel MUST equal `rust-version`. The template MUST NOT place that file under `.auxiliary/configuration`.

#### Scenario: Default answers

- **WHEN** a consumer accepts default answers
- **THEN** `Cargo.toml` sets `rust-version` to `1.98` and root `rust-toolchain.toml` pins channel `1.98`

#### Scenario: Consumer answers an older MSRV

- **WHEN** a consumer answers `rust-version` `1.88`
- **THEN** both the manifest and the root toolchain file use `1.88`

### Requirement: Crate shape and workspace are separate answers

The template MUST offer library and executable as a multi-choice answer defaulting to library, and MUST require at least one. Workspace mode MUST be a single choice of none, real, or isolated, defaulting to none. When real-workspace mode is selected, the template MUST emit members, a root resolver, and `[workspace.package]` inheritance. An isolated `[workspace]` table escapes an ancestor workspace.

#### Scenario: Default crate

- **WHEN** a consumer accepts default crate answers
- **THEN** the rendered project is one library crate and has no `[workspace]` table

#### Scenario: No shape selected

- **WHEN** a consumer selects neither library nor executable
- **THEN** Copier rejects the combination

### Requirement: Extra binaries stay project-owned

The template MUST NOT synthesize an unbounded `[[bin]]` list. The template MUST render its own bin entries from answers in a stable manifest location. Binaries added after the first copy MUST be project-owned; on update, Copier's 3-way merge MUST carry those entries forward, surfacing conflict markers on overlap rather than silently overwriting them.

#### Scenario: Consumer adds a second binary

- **WHEN** a consumer adds a `[[bin]]` entry that the template did not render and then runs `copier update`
- **THEN** that entry remains in the manifest

### Requirement: rustfmt is stable and rooted

The rendered `rustfmt.toml` MUST live at the repository root. It MUST pin `edition` and `style_edition` to 2024. It MUST set `max_width` to 79 and `match_block_trailing_comma` to true. It MUST NOT set `unstable_features` or any nightly-only option.

#### Scenario: Project is rendered

- **WHEN** a consumer generates a project
- **THEN** root `rustfmt.toml` sets both `edition` and `style_edition` to 2024, `max_width` to 79, and does not set `unstable_features`
