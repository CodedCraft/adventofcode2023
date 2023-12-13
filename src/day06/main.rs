use adventofcode2023::*;

main!();

#[derive(Debug)]
struct Records {
    time: i64,
    distance: i64,
}

fn part1(input: Vec<String>) -> Result<()> {
    let times: Vec<i64> = input
        .iter()
        .nth(0)
        .expect("Could not get first line")
        .split(":")
        .nth(1)
        .expect("Could not split at ':'")
        .split_whitespace()
        .map(|str_num| {
            str_num
                .parse()
                .expect(&format!("Could not parse {} into i32", str_num))
        })
        .collect();

    let distances: Vec<i64> = input
        .iter()
        .nth(1)
        .expect("Could not get first line")
        .split(":")
        .nth(1)
        .expect("Could not split at ':'")
        .split_whitespace()
        .map(|str_num| {
            str_num
                .parse()
                .expect(&format!("Could not parse {} into i32", str_num))
        })
        .collect();

    let records: Vec<Records> = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Records { time, distance })
        .collect();

    let mut total_ways_to_win = 1;
    for record in records {
        total_ways_to_win *= race_distance(record.time, record.distance);
    }

    println!("Part 1: {}", total_ways_to_win);

    Ok(())
}

fn race_distance(time: i64, distance: i64) -> i64 {
    let mut ways_to_win = 0;
    for i in 1..time {
        let cur_dist = (time - i) * i;
        if cur_dist > distance {
            ways_to_win += 1;
        }
    }
    ways_to_win
}

fn part2(input: Vec<String>) -> Result<()> {
    let time: i64 = input
        .iter()
        .nth(0)
        .expect("Could not get first line")
        .split(":")
        .nth(1)
        .expect("Could not split at ':'")
        .split_whitespace()
        .collect::<String>()
        .parse()
        .expect("Could not parse number");

    let record: i64 = input
        .iter()
        .nth(1)
        .expect("Could not get first line")
        .split(":")
        .nth(1)
        .expect("Could not split at ':'")
        .split_whitespace()
        .collect::<String>()
        .parse()
        .expect("Could not parse number");

    let mut total_ways_to_win = 1;
    total_ways_to_win *= race_distance(time, record);

    println!("Part 2: {}", total_ways_to_win);

    Ok(())
}
