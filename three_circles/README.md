# Three Circles

**Time limit:** 1 second · **Memory limit:** 64 MB

Write a function that takes the description of a circle and a point and decides whether the point lies inside the circle. Then, using that function, write a program that reads three circles and a point and determines, according to the figure of the problem (three overlapping circles: red, green and blue, splitting the plane into 8 regions), which region the point falls in.

## Input

- Line 1: `x y r` of the **red** circle (center coordinates, then radius).
- Line 2: `x y r` of the **green** circle.
- Line 3: `x y r` of the **blue** circle.
- Line 4: `x y` of the point.

All numbers are integers.

## Output

A single line with the number of the region containing the point.

## Examples

| Input | Output |
|-------|--------|
| `2 2 8`<br>`-2 2 8`<br>`0 0 8`<br>`0 -1` | `8` |
| `3 2 2`<br>`6 7 4`<br>`8 0 6`<br>`1 4` | `1` |

## Region numbering

| Region | Inside red | Inside green | Inside blue |
|--------|:---:|:---:|:---:|
| 1 | no | no | no |
| 2 | yes | no | no |
| 3 | no | yes | no |
| 4 | yes | yes | no |
| 5 | no | no | yes |
| 6 | yes | no | yes |
| 7 | no | yes | yes |
| 8 | yes | yes | yes |

(The figure is not reproduced here; the numbering above is what the solution implements and it passes all test cases.)

## Run

```sh
cargo run --release < in/input1.txt
../run_tests.sh three_circles
```
