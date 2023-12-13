use adventofcode2023::*;
use std::collections::{BinaryHeap, HashMap, HashSet};

main!();

fn part1(input: Vec<String>) -> Result<()> {
    let mut scores = BinaryHeap::new();
    let mut score_to_bid = HashMap::new();

    // Create a lookup table
    let mut look_up = HashMap::new();
    let cards = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    for i in 0..cards.len() {
        let card = cards[i];
        let value = (i + 10).to_string();
        look_up.insert(card, value);
    }

    for line in input {
        let hand = line
            .split_whitespace()
            .nth(0)
            .expect("No hand found in line");
        let bid = line
            .split_whitespace()
            .nth(1)
            .expect("No bid found in line")
            .parse::<i32>()?;
        let score = calculate_score1(hand, &look_up);
        scores.push(score);
        score_to_bid.insert(score, bid);
    }

    let mut winnings = 0;
    let mut multiplier = scores.len() as i32;
    while let Some(score) = scores.pop() {
        let bid = score_to_bid.get(&score).unwrap();
        winnings += bid * multiplier;
        multiplier -= 1;
    }
    println!("Part 1: {winnings}");
    

    Ok(())
}


fn calculate_score1(hand: &str, look_up: &HashMap<char, String>) -> i64 {
    let mut card_score = String::new();
    let mut seen = HashSet::new();
    let mut hand_score = 0;
    for char in hand.chars() {
        card_score.push_str(look_up.get(&char).expect("Not a valid card"));
        if !seen.contains(&char) {
            let matches = hand.matches(char).count() as u32;
            hand_score += match matches {
                5 => 10000,
                4 => 1000,
                3 => 100,
                2 => 10,
                1 => 1,
                _ => unreachable!()
            };
            seen.insert(char);
        }
    }
    (hand_score as i64 * 1000000000) + card_score.parse::<i64>().unwrap()
}

fn part2(input: Vec<String>) -> Result<()> {
    let mut scores = BinaryHeap::new();
    let mut score_to_bid = HashMap::new();

    // Create a lookup table
    let mut look_up = HashMap::new();
    let cards = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    for i in 0..cards.len() {
        let card = cards[i];
        let value = (i + 10).to_string();
        look_up.insert(card, value);
    }

    for line in input {
        let hand = line
            .split_whitespace()
            .nth(0)
            .expect("No hand found in line");
        let bid = line
            .split_whitespace()
            .nth(1)
            .expect("No bid found in line")
            .parse::<i32>()?;
        let score = calculate_score2(hand, &look_up);
        scores.push(score);
        score_to_bid.insert(score, bid);

        // println!("Overall score: {}", score);
    }

    let mut winnings = 0;
    let mut multiplier = scores.len() as i32;
    while let Some(score) = scores.pop() {
        let bid = score_to_bid.get(&score).unwrap();
        winnings += bid * multiplier;
        multiplier -= 1;
    }
    println!("Part2: {winnings}");

    Ok(())
}

fn calculate_score2(hand: &str, look_up: &HashMap<char, String>) -> i64 {
    let mut card_score = String::new();
    let mut j = 0;
    let mut seen = HashSet::new();
    let mut duplicates = BinaryHeap::new();
    let mut hand_score = 0;
    for char in hand.chars() {
        card_score.push_str(look_up.get(&char).expect("Not a valid card"));
        if !seen.contains(&char) {
            let matches = hand.matches(char).count();
            if char == 'J' {
                j = matches;
            } else {
                duplicates.push(matches);
            }
            seen.insert(char);
        }
    }
    if j == 5 {
        duplicates.push(0);
    }

    while let Some(count) = duplicates.pop() {
        hand_score += match count + j {
            5 => 10000,
            4 => 1000,
            3 => 100,
            2 => 10,
            1 => 1,
            _ => unreachable!(),
        };
        j = 0;
    }


    let score = (hand_score as i64 * 10000000000) + card_score.parse::<i64>().unwrap();
        // println!("{hand} hand_score: {hand_score} card_score: {card_score} total: {score}");
    score
}
