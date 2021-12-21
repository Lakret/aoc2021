# Polyglot Advent of Code 2021

The following languages are used:

- Elixir 1.12.2
- Python 3.8.6
- Julia 1.6.3
- Rust 1.56.1

You can execute a day's solution by running the corresponding shell script,
provided you have the used language installed.

## Julia Dependencies

For most of those puzzles, I try to go dependency free.
However, in case of Julia, I'm a bit more relaxed with this, since I also want to see the ecosystem a bit.
To that end, the following deps have been used:

```sh
DataStructures # in d15, for PriorityQueue
Pipe # all days
Plots # d11, for visualization
ProfileView
StatsBase # d03, for mode function
```

## Running Rust Tests

To run tests of a particular day in Rust projects, replace `d20` below
with the corresponding project name, and execute:

```sh
cargo test --bin d20
```

Or, if you'd like to see output of `dbg!`, `println!`, etc.:

```sh
cargo test --bin d20 -- --nocapture
```
