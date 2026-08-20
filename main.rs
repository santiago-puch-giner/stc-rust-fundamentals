use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let a: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();
    let b: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();
    // Print a + b.
    println!("{}", a + b);
}
