# Fixture branch: promotion-red

Template-owned promotion fixture (see `fixtures/promotion/README.md` on
`master` and the archived xrepo-shared-workflows proposal). This branch holds
a real rendered consumer crate at its root: a library crate with a deliberate
clippy violation in `src/lib.rs` (`unused` binding).

- Caller: `.github/workflows/fixture--promotion.yaml` pins candidate
  `b17f64dcedf54a0104a2174d0de04ac8b71a4c86`.
- Expected verdict: red in the lint gate on dev and release, and red in the
  leading validation step for the bogus-profile call — never in checkout or
  workflow resolution.
- Runs on every push to this branch. First tag and all moves remain
  operator-owned.
