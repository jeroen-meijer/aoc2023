use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(
        // cspell: disable
        AssignmentOptions {
            day: 6,
            description: "Wait For It",
            run: _run,
            example_input_day_1: Some(
                "
Time:      7  15   30
Distance:  9  40  200",
            ),
            answer_example_day_1: Some(288.into()),
            answer_day_1: Some(503424.into()),
            example_input_day_2: Some(
                "
Time:      7  15   30
Distance:  9  40  200",
            ),
            answer_example_day_2: Some(71503.into()),
            answer_day_2: Some(32607562.into()),
        },
        // cspell: enable
    )
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>, String> {
    let mut records: Vec<RaceRecord> = RaceRecord::parse_all(context.data);

    if context.part_number == 2 {
        let record = records
            .drain(..)
            .fold(RaceRecord::default(), |acc, cur| RaceRecord {
                time_limit: format!("{}{}", acc.time_limit, cur.time_limit)
                    .parse::<u64>()
                    .unwrap(),
                distance_record: format!("{}{}", acc.distance_record, cur.distance_record)
                    .parse::<u64>()
                    .unwrap(),
            });
        records.push(record)
    }

    let winning_moves = records.into_iter().map(|record| record.get_winning_moves());

    let winning_moves_amounts_multiplied = winning_moves
        .map(|moves| moves.len())
        .fold(1, |acc, cur| acc * cur);

    Ok(Some(winning_moves_amounts_multiplied.into()))
}

#[derive(Debug, Default)]
struct RaceRecord {
    time_limit: u64,
    distance_record: u64,
}

impl RaceRecord {
    fn parse_all(lines: &Vec<String>) -> Vec<RaceRecord> {
        assert_eq!(lines.len(), 2, "Lines must have length of 2");

        let all_numbers = lines
            .into_par_iter()
            .map(|line| {
                line.split(':')
                    .last()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let times_and_distances = all_numbers[0].iter().zip(all_numbers[1].iter());
        times_and_distances
            .map(|(time_limit, distance_record)| RaceRecord {
                time_limit: *time_limit,
                distance_record: *distance_record,
            })
            .collect()
    }

    fn get_winning_moves(&self) -> Vec<RaceStrategy> {
        (1..(self.time_limit - 1))
            .into_par_iter()
            .map(|button_hold_time| RaceStrategy {
                button_hold_time,
                travel_time: self.time_limit - button_hold_time,
            })
            .filter(|strategy| strategy.get_total_distance_traveled() > self.distance_record)
            .collect()
    }
}

struct RaceStrategy {
    button_hold_time: u64,
    travel_time: u64,
}

impl RaceStrategy {
    fn get_total_distance_traveled(&self) -> u64 {
        self.button_hold_time * self.travel_time
    }
}
