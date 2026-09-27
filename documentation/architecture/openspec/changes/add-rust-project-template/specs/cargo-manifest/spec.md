## ADDED Requirements

### Requirement: Identity and edition are answers

The template MUST ask for project name, crate name, description, authors, and license. License MUST default to Apache-2.0. Edition MUST default to 2024 and MUST remain selectable as 2021.

#### Scenario: New project accepts defaults

- **WHEN** a consumer accepts default answers
- **THEN** the rendered manifest uses edition 2024 and license Apache-2.0

#### Scenario: Retrofit selects 2021

- **WHEN** a consumer answers edition 2021
- **THEN** the rendered manifest uses edition 2021

### Requirement: MSRV and toolchain file are the same pin

The template MUST require a `rust-version` answer and MUST default it to `1.98`. It MUST always render `rust-toolchain.toml` at the repository root. The toolchain channel MUST equal `rust-version`. The template MUST NOT place that file under `.auxiliary/configuration`.

#### Scenario: Default answers

- **WHEN** a consumer accepts default answers
- **THEN** `Cargo.toml` sets `rust-version` to `1.98` and root `rust-toolchain.toml` pins channel `1.98`

#### Scenario: Consumer answers an older MSRV

- **WHEN** a consumer answers `rust-version` `1.88`
- **THEN** both the manifest and the root toolchain file use `1.88`

### Requirement: Crate shape and workspace are separate answers

The template MUST offer library, binary, or both, and MUST default to a single crate. A real workspace MUST default off. When a real workspace is selected, the template MUST emit members, a root resolver, and `[workspace.package]` inheritance. An isolated `[workspace]` table MUST be a separate boolean, MUST default off, and MUST be mutually exclusive with real-workspace mode. The template MUST NOT emit both.

#### Scenario: Default crate

- **WHEN** a consumer accepts default crate answers
- **THEN** the rendered project is one crate and has no `[workspace]` table

#### Scenario: Real workspace and isolation both requested

- **WHEN** a consumer enables both real-workspace mode and an isolated `[workspace]` table
- **THEN** Copier rejects the combination

### Requirement: Extra binaries stay project-owned

The template MUST NOT synthesize an unbounded `[[bin]]` list. The template MUST render its own bin entries from answers in a stable manifest location. Binaries added after the first copy MUST be project-owned; on update, Copier's 3-way merge MUST carry those entries forward, surfacing conflict markers on overlap rather than silently overwriting them.

#### Scenario: Consumer adds a second binary

- **WHEN** a consumer adds a `[[bin]]` entry that the template did not render and then runs `copier update`
- **THEN** that entry remains in the manifest

### Requirement: rustfmt is stable and rooted

The rendered `rustfmt.toml` MUST live at the repository root. It MUST pin `edition` and `style_edition` to the answered edition. It MUST set `max_width` to 79 and `match_block_trailing_comma` to true. It MUST NOT set `unstable_features` or any nightly-only option.

#### Scenario: Edition 2024

- **WHEN** a consumer answers edition 2024
- **THEN** root `rustfmt.toml` sets both `edition` and `style_edition` to 2024, `max_width` to 79, and does not set `unstable_features`

#### Scenario: Edition 2021

- **WHEN** a consumer answers edition 2021
- **THEN** root `rustfmt.toml` sets both `edition` and `style_edition` to 2021 and still sets `max_width` to 79
