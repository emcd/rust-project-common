## Context

Copier prints `_message_after_copy` / `_message_after_update` (Jinja-rendered, suppressed in quiet mode). The guide lives outside `template/` so it is never rendered into consumers; the banner links its GitHub page.

## Goals / Non-Goals

**Goals:**

- Every interactive copy/update ends with a pointer to the guide.

**Non-Goals:**

- Rendering the guide into consumers (it describes template-repo operations, not consumer content).

## Decisions

### 1. Blob URL on master, not raw, not pinned

The blob page renders the markdown; the guide changes rarely, so a floating master link avoids per-tag banner churn.

## Risks / Trade-offs

- [Banner ignored] → Accepted; short text, link only.

## Migration Plan

Land, re-validate, tag. No downstream edits.

## Open Questions

None.
