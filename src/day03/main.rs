use adventofcode2023::*;
use std::collections::{HashSet, HashMap};

main!();

#[derive(Clone, Debug)]
struct Number {
    num: String,
    coordinates: Vec<(usize, usize)>,
    include: bool,
}

impl Number {
    fn new() -> Self {
        Number {
            num: "".to_string(),
            coordinates: Vec::new(),
            include: false,
        }
    }
}

fn part1(input: Vec<String>) -> Result<()> {
    let mut numbers: Vec<Number> = Vec::new();
    let mut nearby_symbols: HashSet<(usize, usize)> = HashSet::new();

    for (x, line) in input.iter().enumerate() {
        let mut number = Number::new();
        for (y, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                number.num.push(char);
                number.coordinates.push((x, y));
            } else {
                if number.num.len() != 0 {
                    numbers.push(number.clone());
                    number = Number::new();
                }
                if char != '.' {
                    for xy in xy_to_check(x, y) {
                        nearby_symbols.insert(xy);
                    }
                }
            }
        }
        if number.num.len() != 0 {
            numbers.push(number.clone());
        }
    }

    numbers
        .iter_mut()
        .filter(|number| {
            number
                .coordinates
                .iter()
                .any(|xy| nearby_symbols.contains(xy))
        })
        .for_each(|number| {
            number.include = true;
        });

    let sum: i32 = numbers
        .iter()
        .filter(|number| number.include)
        .map(|number| number.num.parse::<i32>().unwrap())
        .sum();

    println!("{sum}");

    Ok(())
}

fn xy_to_check(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut coordinates = Vec::new();
    for i in 0..=2 {
        // let cx = 139.min(0.max(x as i32 + i - 1));
        let cx = (x as i32 + i - 1).clamp(0, 139) as usize;
        for j in 0..=2 {
            // let cy = 139.min(0.max(y as i32 + j - 1));
            let cy = (y as i32 + j - 1).clamp(0, 139) as usize;
            coordinates.push((cx as usize, cy as usize));
        }
    }
    coordinates
}

fn part2(input: Vec<String>) -> std::io::Result<()> {
    let mut numbers: Vec<Number> = Vec::new();
    let mut gear_nearby: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    // let mut gears: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    for (x, line) in input.iter().enumerate() {
        let mut number = Number::new();
        for (y, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                number.num.push(char);
                number.coordinates.push((x, y));
            } else {
                if number.num.len() != 0 {
                    numbers.push(number.clone());
                    number = Number::new();
                }
                if char == '*' {
                    for xy in xy_to_check(x, y) {
                        gear_nearby.insert(xy, (x, y));
                    }
                }
            }
        }
        if number.num.len() != 0 {
            numbers.push(number.clone());
        }
    }

    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    for mut number in numbers {
        for cor in number.coordinates {
            if !number.include {
                if let Some(gear) = gear_nearby.get(&cor) {
                    gears
                        .entry(*gear)
                        .or_insert_with(Vec::new)
                        .push(number.num.parse::<i32>().unwrap());
                    number.include = true;
                }
            }
        }
    }
    let mut sum = 0;

    for g in gears {
        if g.1.len() > 1 {
            let mut multi = 1;
            for j in g.1 {
                multi *= j;
            }
            sum += multi;
        }
    }
    println!("{sum}");

    Ok(())
}
