## Context

pre-commit supports a `pre-merge-commit` stage running on `git merge`. Hook stages come from `default_stages` (default: every stage) unless the hook pins its own list — `default_install_hook_types` only decides which hook files land in `.git/hooks`, not when hooks run. The four local hooks pin `stages: [ 'pre-commit' ]` and need extending; the pre-push compile/test gates stay push-only because merges should be fast. Hygiene hooks carry no explicit stages, so they follow `default_stages` subject to their upstream pins: measured on the rendered config, nine hygiene hooks run at merge (case-conflict, merge-conflict, symlinks, vcs-permalinks, private-key, mixed-line-ending, json, toml, yaml) while four do not (large-files, destroyed-symlinks, end-of-file-fixer, trailing-whitespace — upstream-pinned to pre-commit, pre-push, and manual). The fixers staying out is load-bearing, not a gap: no hook may modify the working tree during a merge. The explicit stage lists on the four fast hooks must stay: without them `default_stages` would pull the test suite into pre-push and commit-msg.

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

Land, re-validate, tag. Consumers pick it up on next update, then must re-run `pre-commit install --config .auxiliary/configuration/pre-commit.yaml` — `copier update` does not reinstall hooks, so without it the new hook type never lands.

## Open Questions

None.
