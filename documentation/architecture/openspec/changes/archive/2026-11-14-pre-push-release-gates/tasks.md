## 1. Release twins

- [x] 1.1 Render release build, release nextest (with `--no-tests=warn` and `verbose: true`), and release doctest (library shape only) hooks at the pre-push stage, each with the lockfile bootstrap
- [x] 1.2 Extend the copiertv bootstrap file with the hook-to-CI command parity assertion (release hook entries match release-gates steps verbatim)

## 2. Validate

- [x] 2.1 `openspec validate --all --strict` passes
- [x] 2.2 `copiertv validate` passes for default, maximum, and executable-only variants
- [x] 2.3 Rendered hooks run the exact CI release commands; doctest twin renders only for library shapes
- [x] 2.4 No hook carries credentials, publishing, or tag operations; pre-commit stage byte-identical to before
