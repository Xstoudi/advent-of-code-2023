use std::ops::Mul;
use itertools::Itertools;
use crate::traits::solver::Solver;

pub struct DayXX;

impl DayXX {

}

impl Solver for DayXX {
    fn name(&self) -> String {
        String::from("Gear Ratios")
    }

    fn solve_first(&self) -> f64 {
        0.0
    }

    fn solve_second(&self) -> f64 {
        0.0
    }

    fn input_first(&self) -> String {
        include_str!("../resource/dayXXa.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/dayXXb.txt").to_string()
    }
}
