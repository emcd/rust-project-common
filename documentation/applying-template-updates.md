# Applying and Updating from This Template

This guide is for agents (and humans) applying the Rust project template to a
consumer repository for the first time, or pulling template updates into an
already-consumed repository with `copier update`.

## Principles

1. **Accept structure and baselines from the template; preserve
   project-specific content.** The template owns structure, pins,
   baselines, and gates; the consumer owns content, metadata, and
   customizations. On conflict, project-owned files win: `Cargo.toml`
   metadata (version, features, test targets, include lists), `README.md`,
   `.github/workflows/*`, custom pre-commit hooks, `.gitignore`, and any
   implemented source files.
2. **Conflicts are normal and visible.** `copier update` performs a 3-way
   merge. Ordinary overlapping changes surface as inline conflict markers,
   never silent overwrites — except `_skip_if_exists` paths, which are
   preserved as-is. (With `--conflict=rej`, Copier instead leaves `.rej`
   patch fragments alongside the file; check and reconcile every fragment
   by hand.) This covers ordinary 3-way conflicts only — not flags that
   skip merging, recopying over the tree, or every consumer-ownership case.
   Resolve every marker before committing; the `check-merge-conflict`
   hook is a backstop, not the plan.
3. **Keep the template-owned core.** After reconciling per Principle 1,
   the consumer should converge on: root `rust-toolchain.toml` (channel
   equals `rust-version`), root `rustfmt.toml`, `linecheck` hook and
   config, template-owned hook definitions merged with project-owned
   custom hooks (resolve conflicts keeping both sides — never strip one
   side), CI workflows (or a conscious strip decision), and the recorded
   answers file.
4. **Never hand-edit the answers file.** It is overwritten by Copier. Drive
   answers with `--data-file` on copy, plus `--skip-answered` on update,
   and copy from the remote template source (`gh:emcd/rust-project-common`),
   tracking latest (the latest release Copier selects by default — not
   necessarily master HEAD) unless a pinned `--vcs-ref` was explicitly
   requested.
   Caveat: the public mirror can lag the template's production tree — if
   the fix you need is not yet on the mirror, copy from a reachable source
   instead; `--vcs-ref` can only select refs the source already has.
5. **Populate agents after updating, before validating.** Run
   `agentsmgr populate` after the copy/update and conflict resolution so
   template-owned agent files refresh — then run hooks and tests against
   the final tree, then commit. Validating before populating tests a tree
   that is about to change.
6. **Install hooks; do not assume them.** Fresh clones ship sample-only
   `.git/hooks`, and every `fail_fast` gate silently no-ops until hooks
   are installed. Run `pre-commit install --config
   .auxiliary/configuration/pre-commit.yaml` (pre-commit preserves an
   existing pre-push hook as `pre-push.legacy` and chains it).
   `pre-commit --version` passing is not evidence hooks are live — resolve
   the real hooks location with `git rev-parse --git-path hooks`
   (worktrees keep a `.git` file, and `core.hooksPath` may redirect
   elsewhere), confirm an executable dispatcher exists for each required
   type (`pre-commit`, `pre-merge-commit`, `pre-push`) and points at the
   intended config, and distinguish installed dispatchers from successful
   stage execution. Do not silently bypass a custom `core.hooksPath`;
   handle or report it.
7. **MSRV is a decision, not a default.** `rust-version` sets both the
   manifest floor and the toolchain pin. Confirm the intended floor with
   the project owner instead of accepting the default blindly. Pinning an
   exact patch version to dodge environment issues is a workaround, not a
   fix — fix the environment instead.

## First Copy Checklist

- Clean work branch, clean tree.
- Saved answers file; remote template source.
- Run the copy; review the full diff file by file.
- Restore project-owned files; keep template-owned additions.
- Install hooks (see Principle 6).
- Run `agentsmgr populate`, then `cargo fmt --check`, clippy, the test
  suite, linecheck, and the relevant CI workflows.
- Commit with the template ref recorded in the message.

## Update Checklist

- Clean tree. Note the template ref being updated from and to.
- Run `copier update`; resolve all conflict markers.
- Re-run `pre-commit install --config .auxiliary/configuration/pre-commit.yaml` so any new hook types land — updates never reinstall hooks.
- Run `agentsmgr populate`.
- Re-verify hooks and tests; commit.

Populate can delete: replacing a tracked coder-skills directory with a
symlink removes the directory first. Before populating, inventory and back
up project-owned skills and migrate them into the canonical tree; inspect
the populate diff and verify retained contents and links before
committing. If populate already deleted unique content, recover it from
git or a backup into the canonical tree — do not dismiss deletion as
merely layout, and do not blindly restore a legacy directory over a valid
symlink. Sync to current master first.
