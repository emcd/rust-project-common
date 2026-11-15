## Context

Cistella's pre-commit config carries the empty-features flag as a project-owned graft with a comment; agentmux and nb-api carry explicit `--all-features` on the same entries. The template entries carry neither, so the tested feature set is whatever the manifest implies at render time — implicit, and mutable without touching any gate.

## Goals / Non-Goals

**Goals:**

- The feature set under test is written down in the invocation, like the lockfile is.
- Zero behavior change for crates without a `[features]` table (the common case) and for consumers already overriding explicitly.

**Non-Goals:**

- No per-crate feature answers; no conditional rendering on manifest contents (Copier must not parse Cargo.toml to decide flags).
- No doctest flag changes (proposer's scope; doctests compile the lib target where feature unification differs — a separate decision if ever proposed).

## Decisions

### 1. `--no-default-features`, not `--features ""`

The first draft stocked an empty `--features` list, which Tier-1 review refuted with a sentinel crate: it requests no *additional* features but leaves the manifest defaults enabled. The deterministic empty set is delivered by `--no-default-features` alone; the empty `--features` option adds nothing on top and is not used. Cistella's graft intent (a pinned, drift-proof feature set) is preserved and actually delivered by the corrected flag.

### 2. Gates and release builds pinned; smoke build keeps defaults

The stock set is empty, not `--all-features`: each gate tests the minimal configuration every consumer shares, and opting into more features is an explicit consumer act. `--all-features` as a default would force feature-unification failures onto crates that never asked for them (and would slow every gate compiling features nobody runs). The tester dev `cargo build` smoke job keeps manifest defaults deliberately: it still compiles default-gated code so breakage there fails visibly in CI, while the gates enforce the minimal set. (Manifest defaults, not `--all-features`: optional non-default features remain unbuilt everywhere.) Doctests, packaging, and fuzz/bench keep their existing invocations.

### 3. Merge, don't migrate

Feature-bearing consumers keep their explicit flags on update by resolving the conflict hunk toward their entry — the same keep-both-sides discipline as every other project-owned hook customization. No migration code moves flags around. Agentmux and nb-api both define `[features]` tables and already carry explicit `--all-features` overrides, so the stock flag changes nothing for them at merge.

### 4. No answers, no conditions

Whether a crate has features is manifest content, not template input. A Copier question for it would rot the moment the manifest changes without re-answering. The flag is unconditional; overrides live in the rendered file where the manifest lives.

### 5. Syncs after distribute-nextest-config

Both changes edit the Hooks requirement's invocation wording. This delta is authored against current live; at sync time its no-default-features additions merge into whatever invocation text is live then (including the config-file flag if that change lands first), verified clause-by-clause rather than overwritten — the same carry-forward discipline as the doctest zero-test sync.

## Risks / Trade-offs

- [A crate relying on default features in gates silently loses them] → Mitigation: announced behavior change with the release; the failure mode is visible (compile errors or failing tests from missing features), never silent skipping. The two known affected consumers already override and are unaffected.
- [Overrides create update conflicts] → Accepted: that is what the conflict markers are for, and the entries are the exact lines a consumer would already own if they customized flags today.
