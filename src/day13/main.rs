use adventofcode2023::*;

main!();

fn part1(input: Vec<String>) -> Result<()> {
    let patterns: Vec<Vec<Vec<char>>> = input
        .split(|line| line.is_empty())
        .filter(|pattern| !pattern.is_empty())
        .map(|pattern| pattern.iter().map(|line| line.chars().collect()).collect())
        .collect();

    let mut total_score = 0;
    for pattern in patterns {
        let rows = pattern.len();
        let cols = pattern[0].len();

        let transposed_pattern: Vec<Vec<char>> = (0..cols)
            .map(|col| (0..rows).map(|row| pattern[row][col]).collect())
            .collect();
        let v_score = mirror_position(transposed_pattern);

        let h_score = mirror_position(pattern) * 100;
        total_score += h_score;

        total_score += v_score;
    }
    println!("Total score: {total_score}");

    Ok(())
}

fn part2(input: Vec<String>) -> Result<()> {
    let patterns: Vec<Vec<Vec<char>>> = input
        .split(|line| line.is_empty())
        .filter(|pattern| !pattern.is_empty())
        .map(|pattern| pattern.iter().map(|line| line.chars().collect()).collect())
        .collect();

    let mut total_score = 0;
    for pattern in patterns {
        let rows = pattern.len();
        let cols = pattern[0].len();

        let transposed_pattern: Vec<Vec<char>> = (0..cols)
            .map(|col| (0..rows).map(|row| pattern[row][col]).collect())
            .collect();
        let v_score = mirror_position_with_smudge(transposed_pattern);

        let h_score = mirror_position_with_smudge(pattern) * 100;
        total_score += h_score;

        total_score += v_score;
    }

    println!("Total score: {total_score}");

    Ok(())
}

fn mirror_position(pattern: Vec<Vec<char>>) -> usize {
    for i in 1..pattern.len() {
        let upper: Vec<_> = pattern[..i].iter().rev().cloned().collect();
        let lower = &pattern[i..];

        let mut valid_mirror = true;
        let range = upper.len().min(lower.len());
        for j in 0..range {
            if upper[j] != lower[j] {
                valid_mirror = false;
            }
        }
        if valid_mirror {
            return i;
        }
    }
    return 0;
}

fn mirror_position_with_smudge(pattern: Vec<Vec<char>>) -> usize {
    for i in 1..pattern.len() {
        let upper: Vec<_> = pattern[..i].iter().rev().cloned().collect();
        let lower = &pattern[i..];

        let mut smudge_count = 0;
        let range = upper.len().min(lower.len());
        for j in 0..range {
            if upper[j] != lower[j] {
                smudge_count += count_smudges(upper[j].clone(), lower[j].clone());
            }
        }
        if smudge_count == 1 {
            return i;
        }
    }
    return 0;
}

fn count_smudges(upper: Vec<char>, lower: Vec<char>) -> i32 {
    let mut smudges = 0;
    for i in 0..upper.len() {
        if upper[i] != lower[i] {
            smudges += 1;
        }
    }
    smudges
}
