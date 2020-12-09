use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<HashSet<char>>> {
    input.split("\n\n").map(|m| {
        let ret = m.lines().map(|l| {
            l.chars().collect::<HashSet<char>>()
        }).collect::<Vec<_>>();
        ret
    }).collect::<Vec<Vec<_>>>()
}

#[aoc(day6, part1)]
pub fn any_yes(input: &Vec<Vec<HashSet<char>>>) -> u32 {
    let ret = input.iter().map(|m| {
        let mut yeses: HashSet<char> = HashSet::new();
        m.iter().for_each(|a| {
            yeses = yeses.union(&a).copied().collect()
        });
        yeses.len() as u32
    }).collect::<Vec<_>>();
    ret.iter().sum()
}

#[aoc(day6, part2)]
pub fn all_yes(input: &Vec<Vec<HashSet<char>>>) -> u32 {
    let ret = input.iter().map(|m| {
        let mut yeses = m[0].clone();
        m.iter().skip(1).for_each(|a| {
            yeses = yeses.intersection(&a).copied().collect()
        });
        yeses.len() as u32
    }).collect::<Vec<_>>();
    ret.iter().sum()
}