use std::collections::BinaryHeap;
use grid::Grid;
use hashbrown::HashMap;
use crate::traits::solver::Solver;

pub struct Day17;

impl Day17 {
    fn dijkstra(grid: &Grid<u8>, min_step: isize, max_step: isize) -> i64 {
        let start_position = (0, 0);
        let end_position = (grid.rows() - 1, grid.cols() - 1);

        let mut dists = HashMap::new();
        let mut queue = BinaryHeap::new();

        queue.push((0, (start_position, (0, 0))));

        while let Some((cost, (current_position, current_direction))) = queue.pop() {
            if current_position == end_position {
                return -cost;
            }

            if dists.get(&(current_position, current_direction)).is_some_and(|&c| -cost > c) {
                continue;
            }

            for (dy, dx) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                if current_direction == (dy, dx) || current_direction == (-dy, -dx) {
                    continue;
                }

                let mut next_cost = -cost;
                for dist in 1..=max_step {
                    let next_position = (
                        (current_position.0 as isize + dy * dist) as usize,
                        (current_position.1 as isize + dx * dist) as usize,
                    );
                    if next_position.0 >= grid.rows() || next_position.1 >= grid.cols() {
                        continue;
                    }
                    next_cost += grid[next_position] as i64;
                    if dist < min_step {
                        continue;
                    }
                    let key = (next_position, (dy, dx));
                    if next_cost < *dists.get(&key).unwrap_or(&i64::MAX) {
                        dists.insert(key, next_cost);
                        queue.push((-next_cost, key));
                    }
                }
            }
        }

        unreachable!("No path found");
    }

    fn parse_grid(input: &str) -> Grid<u8> {
        let weights = input
            .lines()
            .filter(|line| !line.is_empty())
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10).expect(format!("Invalid input {}", c).as_str()) as u8)
            .collect::<Vec<u8>>();
        let line_size = input
            .lines()
            .next()
            .unwrap()
            .len();

        Grid::from_vec(weights, line_size)
    }
}

impl Solver for Day17 {
    fn day(&self) -> usize {
        17
    }

    fn name(&self) -> String {
        String::from("Clumsy Crucible")
    }

    fn solve_first(&self) -> u128 {
        let grid = Self::parse_grid(&self.input_first());
        Self::dijkstra(&grid, 1, 3) as u128
    }

    fn solve_second(&self) -> u128 {
        let grid = Self::parse_grid(&self.input_first());
        Self::dijkstra(&grid, 4, 10) as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day17.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day17.txt").to_string()
    }
}
