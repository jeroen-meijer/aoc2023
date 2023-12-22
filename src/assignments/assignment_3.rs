use std::ops::RangeInclusive;

use itertools::Itertools;

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(
        // cspell: disable
        AssignmentOptions {
            day: 3,
            description: "Gear Ratios",
            run: _run,
            example_input_day_1: Some(
                "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            ),
            answer_example_day_1: Some(4361.into()),
            answer_day_1: Some(551094.into()),
            example_input_day_2: Some(
                "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            ),
            answer_example_day_2: Some(467835.into()),
            answer_day_2: Some(80179647.into()),
        },
        // cspell: enable
    )
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>, String> {
    let matrix = EngineMatrix::parse(context.data);

    if context.part_number == 1 {
        let numbers_surrounded_by_symbols = matrix
            .symbols
            .iter()
            .flat_map(|SymbolEntry { x, y, character: _ }| {
                let x = *x as i64;
                let y = *y as i64;
                let surrounding_coords = vec![
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1), //
                    (x - 1, y),
                    (x, y),
                    (x + 1, y), //
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1), //
                ]
                .into_iter()
                .filter_map(|(x, y)| {
                    if x < 0i64 || y < 0i64 {
                        None
                    } else {
                        Some((x as usize, y as usize))
                    }
                })
                .collect::<Vec<_>>();

                surrounding_coords.into_iter().flat_map(|(x, y)| {
                    matrix
                        .numbers
                        .iter()
                        .filter(|e| e.y == y && e.x.contains(&x))
                        .collect::<Vec<_>>()
                })
            })
            .unique()
            .map(|e| e.value);

        let sum = numbers_surrounded_by_symbols.sum::<u32>();

        Ok(Some(sum.into()))
    } else {
        let numbers_surrounded_by_stars_multiplied =
            matrix.symbols.iter().filter(|e| e.character == '*').map(
                |SymbolEntry { x, y, character: _ }| {
                    let x = *x as i64;
                    let y = *y as i64;
                    let surrounding_coords = vec![
                        (x - 1, y - 1),
                        (x, y - 1),
                        (x + 1, y - 1), //
                        (x - 1, y),
                        (x, y),
                        (x + 1, y), //
                        (x - 1, y + 1),
                        (x, y + 1),
                        (x + 1, y + 1), //
                    ]
                    .into_iter()
                    .filter_map(|(x, y)| {
                        if x < 0i64 || y < 0i64 {
                            None
                        } else {
                            Some((x as usize, y as usize))
                        }
                    })
                    .collect::<Vec<_>>();

                    let numbers_surrounded = surrounding_coords
                        .into_iter()
                        .flat_map(|(x, y)| {
                            matrix
                                .numbers
                                .iter()
                                .filter(|e| e.y == y && e.x.contains(&x))
                                .collect::<Vec<_>>()
                        })
                        .unique()
                        .collect::<Vec<_>>();

                    if numbers_surrounded.len() != 2 {
                        0
                    } else {
                        vec![numbers_surrounded.first(), numbers_surrounded.last()]
                            .iter()
                            .map(|e| e.unwrap().value)
                            .reduce(|a, b| a * b)
                            .unwrap()
                    }
                },
            );

        let sum = numbers_surrounded_by_stars_multiplied.sum::<u32>();

        Ok(Some(sum.into()))
    }
}

struct EngineMatrix {
    numbers: Vec<NumberEntry>,
    symbols: Vec<SymbolEntry>,
}

#[derive(Debug)]
enum Entry {
    Number(NumberEntry),
    Symbol(SymbolEntry),
}

impl EngineMatrix {
    fn parse(lines: &Vec<String>) -> Self {
        let entries = lines.iter().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, chr)| chr.to_string() != ".")
                .fold(Vec::<Entry>::new(), |mut acc, (x, chr)| {
                    let previous_x = if x == 0 { 0 } else { x - 1 };
                    let last_x = if acc.is_empty() { 0 } else { acc.len() - 1 };

                    if let Some(next_number) = chr.to_digit(10) {
                        let (delete_last, next_entry) = match acc.last() {
                            Some(Entry::Number(last_number_entry))
                                if last_number_entry.x.end() == &previous_x =>
                            {
                                (
                                    true,
                                    NumberEntry {
                                        value: (last_number_entry.value * 10) + next_number,
                                        x: *last_number_entry.x.start()..=x,
                                        y,
                                    },
                                )
                            }
                            _ => (
                                false,
                                NumberEntry {
                                    value: next_number,
                                    x: x..=x,
                                    y,
                                },
                            ),
                        };

                        if delete_last {
                            acc.remove(last_x);
                        }
                        acc.push(Entry::Number(next_entry));
                        acc
                    } else {
                        acc.push(Entry::Symbol(SymbolEntry {
                            x,
                            y,
                            character: chr,
                        }));
                        acc
                    }
                })
        });

        let mut numbers = Vec::<NumberEntry>::new();
        let mut symbols = Vec::<SymbolEntry>::new();

        for entry in entries {
            match entry {
                Entry::Number(e) => numbers.push(e),
                Entry::Symbol(e) => symbols.push(e),
            }
        }

        EngineMatrix { numbers, symbols }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct NumberEntry {
    value: u32,
    x: RangeInclusive<usize>,
    y: usize,
}

#[derive(Debug)]
struct SymbolEntry {
    character: char,
    x: usize,
    y: usize,
}
