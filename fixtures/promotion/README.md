# Promotion fixtures

Template-owned promotion infrastructure for the cross-repo shared tester
(`.github/workflows/xrepo--tester.yaml`). See the archived proposal
`documentation/architecture/openspec/changes/archive/2026-11-20-xrepo-shared-workflows/`
for the full contract.

## Layout

- `answers-{lib,bin,red}.yaml` (here on master): fixed copier answers that
  reproduce each fixture crate byte-identically. Re-render with
  `copier copy <template> <dest> --data-file answers-<name>.yaml`.
- Branch `fixture/promotion-lib`: real rendered library crate at its root,
  clean gates. Expected verdict: green on dev and release.
- Branch `fixture/promotion-bin`: real rendered binary-only crate at its
  root, clean gates. Expected verdict: green on dev and release.
- Branch `fixture/promotion-red`: real rendered library crate at its root
  with a deliberate clippy violation, plus a bogus-profile caller. Expected
  verdict: red in the lint gate (crate) and red in the leading validation
  step (bogus profile) — never in checkout or workflow resolution.

## Lifecycle

Each fixture branch holds a caller workflow (`.github/workflows/fixture--promotion.yaml`)
that pins the candidate shared-workflow commit SHA literally. To test a new
candidate: advance the pinned SHA on each branch, push (pushes trigger the
runs), and record the verdicts. Only on all-expected-verdicts may the
floating major advance, and only by the operator. First tag and all moves
remain operator-owned.
