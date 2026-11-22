use std::io;

fn get_number() -> i32 {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("expected a string as an input");

    line.trim().parse().expect("expected a number")
}

fn get_action() -> char {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("expected a string as an input");

    line.trim()
        .chars()
        .next()
        .expect("at least a character mut be given")
}

fn division(a: i32, b: i32, c: i32, d: i32) -> (i32, i32) {
    (a * d, b * c)
}

fn multiplication(a: i32, b: i32, c: i32, d: i32) -> (i32, i32) {
    (a * c, b * d)
}

fn add(a: i32, b: i32, c: i32, d: i32) -> (i32, i32) {
    (a * d + c * b, b * d)
}

fn minus(a: i32, b: i32, c: i32, d: i32) -> (i32, i32) {
    (a * d - c * b, b * d)
}

fn gcd(n: i32, m: i32) -> i32 {
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

fn simplify(a: i32, b: i32) -> (i32, i32) {
    let mut sign = 1;

    let mut abs_a = a;
    if a < 0 {
        abs_a = -a;
        sign *= -1;
    }
    let mut abs_b = b;
    if b < 0 {
        abs_b = -b;
        sign *= -1;
    }
 
    let g = gcd(abs_a,abs_b);
    (sign * abs_a / g, abs_b / g)
}

fn main() {
    let a = get_number();
    let b = get_number();
    let c = get_number();
    let d = get_number();

    let res: (i32, i32);
    match get_action() {
        '/' => res = division(a, b, c, d),
        '*' => res = multiplication(a, b, c, d),
        '+' => res = add(a, b, c, d),
        '-' => res = minus(a, b, c, d),
        _ => panic!("invalid action bro!"),
    }

    let simple_res = simplify(res.0, res.1);

    println!("{}/{}", simple_res.0, simple_res.1);

}
