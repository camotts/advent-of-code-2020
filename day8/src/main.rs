use std::fs::File;
use std::io::{self, BufRead};
use std::fs;

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("testinput").unwrap();
    let datavec: Result<Vec<_>,_> = data.lines().collect();
}
