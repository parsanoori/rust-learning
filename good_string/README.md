# Good String

**Time limit:** 1 second · **Memory limit:** 64 MB

Arash loves the letter `a`, because his name starts with "A". He calls a string *good* if more than half of its characters are `a`. For example, `aaacb` and `acaa` are good, but `baba` and `abbba` are not.

Arash can turn a string into a good string by deleting some of its characters. Write a program that reads a string and computes the maximum length of a good string Arash can build by deleting characters from it. The input string may already be good; in that case Arash deletes nothing.

It is guaranteed that the input contains at least one `a`.

## Input

A single line containing the string `S`, made only of lowercase English letters.

`1 ≤ |S| ≤ 10⁷`

## Output

Print the maximum length of a good string that can be obtained by deleting characters from `S`.

## Examples

| Input | Output |
|-------|--------|
| `xaxxxxa` | `3` |
| `aaabaa` | `6` |
