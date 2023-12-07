mod assignment_1;

use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;

mod prelude;
pub use prelude::*;
use stopwatch::Stopwatch;

pub fn get_assignments() -> Vec<Assignment> {
    return vec![assignment_1::get_assignment()];
}

#[derive(PartialEq, Clone)]
pub enum Answer {
    None,
    Integer(u32),
    String(String),
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        match self {
            Answer::None => "None".to_string(),
            Answer::Integer(i) => i.to_string(),
            Answer::String(s) => format!("\"{s}\""),
        }
    }
}

impl From<u32> for Answer {
    fn from(value: u32) -> Self {
        Answer::Integer(value)
    }
}

impl From<Option<u32>> for Answer {
    fn from(value: Option<u32>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Answer::None,
        }
    }
}

impl From<i32> for Answer {
    fn from(value: i32) -> Self {
        if value < 0 {
            panic!("Integer value has to be positive.");
        }
        Answer::Integer(value as u32)
    }
}

impl From<Option<i32>> for Answer {
    fn from(value: Option<i32>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Answer::None,
        }
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Answer::String(value)
    }
}

impl From<Option<String>> for Answer {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Answer::None,
        }
    }
}

impl From<&str> for Answer {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}

impl From<Option<&str>> for Answer {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Answer::None,
        }
    }
}

impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        Answer::Integer(value as u32)
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

impl TestCase {
    pub fn from_string(input: &'static str, expected: Option<Answer>) -> TestCase {
        return TestCase {
            input: Some(input),
            expected,
        };
    }

    pub fn from_file(expected: Option<Answer>) -> TestCase {
        return TestCase {
            input: None,
            expected,
        };
    }
}

pub struct TestCaseOutput {
    pub input: String,
    pub expected: Option<Answer>,
    pub actual: Result<Answer, String>,
    pub runtime: Duration,
}

impl TestCaseOutput {
    pub fn get_result(&self) -> TestCaseResult {
        match &self.actual {
            Ok(a) => match a {
                Answer::None => TestCaseResult::NoAnswer,
                _ => match &self.expected {
                    Some(e) => {
                        if a == e {
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
    pub cases: TestCaseGroup<TestCase>,
    _f: InternalAssignmentCallback,
}

type InternalAssignmentCallback = fn(data: &Vec<String>, is_day_2: bool) -> Result<Answer, String>;

impl Assignment {
    pub fn new(
        day: u32,
        description: &'static str,
        cases: TestCaseGroup<TestCase>,
        run: InternalAssignmentCallback,
    ) -> Assignment {
        return Assignment {
            day,
            description,
            cases,
            _f: run,
        };
    }

    pub fn run(&self) -> TestCaseGroup<TestCaseOutput> {
        TestCaseGroup {
            example_day_1: self._run_test_case(&self.cases.example_day_1, false),
            day1: self._run_test_case(&self.cases.day1, false),
            example_day_2: self._run_test_case(&self.cases.example_day_2, true),
            day2: self._run_test_case(&self.cases.day2, true),
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
        return TestCaseOutput {
            input: lines.join("\n"),
            expected,
            actual,
            runtime,
        };
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
