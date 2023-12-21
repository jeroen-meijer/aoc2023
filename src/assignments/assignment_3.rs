use super::prelude::*;
use std::{
    fmt::{write, Display},
    ops::RangeInclusive,
};

const LOGGING_ENABLED: bool = false;

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
            answer_day_1: None,
            example_input_day_2: None,
            answer_example_day_2: None,
            answer_day_2: None,
        },
        // cspell: enable
    )
}

fn _run(data: &Vec<String>, _is_day_2: bool) -> Result<Option<Answer>, String> {
    let matrix = EngineMatrix::parse(data);

    let mut p = false;
    for SymbolEntry { x, y } in matrix.symbols {
        let x = x as i64;
        let y = y as i64;
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
                Some((x as u32, y as u32))
            }
        })
        .collect::<Vec<_>>();

        for (x, y) in surrounding_coords {
            // Check if number is present in these coords.
            // [Y] Save, then eventually sum and return.
            // [N] Ignore.
        }
    }

    Ok(None)
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
                        acc.push(Entry::Symbol(SymbolEntry { x, y }));
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

#[derive(Debug)]
struct NumberEntry {
    value: u32,
    x: RangeInclusive<usize>,
    y: usize,
}

#[derive(Debug)]
struct SymbolEntry {
    x: usize,
    y: usize,
}
