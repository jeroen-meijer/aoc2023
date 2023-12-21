mod assignments;

use assignments::{get_assignments, Assignment, TestCaseOutput};
use owo_colors::OwoColorize;

fn throw_invalid_assignment_number_error() -> ! {
    println!("Invalid assignment number.");
    println!(
        "Usage: src/main.rs <assignment_number[{}]>",
        get_assignments()
            .iter()
            .map(|a| a.day.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    std::process::exit(1);
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 2 {
        throw_invalid_assignment_number_error();
    }

    let should_run_single_assignment = args.len() > 1;
    if !should_run_single_assignment {
        _run_assignments(None)
    } else {
        match args[1].parse::<u32>() {
            Ok(n) => _run_assignments(Some(n)),
            Err(_) => throw_invalid_assignment_number_error(),
        }
    }
}

fn _run_assignments(assignment_number: Option<u32>) {
    let assignments = get_assignments();

    if let Some(n) = assignment_number {
        let assignment = assignments.iter().find(|a| a.day == n);
        match assignment {
            Some(a) => _run_single_assignment(a),
            None => {
                let error_text = format!(
                    "Assignment number {} does not exist. Please choose a valid assignment: {}",
                    n,
                    assignments
                        .iter()
                        .map(|a| a.day.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                println!("{}", error_text.bright_red());
                std::process::exit(1);
            }
        }
    } else {
        for assignment in assignments {
            _run_single_assignment(&assignment);
        }
    }
}

fn _run_single_assignment(assignment: &Assignment) {
    println!(
        "{}",
        format!("Day {}: {}", assignment.day, assignment.description).bold()
    );

    let outputs = assignment.run();

    fn _output_result(name: &str, output: Option<&TestCaseOutput>) {
        const MAX_NAME_CHARS: u8 = 9;
        let pad_length = MAX_NAME_CHARS - name.len() as u8;

        print!("  - {}: {}", name, " ".repeat(pad_length as usize));

        let Some(output) = output else {
            println!("{}", "➖ No input.".black());
            return;
        };

        match output.get_result() {
            assignments::TestCaseResult::NoAnswer => print!("{}", "❓ No answer.".yellow()),
            assignments::TestCaseResult::Correct => print!("{}", "✅ Correct.".green()),
            assignments::TestCaseResult::Incorrect => print!("{}", "❌ Incorrect.".bright_red()),
            assignments::TestCaseResult::Error => print!("{}", "🚨 Error.".red().bold()),
            assignments::TestCaseResult::Unknown => print!("{}", "🤷 Unknown.".bright_yellow()),
        }

        match &output.expected {
            Some(e) => print!(" Expected {}.", e.to_string()),
            _ => (),
        }
        match &output.actual {
            Ok(Some(answer_value)) => {
                print!(" Answered {}.", answer_value.to_string())
            }
            Err(e) => print!(" Error: {}.", e),
            _ => (),
        }
        print!(" ({}ms)", output.runtime.as_millis());

        println!();
    }

    _output_result("Example 1", outputs.example_day_1.as_ref());
    _output_result("Day 1", outputs.day1.as_ref());
    _output_result("Example 2", outputs.example_day_2.as_ref());
    _output_result("Day 2", outputs.day2.as_ref());
}
