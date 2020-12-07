use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::fs;

fn main() {
    problem2();
}

fn problem1() {
    let data = fs::read_to_string("input").unwrap();
    let splits = data.split("\r\n\r\n");
    let mut ct = 0;
    for s in splits {
        if s.contains("byr") && s.contains("iyr") && s.contains("eyr") && s.contains("hgt") && s.contains("hcl") && s.contains("ecl") && s.contains("pid") {
            ct = ct + 1;
        }
    }
    println!("{}", ct);
}

fn problem2() {
    let data = fs::read_to_string("input").unwrap();
    let splits = data.split("\r\n\r\n");
    let mut ct = 0;
    let re = Regex::new("(\\w+):([^\\s]+)").unwrap();
    let heightRegex = Regex::new("(\\d+)(cm|in)").unwrap();
    let colorRegex = Regex::new("#[a-f0-9]{6}").unwrap();
    let passportRegex = Regex::new("\\d{9}").unwrap();
    for s in splits {
        if s.contains("byr") && s.contains("iyr") && s.contains("eyr") && s.contains("hgt") && s.contains("hcl") && s.contains("ecl") && s.contains("pid") {
            let ret = re.find_iter(s).all(|cap| {
                let split = cap.as_str().split(":").collect::<Vec<&str>>();
                match split[0] {
                    "byr" => {
                        let byr: u32 = split[1].parse().unwrap();
                        if byr > 2002 || byr < 1920 {
                            //println!("Not the right age! {}", byr);
                            return false
                        }
                    }
                    "iyr" => {
                        let iyr: u32 = split[1].parse().unwrap();
                        if iyr > 2020 || iyr < 2010 {
                            //println!("Not the right issue! {}", iyr);
                            return false
                        }
                    }
                    "eyr" => {
                        let eyr: u32 = split[1].parse().unwrap();
                        if eyr > 2030 || eyr < 2020 {
                            //println!("Not the right expiration! {}", eyr);
                            return false
                        }
                    }
                    "hgt" => {
                        match heightRegex.captures(split[1]) {
                            Some(x) => {
                                if x.get(2).map_or("", |m| m.as_str()) == "in" {
                                    let height = x.get(1).map_or(100, |m| m.as_str().parse().unwrap());
                                    if height > 76 || height < 59 {
                                        //println!("Not the right height! {}", height);
                                        return false
                                    }
                                } else if x.get(2).map_or("", |m| m.as_str()) == "cm" {
                                    let height = x.get(1).map_or(200, |m| m.as_str().parse().unwrap());
                                    if height > 193 || height < 150 {
                                        //println!("Not the right height! {}", height);
                                        return false
                                    }
                                } else {
                                    return false
                                }
                            },
                            None => return false
                        }
                        
                    }
                    "hcl" => {
                        if !colorRegex.is_match(split[1]) {
                            //println!("Not the right hair color! {}", split[1]);
                            return false
                        }
                    }
                    "ecl" => {
                        if !split[1].contains("amb") && !split[1].contains("blu") && !split[1].contains("brn") && !split[1].contains("gry") && !split[1].contains("grn") && !split[1].contains("hzl") && !split[1].contains("oth") {
                            //println!("Not the right eye color! {}", split[1]);
                            return false
                        }
                    }
                    "pid" => {
                        if !passportRegex.is_match(split[1]) {
                            //println!("Not the right pid! {}", split[1]);
                            return false
                        }
                    }
                    &_ => {}
                }
                true
            });
            if ret{
                println!("---------------------------");
                println!("{}", s);
                ct = ct + 1;
            }
        }
    }
    println!("{}", ct);
}
