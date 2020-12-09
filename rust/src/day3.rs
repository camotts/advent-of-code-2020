use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|m| {
        m.chars().collect()
    }).collect()
}

pub fn simulate(input: &Vec<Vec<char>>, down: usize, right: usize) -> u32 {
    let mut iter = 0;
    input.iter().skip(down).step_by(down).map(|m| {
        iter = iter + right;
        if iter >= m.len() {
            iter = iter - m.len();
        }
        if m[iter] == '#' {
            1
        } else {
            0
        }
    }).sum()
}

#[aoc(day3, part1)]
pub fn single_check(input: &Vec<Vec<char>>) -> u32 {
    simulate(input, 1, 3)
}

#[aoc(day3, part2)]
pub fn multi_check(input: &Vec<Vec<char>>) -> u32 {
    simulate(input, 1, 1) *
    simulate(input, 1, 3) *
    simulate(input, 1, 5) *
    simulate(input, 1, 7) *
    simulate(input, 2, 1)
}