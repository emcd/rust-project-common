## Why

Every template nextest invocation runs flag-bare today, so a consumer with a tuning file and one without silently run different configurations — and consumers who need `--config-file` carry it as a project-owned graft on the template entries. The three reference-adjacent consumers converge on `.auxiliary/configuration/nextest.toml` as the path. Distributing a default config and always passing the flag removes the fork: one canonical path, one invocation shape.

## What Changes

- Render a static default `.auxiliary/configuration/nextest.toml` (header comment plus docs pointer; no behavioral tuning — slow-timeout and friends stay per-repository).
- List it in `_skip_if_exists`: renders only when absent, never merged afterward.
- Template nextest entries (pre-commit, pre-merge-commit, pre-push release, CI release-gates) always pass `--config-file .auxiliary/configuration/nextest.toml`.
- The flag replaces the default autoload path (workspace-root `.config/nextest.toml`), it does not add to it — verified against nextest help. A stale file at the default path stops loading; the canonical path is the single tuning point.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `quality-gates`: the nextest invocations gain the config-file flag; the rendered tree gains the skip-if-exists config file.

## Impact

- `template/.auxiliary/configuration/nextest.toml` (new static file).
- `copier.yaml` (`_skip_if_exists` gains one entry).
- `template/.auxiliary/configuration/pre-commit.yaml.jinja` (three nextest entries).
- `template/.github/workflows/releaser.yaml.jinja` (release test command).
- `openspec/specs/quality-gates/spec.md` (invocation wording plus config-file requirement).
- Consumers with an existing file at that exact path: byte-identical preservation, flag grafts removable. Consumers with tuning at the default autoload path: must move it (one-time, announced).
