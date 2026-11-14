## Context

The tester renders `initialize`, `gates`, and `build` jobs today. Consumers needing more (nb-mcp's provisioned suites, cistella's container legs, conlang's release-mode pre-push equivalents at CI level) hand-edit the rendered file and pay merge conflicts forever. The spike (/tmp/spike, renders /tmp/spk-off and /tmp/spk-on) proved render-time omission and the parsed job graph both ways.

## Goals / Non-Goals

**Goals:**

- A conflict-free home for consumer CI gates with explicit failure semantics.
- A filename scheme where the prefix tells the truth: rendered (`common-`, unprefixed) versus never-rendered (`local--`, `xrepo--`).

**Non-Goals:**

- No remote reusable workflows in this change (xrepo stays exploration).
- No change to what the mandatory gate is or how it runs.
- No stub generation, no runtime conditionals, no secrets for consumer code.

## Decisions

### 1. Answer-gated render, not file detection

GitHub resolves `uses:` at run start; a job referencing a possibly-absent file cannot probe for it first. The answer is the gate: true renders the call, false omits it byte-entirely. Verified in spike: off-render carries zero custom references (grep count zero, parsed job list ends at build).

### 2. `needs: [initialize]`, job-level `uses`, explicit read-only cap

The custom job is a reusable-workflow caller: job-level `uses:`, no `runs-on`, no steps of its own (a step-level `uses:` would demand an action directory, not a workflow). It runs parallel with gates and build for an independent verdict, consistent with the operator's parallel build/gates decision. A red standard gate neither runs nor hides custom results; each gate reports on its own. Permissions are explicit `contents: read` — omitted permissions would inherit repo defaults and establish no cap — and no secrets are inherited or forwarded. The callee owns its runner and checkout.

### 3. Rename rides this change

`core--initializer.yaml` becomes `common-initializer.yaml` with both caller references updated. The rename touches the same files as the callout and re-pin carries it automatically (consumers reference the file only through the rendered workflows, never by hand). No third-party consumers exist outside the fleet.

### 4. No token scope for custom code

The custom job gets checkout only. Consumer gates run unprivileged test code; anything needing OIDC, attestation, or publish scope stays in template-owned jobs. This matches the spike prototype and Advisor's minimal-permission condition.

## Risks / Trade-offs

- [Enabled-but-missing file fails the run] → Intended and documented: an explicit hard failure at resolution, not a silent skip. The answers file records the intent, so the failure is self-explanatory.
- [Consumers must learn the contract] → Mitigation: the answer help text states the file requirement; the applying guide gains one paragraph (with this change).
- [Two gates can disagree] → Accepted: that is the point of an independent verdict. Required checks decide merges.
