use std::io;

struct Point {
    x: i32,
    y: i32,
}

struct Circle {
    p: Point,
    r: i32,
}

fn get_circle() -> Circle {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("expected a string as an input");

    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if numbers.len() < 3 {
        panic!("invalid Circle on input")
    }
    Circle {
        p: Point {
            x: numbers[0],
            y: numbers[1],
        },
        r: numbers[2],
    }
}

fn get_point() -> Point {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("expected a string as an input");

    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if numbers.len() < 2 {
        panic!("invalid Point on input")
    }

    Point {
        x: numbers[0],
        y: numbers[1],
    }
}

fn in_circle(c: &Circle, p: &Point) -> bool {
    let d_x = c.p.x - p.x;
    let d_y = c.p.y - p.y;

    let d2_x = d_x * d_x;
    let d2_y = d_y * d_y;

    let d2 = d2_x + d2_y;

    let r2 = c.r * c.r;

    d2 < r2
}

fn main() {
    let c_red = get_circle();
    let c_green = get_circle();
    let c_blue = get_circle();

    let p = get_point();

    let in_red = in_circle(&c_red, &p);
    let in_green = in_circle(&c_green, &p);
    let in_blue = in_circle(&c_blue, &p);

    if in_red {
        if in_green {
            if in_blue {
                println!("8");
            } else {
                println!("4");
            }
        } else {
            if in_blue {
                println!("6");
            } else {
                println!("2");
            }
        }
    } else {
        if in_green {
            if in_blue {
                println!("7");
            } else {
                println!("3");
            }
        } else {
            if in_blue {
                println!("5");
            } else {
                println!("1");
            }
        }
    }
}
