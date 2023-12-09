use crate::traits::solver::Solver;
pub struct Day01;

impl Solver for Day01 {
    fn day(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("Trebuchet?!")
    }

    fn solve_first(&self) -> u128 {
        self.input_first()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let left_index = line
                    .find(|c: char| c.is_digit(10))
                    .expect("Left digit not found");
                let right_index = line
                    .rfind(|c: char| c.is_digit(10))
                    .expect("Right digit not found");
                let left = line.chars().skip(left_index).take(1).collect::<String>();
                let right = line.chars().skip(right_index).take(1).collect::<String>();

                let result = 10 * left.parse::<i32>().expect("Left is not a digit")
                    + right.parse::<i32>().expect("Right is not a digit");
                result
            })
            .sum::<i32>() as u128
    }

    fn solve_second(&self) -> u128 {
        let word_digit_map = [
            ("one", "one1one"),
            ("two", "two2two"),
            ("three", "three3three"),
            ("four", "four4four"),
            ("five", "five5five"),
            ("six", "six6six"),
            ("seven", "seven7seven"),
            ("eight", "eight8eight"),
            ("nine", "nine9nine"),
        ];

        self.input_second()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut line = line.to_string();

                for (word, digit) in word_digit_map.iter() {
                    line = line.replace(word, digit);
                }
                line = line.replace(char::is_alphabetic, "");

                let left_index = line
                    .find(|c: char| c.is_digit(10))
                    .expect("Left digit not found");
                let right_index = line
                    .rfind(|c: char| c.is_digit(10))
                    .expect("Right digit not found");
                let left = line.chars().skip(left_index).take(1).collect::<String>();
                let right = line.chars().skip(right_index).take(1).collect::<String>();

                let result = 10 * left.parse::<i32>().expect("Left is not a digit")
                    + right.parse::<i32>().expect("Right is not a digit");
                result
            })
            .sum::<i32>() as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day01a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day01b.txt").to_string()
    }
}
