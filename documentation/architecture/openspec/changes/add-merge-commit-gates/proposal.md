## Why

`git merge` creates its commit via the `pre-merge-commit` hook and never invokes `pre-commit`, so the fast fail-fast gates never ran on merge commits. Agentmux landed an over-cap file at integration with no signal for exactly this reason.

## What Changes

- Add `pre-merge-commit` to `default_install_hook_types` in the template pre-commit config.
- Extend every `[ 'pre-commit' ]`-staged hook to `[ 'pre-commit', 'pre-merge-commit' ]`. Pre-push hooks unchanged.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `quality-gates`: fast gates also run at merge-commit time.

## Impact

- Rendered hook config only. Merge commits now run the same fast gates as ordinary commits. No downstream edits.
