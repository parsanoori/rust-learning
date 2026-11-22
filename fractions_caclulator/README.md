# Fraction Calculator

Implement a simple calculator for fractions. It supports the four basic operations and then simplifies the result as much as possible. A fraction **must** be stored using a struct.

## Input

The first 4 lines give the numerator and denominator of two fractions (first fraction's numerator and denominator, then the second's). They are integers, and the denominators are guaranteed to be non-zero.

The next line contains one of the characters `+ - * /` as the operator.

## Output

The simplified result, formatted as in the examples.

## Examples

| Input (one value per line) | Output |
|----------------------------|--------|
| `2` `4` `3` `6` `*` | `1/4` |
| `2` `4` `3` `6` `-` | `0/1` |
| `3` `7` `12` `28` `+` | `6/7` |

*Note:* the original statement shows `0/24` for the second example, but the test data in `out/` expects `0/1`.
