use std::collections::HashMap;
use crate::traits::solver::Solver;

pub struct Day12;

impl Day12 {
    fn char_to_spring_status(c: char) -> SpringStatus {
        match c {
            '.' => SpringStatus::Operational,
            '#' => SpringStatus::Damaged,
            '?' => SpringStatus::Unknown,
            _ => panic!("Invalid spring status")
        }
    }
}

impl Solver for Day12 {
    fn day(&self) -> usize {
        12
    }

    fn name(&self) -> String {
        String::from("Hot Springs")
    }

    fn solve_first(&self) -> u128 {
        let spring_sequences = self.input_first()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split(" ");
                let status = parts
                    .next()
                    .unwrap()
                    .chars()
                    .map(|c| Day12::char_to_spring_status(c))
                    .collect::<Vec<SpringStatus>>();
                let continuous_damaged = parts
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|part| part.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                SpringSequence::new(status, continuous_damaged)
            })
            .collect::<Vec<SpringSequence>>();

        let cached_permutations = HashMap::new();

        let result = spring_sequences
            .iter()
            .map(|spring_sequence| {
                let unknown_indices = spring_sequence.status
                    .iter()
                    .enumerate()
                    .filter(|(_, status)| **status == SpringStatus::Unknown)
                    .map(|(index, _)| index)
                    .collect::<Vec<usize>>();

                let max_permutation_value = u32::pow(2, unknown_indices.len() as u32);

                let permutations = (0..max_permutation_value)
                    .map(|permutation_value| {
                        let defined_status = (0..unknown_indices.len())
                            .map(|index| {
                                let is_set = (permutation_value >> index) & 1 == 1;
                                if is_set {
                                    SpringStatus::Damaged
                                } else {
                                    SpringStatus::Operational
                                }
                            })
                            .collect::<Vec<SpringStatus>>();

                        let mut status = spring_sequence.status.clone();
                        for (index, unknown_index) in unknown_indices.iter().enumerate() {
                            status[*unknown_index] = defined_status[index].clone();
                        }
                        spring_sequence.clone_with_status(status)
                    });

                permutations
                    .filter(|spring_sequence| spring_sequence.is_valid())
                    .count()
            })
            .sum::<usize>();

        println!("Result: {}", result);

        println!("Hello, world!");

        result as u128
    }

    fn solve_second(&self) -> u128 {
        0
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day12a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day12b.txt").to_string()
    }
}

struct SpringSequence {
    status: Vec<SpringStatus>,
    continuous_damaged: Vec<usize>,
}

impl SpringSequence {
    fn new(status: Vec<SpringStatus>, continuous_damaged: Vec<usize>) -> SpringSequence {
        SpringSequence {
            status,
            continuous_damaged,
        }
    }

    fn is_valid(&self) -> bool {
        let mut current_continuous_damaged = 0;
        let mut is_in_continuous_damaged = false;

        let mut continuous_damaged_found = Vec::new();

        for status in &self.status {
            match status {
                SpringStatus::Operational => {
                    if is_in_continuous_damaged {
                        continuous_damaged_found.push(current_continuous_damaged);
                        current_continuous_damaged = 0;
                        is_in_continuous_damaged = false;
                    }
                },
                SpringStatus::Damaged => {
                    is_in_continuous_damaged = true;
                    current_continuous_damaged += 1;
                },
                SpringStatus::Unknown => {
                    panic!("Cannot check validity on undetermined spring status")
                }
            }
        }

        if is_in_continuous_damaged {
            continuous_damaged_found.push(current_continuous_damaged);
        }


        let is_valid = self.continuous_damaged == continuous_damaged_found;


        is_valid
    }

    fn clone_with_status(&self, status: Vec<SpringStatus>) -> SpringSequence {
        SpringSequence {
            status,
            continuous_damaged: self.continuous_damaged.clone(),
        }
    }

}

#[derive(Debug, PartialEq, Clone)]
enum SpringStatus {
    Operational,
    Damaged,
    Unknown
}