use grid::Grid;
use hashbrown::HashMap;
use crate::traits::solver::Solver;

pub struct Day14;

impl Day14 {
    fn char_to_type(c: char) -> TileType {
        match c {
            '.' => TileType::Empty,
            '#' => TileType::Rock,
            'O' => TileType::RollingRock,
            _ => panic!("Unknown tile type: {}", c)
        }
    }

    fn parse_grid(input: String) -> Grid<TileType> {
        let lines = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line|
                line
                    .chars()
                    .map(Day14::char_to_type)
                    .collect::<Vec<TileType>>()
            )
            .collect::<Vec<Vec<TileType>>>();

        let mut grid = Grid::from_vec(
            lines.iter().flatten().map(|t| *t).collect::<Vec<TileType>>(),
            lines[0].len()
        );
        grid.rotate_right();
        grid
    }

    #[allow(dead_code)]
    fn print_grid(grid: &mut Grid<TileType>) {
        grid.rotate_left();
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                unsafe {
                    match grid.get_unchecked(row, col) {
                        TileType::Empty => print!("."),
                        TileType::Rock => print!("#"),
                        TileType::RollingRock => print!("O")
                    }
                }
            }
            println!();
        }
        grid.rotate_right();
    }

    fn evaluate_grid(grid: &Grid<TileType>) -> usize {
        grid
            .iter_rows()
            .enumerate()
            .flat_map(|(_, row)| {
                row
                    .enumerate()
                    .map(|(x, tile)|
                        match tile {
                            TileType::RollingRock => {
                                x + 1
                            },
                            _ => {
                                0
                            }
                        }
                    )
            })
            .sum()
    }

    fn tilt(grid: &mut Grid<TileType>) {
        let row_count = grid.rows();
        let col_count = grid.cols();
        let mut resulting_grid = Grid::new(row_count, col_count);

        grid
            .iter_rows()
            .enumerate()
            .for_each(|(y, row)| {
                let mut rolling_count = 0;
                row
                    .enumerate()
                    .for_each(|(x, tile)| {
                        let result = match tile {
                            TileType::RollingRock => {
                                rolling_count += 1;
                                TileType::Empty
                            },
                            TileType::Rock => {
                                for i in 0..rolling_count {
                                    resulting_grid[(y, x - 1 - i)] = TileType::RollingRock;
                                }
                                rolling_count = 0;
                                TileType::Rock
                            },
                            TileType::Empty => {
                                TileType::Empty
                            }
                        };
                        resulting_grid[(y, x)] = result;
                    });
                for i in 0..rolling_count {
                    resulting_grid[(y, col_count - 1 - i)] = TileType::RollingRock;
                }
            });

        *grid = resulting_grid;
    }

    fn cycle(grid: &mut Grid<TileType>) {
        for _ in 0..4 {
            Day14::tilt(grid);
            // println!("After tilt {}: ", i+1);
            // Day14::print_grid(grid);
            // println!();
            grid.rotate_right();
        }
    }
}

impl Solver for Day14 {
    fn day(&self) -> usize {
        14
    }

    fn name(&self) -> String {
        String::from("Parabolic Reflector Dish")
    }

    fn solve_first(&self) -> u128 {
        let mut grid = Day14::parse_grid(self.input_first());

        Day14::tilt(&mut grid);

        Day14::evaluate_grid(&grid) as u128
    }

    fn solve_second(&self) -> u128 {
        let mut grid = Day14::parse_grid(self.input_second());

        let mut memoize = HashMap::new();

        let cycle_count = 1000000000;

        for i in 0..cycle_count {
            Day14::cycle(&mut grid);
            let maybe_memoized_index = memoize.get(&grid.clone().into_vec());
            match maybe_memoized_index {
                Some(memoized_index) => {
                    let cycle_length = i - memoized_index;
                    let remaining_cycles = cycle_count - i - 1;
                    let remaining_cycles = remaining_cycles % cycle_length;
                    for _ in 0..remaining_cycles {
                        Day14::cycle(&mut grid);
                    }
                    break;
                },
                None => {
                    memoize.insert(grid.clone().into_vec(), i);
                }
            }
        }

        Day14::evaluate_grid(&grid) as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day14.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day14.txt").to_string()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Eq, Hash)]
enum TileType {
    #[default]
    Empty,
    Rock,
    RollingRock
}