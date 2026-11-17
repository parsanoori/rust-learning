# Rust Learning

Small programs written while learning Rust. Each folder is its own Cargo project.

| Project | Topic | Status | Tests |
|---------|-------|:------:|:-----:|
| [guessing_game](guessing_game/) | Rust book ch. 2: stdin, `match`, loops, `rand` crate | ✅ | — |
| [happy_numbers](happy_numbers/) | Functions, loops, digit arithmetic | ✅ | 50/50 |
| [complete_numbers](complete_numbers/) | Functions, loops, ranges | ✅ | 10/10 |
| [random_walk](random_walk/) | `rand` crate, closures/iterators, floats, simulation | ✅ | 30/30 |
| [three_circles](three_circles/) | Structs, functions, parsing input, nested conditionals | ✅ | 10/10 |
| [fast_power](fast_power/) | Modular exponentiation, `u64` arithmetic | ✅ | 25/25 |
| [gcd](gcd/) | Recursion, Euclid-style GCD | ✅ | 25/25 |

## Testing

Each problem folder (except `guessing_game`) has `in/inputN.txt` and `out/outputN.txt` test cases. Run:

```sh
./run_tests.sh                  # all problems
./run_tests.sh happy_numbers    # a single problem
```

The script builds the project with `cargo build --release`, feeds every input to the binary (per-problem timeout, 1 second by default) and compares the output with the expected one, ignoring trailing whitespace. Problems with a `tester.cpp` (random_walk, whose output is random) are checked with that tester instead, compiled with `g++`. It exits non-zero if any test fails.

The original test archives (`*.zip`) are git-ignored.

## Concepts covered so far

- Cargo projects, dependencies (`rand`)
- Reading input from stdin, parsing with `parse()` and error handling via `match`
- Functions, structs, borrowing (`&Circle`, `&Point`)
- Loops, ranges (`a..=b`), integer arithmetic
- Recursion, modular arithmetic (square-and-multiply)
