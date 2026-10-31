## 1. Doctest gate

- [x] 1.1 Add the `cargo-doctest` hook (`bash -c 'test -f Cargo.lock || cargo generate-lockfile; exec cargo test --doc --locked'`) on pre-commit and pre-merge-commit stages, rendered only when the crate shape includes library
- [x] 1.2 Confirm all three copiertv variants render and validate, including a doctest-bearing probe render

## 2. Tag

- [ ] 2.1 Tag and push after review approval
