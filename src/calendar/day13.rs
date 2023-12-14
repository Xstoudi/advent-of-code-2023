use grid::{Grid};
use itertools::Itertools;
use crate::traits::solver::Solver;

pub struct Day13;

impl Day13 {
    fn find_horizontal_mirror(grid: &Grid<char>, row: usize) -> usize {
        let mut diff = 0;

        loop {
            if row < diff || row + 1 + diff >= grid.rows() {
                break;
            }

            let row_a = grid.iter_row(row - diff).map(|c| *c).collect::<Vec<char>>();
            let row_b = grid.iter_row(row + 1 + diff).map(|c| *c).collect::<Vec<char>>();
            if row_a == row_b {
                diff += 1;
                continue;
            }
            break;
        }

        diff
    }

    fn find_vertical_mirror(grid: &Grid<char>, col: usize) -> usize {
        let mut diff = 0;

        loop {
            if col < diff || col + 1 + diff >= grid.cols() {
                break;
            }

            let col_a = grid.iter_col(col - diff).map(|c| *c).collect::<Vec<char>>();
            let col_b = grid.iter_col(col + 1 + diff).map(|c| *c).collect::<Vec<char>>();
            if col_a == col_b {
                diff += 1;
                continue;
            }
            break;
        }
        diff
    }

    fn find_mirror(grid: Grid<char>, exclude: (&'static str, usize, usize)) -> Option<(&'static str, usize, usize)> {
        let possibles = vec![
            (0..grid.rows() - 1)
                .map(|row_index| (row_index, Day13::find_horizontal_mirror(&grid, row_index)))
                .filter(|(_, mirror_size)| *mirror_size > 0)
                .filter(|(row_index, mirror_size)|
                    (*row_index as isize + 1) - *mirror_size as isize == 0 || row_index + mirror_size == grid.rows() -1
                )
                .filter(|(col_index, mirror_size)| {
                    "row" != exclude.0
                        || *col_index != exclude.1
                        || *mirror_size != exclude.2
                })
                .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
                .last()
                .map(|(col_index, mirror_size)| ("row", col_index, mirror_size))
                .unwrap_or(("none", 0, 0)),
            (0..grid.cols() - 1)
                .map(|col_index| (col_index, Day13::find_vertical_mirror(&grid, col_index)))
                .filter(|(_, mirror_size)| *mirror_size > 0)
                .filter(|(col_index, mirror_size)|
                    (*col_index as isize + 1) - *mirror_size as isize == 0 || col_index + mirror_size == grid.cols() - 1
                )
                .filter(|(col_index, mirror_size)| {
                    "col" != exclude.0
                        || *col_index != exclude.1
                        || *mirror_size != exclude.2
                })
                .sorted_by(|a, b|
                    Ord::cmp(&a.1, &b.1)
                )
                .last()
                .map(|(col_index, mirror_size)| ("col", col_index, mirror_size))
                .unwrap_or(("none", 0, 0))
        ];

        let filtered_possibles = possibles
            .iter()
            .filter(|(name, index, mirror_size)| {
                *name != exclude.0
                    || *index != exclude.1
                    || *mirror_size != exclude.2
            })
            .filter(|(name, _, _)| *name != "none")
            .collect::<Vec<&(&str, usize, usize)>>();

        filtered_possibles
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.2, &b.2))
            .last()
            .map(|x| **x)
    }
}

impl Solver for Day13 {
    fn day(&self) -> usize {
        13
    }

    fn name(&self) -> String {
        String::from("Point of Incidence")
    }

    fn solve_first(&self) -> u128 {
        let grids = self.input_first()
            .replace("\r\n", "\n")
            .split("\n\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(String::from)
            .map(|str| {
                Grid::from_vec(
                    str
                        .chars()
                        .filter(|x| *x != '\n')
                        .collect::<Vec<char>>(),
                    str
                        .lines()
                        .last()
                        .unwrap()
                        .len(),
                )
            })
            .collect::<Vec<Grid<char>>>();

        let mut sum = 0;
        for grid in grids {
            let result = Day13::find_mirror(grid, ("none", 0, 0)).expect("No mirror found");
            sum += match result.0 {
                "col" => result.1 + 1,
                "row" => (result.1 + 1) * 100,
                _ => 0,
            };
        }

        sum as u128
    }

    // 96444 too low
    fn solve_second(&self) -> u128 {
        let grids = self.input_second()
            .replace("\r\n", "\n")
            .split("\n\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(String::from)
            .map(|str| {
                let initial = Grid::from_vec(
                    str
                        .chars()
                        .filter(|x| *x != '\n')
                        .collect::<Vec<char>>(),
                    str
                        .lines()
                        .last()
                        .unwrap()
                        .len(),
                );

                (
                    initial.clone(),
                    (0..initial.cols())
                        .flat_map(|col|
                            (0..initial.rows())
                                .map(|row| {
                                    let mut grid = initial.clone();
                                    grid[(row, col)] = if grid[(row, col)] == '.' { '#' } else { '.' };
                                    grid
                                })
                                .collect::<Vec<Grid<char>>>()
                        )
                        .collect::<Vec<Grid<char>>>()
                )
            })
            .collect::<Vec<(Grid<char>, Vec<Grid<char>>)>>();

        let mut sum = 0;
        for (initial, variants) in grids {
            let initial_result = Day13::find_mirror(initial, ("none", 0, 0)).unwrap_or(("none", 0, 0));

            let variant_result = variants
                .iter()
                .map(|grid| Day13::find_mirror(grid.clone(), initial_result))
                .filter(|maybe_result| maybe_result.is_some())
                .map(|maybe_result| maybe_result.unwrap())
                .collect::<Vec<(& str, usize, usize)>>()
                .iter()
                .sorted_by(|a, b| Ord::cmp(&a.2, &b.2))
                .last()
                .map(|x| *x)
                .unwrap_or(("none", 0, 0));

            sum += match variant_result.0 {
                "col" => variant_result.1 + 1,
                "row" => (variant_result.1 + 1) * 100,
                _ => 0,
            };
        }

        sum as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day13.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day13.txt").to_string()
    }
}
