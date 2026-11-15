use rand::Rng;
use std::io;

/// Number of simulated walks that are averaged together.
const NUMBER_OF_SIMS: u32 = 10_000;

fn get_steps() -> u32 {
    let mut n_str = String::new();

    io::stdin()
        .read_line(&mut n_str)
        .expect("expected valid input");

    n_str.trim().parse().expect("expected valid number")
}

/// Simulates one 1D walk: each step is left, right or stay (equal chance).
/// Returns the final displacement from the origin.
fn walk_1d(s: u32, rng: &mut impl Rng) -> i32 {
    let mut pos: i32 = 0;
    for _ in 0..s {
        match rng.random_range(0..3) {
            0 => pos -= 1,
            1 => pos += 1,
            _ => {}
        }
    }
    pos
}

/// Simulates one 3D walk: each step is +/-x, +/-y, +/-z or stay (equal chance).
/// Returns the distance from the origin.
fn walk_3d(s: u32, rng: &mut impl Rng) -> f64 {
    let (mut x, mut y, mut z) = (0i64, 0i64, 0i64);
    for _ in 0..s {
        match rng.random_range(0..7) {
            0 => x += 1,
            1 => x -= 1,
            2 => y += 1,
            3 => y -= 1,
            4 => z += 1,
            5 => z -= 1,
            _ => {}
        }
    }
    ((x * x + y * y + z * z) as f64).sqrt()
}

/// Average absolute distance from the origin over many 1D walks.
fn average_1d(s: u32) -> f64 {
    let mut rng = rand::rng();
    let sum: f64 = (0..NUMBER_OF_SIMS)
        .map(|_| walk_1d(s, &mut rng).abs() as f64)
        .sum();
    sum / NUMBER_OF_SIMS as f64
}

/// Average distance from the origin over many 3D walks.
fn average_3d(s: u32) -> f64 {
    let mut rng = rand::rng();
    let sum: f64 = (0..NUMBER_OF_SIMS).map(|_| walk_3d(s, &mut rng)).sum();
    sum / NUMBER_OF_SIMS as f64
}

fn main() {
    let s = get_steps();

    println!(
        "Average disposition of a one dimensional random walk in {} steps is: {:.1} unit",
        s,
        average_1d(s)
    );
    println!(
        "Average disposition of a three dimensional random walk in {} steps is: {:.1} unit",
        s,
        average_3d(s)
    );
}
