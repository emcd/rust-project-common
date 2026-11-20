# quality-gates

## Purpose

Pre-commit hooks, CI parity, platform matrix, and line budgets.

## Requirements

### Requirement: Hooks use the pre-commit framework
The template MUST render `.auxiliary/configuration/pre-commit.yaml` for the pre-commit framework. It MUST NOT install hooks by copying shell scripts into `.git/hooks`. The pre-commit stage MUST consist of clippy with `-D warnings` and no default features on all targets, rustfmt `--check`, linecheck, nextest with zero tests tolerated as a warning, an explicit `--config-file .auxiliary/configuration/nextest.toml`, and no default features, and, when the crate shape includes library, a doctest gate running `bash -c 'test -f Cargo.lock || cargo generate-lockfile; exec cargo test --doc --locked'`, and the standard file-hygiene hooks: large-file, filename-case-conflict, merge-conflict-marker, symlink, VCS-permalink, destroyed-symlink, private-key, mixed-line-ending, JSON, TOML, and YAML checks, plus end-of-file and trailing-whitespace fixers. The doctest gate MUST be rendered only when the crate shape includes library. The same fast gates MUST also run at the `pre-merge-commit` stage, so merge commits cannot bypass them. The pre-push stage MUST consist of release build with no default features (`cargo build --release --locked --all-targets --no-default-features`), release nextest with zero tests tolerated as a warning, an explicit `--config-file .auxiliary/configuration/nextest.toml`, and no default features (`cargo nextest run --no-default-features --release --locked --no-tests=warn --config-file .auxiliary/configuration/nextest.toml`), and, when the crate shape includes library, a release doctest gate (`cargo test --doc --release --locked`), each with the lockfile bootstrap. When fuzzing is enabled, a bounded `cargo fuzz` run MUST be a pre-push hook. When benchmarking is enabled, a `cargo bench` run MUST be a pre-push hook. The mandatory merge-time gates are exactly the fast gates listed for the pre-commit stage; hygiene hooks run at merge per their upstream stage pins, which the template does not override. Merge-stage gates behave exactly as on ordinary commits, including lockfile bootstrap; if a hook modifies tracked files, pre-commit fails the merge visibly rather than completing it silently.

#### Scenario: Hooks are installed

- **WHEN** a consumer installs hooks from the rendered configuration
- **THEN** the pre-commit framework runs that file and no template hook is a copied script under `.git/hooks`

#### Scenario: Merge commit runs fast gates

- **WHEN** a consumer creates a merge commit with hooks installed
- **THEN** the clippy, fmt, linecheck, nextest, and (for library crates) doctest hooks run at `pre-merge-commit` and a violation blocks the merge

#### Scenario: Push runs release gates

- **WHEN** a consumer pushes with hooks installed
- **THEN** the release build, release nextest, and (for library crates) release doctest hooks run at `pre-push` and a violation blocks the push

#### Scenario: Doctests execute

- **WHEN** a consumer commits a doc comment containing a failing code example
- **THEN** the doctest gate fails before the commit completes

#### Scenario: Fresh scaffold with no tests

- **WHEN** a consumer commits to a rendered project that has no tests yet
- **THEN** the nextest gate warns about zero tests and passes instead of failing

### Requirement: CI runs the same config for one stage

The tester MUST run its gates through the versioned cross-repo reusable workflow `xrepo--tester.yaml` instead of rendered inline steps, calling it once with cargo-profile `dev` and once with cargo-profile `release` in parallel. Every call MUST run lint (clippy with `-D warnings`, rustfmt `--check`, linecheck), then build, then test plus doctests for library crates, as explicit steps on every target platform — not by invoking the pre-commit runner. The dev call MUST NOT run pre-push hooks or live hooks. The release call MUST run the release build, release nextest with zero tests tolerated as a warning, and release doctests for library crates. The reusable workflow MUST declare `platforms`, `cargo-profile` (validated string), and `has-library` inputs and fan out internally, MUST reject unknown profiles and any `platforms` entry outside the supported runner/target mapping in a leading validation step, and MUST NOT interpolate input values into shell source, MUST pin its tool dependencies, and MUST install project hook dependencies before the gate. All cross-repo `uses:` calls MUST declare explicit `permissions: contents: read` or less; the shared callee MUST require no more than `contents: read` and accept no secrets. Local `package` and `hygiene` jobs MUST run with least-privilege permissions and set `persist-credentials: false` on checkouts that never push; write and OIDC privileges stay confined to the existing local publisher jobs, and no permissions block relies on consumer repo defaults. The tester MUST trigger on `pull_request`, never on privileged `pull_request_target` execution of PR code. The test runner MUST be nextest; a doctest-only runner MAY accompany it. Both profiles are mandatory gates on every selected OS by CPU pair, except a step scoped out as genuinely inapplicable. The tester MUST render exactly one non-matrix `final-gate` job that branch protection targets: it runs after all gate jobs including `custom` whenever custom CI is enabled, with `if: always()`, and fails unless every needed result is `success`, so required checks name one stable context instead of per-platform names. The tester MUST also render exactly one single-runner `hygiene` job running the file-hygiene hook IDs (large-file, filename-case-conflict, merge-conflict-marker, symlink, VCS-permalink, destroyed-symlink, private-key, mixed-line-ending, JSON, TOML, and YAML checks, plus end-of-file and trailing-whitespace fixers) via the pre-commit runner with an explicit hook list. The local pre-commit hook configuration is unaffected by this requirement; a copiertv assertion MUST verify every gated CI step command matches its hook entry modulo the lockfile-bootstrap wrapper, and MUST fail validation for any CI command with no twin outside the explicit no-twin allowlist (dev builds, setup and toolchain steps, lockfile generation, packaging, attestation, artifact upload and download, release creation and upload, and the explicit hygiene hook-ID list). When the consumer enables custom CI, the tester MUST render exactly one additional `custom` job that calls the `./.github/workflows/local--tester.yaml` reusable workflow at job level with explicit `contents: read` permissions and no secrets, passing the full `platforms` array with `cargo-profile: dev`, needing only `initialize`. The custom call MUST pass exactly the two typed inputs — `platforms` (string carrying a JSON-encoded array) and `cargo-profile` (string) — and MUST NOT pass `has-library`, which the shared call requires but the custom contract omits; the callee MAY declare additional non-required inputs with defaults. Fan-out happens inside the called workflow in both cases, and the caller never fans out. When custom CI is enabled and no consumer-authored `local--tester.yaml` exists, the template MUST render a starting-point stub there instead, and that stub MUST fail loudly with implementation instructions rather than pass. The stub MUST declare the required inputs and an explicit `permissions: contents: read` block as its starting point. The custom job MUST be omitted byte-entirely when custom CI is disabled, MUST NOT be hidden behind a runtime condition, and MUST be part of the mandatory gate set whenever rendered. The releaser MUST render exactly one local `package` job per selected platform that runs regardless of custom CI, dry-run mode, and publication flags. `publish-crates-io` and `build-binaries` MUST wait on whole-`platforms` release-gate success and `package` success, plus `custom-release` success whenever rendered; the publishers keep their existing tag, dry-run, and publication predicates and their credential and attestation protections unchanged. When the consumer enables custom release CI under the same single answer, the releaser MUST render exactly one additional `custom-release` job calling the `./.github/workflows/local--releaser.yaml` with the full `platforms` array and `cargo-profile: release`, needing only `initialize`; `publish-crates-io` and `build-binaries` MUST additionally wait on it; `publish-github` MUST wait transitively via `build-binaries`, unconditionally whenever that publisher exists. Custom jobs are self-contained verifications (they check out and build what they need themselves). The same stub, omission, no-hiding, no-secrets, and no-`has-library` rules apply; both job-level `uses:` calls MUST carry explicit `contents: read` and no secrets. Both stub paths MUST be `_skip_if_exists`-protected so consumer implementations are never overwritten.

#### Scenario: Gates job on pull request

- **WHEN** a pull request runs the tester
- **THEN** the shared dev gate runs lint, build, and tests explicitly without invoking the pre-commit runner, and does not run pre-push hooks

#### Scenario: Release track runs in parallel

- **WHEN** a pull request runs the tester
- **THEN** the shared release gate runs the release build, release tests, and release doctests when `has-library`, alongside the dev gate

#### Scenario: Rendered workflows pass the template's own gates

- **WHEN** a consumer renders the template
- **THEN** every rendered file already satisfies the hygiene hooks with no fixer rewrites

#### Scenario: Custom CI disabled

- **WHEN** a consumer renders the template with custom CI off
- **THEN** the tester contains no custom job and no reference to any local workflow file

#### Scenario: Custom CI enabled

- **WHEN** a consumer enables custom CI and provides `local--tester.yaml`
- **THEN** the tester runs the consumer workflow as an independent gate alongside the template gates

#### Scenario: Custom CI enabled without the file

- **WHEN** a consumer enables custom CI but provides no `local--tester.yaml`
- **THEN** the template renders a stub that fails loudly with implementation instructions instead of passing silently

#### Scenario: Custom call carries no library shape

- **WHEN** a consumer enables custom CI
- **THEN** the custom call passes `platforms` and `cargo-profile` only, and the consumer workflow encodes library-ness directly instead of receiving it

#### Scenario: Required checks name one gate per workflow

- **WHEN** branch protection requires the tester to pass
- **THEN** it names the single `final-gate` job, which fails unless every gate job in the tester succeeded

#### Scenario: Hygiene runs as an explicit job

- **WHEN** a pull request runs the tester
- **THEN** a single-runner hygiene job runs the file-hygiene hook IDs without invoking the full pre-commit stage

### Requirement: Selected OS and CPU pairs are real jobs

Operating system and CPU architecture MUST be separate answers. Linux and macOS, and both x86-64 and arm64, MUST be selected by default. Windows MUST default off. The workflow matrix MUST be the cartesian product of the selected values, the initializer MUST emit only selected pairs, and tester and releaser MUST consume those outputs. The template MUST render initializer, tester, and releaser workflows; the releaser MUST publish to crates.io only when publication is enabled. Release build, test, and package gates MUST run in each selected cell; publication MUST be a single non-matrix publish job that depends on the success of the entire matrix and MUST be rendered only when publication is enabled. When Windows is selected, a Windows cell MUST exist for each selected CPU architecture and for no other. Each selected pair MUST run its applicable build, test, and mandatory quality gates; a platform-specific hook MAY be scoped out only where it is genuinely inapplicable; an unsupported mandatory step MUST fail visibly; and the template MUST NOT emit a pair and then exclude it from those gates.

#### Scenario: Default platforms

- **WHEN** a consumer accepts default platform answers
- **THEN** the matrix contains Linux and macOS on both x86-64 and arm64, and contains no Windows job

#### Scenario: Windows selected with both CPUs

- **WHEN** a consumer selects Windows and both x86-64 and arm64
- **THEN** the matrix adds Windows x86-64 and Windows arm64 as jobs that run the mandatory gates

#### Scenario: Windows selected with x86-64 only

- **WHEN** a consumer selects Windows and only x86-64
- **THEN** the matrix contains a Windows x86-64 job and no Windows arm64 job

#### Scenario: Mandatory gate has no equivalent

- **WHEN** a selected OS and CPU pair has no equivalent for a mandatory gate
- **THEN** that gate fails the job and is not skipped

### Requirement: CI builds with the toolchain pin

The normal build MUST use the channel in root `rust-toolchain.toml`. The template MUST NOT add a second job that builds with a different compiler.

#### Scenario: Default toolchain

- **WHEN** `rust-version` is `1.98`
- **THEN** the build job uses toolchain `1.98` and no other compiler job is rendered

### Requirement: linecheck is always present

The template MUST always render the linecheck hook, config, and CI install. It MUST NOT offer an answer to disable linecheck. The initial config MUST warn at 800 and error at 1000. Thresholds and exclusions MUST be project-owned after the first copy; the linecheck config file MUST use `_skip_if_exists` so a later update MUST NOT overwrite them.

#### Scenario: Project is rendered

- **WHEN** a consumer generates a project
- **THEN** the linecheck hook, config, and CI install all exist, and the config starts at warn 800 and error 1000

#### Scenario: Consumer edits thresholds

- **WHEN** a consumer changes the linecheck warn or error numbers and then runs `copier update`
- **THEN** those numbers remain the consumer's values

### Requirement: Nextest config is rendered
The template MUST render a default `.auxiliary/configuration/nextest.toml` carrying no behavioral tuning, and MUST list it in `_skip_if_exists` so later copies and updates never overwrite consumer tuning. Every template nextest invocation MUST pass `--config-file` naming that path.

#### Scenario: Project is rendered

- **WHEN** a consumer generates a project
- **THEN** the config file exists with no tuning, and every nextest entry names it

#### Scenario: Consumer tunes nextest

- **WHEN** a consumer edits the config and then runs `copier update`
- **THEN** those edits remain the consumer's values

### Requirement: Cross-repo workflows are versioned releases

Every change to a file under repo-root `.github/workflows/xrepo--*.yaml` MUST cut an immutable `@gha-X.Y` tag and MUST advance the floating `@gha-N` major tag, unless the change is breaking, in which case a new major series begins. Rendered callers MUST pin the floating major by default. The floating major MUST NOT advance to a candidate unless every promotion fixture caller reports its expected verdict for that exact candidate commit. Fixture callers live on dedicated template-owned `fixture/*` branches, separate from the candidate commit: a committed fixture cannot name its own commit SHA and job-level `uses:` takes no expressions. Jobs in a called workflow check out the caller repository at the caller ref while the template root has no Cargo.toml, so each caller branch MUST hold a real rendered crate at its checkout root. The `fixture/promotion-lib` caller (library-bearing crate, `has-library` true) and the `fixture/promotion-bin` caller (binary-only crate, `has-library` false) MUST be green on dev and release; the `fixture/promotion-red` caller (deliberate gate violation) MUST fail in the gate step, and a bogus-`cargo-profile` dispatch MUST fail in the leading validation step. Controls MUST fail for their intended validator/gate reason, never for workflow-resolution or checkout errors. Tag creation, update, and deletion for shared-workflow refs and floating majors MUST be restricted to the operator-owned promotion process, and published immutable `@gha-X.Y` tags MUST never move or be deleted. At promotion, the process MUST recheck evidence freshness — exact candidate SHA, fixture caller revisions, and current tag/main eligibility — before moving the floating ref. Recovery rolls the floating major back only to a retained immutable pin, only with operator authorization, and only with fixture-green evidence for that pin; no arbitrary retarget. The initializer, custom job, and publish jobs MUST NOT move cross-repo under this scheme.

#### Scenario: Shared workflow changes

- **WHEN** the template repo changes a cross-repo workflow without breaking its contract
- **THEN** a new immutable minor tag appears and the floating major advances to it, with no consumer file touched

#### Scenario: Shared workflow breaks its contract

- **WHEN** a cross-repo change breaks inputs, outputs, or behavior callers rely on
- **THEN** it ships under a new `@gha-<next-N>` major and the previous major stays valid

#### Scenario: Promotion controls fail for gate reasons

- **WHEN** the red promotion caller or a bogus-profile dispatch runs against a candidate
- **THEN** the run fails in the gate step or the leading validation step respectively, never in workflow resolution or checkout

#### Scenario: Unsupported runner rejected

- **WHEN** `platforms` contains a runner/target outside the supported mapping
- **THEN** the leading validation step fails before fan-out
