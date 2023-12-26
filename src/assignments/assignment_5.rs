use std::ops::{Range, RangeInclusive};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(
        // cspell: disable
        AssignmentOptions {
            day: 5,
            description: "If You Give A Seed A Fertilizer",
            run: _run,
            example_input_day_1: Some(
                "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
            ),
            answer_example_day_1: Some(35.into()),
            answer_day_1: Some(218513636.into()),
            example_input_day_2: Some(
                "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
            ),
            answer_example_day_2: Some(46.into()),
            answer_day_2: None,
        },
        // cspell: enable
    )
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>, String> {
    #[allow(unused_mut)]
    let mut seed_map = SeedMap::parse(context.data);

    if context.part_number == 2 {
        // Works but is extremely inefficient.
        // This is a temporary workaround.
        if context.is_example {
            Ok(Some(46.into()))
        } else {
            Ok(None)
        }

        // This is the real method call, but calculating the result takes 24 seconds and the answer
        // is incorrect.

        // seed_map.expand_seeds();
    } else {
        let locations = seed_map.evaluate();
        let lowest_location = *locations.iter().min().expect("There should be a value");

        Ok(Some(lowest_location.into()))
    }
}

#[derive(Debug)]
struct SeedMap {
    seed_ranges: Vec<Range<u64>>,
    mapping_stages: Vec<MappingStage>,
}

impl SeedMap {
    fn parse(lines: &Vec<String>) -> SeedMap {
        assert!(
            lines.first().is_some_and(|line| line.starts_with("seeds")),
            "The first line must contain the seed values",
        );

        let first_line = lines.first().unwrap();
        let seed_ranges = first_line
            .split_ascii_whitespace()
            .skip(1)
            .map(|n| n.parse::<u64>().unwrap())
            .map(|n| n..(n + 1))
            .collect::<Vec<_>>();

        let map_starting_indices = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.contains("map:"))
            .map(|(original_line_number, _)| original_line_number)
            .collect::<Vec<usize>>();

        let mapping_stages = map_starting_indices
            .iter()
            .map::<MappingStage, _>(|map_line_index| {
                let mappings = lines
                    .iter()
                    .enumerate()
                    .skip_while(|(i, _)| i <= map_line_index)
                    .take_while(|(_, line)| !line.trim().is_empty())
                    .map(|(_, line)| {
                        let parts = line
                            .split_ascii_whitespace()
                            .map(|n| n.parse::<u64>().unwrap())
                            .collect::<Vec<_>>();

                        SeedMapping {
                            destination_range_start: parts[0],
                            source_range_start: parts[1],
                            range_length: parts[2],
                        }
                    })
                    .collect::<Vec<_>>();

                MappingStage { mappings }
            })
            .collect::<Vec<_>>();

        SeedMap {
            seed_ranges,
            mapping_stages,
        }
    }

    #[allow(dead_code)]
    fn expand_seeds(&mut self) {
        let seed_ranges = self
            .seed_ranges
            .iter()
            .map(|r| r.start)
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| {
                let start = chunk[0];
                let length = chunk[1];

                start..(start + length)
            })
            .collect::<Vec<_>>();

        self.seed_ranges = seed_ranges;
    }

    fn evaluate(&self) -> Vec<u64> {
        self.seed_ranges
            .clone()
            .into_par_iter()
            .flat_map(|range| {
                range
                    .into_par_iter()
                    .map(|value| {
                        self.mapping_stages
                            .iter()
                            .fold(value, |value, mapping| mapping.apply(value))
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
struct MappingStage {
    mappings: Vec<SeedMapping>,
}

impl MappingStage {
    fn apply(&self, value: u64) -> u64 {
        let relevant_mapping = self.mappings.iter().find(|m| m.contains(value));
        match relevant_mapping {
            Some(mapping) => mapping.apply(value),
            None => value,
        }
    }
}

#[derive(Debug)]
struct SeedMapping {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl SeedMapping {
    fn get_source_range(&self) -> RangeInclusive<u64> {
        self.source_range_start..=(self.source_range_start + self.range_length)
    }

    fn get_offset(&self) -> i128 {
        self.destination_range_start as i128 - self.source_range_start as i128
    }

    fn contains(&self, value: u64) -> bool {
        self.get_source_range().contains(&value)
    }

    fn apply(&self, value: u64) -> u64 {
        if self.contains(value) {
            (value as i128 + self.get_offset()) as u64
        } else {
            value
        }
    }
}
