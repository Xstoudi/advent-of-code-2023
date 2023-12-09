use enum_map::{Enum, enum_map};
use crate::traits::solver::Solver;

pub struct Day02;

impl Day02 {
    fn get_games(&self, input: String) -> Vec<Game> {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let id = line
                    .chars()
                    .skip(5)
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<i32>()
                    .expect("Id is not a digit");

                let draws = line
                    .split(":")
                    .nth(1)
                    .expect("Missing draw list")
                    .split(",")
                    .flat_map(|e| e.split(";"))
                    .map(|e| e.trim())
                    .map(|e| {
                        let mut split = e.split(" ");
                        let count = split.nth(0).expect("Missing count");
                        let color = split.nth(0).expect("Missing color");
                        (count, color)
                    })
                    .map(|(count, color)| {
                        let color = match color {
                            "red" => Color::Red,
                            "green" => Color::Green,
                            "blue" => Color::Blue,
                            _ => panic!("Unknown color"),
                        };
                        let count = count
                            .chars()
                            .take_while(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<i32>()
                            .expect("Count is not a digit");
                        Draw { color, count }
                    })
                    .collect::<Vec<Draw>>();
                Game { id, draws }
            }).collect::<Vec<Game>>()
    }
}

impl Solver for Day02 {
    fn day(&self) -> usize {
        2
    }

    fn name(&self) -> String {
        String::from("Cube Conundrum")
    }

    fn solve_first(&self) -> u128 {
        self.get_games(self.input_first())
            .iter()
            .filter(|game| {
                game.draws.iter().all(|draw| {
                    match draw.color {
                        Color::Red => draw.count <= 12,
                        Color::Green => draw.count <= 13,
                        Color::Blue => draw.count <= 14,
                    }
                })
            })
            .map(|game| game.id)
            .sum::<i32>() as u128
    }

    fn solve_second(&self) -> u128 {
        self.get_games(self.input_second())
            .iter()
            .map(|game| {
                let mut map = enum_map! {
                    Color::Red => 0,
                    Color::Green => 0,
                    Color::Blue => 0,
                };

                for draw in game.draws.iter() {
                    if map[draw.color] < draw.count {
                        map[draw.color] = draw.count;
                    }
                }

                return map
            })
            .map(|map| {
                map.values().product::<i32>()
            })
            .sum::<i32>() as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day02a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day02b.txt").to_string()
    }
}

struct Game {
    id: i32,
    draws: Vec<Draw>
}

#[derive(Clone, Copy, Enum)]
enum Color {
    Red,
    Green,
    Blue,
}

struct Draw {
    color: Color,
    count: i32,
}