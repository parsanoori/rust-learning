use std::io;

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

fn gcd(n: u64, m: u64) -> u64 {
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    if n < m {
        return gcd(n, m-n);
    } else {
        return gcd(m,n-m);
    }
}

fn main() {
    let numbers = get_numbers();
    println!("{}", gcd(numbers.0, numbers.1));
}

