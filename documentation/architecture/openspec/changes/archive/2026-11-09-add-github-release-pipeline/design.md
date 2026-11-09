## Context

The current releaser renders `initialize`, per-cell `release-gates` (build, test, package), and optional `publish-crates-io`. There is no path from a built binary to a GitHub release. The agentmux releaser (`build-binaries` per platform → `publish-github` with checksums, `actions/attest` provenance, `gh release` create/upload) and the Python template (`create-executables` gated on `enable_executables`, `publish-github` with artifact restore, checksums, attestation, towncrier notes) agree on the shape. Rust binaries need no PyInstaller equivalent: `cargo build --release` output, copied to a deterministic name, is the artifact.

## Goals / Non-Goals

**Goals:**

- Binary crates get per-platform release binaries attached to the tag's GitHub release, with checksums and provenance attestation, by re-pin alone.
- Action pins track latest releases, not the moment the references were copied.

**Non-Goals:**

- No `.crate`-file attachment for library-only crates (no precedent in either Rust reference; crates.io plus docs.rs already distribute libraries).
- No towncrier/release-notes generation (the Rust template has no changelog pipeline; `--generate-notes` per the agentmux precedent).
- No MCP-registry publishing (project-specific distribution, not template material).
- No change to `publish-crates-io` semantics.

## Decisions

### 1. Condition: executable shape plus publication enabled

`{%- if enable_publication %}` already wraps publishing; the new jobs nest `{%- if 'executable' in crate_shape %}` inside it. Library-only crates render neither job (no binaries exist). Publication-off renders neither either (a GitHub release is publication). No new answer: `crate_shape` already carries the knowledge, per the operator's direction.

### 2. `build-binaries` mirrors the agentmux job, adapted to cells

Same steps (checkout, toolchain, target add, release build, deterministic name, upload) but over the template's `cells` matrix instead of `platforms`. Artifact name `binary-<os>-<cpu>` (unique per cell by construction, unlike runner labels which the answers could theoretically collide). Lockfile bootstrap plus `--locked` per the template-wide lockfile principle (the references skip this; the template does not build unlocked anywhere).

### 3. `publish-github` mirrors the agentmux job, plus the bundle fix

Same permissions (`attestations`, `contents`, `id-token` write plus `artifact-metadata`), same step sequence (download all, checksums over `{{ crate_name }}-*`, `actions/attest` subject-path, `gh release create --generate-notes`, `gh release upload`), same `needs: [initialize, build-binaries]` (independent of `publish-crates-io`, so a crates-io failure does not suppress the GitHub release and vice versa — exactly the reference semantics). Same tag-ref plus dry-run-disabled gate as `publish-crates-io`. One deliberate improvement over both references: the attest step carries `id: attest` and a staging step copies `steps.attest.outputs.bundle-path` to a deterministic `{{ crate_name }}-provenance.sigstore.json` inside `artifacts/`, so the existing `artifacts/*` upload attaches the bundle to the release. The references leave the bundle API-only while implying release attachment; the template's scenario promises attachment, so the template fulfills it.

### 4. Latest pins, reference conventions for precision

`checkout@v7`, `upload-artifact@v7`, `download-artifact@v8`, `attest@v4` (major tags, as the references pin actions), `crates-io-auth-action@v1.0.6` (exact patch, as the references pin that action exactly). v7 checkout's hardened defaults target `pull_request_target` abuse; these workflows use `pull_request`/`push`/dispatch, and the template's own CI exercises the checkout step on every push. Pre-commit/linecheck/nextest tool pins are untouched (exact-version reproducibility is template policy; the action-pin question is separate because actions resolve at run time from moving majors).

### 5. Tester/initializer stay out

Unchanged by this work. The parity architecture and cells contract were authorized in the founding change; the only tester touch is the checkout pin.

## Risks / Trade-offs

- [Rendered-workflow defects cannot be executed in-template] → Mitigation: copiertv renders all variants and asserts structure; first live execution happens at a consumer tag. Residual risk is real (it bit us on `environment`) and is stated, not hidden. Approots' upcoming publication is the proving run.
- [v7 checkout behavior change] → Mitigation: standard triggers only; template CI runs checkout@v7 on every push; consumer adoption will confirm across the matrix.
- [Two publish jobs double-failure modes] → Accepted: independent channels fail independently by design (a crates-io outage must not eat the GitHub release).
- [Ref metadata interpolated into shell is injection] → Mitigation: all ref, matrix, and step-output data crosses into `run` scripts via `env` only (RELEASE_TAG, GITHUB_REPOSITORY, BUNDLE_PATH, BINARY_TARGET/OS); scripts reference quoted expansions and never expression text. The ported reference pattern interpolated `github.ref_name` inline — copying reference source does not transfer safety, so the rule is stated explicitly. Proven with a hostile-tag fixture (quote plus command substitution): no execution, byte-exact literal argv. A leading-dash tag is additionally fenced off by placing all gh options before `--` with the tag after it (F3); verified against real gh that `--help` past the delimiter parses positionally.
- [Attestation requires a capable repository plan] → Documented, not worked around: public repository or GitHub Enterprise Cloud for private ones. On other plans the publish-github job fails visibly at the attest step; attestation is never silently skipped. The rendered operator comment states the prerequisite; the spec locks the fail-visible behavior. This is also why `contents: read` stays (Decision 4 of the previous change): checkout must succeed everywhere so the failure, when it comes, is at attestation and states its own cause.
