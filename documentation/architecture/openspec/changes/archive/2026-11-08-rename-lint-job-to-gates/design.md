## Context

`template/.github/workflows/tester.yaml.jinja` renders two matrix jobs behind the initializer: `lint` (runs the pre-commit stage: clippy, fmt, linecheck, nextest, doctests, hygiene) and `build` (`cargo build --locked --all-targets`). Both depend only on `initialize` and run in parallel. The live `quality-gates` spec pins the gate job's behavior under the name "lint job". The pre-commit hook named `Lint (Rust: Clippy)` is genuinely a lint and keeps its name.

## Goals / Non-Goals

**Goals:**

- The gate job's name describes its function (the mandatory pre-commit-stage gate), matching spec language that already calls the stage "the mandatory gate".

**Non-Goals:**

- No behavior, ordering, matrix, pin, or step changes.
- No change to the `build` job or to the clippy hook's display name.
- No job reordering; `build`/`gates` sequencing is tracked in the project notebook, not here.

## Decisions

### 1. `gates`, not `git-hooks`

`gates` names the job's function and matches existing spec vocabulary ("the mandatory gate"). `git-hooks` names the mechanism, and CI installs no hooks at all — it invokes the pre-commit runner directly. Chosen per operator direction with this rationale recorded.

### 2. Pure rename, three files

Job id in `tester.yaml.jinja`, spec wording plus scenario title in `quality-gates`, and the copiertv bootstrap comment. Nothing else reads the job name in-repo (verified by grep: remaining `lint` hits are the clippy hook name, review-procedure prose, and archived history, all correctly unchanged).

### 3. Consumer break is announced, not mitigated in-template

Required status checks referencing `lint` live in consumer branch protection, which the template cannot migrate. The change ships with an explicit **BREAKING** note and a tag announcement naming the old and new job ids. No compatibility alias: GitHub has no job-alias mechanism, and duplicating the job would double CI cost.

## Risks / Trade-offs

- [Consumer required checks fail closed on old name] → Mitigation: breaking note in proposal plus tag announcement; consumers update protection rules when re-pinning. Approots (just re-pinned), LitRPG (mid-retrofit), and Praetor (adopting) are notified directly.
- [A future script treats job names as stable API] → Accepted: job ids were never contracted as stable; the spec contracts behavior, and the spec moves with the rename.
