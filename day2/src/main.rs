use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let re = Regex::new("^(\\d*)-(\\d*) (.): (.*)$").unwrap();
    let mut valid_ct = 0;
    if let Ok(lines) = read("./input") {
        for line in lines {
            if let Ok(ip) = line {
                let caps = re.captures(&ip).unwrap();
                let min = caps.get(1).map_or(1000000, |m| m.as_str().parse().unwrap());
                let max = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap());
                let target = caps.get(3).map_or("", |m| m.as_str());
                let word = caps.get(4).map_or("", |m| m.as_str());
                let ct = word.matches(target).count();
                if ct <= max && ct >= min {
                    valid_ct = valid_ct + 1;
                }
            }
        }
    }
    println!("{}", valid_ct);
}

fn read<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}