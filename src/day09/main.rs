use adventofcode2023::*;

main!();

fn part1(input: Vec<String>) -> Result<()> {
    let sensor_history: Vec<Vec<i32>> = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;
    for line in sensor_history {
        let prediction_value = prediction_calculator1(&line);
        sum += prediction_value;
    }
    println!("Part1: {sum}");

    Ok(())
}

fn prediction_calculator1(line: &Vec<i32>) -> i32 {
    let mut current_line = line.clone();
    let mut last_values = vec![current_line.last().unwrap().clone()];

    loop {
        let mut all_zeros = true;
        let mut next_line = Vec::new();
        for i in 0..(current_line.len() - 1) {
            let j = i + 1;
            let difference = current_line[j] - current_line[i];
            if difference != 0 {
                all_zeros = false;
            }
            next_line.push(difference);
        }
        current_line = next_line;
        let last_value = current_line.last().unwrap().clone();
        last_values.push(last_value);
        if all_zeros {
            return last_values.iter().sum();
        }
    }

}

fn part2(input: Vec<String>) -> Result<()> {
    let sensor_history: Vec<Vec<i32>> = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;
    for line in sensor_history {
        let prediction_value = prediction_calculator2(&line);
        // println!("{prediction_value}");
        sum += prediction_value;
    }
    println!("Part2: {sum}");

    Ok(())
}

fn prediction_calculator2(line: &Vec<i32>) -> i32 {
    let mut current_line = line.clone();
    let mut first_values = vec![current_line.first().unwrap().clone()];

    loop {
        let mut all_zeros = true;
        let mut next_line = Vec::new();
        for i in 0..(current_line.len() - 1) {
            let j = i + 1;
            let difference = current_line[j] - current_line[i];
            if difference != 0 {
                all_zeros = false;
            }
            next_line.push(difference);
        }
        current_line = next_line;
        let first_value = current_line.first().unwrap().clone();
        first_values.push(first_value);
        if all_zeros {
            let mut division = 0;
            for value in first_values.iter().rev() {
                division = value - division;
            }

            return division;
        }
    }

}
