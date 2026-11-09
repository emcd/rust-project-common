## Context

`copier.yaml` `_skip_if_exists` currently protects the linecheck config, `src/bin/<crate>.rs`, and `src/lib.rs`: those paths render only when absent and are never merged on update. A changelog has the same ownership shape — template-seeded, consumer-written thereafter — and the three reference projects use a byte-identical Keep-a-Changelog header, so the starter carries no project-specific content to parameterize.

## Goals / Non-Goals

**Goals:**

- Every new render starts with the fleet changelog convention.
- Consumer release history is never clobbered or merged by later updates.

**Non-Goals:**

- No generated entries, no version seeding, no changelog tooling or CI checks.
- No changelog for the template repository itself in this change.

## Decisions

### 1. Static file, not a Jinja template

The starter contains no project name, version, or date — the references demonstrate that the header is identical everywhere. A static file cannot drift from answers and needs no render test beyond byte comparison.

### 2. `_skip_if_exists`, not mergeable content

A changelog is append-only consumer history; a three-way merge on update could interleave or conflict with release notes. Skip-if-exists gives the strongest guarantee (preserved as-is, per the applying guide) and matches the already-protected implemented-source precedent.

### 3. Unreleased only, no 0.1.0 seed

Seeding a version number and date would assert a release that never happened. The entry shape (`## [X.Y.Z] - YYYY-MM-DD` with `### Added/Changed/Fixed` subsections) is established by convention and visible in every sibling file; the first real release writes the first entry.

## Risks / Trade-offs

- [Consumers with an existing changelog at another path] → Accepted: the starter renders only when root `CHANGELOG.md` is absent; a differently-pathed file is untouched and the consumer keeps it (two changelogs would be their explicit situation, not template damage).
- [Template can never evolve the header] → Accepted: `_skip_if_exists` is one-way by design; a future header change would need a new mechanism, same as the protected sources.
