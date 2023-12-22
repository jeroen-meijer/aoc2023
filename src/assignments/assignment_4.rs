use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(
        // cspell: disable
        AssignmentOptions {
            day: 4,
            description: "Scratchcards",
            run: _run,
            example_input_day_1: Some(
                "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            ),
            answer_example_day_1: Some(13.into()),
            answer_day_1: Some(20107.into()),
            example_input_day_2: Some(
                "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            ),
            answer_example_day_2: Some(30.into()),
            answer_day_2: Some(8172507.into()),
        },
        // cspell: enable
    )
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>, String> {
    let cards = context
        .data
        .into_iter()
        .map(|line| Card::parse(line))
        .collect::<Vec<_>>();

    if context.part_number == 1 {
        _run_part_1(cards)
    } else {
        _run_part_2(cards)
    }
}

fn _run_part_1(cards: Vec<Card>) -> Result<Option<Answer>, String> {
    let scores_by_card =
        cards
            .iter()
            .map(|card| card.get_overlapping_numbers())
            .map(|overlapping_numbers| {
                if overlapping_numbers.is_empty() {
                    0
                } else {
                    2u64.pow(overlapping_numbers.len() as u32 - 1)
                }
            });

    let sum = scores_by_card.sum::<u64>();

    Ok(Some(sum.into()))
}

fn _run_part_2(cards: Vec<Card>) -> Result<Option<Answer>, String> {
    let mut cards_and_amounts_by_id = cards
        .iter()
        .map(|card| (card.id, (card, 1)))
        .collect::<HashMap<_, _>>();

    let mut keys = cards_and_amounts_by_id
        .iter()
        .map(|(key, _)| key.clone())
        .collect::<Vec<_>>();
    keys.sort();

    for id in keys {
        let (card, copies) = cards_and_amounts_by_id.get(&id).unwrap().clone();
        let hits = card.get_overlapping_numbers().len();
        for i in 0..hits {
            let id_to_add_copy_to = id + 1 + i as u32;
            let entry = cards_and_amounts_by_id.get_mut(&id_to_add_copy_to);
            match entry {
                Some(entry) => entry.1 += copies,
                None => {}
            }
        }
    }

    let total_cards = cards_and_amounts_by_id
        .into_iter()
        .map(|(_, (_, copies))| copies)
        .sum::<u64>();

    Ok(Some(total_cards.into()))
}

struct Card {
    id: u32,
    received_numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl Card {
    fn parse(line: &String) -> Self {
        let id = line
            .chars()
            .skip("Card ".len())
            .take_while(|chr| chr != &':')
            .join("")
            .trim()
            .parse::<u32>()
            .expect("Expected a valid ID");

        let mut number_parts = line
            .split(':')
            .last()
            .unwrap()
            .split('|')
            .map(|group_str| {
                group_str
                    .split_ascii_whitespace()
                    .map(|num_str| num_str.trim().parse::<u32>().expect("Expected a valid u32"))
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(
            number_parts.len(),
            2,
            "Expected 2 number parts, instead got {}",
            number_parts.len()
        );

        let winning_numbers = number_parts.pop().unwrap();
        let received_numbers = number_parts.pop().unwrap();

        Card {
            id,
            winning_numbers,
            received_numbers,
        }
    }

    fn get_overlapping_numbers(&self) -> HashSet<&u32> {
        self.winning_numbers
            .intersection(&self.received_numbers)
            .collect::<HashSet<_>>()
    }
}
