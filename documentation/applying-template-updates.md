# Applying and Updating from This Template

This guide is for agents (and humans) applying the Rust project template to a
consumer repository for the first time, or pulling template updates into an
already-consumed repository with `copier update`.

## Principles

1. **Reconcile, do not blindly accept.** A template copy renders generic
   content. Project-owned files always win on conflict: `Cargo.toml`
   metadata (version, features, test targets, include lists), `README.md`,
   `.github/workflows/*`, custom pre-commit hooks, `.gitignore`, and any
   implemented source files.
2. **Conflicts are normal and visible.** `copier update` performs a 3-way
   merge. Overlapping changes surface as inline conflict markers (or `.rej`
   files), never silent overwrites — except `_skip_if_exists` paths, which
   are preserved as-is. Resolve every marker before committing; the
   `check-merge-conflict` hook is a backstop, not the plan.
3. **Keep the template-owned core.** After reconciling, the consumer should
   converge on: root `rust-toolchain.toml` (channel equals `rust-version`),
   root `rustfmt.toml`, `linecheck` hook and config, the pre-commit stage
   hooks, CI workflows (or a conscious strip decision), and the recorded
   answers file.
4. **Never hand-edit the answers file.** It is overwritten by Copier. Drive
   answers with `--data-file` plus `--skip-answered`, and copy from the
   remote template source (`gh:emcd/rust-project-common`), tracking latest
   unless a pinned `--vcs-ref` was explicitly requested.
5. **Populate agents after updating, before committing.** Run
   `agentsmgr populate` after the copy/update and conflict resolution so
   template-owned agent files refresh, then run hooks and tests.
6. **MSRV is a decision, not a default.** `rust-version` sets both the
   manifest floor and the toolchain pin. Confirm the intended floor with
   the project owner instead of accepting the default blindly. Pinning an
   exact patch version to dodge environment issues is a workaround, not a
   fix — fix the environment instead.

## First Copy Checklist

- Clean work branch, clean tree.
- Saved answers file; remote template source.
- Run the copy; review the full diff file by file.
- Restore project-owned files; keep template-owned additions.
- Run `cargo fmt --check`, clippy, the test suite, linecheck, and the
  relevant CI workflows.
- Commit with the template ref recorded in the message.

## Update Checklist

- Clean tree. Note the template ref being updated from and to.
- Run `copier update`; resolve all markers and `.rej` files.
- Re-verify hooks and tests; `agentsmgr populate`; commit.
