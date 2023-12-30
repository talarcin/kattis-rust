use icy_itinerary::solve;
use std::io::{self, BufRead};

fn main() {
    let lines = read_lines();
    println!("{}", solve(lines));
}

fn read_lines() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if line == String::from("") {
            break;
        }
        lines.push(line.trim().to_string());
    }

    lines
}
