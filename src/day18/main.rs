use adventofcode2023::*;
use std::collections::VecDeque;

main!();

fn part1(input: Vec<String>) -> Result<()> {
    let movements: Vec<_> = input
        .iter()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|parts| {
            (
                match parts[0] {
                    "R" => (0, 1),
                    "L" => (0, -1),
                    "U" => (-1, 0),
                    "D" => (1, 0),
                    _ => panic!("Invalid direction"),
                },
                parts[1].parse::<i128>().unwrap(),
            )
        })
        .collect();

    let area = calculate_area(movements);
    println!("Part 1: {area}");

    Ok(())
}

fn part2(input: Vec<String>) -> Result<()> {
    let movements: Vec<_> = input
        .iter()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|parts| {
            let x: (i128, i128) = match parts[2][7..8].parse().unwrap() {
                0 => (0, 1),
                1 => (1, 0),
                2 => (0, -1),
                3 => (-1, 0),
                _ => unreachable!(),
            };
            let y: i128 = i128::from_str_radix(&parts[2][2..7], 16).unwrap();
            (x, y)
        })
        .collect();

    let area = calculate_area(movements);
    println!("Part 2: {area}");

    Ok(())
}

fn calculate_area(movements: Vec<((i128, i128), i128)>) -> i128 {
    let mut points = VecDeque::new();
    points.push_back((0, 0));

    let (mut x, mut y) = (0, 0);
    let mut n_sum = 0;

    // Collect points
    for ((dx, dy), n) in movements {
        (x, y) = (x + dx * n, y + dy * n);
        n_sum += n;
        points.push_back((x, y));
    }

    // Shoe Lace Formula
    let mut left = 0;
    let mut right = 0;
    let (mut x1, mut y1) = points.pop_front().unwrap();
    while let Some((x2, y2)) = points.pop_front() {
        left += x1 * y2;
        right += x2 * y1;
        x1 = x2;
        y1 = y2;
    }
    let sum = (left - right).abs() / 2;

    // Pick's Theorem
    let i = sum - n_sum/2 + 1;
    i + n_sum
}
