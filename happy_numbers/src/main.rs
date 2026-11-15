use std::{io};

fn happy(n: u32) -> bool {
    let mut sum: u32 = n;
    while sum != 1 && sum != 4 {
        let mut new_sum: u32 = 0;
        while sum != 0 {
            let dig = sum % 10;
            new_sum += dig * dig;
            sum /= 10;
        }
        sum = new_sum;
    }
    sum == 1
}
fn main() {
    let mut n_str = String::new();

    io::stdin()
        .read_line(&mut n_str)
        .expect("expetced valid input");

    let n: u32 = match n_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            print!("expected valid number");
            return;
        }
    };

    if happy(n) {
        print!("HAPPY");
    } else {
        print!("SAD");
    }
}
