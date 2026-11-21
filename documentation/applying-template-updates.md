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
   answers file. Project-specific CI gates that must survive updates do not
   belong inside the rendered workflows: answer `enable_custom_ci` and put
   tester gates in a consumer-authored `./.github/workflows/local--tester.yaml`
   and release gates in `./.github/workflows/local--releaser.yaml`, which the
   rendered `custom` and `custom-release` jobs call as independent gates. Both
   files declare exactly the `platforms` and `cargo-profile` inputs (no
   `has-library`: the author knows their own crate shape); the template renders
   failing-loudly protected stubs whenever they are absent, so an
   unimplemented gate reads red, never green. Existing no-input custom files
   and any consumer-owned readers of the old initializer `cells` output must
   be migrated by hand at re-pin: add the declared inputs and rebind to
   `needs.initialize.outputs.platforms`. Copier rewrites rendered callers
   only, never consumer-owned readers.
4. **Shared gates are hermetic; environmental suites live under an
   explicit feature in custom gates.** Consumer suites MUST pass under
   the shared gates with no ambient tools or services — this covers the
   with-defaults doctest gates as well as the no-default-features nextest
   gates. Suites needing environment move under an explicit non-default
   Cargo feature exercised in the consumer-owned custom gates from
   Principle 3, which provision the pinned dependency and fail if it is
   absent or the expected suite did not run. The feature makes the suite
   opt-in for a developer command, not optional for merge/release where
   it proves a supported contract: wire the flag into every executing
   gate (matrix, release, pre-push equivalents) and document the dev-seat
   default, or live coverage rots silently. Keep bare `cargo test` fast
   and hermetic; provide one documented maintainer command for full
   validation on the same invocation as the project gates. Validate
   defaults explicitly where the minimal-feature jobs leave coverage gaps,
   and gate live suites as a module or target with a visible roster
   (split mixed modules rather than gating their hermetic cases). Never
   let the live feature be transitively enabled by ordinary product
   features. Verify both legs — hermetic shared matrix and live suite
   with the pinned dependency — on the same SHA before calling a retrofit
   done. Use test results to verify that the expected cases ran;
   passing the feature flag alone is not evidence.

   Worked example: nbspec's `live-nb` feature gates its live interchange
   tests behind Unix plus the flag (fail-closed without pinned nb kept);
   hermetic tests stay in the shared gates; `local--tester` and
   `local--releaser` pass the flag per cell so eligible Unix cells run
   the live suite. The live cases are not compiled on Windows.
5. **Never hand-edit the answers file.** It is overwritten by Copier. Drive
   answers with `--data-file` on copy, plus `--skip-answered` on update,
   and copy from the remote template source (`gh:emcd/rust-project-common`),
   tracking latest (the latest release Copier selects by default — not
   necessarily master HEAD) unless a pinned `--vcs-ref` was explicitly
   requested.
   Caveat: the public mirror can lag the template's production tree — if
   the fix you need is not yet on the mirror, copy from a reachable source
   instead; `--vcs-ref` can only select refs the source already has.
6. **Populate agents after updating, before validating.** Run
   `agentsmgr populate` after the copy/update and conflict resolution so
   template-owned agent files refresh — then run hooks and tests against
   the final tree, then commit. Validating before populating tests a tree
   that is about to change.
7. **Install hooks; do not assume them.** Fresh clones ship sample-only
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
8. **MSRV is a decision, not a default.** `rust-version` sets both the
   manifest floor and the toolchain pin. Confirm the intended floor with
   the project owner instead of accepting the default blindly. Pinning an
   exact patch version to dodge environment issues is a workaround, not a
   fix — fix the environment instead.
9. **Cross-repo workflows version independently of the template.** The
   shared tester lives outside the template tree and is referenced by
   floating major tag (`@gha-1`), which rendered callers pin by default;
   every shared change cuts an immutable `@gha-X.Y` tag that advances the
   float, and breaking changes start a new major series. The initial tag
   ships before any consumer renders a caller; floating tags move only
   after fixture-gated promotion, and only by the operator. Never retarget
   a float by hand, and never move or delete an immutable tag.

## First Copy Checklist

- Clean work branch, clean tree.
- Saved answers file; remote template source.
- Run the copy; review the full diff file by file.
- Restore project-owned files; keep template-owned additions.
- Install hooks (see `Install hooks; do not assume them` under Principles).
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
