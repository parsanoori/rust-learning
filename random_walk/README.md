# Random Walk

**Time limit:** 10 seconds · **Memory limit:** 64 MB

Random walk is one of the best-known modelling problems: the path of a moving object that moves randomly in some space. You must compute how far, on average, an object ends up from the origin after a given number of steps.

## Rules

- The number of steps is read from standard input.
- In each step the object moves randomly along one of the Cartesian axis directions, or stays in place. In 2D, for example, it could go left, right, up, down, or not move at all.
- Every one of these options has the same probability.
- To compute the average, run the simulation `n` times. Each run's result is the absolute distance of the object from the origin; average all of them.
- The choice of `n` is yours. Too few runs and the average is inaccurate; too many and the program exceeds the time limit.

## Tasks

1. Implement a function that takes the number of steps and, using the standard random number generator, simulates a **one-dimensional** walk (left / right / stay) and returns the displacement after that many steps.
2. **Bonus (50 points):** do the same for **three-dimensional** space.
3. Implement a function that takes the number of steps and computes the average displacement with acceptable accuracy, calling the previous function many times and averaging the results of the `n` runs.

Notes:

- The logic must live outside `main`; `main` should only handle input/output and call other functions. Helper functions are allowed.
- Everything except random number generation and square roots must be implemented by yourself.
- For the non-bonus part only one dimension is required (left and right). The 2D example above is just for intuition.
- Judging for this problem differs from the others and may take longer.

## Input

One line with the non-negative integer `s`, the number of steps.

`0 ≤ s ≤ 10000`

## Output

The average distance from the origin, with one decimal digit. Pay close attention to the exact output format.

## Examples

Input:
```
3
```
Output:
```
Average disposition of a one dimensional random walk in 3 steps is: 1.1 unit
```

Bonus example, input:
```
210
```
Output:
```
Average disposition of a one dimensional random walk in 210 steps is: 9.4 unit
Average disposition of a three dimensional random walk in 210 steps is: 12.6 unit
```

## Testing

Since the output is random, `tester.cpp` (from the problem's test archive) checks the format with a regex and accepts an answer within ±5 of the expected value. `run_tests.sh` compiles it with `g++` automatically.

```sh
./run_tests.sh random_walk
```

## Implementation notes

- A step is left/right/stay with probability 1/3 each in 1D, and ±x/±y/±z/stay with probability 1/7 each in 3D. The expected outputs (e.g. 10.3 for 246 steps) only match when "stay" is included.
- Each average uses 10,000 simulated walks (about 1.6 s for the largest input).
