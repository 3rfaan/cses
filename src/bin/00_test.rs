use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = String::new();

    input.read_line(&mut line).unwrap();

    let mut split = line.split_whitespace();
    let a: i32 = split.next().unwrap().parse().unwrap();
    let b: i32 = split.next().unwrap().parse().unwrap();

    println!("{}", a + b);
}
