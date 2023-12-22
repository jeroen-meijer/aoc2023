use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;

use stopwatch::Stopwatch;

mod assignment_1;
mod assignment_2;
mod assignment_3;
mod assignment_4;

mod prelude;

pub fn get_assignments() -> Vec<Assignment> {
    let assignments = vec![
        assignment_1::get_assignment(),
        assignment_2::get_assignment(),
        assignment_3::get_assignment(),
        assignment_4::get_assignment(),
    ];
    let assignments_by_day =
        assignments
            .iter()
            .fold(HashMap::<u32, Vec<&Assignment>>::new(), |mut acc, cur| {
                acc.entry(cur.day).or_default().push(cur);
                acc
            });
    for (day, assignments) in assignments_by_day {
        if assignments.len() > 1 {
            let assignment_names = assignments
                .iter()
                .map(|a| a.description)
                .collect::<Vec<_>>();
            panic!(
                "Found duplicate assignment day number {} for assignments: {:?}",
                day, assignment_names
            );
        }
    }

    assignments
}

#[derive(PartialEq, Clone)]
pub enum Answer {
    Integer(u64),
    String(String),
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        match self {
            Answer::Integer(i) => i.to_string(),
            Answer::String(s) => format!("\"{s}\""),
        }
    }
}

impl From<u64> for Answer {
    fn from(value: u64) -> Self {
        Answer::Integer(value)
    }
}

impl From<u32> for Answer {
    fn from(value: u32) -> Self {
        Answer::Integer(value.into())
    }
}

impl From<i32> for Answer {
    fn from(value: i32) -> Self {
        if value < 0 {
            panic!("Integer value has to be positive.");
        }
        Answer::Integer(value as u64)
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Answer::String(value)
    }
}

impl From<&str> for Answer {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}

impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        Answer::Integer(value as u64)
    }
}

pub struct TestCaseGroup<T> {
    pub example_day_1: T,
    pub day1: T,
    pub example_day_2: T,
    pub day2: T,
}

pub struct TestCase {
    pub input: Option<&'static str>,
    pub expected: Option<Answer>,
}

pub struct TestCaseOutput {
    pub input: String,
    pub expected: Option<Answer>,
    pub actual: Result<Option<Answer>, String>,
    pub runtime: Duration,
}

impl TestCaseOutput {
    pub fn get_result(&self) -> TestCaseResult {
        match &self.actual {
            Ok(a) => match a {
                None => TestCaseResult::NoAnswer,
                Some(answer_value) => match &self.expected {
                    Some(expected_value) => {
                        if answer_value == expected_value {
                            TestCaseResult::Correct
                        } else {
                            TestCaseResult::Incorrect
                        }
                    }
                    None => TestCaseResult::Unknown,
                },
            },
            Err(_) => TestCaseResult::Error,
        }
    }
}

pub enum TestCaseResult {
    NoAnswer,
    Unknown,
    Correct,
    Incorrect,
    Error,
}

pub struct Assignment {
    pub day: u32,
    pub description: &'static str,
    pub cases: TestCaseGroup<Option<TestCase>>,
    _f: InternalAssignmentCallback,
}

type InternalAssignmentCallback =
    fn(data: &Vec<String>, is_day_2: bool) -> Result<Option<Answer>, String>;

pub struct AssignmentOptions {
    day: u32,
    description: &'static str,
    run: InternalAssignmentCallback,
    example_input_day_1: Option<&'static str>,
    answer_example_day_1: Option<Answer>,
    answer_day_2: Option<Answer>,
    example_input_day_2: Option<&'static str>,
    answer_example_day_2: Option<Answer>,
    answer_day_1: Option<Answer>,
}

impl Assignment {
    pub fn new(options: AssignmentOptions) -> Assignment {
        return Assignment {
            day: options.day,
            description: options.description,
            cases: TestCaseGroup {
                example_day_1: options.example_input_day_1.and(Some(TestCase {
                    input: options.example_input_day_1,
                    expected: options.answer_example_day_1,
                })),
                day1: Some(TestCase {
                    input: None,
                    expected: options.answer_day_1,
                }),
                example_day_2: options.example_input_day_2.and(Some(TestCase {
                    input: options.example_input_day_2,
                    expected: options.answer_example_day_2,
                })),
                day2: Some(TestCase {
                    input: None,
                    expected: options.answer_day_2,
                }),
            },
            _f: options.run,
        };
    }

    pub fn run(&self) -> TestCaseGroup<Option<TestCaseOutput>> {
        TestCaseGroup {
            example_day_1: self
                .cases
                .example_day_1
                .as_ref()
                .map(|case| self._run_test_case(case, false)),
            day1: self
                .cases
                .day1
                .as_ref()
                .map(|case| self._run_test_case(case, false)),
            example_day_2: self
                .cases
                .example_day_2
                .as_ref()
                .map(|case| self._run_test_case(case, true)),
            day2: self
                .cases
                .day2
                .as_ref()
                .map(|case| self._run_test_case(case, true)),
        }
    }

    fn _run_test_case(&self, test_case: &TestCase, is_day_2: bool) -> TestCaseOutput {
        let lines = match test_case.input {
            Some(i) => i.lines().map(|s| s.to_string()).collect::<Vec<_>>(),
            None => {
                let filename = format!("src/assignments/assignment_{}.txt", self.day);
                let lines = match _read_lines(&filename) {
                    Ok(lines) => lines,
                    Err(e) => panic!(
                        "
Could not read file \"{}\": {}

Assignment {} has one or more test cases that require an input file.
This file should be located at \"{}\", but it could not be read.

Create this file and try again.",
                        &filename, e, self.day, &filename
                    ),
                };
                lines.filter_map(Result::ok).collect::<Vec<_>>()
            }
        };
        let lines = lines
            .iter()
            .skip_while(|a| a.is_empty())
            .map(|a| a.to_string())
            .collect::<Vec<_>>();

        let expected = test_case.expected.clone();

        let mut stopwatch = Stopwatch::start_new();
        let actual = (self._f)(&lines, is_day_2);
        let runtime = stopwatch.elapsed();
        stopwatch.stop();

        TestCaseOutput {
            input: lines.join("\n"),
            expected,
            actual,
            runtime,
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
