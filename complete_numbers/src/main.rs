use std::io;

fn is_complete(n: u32) -> bool {
    let mut sum: u32 = 0;
    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }
    sum == n
}

fn main() {

    let mut start_str = String::new();
    let mut end_str = String::new();

    io::stdin().read_line(&mut start_str)
        .expect("failed to read line");

    io::stdin().read_line(&mut end_str)
        .expect("failed to read line");


    let start: u32 = match start_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("starting value expected to be a number");
            return;
        }
    };

    let end: u32 = match end_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ending value expected to be a number");
            return;
        }
    };

    for n in start..=end {
        if is_complete(n) {
            print!("{}\n", n);
        }
    }

}
