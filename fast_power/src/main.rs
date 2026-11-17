use std::io;

const MOD: u64 = 1_000_000_007;

fn get_numbers() -> (u64, u64) {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("expected a string as an input");

    let numbers: Vec<u64> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    if numbers.len() < 2 {
        panic!("invalid numbers on input")
    }

    (numbers[0], numbers[1])
}

fn fast_power(n: u64, mut m: u64) -> u64 {
    let mut res = 1;
    let mut n_2p = n;
    while m != 0 {
        if m % 2 == 1 {
            res = (res * n_2p) % MOD;
        }
        n_2p = (n_2p * n_2p) % MOD;
        m /= 2;
    }
    res
}

fn main() {
    let numbers = get_numbers();
    let n = numbers.0;
    let m = numbers.1;


    println!("{}", fast_power(n, m));
}
