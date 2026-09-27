## Context

Consumers need a front door: title, health badges, install path, license. The Python template's `README.rst` is the model, translated to Markdown for the Rust ecosystem (crates.io renders `README.md`). The root README is explicitly not a documentation tree, so the ban on `documentation/` stands untouched.

## Goals / Non-Goals

**Goals:**

- One `README.md` in every rendered project with title, badges, install/usage stub, and license line.
- Badges reflect answers: crates.io version and docs.rs links only when publication is on; CI and license badges always.

**Non-Goals:**

- A `documentation/` tree, Sphinx, mdbook, or a Pages site (Phase 3).
- `#![doc = include_str!]` embedding (Phase 2, needs real API surface).
- Release-index or download-link machinery.

## Decisions

### 1. Markdown modeled on the Python README

Sections follow `README.rst`: license-header-free title (Markdown has no comment convention worth mimicking here), badges, install, usage, license. Conditional blocks for publication-gated badges and binary install instructions mirror the Python conditionals.

### 2. Root file, no copier conditional path

`README.md` renders unconditionally as `template/README.md.jinja`. Its interior uses answer conditionals; the file itself always exists.

## Risks / Trade-offs

- [Badge URLs drift (shields, GitHub, docs.rs)] → Standard public URL shapes; projects own them after copy.
- [README content rots as features change] → Accepted. The install/usage body is a stub with a todo marker, same as the Python template.

## Migration Plan

Land the template file and the spec allowance (one MODIFIED requirement, one ADDED requirement), re-validate the answer matrix, tag. No downstream edits.

## Open Questions

None.
