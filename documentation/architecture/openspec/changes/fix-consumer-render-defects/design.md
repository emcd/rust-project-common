## Context

`pre-commit run` resolves its config from `.pre-commit-config.yaml` at the repo root unless `--config` is given. The template keeps the config under `.auxiliary/configuration/`, so the CI invocation must name it explicitly — local developers get it from `pre-commit install --config`, CI does not. The releaser's trailing `{% endif %}` on its own line leaves the render ending in a blank line under default Jinja whitespace rules; the template's own end-of-file-fixer then rewrites it and fails the run.

## Goals / Non-Goals

**Goals:**

- Fresh-adoption CI passes the lint job unmodified.
- Rendered files pass the template's own hygiene gates byte-for-byte.

**Non-Goals:**

- Changing hook or workflow behavior, only their invocation and rendering.

## Decisions

### 1. Explicit --config in CI, not a root symlink

Pointing at the real path keeps one config file. A root symlink would add a second root file to every consumer for CI's benefit alone.

### 2. Trim at the source, not in CI

Fix the template whitespace so the render is already clean, rather than tolerating fixer rewrites.

## Risks / Trade-offs

- None material; both changes are strictly bug fixes with no behavioral surface.

## Migration Plan

Land, re-validate, tag. Consumers pick it up on next update.

## Open Questions

None.
