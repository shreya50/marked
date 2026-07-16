use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if let Some(path) = std::env::args().nth(1) {
        input = std::fs::read_to_string(path).unwrap_or_else(|error| { eprintln!("marked-rs: {error}"); std::process::exit(1) });
    } else {
        io::stdin().read_to_string(&mut input).expect("failed to read standard input");
    }
    print!("{}", marked_rs::parse(&input));
}
