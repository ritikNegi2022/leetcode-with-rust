# rust-leetcode

Collection of Rust solutions for LeetCode-style problems.

## Overview

- Language: Rust
- Purpose: solve and track algorithmic problems as small, self-contained modules.

## Structure

- `src/` — Rust source code
- `src/problems/` — individual problem folders, each with a `mod.rs` implementing a `run()` function

## How to run

Run the project with Cargo:

```bash
cargo run
```

This executes `src/main.rs`, which will call problem `run()` functions; individual problem modules also print example output when executed.

## Adding problems

1. Create a new folder under `src/problems/` named with the problem id and a short slug (e.g. `p123_some_title`).
2. Add a `mod.rs` implementing a `run()` function and the solution.
3. Export or call the module from `src/problems/mod.rs` and/or `src/main.rs` as appropriate.

## Problems

See the `src/problems` directory for per-problem READMEs.
