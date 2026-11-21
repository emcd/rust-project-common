# Fixture branch: promotion-lib

Template-owned promotion fixture (see `fixtures/promotion/README.md` on
`master` and the archived xrepo-shared-workflows proposal). This branch holds
a real rendered consumer crate at its root: a library crate with clean gates.

- Caller: `.github/workflows/fixture--promotion.yaml` pins candidate
  `b17f64dcedf54a0104a2174d0de04ac8b71a4c86`.
- Expected verdict: green on dev and release (`has-library: true`).
- Runs on every push to this branch. First tag and all moves remain
  operator-owned.
