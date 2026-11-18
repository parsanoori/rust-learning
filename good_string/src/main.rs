use std::io;

fn good_string(s: &str) -> usize {
    let mut a_count = 0;
    for c in s.chars() {
        if c == 'a' {
            a_count += 1;
        }
    }

    if a_count > s.len() / 2 {
        return s.len();
    }

    if a_count == 0 {
        return 0;
    }

    let res = a_count * 2 - 1;
    res
}

fn get_string() -> String {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("expected a string as an input");

    line.trim_end().to_string()
}

fn main() {
    let s = get_string();
    println!("{}", good_string(&s));
}
