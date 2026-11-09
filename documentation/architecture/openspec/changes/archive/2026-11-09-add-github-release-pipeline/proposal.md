## Why

The Rust template has no GitHub-release pipeline: tagged releases publish to crates.io (when enabled) and stop there. Both established precedents ship one — the Python template renders `create-executables` plus `publish-github` (checksums, provenance attestation, `gh release` create/upload), and the agentmux/nb-mcp-server releasers do the same natively. The Rust template already knows whether executables exist (`crate_shape`), so the binary jobs can be templated on that knowledge instead of omitted. Approots is about to publish a binary crate through this workflow and would otherwise hand-roll a second time what two precedents already standardized.

## What Changes

- Render a per-cell `build-binaries` matrix job (release build, deterministic binary name `{{ crate_name }}-<target>[.exe]`, artifact upload) when the crate shape includes `executable` and publication is enabled.
- Render a single-cell `publish-github` job (download all binaries, SHA256SUMS, `actions/attest` provenance, `gh release create --generate-notes`, `gh release upload`) under the same condition, gated on tag refs with dry-run disabled, `needs: [initialize, build-binaries]`.
- The `publish-crates-io` job is unchanged in identity and trigger; both publish jobs run (crates-io and GitHub are independent distribution channels, mirroring the references).
- Fold in latest action pins (operator direction: latest, not reference-frozen): `actions/checkout@v7`, `rust-lang/crates-io-auth-action@v1.0.6`, `actions/upload-artifact@v7`, `actions/download-artifact@v8`, `actions/attest@v4`. Same bumps in the template repo's own `validate-template.yaml`.
- **BREAKING**: none for existing consumers (new jobs only render where no job rendered before). Pushing a `v0.x` tag with publication enabled now additionally creates a GitHub release with binaries when the shape includes executables.

## Capabilities

### New Capabilities

None (release distribution is covered by the existing publication capability family).

### Modified Capabilities

- `optional-features`: the publication requirement gains the binary-build/publish-github pipeline, its conditions, and its provenance/attestation behavior.

## Impact

- `template/.github/workflows/releaser.yaml.jinja` (two new jobs, pin bumps).
- `template/.github/workflows/tester.yaml.jinja` (checkout pin only).
- `.github/workflows/validate-template.yaml` (checkout pin only).
- `openspec/specs/optional-features/spec.md` (publication requirement plus scenarios).
- Consumers: binary crates gain GitHub releases on re-pin; library-only renders are unaffected.
