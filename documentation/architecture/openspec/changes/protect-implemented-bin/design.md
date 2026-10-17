## Context

`_skip_if_exists` accepts Jinja-templated paths (proven: pre-existing consumer `main` survives `--overwrite`; absent files still render). This is the same mechanism already used for linecheck thresholds.

## Goals / Non-Goals

**Goals:**

- Never overwrite an implemented entry point.

**Non-Goals:**

- Extending the same protection to `lib.rs` (bare empty stub; revisit if requested).

## Decisions

### 1. Skip list over documentation

The mechanism enforces itself; a warning comment would rely on discipline during automated updates.

## Risks / Trade-offs

- [Template improvements to the stub never reach consumers who implemented it] → Accepted; implemented code is consumer-owned by definition.

## Migration Plan

Land, re-validate, tag. Consumers pick it up on next update.

## Open Questions

None.
