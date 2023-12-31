use super::prelude::*;
use std::collections::HashMap;

pub fn get_assignment() -> Assignment {
    Assignment::new(
        // cspell: disable
        AssignmentOptions {
            day: 2,
            description: "Cube Conundrum",
            run: _run,
            example_input_day_1: Some(
                "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            ),
            answer_example_day_1: Some(8.into()),
            answer_day_1: Some(3059.into()),
            example_input_day_2: Some(
                "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            ),
            answer_example_day_2: Some(2286.into()),
            answer_day_2: Some(65371.into()),
        },
        // cspell: enable
    )
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>, String> {
    let games = context
        .data
        .iter()
        .map(|line| Game::parse(&line))
        .collect::<Vec<_>>();

    if context.part_number == 1 {
        _run_part_1(context, games)
    } else {
        _run_part_2(context, games)
    }
}

fn _run_part_1(
    context: AssignmentRuntimeContext,
    games: Vec<Game>,
) -> Result<Option<Answer>, String> {
    let limits = CubeLimits {
        red: 12,
        green: 13,
        blue: 14,
    };

    if context.logging_enabled {
        for game in &games {
            print!("Game {}: ", game.id);
            for pull in &game.pulls {
                let mut pull_parts: Vec<String> = vec![];

                if pull.reds != 0 {
                    pull_parts.push(format!("{} red", pull.reds));
                }
                if pull.greens != 0 {
                    pull_parts.push(format!("{} green", pull.greens));
                }
                if pull.blues != 0 {
                    pull_parts.push(format!("{} blue", pull.blues));
                }
                print!("{}", pull_parts.join(", "));
                print!("; ");
            }
            println!("");
        }
    }

    let possible_game_ids_sum = games
        .iter()
        .filter(|game| {
            game.pulls.iter().all(|pull| {
                pull.reds <= limits.red && pull.greens <= limits.green && pull.blues <= limits.blue
            })
        })
        .map(|game| game.id)
        .sum::<u32>();

    Ok(Some(possible_game_ids_sum.into()))
}

fn _run_part_2(
    _context: AssignmentRuntimeContext,
    games: Vec<Game>,
) -> Result<Option<Answer>, String> {
    let games_with_minimal_pulls = games.iter().map(|game| {
        (
            game,
            game.pulls.iter().fold(Pull::default(), |acc, pull| Pull {
                reds: acc.reds.max(pull.reds),
                greens: acc.greens.max(pull.greens),
                blues: acc.blues.max(pull.blues),
            }),
        )
    });

    let game_powers_summed = games_with_minimal_pulls
        .map(|(_, pull)| pull.reds * pull.greens * pull.blues)
        .sum::<u32>();

    Ok(Some(game_powers_summed.into()))
}

struct CubeLimits {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

impl Game {
    fn parse(line: &String) -> Game {
        let parts = line.split(":").collect::<Vec<_>>();
        let game_string_part = parts.first().unwrap();
        let pulls_string_part = parts.last().unwrap();

        let id = game_string_part
            .split(" ")
            .last()
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap();

        let pulls = pulls_string_part
            .split(";")
            .map(|pull_string| Pull::parse(&pull_string.to_owned()))
            .collect::<Vec<_>>();

        Game { id, pulls: pulls }
    }
}

#[derive(Debug, Default)]
struct Pull {
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Pull {
    fn parse(line: &String) -> Pull {
        let parts = line
            .split(",")
            .map(str::trim)
            .map(|item| {
                let item_parts = item.split(" ").collect::<Vec<_>>();
                let count = item_parts.first().unwrap().parse::<u32>().unwrap();
                let cube_type = item_parts.last().unwrap();

                (*cube_type, count)
            })
            .fold(HashMap::<&str, u32>::new(), |mut acc, cur| {
                acc.insert(cur.0, cur.1);
                acc
            });

        Pull {
            reds: *parts.get("red").unwrap_or(&0),
            greens: *parts.get("green").unwrap_or(&0),
            blues: *parts.get("blue").unwrap_or(&0),
        }
    }
}
