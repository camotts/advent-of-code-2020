use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|i| {
        i.chars().fold(0, |acc, c| {
            if c == 'F' || c == 'L' {
                acc << 1
            } else {
                (acc << 1) + 1
            }
        })
    }).collect()
}

#[aoc(day5, part1)]
pub fn highest_seat(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

#[inline]
fn sum_from_to(low: u32, high: u32) -> u32 {
    (low..=high).sum()
}

#[aoc(day5, part2)]
pub fn find_id(input: &[u32]) -> u32 {
    sum_from_to(*input.iter().min().unwrap(), *input.iter().max().unwrap())
        - input.iter().sum::<u32>()
}