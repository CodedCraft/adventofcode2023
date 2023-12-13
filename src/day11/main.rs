use adventofcode2023::*;
use std::collections::{VecDeque, HashSet};


main!();

fn part1(input: Vec<String>) -> Result<()> {
    // 1. Build expanded universe
    let line_length = input.first().unwrap().len();
    let mut galaxies_in_col = vec![false; line_length];
    let mut universe = Vec::new();

    for line in input {
        let mut galaxies_in_row = false;
        let mut row = Vec::new();
        for (i, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies_in_row = true;
                galaxies_in_col[i] = true;
            }
            row.push(char);
        }
        universe.push(row);
        if !galaxies_in_row {
            let empty_line = vec!['.'; line_length];
            universe.push(empty_line);
        }
    }

    let mut offset = 0;
    for (i, col) in galaxies_in_col.iter().enumerate() {
        if !col {
            for row in universe.iter_mut() {
                row.insert(i + offset, '.');
            }
            offset += 1;
        }
    }

    // 2. Get positions of all galaxies
    let mut galaxy_positions = VecDeque::new();
    for (x, row) in universe.iter().enumerate() {
        for (y, &galaxy) in row.iter().enumerate() {
            if galaxy == '#' {
                galaxy_positions.push_back((x as i32, y as i32));
            }
        }
    }

    // 3. Calculate distance between all galaxies
    let mut total_distance = 0;
    while let Some(cur_galaxy) = galaxy_positions.pop_front() {
        for remaining_galaxy in galaxy_positions.iter() {
            let x_distance = (remaining_galaxy.0 - cur_galaxy.0).abs();
            let y_distance = (remaining_galaxy.1 - cur_galaxy.1).abs();
            total_distance += x_distance + y_distance;
        }
    }

    println!("Total: {total_distance}");

    Ok(())
}

fn part2(input: Vec<String>) -> Result<()> {
    // 1. Build universe
    let line_length = input.first().unwrap().len();
    let mut galaxies_in_col = vec![false; line_length];
    let mut universe = Vec::new();
    let mut empty_rows = HashSet::new();
    let mut empty_cols = HashSet::new();

    for (x , line) in input.iter().enumerate() {
        let mut galaxies_in_row = false;
        let mut row = Vec::new();
        for (i, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies_in_row = true;
                galaxies_in_col[i] = true;
            }
            row.push(char);
        }
        universe.push(row);
        if !galaxies_in_row {
            empty_rows.insert(x);
        }
    }

    for (y, col) in galaxies_in_col.iter().enumerate() {
        if !col {
            empty_cols.insert(y);
            }
    }

    // 2. Get positions of all galaxies
    let mut galaxy_positions = VecDeque::new();
    for (x, row) in universe.iter().enumerate() {
        for (y, &galaxy) in row.iter().enumerate() {
            if galaxy == '#' {
                galaxy_positions.push_back((x as i32, y as i32));
            }
        }
    }

    // 3. Calculate distance between all galaxies
    let space_between = 1_000_000;
    let mut total_distance: i64= 0;
    while let Some(cur_galaxy) = galaxy_positions.pop_front() {
        for remaining_galaxy in galaxy_positions.iter() {
            let (x_start, x_end) = match remaining_galaxy.0 > cur_galaxy.0 {
                true => (cur_galaxy.0 as usize, remaining_galaxy.0 as usize),
                false => (remaining_galaxy.0 as usize, cur_galaxy.0 as usize),
            };

            let mut x_distance = 0;
            for i in x_start..x_end {
                if empty_rows.contains(&i) {
                    x_distance += space_between - 1;
                }
            }

            let (y_start, y_end) = match remaining_galaxy.1 > cur_galaxy.1 {
                true => (cur_galaxy.1 as usize, remaining_galaxy.1 as usize),
                false => (remaining_galaxy.1 as usize, cur_galaxy.1 as usize),
            };

            let mut y_distance = 0;
            for i in y_start..y_end {
                if empty_cols.contains(&i) {
                    y_distance += space_between - 1;
                }
            }

            x_distance += (remaining_galaxy.0 - cur_galaxy.0).abs();
            y_distance += (remaining_galaxy.1 - cur_galaxy.1).abs();
            total_distance += x_distance as i64 + y_distance as i64;
        }
    }

    println!("Total: {total_distance}");

    Ok(())
}
