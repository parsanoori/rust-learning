# Complete Numbers

**Time limit:** 1 second · **Memory limit:** 64 MB

A *perfect (complete) number* is a positive integer equal to the sum of its proper positive divisors (all positive divisors except the number itself).

Write a function that checks whether a number is perfect. Then, using it, write a program that prints all perfect numbers in the range `[a, b]`.

## Input

- Line 1: the number `a`.
- Line 2: the number `b`.

`1 ≤ a ≤ b ≤ 10000`

## Output

Print each perfect number in the range on its own line.

## Examples

| Input | Output |
|-------|--------|
| `1`<br>`100` | `6`<br>`28` |
| `300`<br>`10000` | `496`<br>`8128` |

## Run

```sh
cargo run --release < in/input1.txt
../run_tests.sh complete_numbers
```
