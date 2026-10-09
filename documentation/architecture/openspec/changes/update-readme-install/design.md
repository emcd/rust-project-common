## Context

Badges and install notes must be true on first copy. Registry commands require publication; path assumptions require a layout we cannot guarantee. The Python README's badge tail (last-commit, Copier, pre-commit) ports directly; Hatch/pyright/ruff badges have no Rust equivalents worth inventing.

## Goals / Non-Goals

**Goals:**

- Installation section exists if and only if publication is on.
- Badge set: top row unchanged; `Additional Badges` carries last-commit, Copier, pre-commit always, crates.io downloads when published.

**Non-Goals:**

- `Other Projects by This Author` (waits on the operator's project list).
- `Contribution` section (waits on documentation-common factoring).

## Decisions

### 1. Drop instead of stubbing

No Installation section when unpublished, rather than a todo placeholder. A section that cannot say anything true should not exist.

## Risks / Trade-offs

- [Consumers adding unpublished install notes by hand] → Accepted; project-owned after copy.

## Migration Plan

Land the template edit and the spec touch, re-validate the matrix, tag. No downstream edits.

## Open Questions

None.
