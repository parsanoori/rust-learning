use rand;
use std::io;

const NUMBER_OF_SIMS: i32 = 1000;
const BATCH: i32 = 100;

fn get_steps() -> u32 {
    let mut n_str = String::new();

    io::stdin()
        .read_line(&mut n_str)
        .expect("expetced valid input");

    let n: u32 = match n_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("expected valid number");
        }
    };

    n
}

fn move_1d(s: u32) -> i32 {
    let mut disp: i32 = 0;
    for _ in 0..s {
        let m = rand::random_range(1..=3);
        if m == 1 {
            disp += 1;
        } else if m == 2 {
            disp -= 1;
        } else if m == 3 {
            // stay
        }
    }
    disp
}

fn simulate_1d(s: u32) -> f32 {
    let mut sum: f32 = 0.0;
    for _ in 0..NUMBER_OF_SIMS {
        sum += move_1d(s).abs() as f32;
    }
    sum / (NUMBER_OF_SIMS as f32)
}

fn move_3d(s: u32) -> f64 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;

    for _ in 0..s {
        let m = rand::random_range(1..=7);
        if m == 1 {
            x += 1;
        } else if m == 2 {
            x -= 1;
        } else if m == 3 {
            y += 1;
        } else if m == 4 {
            y -= 1;
        } else if m == 5 {
            z += 1;
        } else if m == 6 {
            z -= 1;
        } else {
            // stay
        }
    }

    let move_sqrd = x * x + y * y + z * z;
    let res = (move_sqrd as f64).sqrt();
    res
}

fn simulate_3d(s: u32) -> f64 {
    let mut avg: f64 = 0.0;
    for _ in 0..NUMBER_OF_SIMS / BATCH {
        let mut sum: f64 = 0.0;
        for _ in 0..BATCH {
            let moved = move_3d(s);
            sum += moved;
        }
        let to_add = sum / (BATCH as f64);
        avg += to_add;
    }
    avg / ((NUMBER_OF_SIMS / BATCH) as f64)
}

fn main() {
    let s = get_steps();
    let average_1d = simulate_1d(s);
    let average_3d = simulate_3d(s);

    println!(
        "Average disposition of a one dimensional random walk in {} steps is: {:.1} unit",
        s, average_1d
    );
    println!(
        "Average disposition of a three dimensional random walk in {} steps is: {:.1} unit",
        s, average_3d
    );
}
