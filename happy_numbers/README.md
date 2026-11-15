# Happy and Sad Numbers

**Time limit:** 1 second · **Memory limit:** 64 MB

Take a natural number `n`. At each step, replace `n` with the sum of the squares of its digits. Continue until you reach 1 or 4. If you reach 1, `n` is a *happy* number; otherwise it is *sad*.

Examples:

- `23 → 2² + 3² = 13 → 1² + 3² = 10 → 1² + 0² = 1` — so 23 is happy.
- `24 → 2² + 4² = 20 → 2² + 0² = 4` — so 24 is sad.

Write a function that tells whether a number is happy or sad, then a program that reads a natural number and reports which it is.

*Note:* it can be proven mathematically that this process always terminates, so there is no risk of an infinite loop.

## Input

A single line containing `n` (`1 ≤ n ≤ 10⁴`).

## Output

Print `HAPPY` if the number is happy, otherwise `SAD`.

## Examples

| Input | Output |
|-------|--------|
| `23` | `HAPPY` |
| `24` | `SAD` |

## Run

```sh
cargo run --release < in/input1.txt
../run_tests.sh happy_numbers
```
