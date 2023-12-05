use crate::calendar::day01::Day01;
use crate::calendar::day02::Day02;
use crate::calendar::day03::Day03;
use crate::calendar::day04::Day04;
use crate::calendar::day05::Day05;
use crate::traits::solver::Solver;

pub fn get() -> Vec<&'static dyn Solver> {
    vec![
        &Day01,
        &Day02,
        &Day03,
        &Day04,
        &Day05,
    ]
}

