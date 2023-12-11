pub use anyhow::Result;
use std::{env, fs, path::PathBuf};

pub enum Part {
    Part1,
    Part2,
    Both,
}

pub fn parse(file: String) -> Result<Vec<String>> {
    let mut path: PathBuf = env::current_exe()?
        .to_str()
        .unwrap()
        .replace("/target/debug", "/src")
        .into();

    path = path.join(file);

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
    } else {
        return Part::Both;
    }
}

pub fn input() -> String {
    let args: Vec<String> = env::args().collect();
    match args.get(2) {
        Some(sample) => return sample.clone() + ".txt",
        None => "input.txt".to_string(),
    }
}

#[macro_export]
macro_rules! main {
    () => {
        fn main() -> Result<()> {
            let file = input();

            let input = parse(file)?;

            match run() {
                Part::Both => {
                    part1(input.clone())?;
                    part2(input)?;
                }
                Part::Part1 => part1(input)?,
                Part::Part2 => part2(input)?,
            };

            Ok(())
        }
    };
}
