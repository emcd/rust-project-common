## Context

`documentation/applying-template-updates.md` carries 8 Principles governing first-copy and update retrofits. Principle 3 assigns custom project gates to consumer-owned `local--tester.yaml` / `local--releaser.yaml` files; Principle 8 versions the shared xrepo workflows. Neither states what may run in shared gates versus custom gates. Mechanism as built: shared nextest gates (dev, release) run with no default features, while shared library doctest gates (dev, release) run WITH defaults, matching local parity — so the "hermetic" rule is about ambient tools and services, not about feature flags, and it covers the with-defaults doctests too. The nbspec retrofit proved the gap: its live-nb interchange suite failed all 6 shared cells for want of a `nb` binary the shared lane never provisions. The four-way (NbApi Owner, Nbspec Owner, Quasar) settled the doctrine; this design places it in the guide.

## Goals / Non-Goals

**Goals:**

- Give every retrofit a single named rule: shared gates are hermetic, environmental suites live under an explicit non-default feature in custom gates.
- Record the settled qualifications (opt-in vs merge-optional, maintainer alias, explicit defaults validation, module-unit gating, no transitive enablement) so they survive as written guidance, not tribal memory.
- Name the verification bar (both legs, same SHA) the template owner holds at retrofit review.

**Non-Goals:**

- No rendering, workflow, input, or gate-behavior changes. The single spec delta is docs-content-only: it pins what the guide MUST state and changes no gate behavior (`quality-gates` requirements are untouched).
- No retrofit of already-landed consumers and no exemption process. Existing consumers adopt the doctrine at their next update touching test structure.
- No normative gate-behavior requirement (e.g., in `quality-gates`). If enforcement language proves necessary later, that is a separate change.

## Decisions

- **New Principle placed directly after Principle 3** (rather than amending 3 or appending at the end). Principle 3 already introduces the custom-gate files the doctrine assigns environmental suites to, so adjacency reads naturally; a standalone number keeps it citable in retrofit reviews. Alternatives rejected: folding into Principle 3 would bloat an ownership rule with a QA contract; appending after Principle 8 would detach it from the custom-gate context.
- **Include the nbspec `live-nb` instance as a worked example** (rather than generic-only text). It is already agreed by all four parties and makes the abstract pattern concrete (feature name, provisioning in custom matrix jobs, fail-closed kept). Kept to a short paragraph so the Principle stays generic.
- **Verification bar lives in the guide as a consumer expectation, not as QA-owned enforcement text.** Quasar owns enforcement through review; the guide tells the consumer what the reviewer will hold them to.

## Risks / Trade-offs

- [Risk] Consumers with existing ambient-dependent suites face retrofit work they did not budget → Mitigation: the Principle is explicit that there is no exemption path, and the kickoff (not mid-retrofit) is where the template owner surfaces the cost.
- [Risk] Guidance without normative spec teeth can drift from enforcement → Mitigation: accepted; the four-way participants (including QA) reviewed this placement, and a normative follow-up remains available as a separate change if drift appears.
- [Risk] Example rots as nbspec evolves → Mitigation: example cites the pattern, not nbspec internals; nbspec is the first live consumer testbed and stays visible.

## Migration Plan

Docs-only. Land with the normal template release flow; consumers pick it up as guide text on next read. No rollback needed beyond revert.

## Open Questions

- None for this change. Deferred deliberately: whether `quality-gates` gains a normative hermetic requirement (separate change if needed).
