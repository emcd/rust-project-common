## Context

pre-commit supports a `pre-merge-commit` stage running on `git merge`. The template's hygiene hooks (no explicit stages) inherit installed hook types automatically; only the four local hooks with explicit `stages: [ 'pre-commit' ]` need extending. Pre-push compile/test gates stay push-only: merges should be fast.

## Goals / Non-Goals

**Goals:**

- Merge commits run clippy, fmt check, linecheck, and nextest.

**Non-Goals:**

- Running pre-push gates (fuzz, bench, release build) at merge time.
- Agentmux-specific flock serialization and libghostty-vt lint decoupling (not portable to the generic template).

## Decisions

### 1. Extend stages, don't duplicate hooks

One hook entry, two stages — pre-commit framework handles dispatch.

## Risks / Trade-offs

- [Slower merges] → Accepted; these are the fast gates, and a dark gate is worse.

## Migration Plan

Land, re-validate, tag. Consumers pick it up on next update (hook reinstall picks up the new type).

## Open Questions

None.
