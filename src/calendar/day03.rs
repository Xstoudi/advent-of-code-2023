use std::ops::Mul;
use itertools::Itertools;
use crate::traits::solver::Solver;

pub struct Day03;

impl Day03 {
    fn is_next_to_symbol(&self, i: i32, j: i32, map: Vec<Vec<char>>) -> bool {
        let rows = map.len();
        let cols = map[0].len();

        let directions = vec![
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1), (1, 0),  (1, 1),
        ];

        for (di, dj) in directions {
            let new_i = i + di;
            let new_j = j + dj;

            if new_i >= 0 && new_i < rows as i32 && new_j >= 0 && new_j < cols as i32 {
                let new_i = new_i as usize;
                let new_j = new_j as usize;

                if map[new_i][new_j] == '#' {
                    return true;
                }
            }
        }

        false
    }

    fn numbers_around(&self, i: i32, j: i32, map: Vec<Vec<char>>) -> Vec<i32> {
        let rows = map.len();
        let cols = map[0].len();

        let directions = vec![
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1), (1, 0),  (1, 1),
        ];

        let mut positions = vec![];
        for (di, dj) in directions {
            let new_i = i + di;
            let new_j = j + dj;

            if new_i >= 0 && new_i < rows as i32 && new_j >= 0 && new_j < cols as i32 {
                let new_i = new_i as usize;
                let original_j = new_j as usize;
                let mut new_j = new_j as usize;

                if map[new_i][new_j].is_digit(10) {
                    let mut local_positions = vec![];
                    local_positions.push((new_i, new_j));
                    while new_j > 0 && map[new_i][new_j - 1].is_digit(10) {
                        new_j -= 1;
                        local_positions.reverse();
                        local_positions.push((new_i, new_j));
                        local_positions.reverse();

                    }
                    new_j = original_j;
                    while new_j < cols - 1 && map[new_i][new_j + 1].is_digit(10) {
                        new_j += 1;
                        local_positions.push((new_i, new_j));
                    }

                    if positions.contains(&local_positions) {
                        continue;
                    }
                    positions.push(local_positions);
                }
            }
        }

        positions
            .iter()
            .map(|local_positions| {
                let mut number = String::from("");
                for (i, j) in local_positions {
                    number.push(map[*i][*j]);
                }
                number.parse::<i32>().expect("Not a number")
            })
            .collect::<Vec<i32>>()
    }
}

impl Solver for Day03 {
    fn day(&self) -> usize {
        3
    }

    fn name(&self) -> String {
        String::from("Gear Ratios")
    }

    fn solve_first(&self) -> u128 {
        let controls = self.input_first()
            .lines()
            .flat_map(|line| line.chars())
            .filter(|c| !c.is_digit(10) && !c.eq(&'.'))
            .unique()
            .collect::<Vec<char>>();

        let lines = self.input_first()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.replace(|c: char| controls.contains(&c), "#"))
            .map(|line| format!("{}.", line))
            .collect::<Vec<String>>();

        let column_length = lines.clone().len();
        let row_length = lines.clone().iter().next().expect("No lines").len();

        let mut grid_raw = vec![vec![' '; row_length]; column_length];

        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid_raw[i][j] = c
            }
        }

        // iterate over lines and columns of grid_raw on next line
        let mut numbers = vec![];
        for (i, line) in grid_raw.iter().enumerate() {
            let mut current_number = String::from("");
            let mut is_next_to_symbol = false;
            for (j, c) in line.iter().enumerate() {
                if c.is_digit(10) {
                    current_number.push(*c);
                    is_next_to_symbol = is_next_to_symbol || self.is_next_to_symbol(i as i32, j as i32, grid_raw.clone());
                    continue;
                }
                if is_next_to_symbol {
                    numbers.push(current_number.parse::<i32>().expect("Not a number"));
                }

                current_number = String::from("");
                is_next_to_symbol = false;
            }
        }

        numbers.iter().sum::<i32>() as u128
    }

    fn solve_second(&self) -> u128 {
        let lines = self.input_second()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let column_length = lines.clone().len();
        let row_length = lines.clone().iter().next().expect("No lines").len();

        let mut grid_raw = vec![vec![' '; row_length]; column_length];

        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid_raw[i][j] = c
            }
        }

        let mut sum = 0;
        for (i, line) in grid_raw.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '*' {
                    let numbers = self.numbers_around(i as i32, j as i32, grid_raw.clone());
                    if numbers.len() != 2 {
                        continue;
                    }
                    sum += numbers.get(0).expect("No numbers left").mul(numbers.get(1).expect("No numbers right"));
                }
            }
        }

        sum as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day03a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day03b.txt").to_string()
    }
}
