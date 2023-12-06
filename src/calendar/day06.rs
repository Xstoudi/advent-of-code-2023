use std::ops::Mul;
use itertools::{Itertools, izip};
use crate::traits::solver::Solver;

pub struct Day06;

impl Day06 {
    fn pairs(&self, numbers: Vec<u64>) -> Vec<(u64, u64)> {
        numbers
            .iter()
            .enumerate()
            .filter_map(|(index, &value)| {
                let paired = numbers.get(index + numbers.len() / 2);
                paired
                    .map(|x| (value, *x))
            })
            .collect::<Vec<(u64, u64)>>()
    }
    fn compute_for_pairs(&self, pairs: Vec<(u64, u64)>) -> f64 {
        pairs
            .iter()
            .map(|(time, record)| {
                (0..*time)
                    .map(|time_charging| {
                        let remaining_time = time - time_charging;
                        let distance = time_charging * remaining_time;
                        (distance, time_charging)
                    })
                    .filter(|(distance, time_charging)| distance > &record)
                    .take(1)
                    .map(|(distance, time_charging)| time - (time_charging * 2) + 1)
                    .last()
                    .unwrap_or(0)
            })
            .product::<u64>() as f64
    }
}

impl Solver for Day06 {
    fn day(&self) -> usize {
        6
    }

    fn name(&self) -> String {
        String::from("Wait For It")
    }

    fn solve_first(&self) -> f64 {
        let parsed = self.input_first()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split(":"))
            .flat_map(|mut line|
                line
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            )
            .collect::<Vec<u64>>();

        let pairs = self.pairs(parsed);

        self.compute_for_pairs(pairs)
    }

    fn solve_second(&self) -> f64 {
        let parsed = self.input_second()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split(":"))
            .flat_map(|mut line|
                line
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split(" ")
                    .join("")
                    .parse::<u64>()
            )
            .collect::<Vec<u64>>();

        let pairs = self.pairs(parsed);

        self.compute_for_pairs(pairs)
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day06a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day06b.txt").to_string()
    }
}
