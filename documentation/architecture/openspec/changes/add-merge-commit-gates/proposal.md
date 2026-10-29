## Why

A merge creates its commit through the `pre-merge-commit` hook; a conflicted or `--no-commit` merge finalized later via `git commit` runs `pre-commit` instead, and a fast-forward creates no merge commit at all. Where the `pre-merge-commit` hook does run, `pre-commit` never fires alongside it, so those merge commits run none of the fast gates — and a merge is a normal way for over-cap or failing code to enter a trunk. (The Agentmux over-cap incident is a case of dark gates from hooks never installed — a different failure with a different fix.) This change narrows that gap; it is not an unbypassable trunk or server-side gate.

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
