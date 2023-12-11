pub use anyhow::Result;
use std::{env, fs};

pub enum Part {
    Part1,
    Part2,
    Both,
}
pub enum Input {
    Input,
    Sample,
}

pub fn parse(file: &str) -> Result<Vec<String>> {
    let args: Vec<String> = env::args().collect();
    let mut path = env::current_dir()?.join("src/");

    if let Some(day) = args[0].split('/').last() {
        path = path.join(day).join(file);
    }

    let input = fs::read_to_string(path)?;
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    Ok(lines)
}

pub fn run() -> Part {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "part1" => return Part::Part1,
            "part2" => return Part::Part2,
                _ => Part::Both,
        }
    }
    else {
        return Part::Both;
    }
}

pub fn input() -> Input {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        match args[2].as_str() {
            "sample" => return Input::Sample,
            "input" => return Input::Input,
                _ => return Input::Input,
        }
    }
    else {
        return Input::Input;
    }
}
