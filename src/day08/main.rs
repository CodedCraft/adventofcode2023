use adventofcode2023::*;
use std::collections::HashMap;
use num_integer::lcm;

main!();

fn part1(input: Vec<String>) -> Result<()> {
    let directions: Vec<usize> = input[0]
        .chars()
        .map(|char| match char {
            'R' => 1,
            'L' => 0,
            _ => unreachable!("No 'L' or 'R' found"),
        })
        .collect();

    let mut current_point = "AAA";

    let mut map = HashMap::new();
    for line in input.iter().skip(2) {
        let source = line
            .split_whitespace()
            .nth(0)
            .expect("Could not get source");

        let destination: Vec<&str> = line
            .split(['(', ',', ')'].as_ref())
            .skip(1)
            .take(2)
            .map(|point| point.trim())
            .collect();

        map.insert(source, destination);
    }

    let mut steps: i64 = 0;
    while current_point != "ZZZ" {
        for &direction in &directions {
            current_point = map.get(&current_point).expect("Point not in map")[direction];
            steps += 1;
            if current_point == "ZZZ" {
                println!("Part1: {steps}");
                return Ok(())
            }
        }
    }

    Ok(())
}

fn part2(input: Vec<String>) -> Result<()> {
    let directions: Vec<usize> = input[0]
        .chars()
        .map(|char| match char {
            'R' => 1,
            'L' => 0,
            _ => unreachable!("No 'L' or 'R' found"),
        })
        .collect();

    let mut nodes: Vec<&str> = Vec::new();
    let mut map = HashMap::new();

    for line in input.iter().skip(2) {
        let source = line
            .split_whitespace()
            .nth(0)
            .expect("Could not get source");

        let destination: Vec<&str> = line
            .split(['(', ',', ')'].as_ref())
            .skip(1)
            .take(2)
            .map(|point| point.trim())
            .collect();

        if source.chars().nth(2).unwrap() == 'A' {
            nodes.push(source);
        }
        map.insert(source, destination);
    }

    let mut steps_per_node = Vec::new();

    for &node in &nodes {
        let mut found_z = false;
        let mut steps: u128 = 0;
        let mut cur_node = node;
        while !found_z {
            for &direction in &directions {
                steps += 1;
                cur_node = map.get(cur_node).unwrap()[direction];
                if cur_node.ends_with('Z') {
                    found_z = true;
                    steps_per_node.push(steps);
                }
            }
        }
    }

    let mut result = steps_per_node[0];
    for &num in &steps_per_node[1..] {
         result = lcm(result, num);
    }
    println!("Part2: {result}");

    Ok(())
}
