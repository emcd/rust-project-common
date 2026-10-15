## Context

Documentation Owner confirms both targets live (verified 200): conduct and contribution pages under `https://emcd.github.io/documentation-common/v1/sphinx-html/common/`. Pinning guidance: float `v1`, never bare versions; anchor IDs are ADR 003 contract within the docs-major line.

## Goals / Non-Goals

**Goals:**

- Contribution section in every rendered README with CoC and guide links on the floating `v1` line.

**Non-Goals:**

- Shipping documentation trees (ban stands).
- Release index or Pages rustdoc (Phase 3).

## Decisions

### 1. Float v1, link out only

URLs pin the `v1` line per guidance. No vendored content, so the tree ban is untouched.

## Risks / Trade-offs

- [Links rot if the v1 line moves] → Accepted; stable-line contract plus compat retention is Documentation Owner's guarantee.

## Migration Plan

Land the template edit and spec touch, re-validate the matrix, tag. No downstream edits.

## Open Questions

None.
