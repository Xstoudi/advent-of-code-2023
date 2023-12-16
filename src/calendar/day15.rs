use itertools::Itertools;
use crate::traits::solver::Solver;

pub struct Day15;

impl Day15 {
    fn hash(input: String) -> u32 {
        input
            .chars()
            .fold(0, |mut acc, c| {
                acc += c as u32;
                acc *= 17;
                acc %= 256;
                acc
            })
    }
}

impl Solver for Day15 {
    fn day(&self) -> usize {
        15
    }

    fn name(&self) -> String {
        String::from("Lens Library")
    }

    fn solve_first(&self) -> u128 {
        self.input_first()
            .lines()
            .join("")
            .split(",")
            .map(String::from)
            .map(Day15::hash)
            .sum::<u32>() as u128
    }

    fn solve_second(&self) -> u128 {
        self.input_second()
            .lines()
            .join("")
            .split(",")
            .map(|step| {

                Step::new(
                    step
                        .chars()
                        .take_while(char::is_ascii_alphabetic)
                        .collect::<String>(),
                    step
                        .chars()
                        .skip_while(char::is_ascii_alphabetic)
                        .take(1)
                        .last()
                        .expect("Failed to parse operation"),
                    step
                        .chars()
                        .skip_while(char::is_ascii_alphabetic)
                        .skip(1)
                        .take_while(char::is_ascii_digit)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap_or(0)
                )
            })
            .fold(
                (0..256)
                      .map(|_| Vec::<Lens>::new())
                      .collect::<Vec<Vec<Lens>>>(),
                |mut acc, step| {
                    let lens = Lens::new(step.label, step.focal_length);
                    let hash = step.hash as usize;
                    match step.operation {
                        '-' => {
                            acc[hash]
                                .clone()
                                .iter()
                                .find_position(|l| l.label == lens.label)
                                .map(|(i, _)| acc[hash].remove(i));
                        },
                        '=' => {
                            acc[hash]
                                .clone()
                                .iter()
                                .find_position(|l| l.label == lens.label)
                                .map(|(i, _)| acc[hash][i] = lens.clone())
                                .or_else(|| {
                                    acc[hash].push(lens);
                                    Some(())
                                });
                        },
                        _ => panic!("Unknown operation")
                    }
                    acc
                }
            )
            .iter()
            .enumerate()
            .flat_map(|(box_index, lenses)|
                lenses
                    .iter()
                    .enumerate()
                    .map(move |(lens_slot, lens)| (box_index + 1) * (lens_slot + 1) * lens.focal_length)
            )
            .sum::<usize>() as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day15.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day15.txt").to_string()
    }
}

struct Step {
    label: String,
    operation: char,
    focal_length: usize,
    hash: u32,
}

impl Step {
    fn new(label: String, operation: char, focal_length: usize) -> Self {
        let hash = Day15::hash(label.clone());
        Self {
            label,
            operation,
            focal_length,
            hash,
        }
    }
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl Lens {
    fn new(label: String, focal_length: usize) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}
