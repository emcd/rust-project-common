## 1. Callout

- [x] 1.1 Rename the initializer template to `common-initializer.yaml.jinja` and update both caller references
- [x] 1.2 Add the `enable_custom_ci` boolean answer (default false) with contract-stating help text
- [x] 1.3 Render the conditional `custom` job (needs initialize only, checkout plus local reusable, no tokens, no runtime guard)
- [x] 1.4 Extend the copiertv bootstrap file with the naming-inventory assertion (common-initializer present, no `--` files rendered, custom references only when enabled)
- [x] 1.5 Document the contract in the applying guide (one paragraph)

## 2. Validate

- [x] 2.1 `openspec validate --all --strict` passes
- [x] 2.2 `copiertv validate` passes for default, maximum, and executable-only variants
- [x] 2.3 Disabled render carries zero custom references; enabled render carries the exact job with no `if:` guard
- [x] 2.4 Enabled render parses and the job graph matches the spec (custom alongside gates and build)
