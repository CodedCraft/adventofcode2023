use adventofcode2023::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashSet, VecDeque},
    usize,
};

main!();
fn part1(input: Vec<String>) -> Result<()> {
    let grid = input
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let start = [0, -1, 0, 1];
    println!("Part 1: {}", count_energized_panels(grid.clone(), start));
    Ok(())
}

fn part2(input: Vec<String>) -> Result<()> {
    let grid = input
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let columns = grid[0].len() as i32;
    let rows = grid.len() as i32;

    let best_config = (0..columns)
        .into_par_iter()
        .flat_map(|i| {
            let top_start = [-1, i, 1, 0];
            let bottom_start = [rows, i, -1, 0];

            vec![
                count_energized_panels(grid.clone(), top_start),
                count_energized_panels(grid.clone(), bottom_start),
            ]
        })
        .chain((0..rows).into_par_iter().flat_map(|i| {
            let left_start = [i, -1, 0, 1];
            let right_start = [i, columns, 0, -1];

            vec![
                count_energized_panels(grid.clone(), left_start),
                count_energized_panels(grid.clone(), right_start),
            ]
        }))
        .reduce(|| usize::min_value(), usize::max);

    println!("Part 2: {}", best_config);

    Ok(())
}

fn count_energized_panels(grid: Vec<Vec<char>>, start: [i32; 4]) -> usize {
    let mut visited = HashSet::new();
    let mut beams: VecDeque<[i32; 4]> = VecDeque::from([start]);

    while let Some([mut x, mut y, mut dx, mut dy]) = beams.pop_front() {
        x += dx;
        y += dy;

        if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
            continue;
        }

        match grid[x as usize][y as usize] {
            '.' => (),
            '/' => {
                (dx, dy) = (-dy, -dx);
            }
            '\\' => {
                (dx, dy) = (dy, dx);
            }
            '-' => {
                if dx != 0 {
                    (dx, dy) = (dy, dx);
                    if !visited.contains(&[x, y, -dx, -dy]) {
                        beams.push_back([x, y, -dx, -dy]);
                        visited.insert([x, y, -dx, -dy]);
                    }
                }
            }
            '|' => {
                if dy != 0 {
                    (dx, dy) = (dy, dx);
                    if !visited.contains(&[x, y, -dx, -dy]) {
                        beams.push_back([x, y, -dx, -dy]);
                        visited.insert([x, y, -dx, -dy]);
                    }
                }
            }
            _ => unreachable!(),
        }
        if !visited.contains(&[x, y, dx, dy]) {
            visited.insert([x, y, dx, dy]);
            beams.push_back([x, y, dx, dy]);
        }
    }

    visited
        .iter()
        .map(|&entry| (entry[0], entry[1]))
        .collect::<HashSet<_>>()
        .len()
}
