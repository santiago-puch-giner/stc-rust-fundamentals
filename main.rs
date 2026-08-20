use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    // Print the uppercase version.
    println!("{}", line.to_uppercase());
}
