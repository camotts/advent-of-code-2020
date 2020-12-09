use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|m| {
        m.parse().unwrap()
    }).collect()
}

static PREAMBLE: usize = 25;

#[aoc(day9, part1)]
pub fn find_odd_one(input: &[u64]) -> u64 {
    let mut cache = VecDeque::from(input[0..input.len()].to_vec());
    if input.len() >= PREAMBLE {
        cache = VecDeque::from(input[0..PREAMBLE].to_vec());
    }
    let mut iter = 0;
    match input[PREAMBLE..].iter().find_map(|check| {
        iter = 0;
        if !cache.iter().all(|m| {
            for j in iter .. cache.len() {
                if m + cache[j] == *check {
                    return false
                }
            }
            iter = iter + 1;
            true
        }) {
            cache.pop_front();
            cache.push_back(check.clone());
            None
        } else {
            Some(check)
        }
    }) {
        Some(x) => *x,
        None => 0
    }
}

#[aoc(day9, part2)]
pub fn  crack_the_code(input: &[u64]) -> u64 {
    let mut cache = VecDeque::from(input[0..input.len()].to_vec());
    if input.len() >= PREAMBLE {
        cache = VecDeque::from(input[0..PREAMBLE].to_vec());
    }
    let mut iter = 0;
    match input[PREAMBLE..].iter().find_map(|check| {
        iter = 0;
        if !cache.iter().all(|m| {
            for j in iter .. cache.len() {
                if m + cache[j] == *check {
                    return false
                }
            }
            iter = iter + 1;
            true
        }) {
            cache.pop_front();
            cache.push_back(check.clone());
            None
        } else {
            Some(check)
        }
    }) {
        None => 0,
        Some(found) => {
            iter = 0;
            let mut sum: u64 = 0;
            let mut maxContig = 0;
            let cacheVec = Vec::from(input);
            match cacheVec.iter().find_map(|m| {
                maxContig =iter+1;
                sum = 0;
                for j in iter .. cacheVec.len()-1 {
                    sum = sum + cacheVec[j] as u64;
                    if sum + cacheVec[j+1] > *found {
                        maxContig = j;
                        break
                    }
                }
                if sum == *found {
                    let max = cacheVec[iter..maxContig].iter().max()?;
                    let min = cacheVec[iter..maxContig].iter().min()?;
                    Some(max + min)
                } else {
                    iter = iter + 1;
                    None
                }
            }) {
                Some(x) => x,
                None => 0
            }
        }
    }
}