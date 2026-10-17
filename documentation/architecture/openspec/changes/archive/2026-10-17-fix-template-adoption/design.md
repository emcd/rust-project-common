## Context

Copier writes the answers file only if the template ships `{{ _copier_conf.answers_file }}.jinja` (proven against Copier 9.18.2; the Python template has it, ours did not). The authors `[]` crash reproduces in `cast_answer` before any validator runs, so neither validators nor prompt design can guard it; it is reachable via hand-edited answers files and via API or data-file input. Machine-written files always serialize correctly, which is what the answers template guarantees. `src/main.rs` as the default bin hijacks `cargo run` in multi-bin projects and is resurrected by every update; `src/bin/` assumes multiplicity from the start at the cost of `cargo new` idiom parity.

## Goals / Non-Goals

**Goals:**

- Copies record answers; updates have lineage without manual steps.
- No phantom or hijacking binaries in any shape combination.
- No new answers.

**Non-Goals:**

- Accepting non-string authors (kept `str` deliberately).
- Retrofit reconciliation (one-time, hand-done).

## Decisions

### 1. Answers template over documentation

Ship the file; the manual step disappears instead of being documented.

### 2. `src/bin/` over `src/main.rs`

Ongoing update cost beats one-time idiom cost. No `bin_layout` answer.

## Risks / Trade-offs

- [Greenfield single-binary crates look non-idiomatic] → Accepted; fleet members know the layout, and `cargo run` behaves identically.

## Migration Plan

Land, re-validate the matrix, tag. Existing consumers pick it up on next update; deleted `src/main.rs` files stay deleted only if consumers exclude them (standard Copier semantics, worth one line in the review packet, not the spec).

## Open Questions

None.
