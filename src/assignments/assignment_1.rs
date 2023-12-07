use std::ops::Range;

use regex::Regex;

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    return Assignment::new(
        1,
        "Calorie Counting",
        TestCaseGroup {
            // cspell: disable
            example_day_1: TestCase::from_string(
                "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
                Some(142.into()),
            ),
            day1: TestCase::from_file(Some(54877.into())),
            example_day_2: TestCase::from_string(
                "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
                Some(281.into()),
            ),
            day2: TestCase::from_file(None),
            // cspell: enable
        },
        _run,
    );
}

fn _run(data: &Vec<String>, is_day_2: bool) -> Result<Answer, String> {
    const WORD_TO_DIGIT: [(&str, u8); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    Ok(data
        .iter()
        .map(|line| {
            let mut all_numbers = vec![];

            let numbers_by_index = line
                .chars()
                .enumerate()
                .filter_map(|(index, c)| c.to_digit(10).map(|d| (index, d as u8)))
                .collect::<Vec<_>>();
            all_numbers.extend(numbers_by_index);

            if is_day_2 {
                let regex_matches_per_digit = WORD_TO_DIGIT.map(|(word, digit)| {
                    let pattern = Regex::new(word).unwrap();
                    pattern
                        .find_iter(line)
                        .map(|m| (m.range(), digit))
                        .collect::<Vec<_>>()
                });

                let mut regex_matches_with_range = regex_matches_per_digit
                    .iter()
                    .flat_map(|e| e)
                    .collect::<Vec<_>>();

                regex_matches_with_range
                    .sort_by(|(range_a, _), (range_b, _)| range_a.start.cmp(&range_b.start));

                let regex_matches_by_index = regex_matches_with_range
                    .iter()
                    .fold(Vec::<&(Range<usize>, u8)>::new(), |mut acc, cur| {
                        if !acc.iter().any(|e| e.0.end >= cur.0.start) {
                            acc.push(cur)
                        }
                        acc
                    })
                    .iter()
                    .map(|(range, digit)| (range.start, *digit))
                    .collect::<Vec<_>>();

                all_numbers.extend(regex_matches_by_index);
            }

            all_numbers.sort_by_key(|(index, _)| *index);

            if all_numbers.is_empty() {
                return 0;
            }

            let res = format!(
                "{}{}",
                all_numbers.first().unwrap().1,
                all_numbers.last().unwrap().1
            )
            .parse::<u32>()
            .unwrap();

            res
        })
        .sum::<u32>()
        .into())
}
