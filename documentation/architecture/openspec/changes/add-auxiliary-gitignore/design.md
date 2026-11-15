## Context

Scratch locations are fleet convention (agent instructions), but the Rust template never rendered an ignore file for them, so every consumer tree shows `.auxiliary/scribbles/` and `.auxiliary/temporary/` as untracked noise. The Python template ships one with two additional Python-specific entries (`/artifacts/`, `/caches/` for hatch builds and pytest caches) that have no Rust equivalent and are deliberately not carried over.

## Goals / Non-Goals

**Goals:**

- Scratch dirs are ignored from the first render in every consumer.
- Consumer additions to the file survive updates via merge, not freezing.

**Non-Goals:**

- No `_skip_if_exists` entry: freezing would block future template additions; three-way merge is the correct mechanism for an additive file.
- No other ignore rules (build outputs are covered by root `.gitignore`'s `/target`).

## Decisions

### 1. Two entries, with a pointer comment

`/scribbles/` and `/temporary/` only. The comment points at the agent instructions rather than restating conventions that live elsewhere and could drift.

### 2. Same two lines in this repository

The template repo itself accumulates scratch in the same directories (dry renders, fixture runs); ignoring them here keeps the working tree clean and dogfoods the rule.

## Risks / Trade-offs

- [A consumer already ignores these paths globally or locally] → Harmless duplication: an extra ignore line changes no behavior.
- [Template later adds entries] → Handled by normal three-way merge; both sides kept.
