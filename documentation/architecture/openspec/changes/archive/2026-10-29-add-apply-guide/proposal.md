## Why

Template consumers (especially agents) need the apply/update discipline written down in the template repo itself, surfaced at the moment of use — otherwise reconcile knowledge lives only in chat history.

## What Changes

- Add `documentation/applying-template-updates.md` (non-rendered repo file).
- Emit `_message_after_copy` and `_message_after_update` banners pointing at its GitHub page.
- Amend template-layout with an ADDED banner requirement.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- None. New ADDED requirement in existing `template-layout` spec.

## Impact

- Banner text on every copy/update (suppressed in quiet mode). No rendered output changes. No downstream edits.
