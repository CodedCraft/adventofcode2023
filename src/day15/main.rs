use adventofcode2023::*;
use std::collections::HashMap;

main!();

fn part1(input: Vec<String>) -> Result<()> {
    let init_sequence: Vec<&str> = input[0].split(',').map(|seq| seq.trim()).collect();
    let sum: usize = init_sequence.iter().map(|&seq| hash(seq)).sum();

    println!("Part 1: {sum}");

    Ok(())
}

#[derive(Clone)]
struct Box<'a> {
    map: HashMap<&'a str, usize>,
    vec: Vec<i32>,
}

fn part2(input: Vec<String>) -> Result<()> {
    let mut boxes = vec![
        Box {
            map: HashMap::new(),
            vec: Vec::new()
        };
        256
    ];
    let mut focusing_power = 0;

    let init_sequence: Vec<&str> = input[0].split(',').map(|seq| seq.trim()).collect();
    for &seq in init_sequence.iter() {
        let lens;
        let box_num;
        if let Some(i) = seq.find("=") {
            lens = &seq[..i];
            box_num = hash(lens);
            let focal_length = seq[i + 1..].parse::<i32>()?;

            if let Some(&j) = boxes[box_num].map.get(lens) {
                boxes[box_num].vec[j] = focal_length;
            } else {
                let v_idx = boxes[box_num].vec.len();
                boxes[box_num].vec.push(focal_length);
                boxes[box_num].map.insert(lens, v_idx);
            }
        } else {
            let i = seq.find("-").expect("lens has neither '-' nor '='");
            lens = &seq[..i];
            box_num = hash(lens);
            if let Some(&j) = boxes[box_num].map.get(lens) {
                boxes[box_num].vec[j] = 0;
                boxes[box_num].map.remove(lens);
            }
        }
    }
    for (i, b) in boxes.iter().enumerate() {
        for (j, &fl) in b.vec.iter().filter(|&&fl| fl != 0).enumerate() {
            focusing_power += (i + 1) as i32 * (j + 1) as i32 * fl;
        }
    }

    println!("Part 2: {focusing_power}");
    Ok(())
}

fn hash(seq: &str) -> usize {
    let mut hash_sum = 0;
    for char in seq.chars() {
        hash_sum = (hash_sum + char as usize) * 17 % 256;
    }
    hash_sum
}
