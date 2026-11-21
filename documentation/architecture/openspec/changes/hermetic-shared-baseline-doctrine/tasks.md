## 1. Guide text

- [x] 1.1 Add the hermetic-baseline Principle to `documentation/applying-template-updates.md` directly after Principle 3, with the qualifications from the proposal (opt-in vs merge-optional, maintainer alias, explicit defaults validation, module-unit gating, no transitive enablement) and the verification bar (both legs, same SHA)
- [x] 1.2 Include the short `live-nb` worked-example paragraph per design.md
- [x] 1.3 Verify the new Principle satisfies the added `template-layout` spec requirement (scenarios: environmental suite directed to feature+custom gates; no exemption path offered; with-defaults doctests in scope)
- [x] 1.4 Renumber Principles after insertion (new Principle takes number 4) and change the First Copy Checklist's `Install hooks (see Principle 6)` to a name-based reference (`Install hooks` under Principles) so future insertions never break it

## 2. Validation and review

- [x] 2.1 Run `openspec validate --all --strict` green (structure check; note it does not check agreement among the change's own documents — sweep proposal/design/delta for the same claims by hand)
- [ ] 2.2 Route the Tier-1 review packet to Reviewer General after operator pre-review; no implementation until Tier-1, Tier-2 (if engaged), and operator signoff
