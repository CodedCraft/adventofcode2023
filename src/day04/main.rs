use adventofcode2023::*;
use std::collections::HashSet;

main!();

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    scratch_numbers: Vec<i32>,
}
impl Card {
    fn new() -> Self {
        Card {
            id: 0,
            winning_numbers: HashSet::new(),
            scratch_numbers: Vec::new(),
        }
    }
}
fn part1(input: Vec<String>) -> Result<()> {
    let mut cards = Vec::new();
    let mut points = 0;

    for line in input {
        let mut current_card = Card::new();

        // Get ID
        if let Some(id_str) = line.split(":").nth(0) {
            if let Some(id) = id_str.split_whitespace().nth(1) {
                current_card.id = id.parse()?;
            };
        }

        // Get all numbers
        if let Some(numbers) = line.split(":").nth(1) {
            // Get HashSet of winning numbers
            if let Some(winning_numbers_str) = numbers.split("|").nth(0) {
                let winning_numbers: Vec<&str> = winning_numbers_str.split_whitespace().collect();
                for winning_number in winning_numbers {
                    current_card.winning_numbers.insert(winning_number.parse()?);
                }
            }

            // Get scratch numbers
            if let Some(scratch_numbers_str) = numbers.split("|").nth(1) {
                let scratch_numbers: Vec<&str> = scratch_numbers_str.split_whitespace().collect();
                for scratch_number in scratch_numbers {
                    current_card.scratch_numbers.push(scratch_number.parse()?);
                }
            }

            cards.push(current_card);
        }
    }

    for card in cards {
        let mut card_points = 0;
        for scratch_number in card.scratch_numbers {
            if card.winning_numbers.contains(&scratch_number) {
                card_points *= 2;
                if card_points == 0 {
                    card_points += 1;
                }
            }
        }
        points += card_points;
    }
    println!("{}", points);

    Ok(())
}

fn part2(input: Vec<String>) -> Result<()> {
    let mut cards = Vec::new();

    for line in input {
        let mut current_card = Card::new();

        // Get ID
        if let Some(id_str) = line.split(":").nth(0) {
            if let Some(id) = id_str.split_whitespace().nth(1) {
                current_card.id = id.parse()?;
            };
        }

        // Get all numbers
        if let Some(numbers) = line.split(":").nth(1) {
            // Get HashSet of winning numbers
            if let Some(winning_numbers_str) = numbers.split("|").nth(0) {
                let winning_numbers: Vec<&str> = winning_numbers_str.split_whitespace().collect();
                for winning_number in winning_numbers {
                    current_card.winning_numbers.insert(winning_number.parse()?);
                }
            }

            // Get scratch numbers
            if let Some(scratch_numbers_str) = numbers.split("|").nth(1) {
                let scratch_numbers: Vec<&str> = scratch_numbers_str.split_whitespace().collect();
                for scratch_number in scratch_numbers {
                    current_card.scratch_numbers.push(scratch_number.parse()?);
                }
            }

            cards.push(current_card);
        }
    }

    let mut number_of_cards = cards.len() as i32;
    let mut stack_of_cards = Vec::new();
    let mut next_stack = Vec::new();
    let cards2 = cards.clone();
    let range = cards.len();

    for card in cards {
        stack_of_cards.push(card.clone());
        while let Some(current_card) = stack_of_cards.pop() {
            let mut won_cards = 0;
            for scratch_number in current_card.scratch_numbers {
                if current_card.winning_numbers.contains(&scratch_number) {
                    won_cards += 1;
                }
            }
            let idx = current_card.id as usize;
            number_of_cards += won_cards;
            for id in idx..range.min(idx + won_cards as usize) {
                next_stack.push(cards2[id].clone());
            }
        }
        stack_of_cards = next_stack;
        next_stack = Vec::new();
    }

    println!("{}", number_of_cards);

    Ok(())
}
