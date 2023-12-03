use took::Timer;
use crate::structs::result::Result;

pub trait Solver {
    fn name(&self) -> String;
    fn run(&self, index: usize) -> Vec<Result> {
        let first_timer = Timer::new();
        let first_result  = (self.solve_first(), first_timer.took().into_std());

        let second_timer = Timer::new();
        let second_result = (self.solve_second(), second_timer.took().into_std());

        let binding = (index + 1).to_string();
        vec![
            Result::new(String::from("[") + binding.as_str() + &String::from("-1] ") + &self.name(), first_result.0, first_result.1),
            Result::new(String::from("[") + binding.as_str() + &String::from("-2] ") + &self.name(), second_result.0, second_result.1),
        ]
    }
    fn solve_first(&self) -> f64;
    fn solve_second(&self) -> f64;

    fn input_first(&self) -> String;
    fn input_second(&self) -> String;
}
