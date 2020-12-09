use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::{HashSet};

#[derive(Clone)]
pub struct Command {
    name: String,
    //sign: char,
    number: i32,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Command> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\w{3}) (-\d+|\+\d+)$").unwrap();
    }
    input.lines().map(|m| {
        m.to_owned();
        let caps = PATTERN.captures(m).unwrap();
        Command {
            name: caps[1].into(),
            number: caps[2].parse().unwrap()
        }
    })
    .collect()
}

#[aoc(day8, part1)]
pub fn detect_loop(input: &[Command]) -> i32 {
    let mut iterator = 0;
    let mut acc = 0;
    let mut executed = HashSet::new();
    let mut next_command = vec![input[iterator].clone()];
    while let Some(command) = next_command.pop() {
        if executed.contains(&iterator) || input.len() < iterator{
            break
        }
        executed.insert(iterator);
        match &command.name[..] {
            "acc" => {
                acc = acc + command.number;
                iterator = iterator + 1;
            },
            "nop" => iterator = iterator + 1,
            "jmp" => iterator = (iterator as i32 + command.number) as usize,
            _ => {}
        }
        next_command.push(input[iterator].clone());
    }
    acc
}

pub fn detect_loop_option(input: Vec<Command>) -> Option<i32> {
    let mut iterator = 0;
    let mut acc = 0;
    let mut executed = HashSet::new();
    let mut next_command = vec![input[iterator].clone()];
    while let Some(command) = next_command.pop() {
        if executed.contains(&iterator) || input.len() < iterator{
            return None
        }
        executed.insert(iterator);
        match &command.name[..] {
            "acc" => {
                acc = acc + command.number;
                iterator = iterator + 1;
            },
            "nop" => iterator = iterator + 1,
            "jmp" => iterator = (iterator as i32 + command.number) as usize,
            _ => {}
        }
        if iterator < input.len() {
            next_command.push(input[iterator].clone());
        }
    }
    Some(acc)
}

#[aoc(day8, part2)]
pub fn fix_prog(input: &[Command]) -> i32 {
    let mut i: usize = 0;
    match input.iter().find_map(|m| {
        let mut new_commands = input.to_owned();
        
        match &m.name[..] {
            "nop" =>{
                let new_iter = i as i32 + m.number;
                if new_iter < 0 || new_iter >= input.len() as i32 {
                    return None
                }
                new_commands[i] = Command{
                    name: "jmp".to_string(),
                    number: m.number
                }
            },
            "jmp" =>{
                new_commands[i] = Command{
                    name: "nop".to_string(),
                    number: m.number
                }
            },
            _ => {}
        }
        i = i + 1;
        detect_loop_option(new_commands)
    }) {
        Some(x) => x,
        None => 0
    }
}