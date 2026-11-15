# Guessing Game

The guessing game from chapter 2 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html). It has no test cases, so there is nothing to run through `run_tests.sh`.

The program picks a random number between 1 and 100 and repeatedly asks for a guess, answering "Too small!", "Too big!" or "You win!".

Concepts practiced: `let`/`mut`, reading stdin, `String`, `match` on `Result` and `Ordering`, `loop`/`break`/`continue`, and using an external crate (`rand`).

> Note: the program prints the secret number on startup (debugging leftover from the tutorial).

## Run

```sh
cargo run
```
