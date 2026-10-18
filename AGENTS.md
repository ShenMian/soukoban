# AGENTS.md

Instructions for agents working in this repository. Keep this file small and
about how to work here, not about what the project is.

## Repository

`soukoban` is a Rust library implementing Sokoban algorithms and
data structures. Read `README.md` for features and usage, `docs/benchmark.md`
for benchmark methodology. `src/lib.rs` is the module map. The README is
included as crate-level docs, so its example code must keep compiling.

## Workflow

- Edit the working tree only; do not modify the git index (staging area)
  unless the user explicitly asks, including `git add`, `git reset`,
  `git restore --staged`, `git rm --cached`, `git stash`, or any other command
  that writes to the index.
- Do not commit or push unless the user explicitly asks.
- Do not add dependencies without asking. The dependency set is deliberately
  small.
- Do not modify files under `assets/`.

## Commands

- `cargo +nightly fmt` — required after edits; nightly is only for formatting
  (`rustfmt.toml` uses nightly-only options), build and test on stable.
- `cargo clippy --workspace --all-targets --all-features -- --deny warnings` —
  CI treats warnings as errors.
- `cargo test --workspace --all-features --all-targets` — main test suite.
- `cargo test --workspace --all-features --doc` — doctests, run separately.
- `cargo llvm-cov --workspace --all-features --no-cfg-coverage` — test
  coverage summary.
- `cargo doc --no-deps` — check docs build; `missing_docs` is denied.
- `cargo bench` — criterion benchmarks in `benches/`; slow, run only when the
  task is about performance.

## Code style

- Write code, docs, and comments in the style of the surrounding code. When a
  question is not settled by the codebase, follow the Rust standard library
  conventions.
- Keep comments to a strict minimum. Only comment non-trivial code, or public
  definitions whose arguments are not self-explanatory. Do not remove existing
  comments unless they are directly related to your change.
- Every public item needs a doc comment (`#![deny(missing_docs)]`). Document
  public panics, errors, and unsafe contracts under `# Panics` / `# Errors` /
  `# Safety` when they apply.
- Prefer the re-exported `FxHashMap` / `FxHashSet` over `std::collections`
  types in library code.
- No emoji anywhere. Code, comments, docs, and commits are in English.

## Tests

- Follow the conventions of the existing tests in `tests/`, including their
  shared helpers.
- A bug fix should come with a regression test.

## Commits

- Follow Conventional Commits with a module scope: `type(scope): summary`,
  e.g. `perf(path_finding): reuse reachable distances in box waypoints`.
  Common types: `feat`, `fix`, `perf`, `refactor`, `style`, `docs`, `test`,
  `ci`, `build`.
- Mark public API breaking changes with `!` before the colon:
  `refactor(map)!: restrict with_dimensions visibility`.
- Only create commits when the user asks.
