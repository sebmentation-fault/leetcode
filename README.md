# Leetcode

A repository for all my leetcode solutions (in Rust).

## Starting a new Problem

To start a new problem, in order, use the `add_new_problem.sh` script, which will update the root [Cargo.toml](./Cargo.toml) file.

It also creates a new directory in [problems/](./problems/), with suitable `main.rs` and `solution.rs` files. The `main.rs` file can be used to test the solution on a primitive case.

## Running a Problem.

Either:

1. `cd` into a specific problem directory, and run `cargo run`, or
2. In the root of this repo, run `cargo run -p problem_x` (where x is the problem to test).
