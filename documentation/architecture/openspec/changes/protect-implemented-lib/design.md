## Context

Same mechanism as the entry-point guard, proven on Copier 9.18.2: templated `_skip_if_exists` paths resolve against answers, existing files survive, absent files render.

## Goals / Non-Goals

**Goals:**

- Never overwrite an implemented library root.

**Non-Goals:**

- New answers. The guard is answer-free, like the bin guard.

## Decisions

### 1. Extend the guard, don't add answers

`src/lib.rs` joins `src/bin/<crate>.rs` in `_skip_if_exists`. Projects without a lib still get the bare stub; projects with one keep it.

## Risks / Trade-offs

- [Template improvements to the stub never reach consumers who implemented it] → Accepted; implemented code is consumer-owned by definition.

## Migration Plan

Land, re-validate, tag. LitRPG answers library-only with all bins project-owned.

## Open Questions

None.
