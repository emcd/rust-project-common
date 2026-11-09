## 1. Release pipeline

- [x] 1.1 Render `build-binaries` matrix job (checkout, toolchain, target, lockfile bootstrap, release build, deterministic name, artifact upload) when executable shape and publication are both enabled
- [x] 1.2 Render `publish-github` job (download all, SHA256SUMS, attest provenance, gh release create with generated notes, gh release upload) under the same condition with the tag-ref plus dry-run-disabled gate
- [x] 1.3 Bump rendered pins to latest: checkout v7 (tester plus releaser), crates-io-auth-action v1.0.6; bump the template repo's own validate-template.yaml checkout to v7
- [x] 1.4 Stage the attest bundle under a deterministic name in artifacts/ (F1); extend the rendered operator prerequisites with the attestation plan requirement (F2)
- [x] 1.5 Pass tag, repository, matrix, and bundle-path data into run scripts via env only (A1); no expression interpolation in shell source

## 2. Validate

- [x] 2.1 `openspec validate --all --strict` passes
- [x] 2.2 `copiertv validate` passes for default, maximum, and executable-only variants
- [x] 2.3 Maximum render (executable plus publication) contains build-binaries and publish-github with reference-matching identity, environment-free job config, permissions, and gates; default and executable-only renders contain neither new job
- [x] 2.4 Attestation clause reversal is the only live-requirement semantic change besides the additive pipeline
- [x] 2.5 copiertv asserts bundle staging whenever publish-github renders; fixture run proves the upload glob selects the staged bundle
- [x] 2.6 Hostile-tag fixture proves env-handoff controls: no command execution, byte-exact literal tag argv on create and upload; leading-dash tag fenced by `--` option delimiter, verified against real gh
