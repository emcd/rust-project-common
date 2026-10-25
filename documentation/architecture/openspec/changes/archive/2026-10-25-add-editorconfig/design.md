## Context

Python `template/.editorconfig` sets root, utf-8, lf, 4-space, final newline, trim, 79-col for md/py/rs/rst, indent 2 for toml/yaml. The Rust port keeps the base and md/rs/toml/yaml sections, dropping py/rst. Rust 4-space matches rustfmt defaults; 79 matches `rustfmt.toml` and linecheck policy.

## Goals / Non-Goals

**Goals:**

- Every rendered project gets `.editorconfig` with fleet-wide whitespace rules.

**Non-Goals:**

- `.gitattributes` LFS line (no tarball artifacts in the Rust template).
- Auxiliary placeholder dirs (Python-workflow-specific; add on demand).

## Decisions

### 1. Static file, unconditional

No answers affect it; renders always.

## Risks / Trade-offs

- [Editors ignoring EditorConfig without a plugin] → Accepted; pre-commit fixers remain the enforcement path.

## Migration Plan

Land, re-validate the matrix, tag. Existing consumers pick it up on next update.

## Open Questions

None.
