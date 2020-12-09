use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|m| {
        m.parse().unwrap()
    }).collect()
}

#[aoc(day1, part1)]
pub fn find_sum(input: &[u32]) -> u32 {
    let mut iter = 1;
    match input.iter().find_map(|a| {
        match input.iter().skip(iter).find_map(|b| {
            if a + b == 2020 {
                Some(b)
            } else {
                None
            }
        }) {
            Some(x) => Some(x * a),
            None => {
                iter = iter + 1;
                None
            }
        }
    }) {
        Some(x) => x,
        None => 0
    }
}

#[aoc(day1, part2)]
pub fn find_sum_3(input: &[u32]) -> u32 {
    let mut iter = 1;
    let mut inner_iter = iter;
    match input.iter().find_map(|a| {
        inner_iter = iter;
        match input.iter().skip(iter).find_map(|b| {
            match input.iter().skip(inner_iter).find_map(|c| {
                if a + b + c == 2020 {
                    Some(c)
                } else {
                    None
                }
            }) {
                Some(x) => Some(x * b),
                None => {
                    inner_iter = inner_iter + 1;
                    None
                }
            }
        }) {
            Some(x) => Some(x * a),
            None => {
                iter = iter + 1;
                None
            }
        }
    }) {
        Some(x) => x,
        None => 0
    }
}