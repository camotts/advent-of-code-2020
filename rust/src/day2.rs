use aoc_runner_derive::{aoc};
use regex::Regex;

#[aoc(day2, part1)]
pub fn verify_password_count(input: &str) -> usize {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\d*)-(\d*) (.): (.*)$").unwrap();
    }
    input.lines()
    .filter_map(|l| {
        let caps = PATTERN.captures(l).unwrap();
        let min: usize = caps.get(1)?.as_str().parse().unwrap();
        let max: usize = caps.get(2)?.as_str().parse().unwrap();
        let target = caps.get(3)?.as_str();
        let word = caps.get(4)?.as_str();
        let ct = word.matches(target).count();
        if ct <= max && ct >= min {
            Some(1)
        } else {
            None
        }
    })
    .count()
}