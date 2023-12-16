use grid::Grid;
use itertools::Itertools;
use crate::traits::solver::Solver;

pub struct Day16;

impl Day16 {
    #[allow(dead_code)]
    fn print_grid(grid: &mut Grid<Tile>) {
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                unsafe {
                    let tile = grid.get_unchecked(row, col);
                    print!("{}", if tile.energized { '#' } else { '.' });
                }
            }
            println!();
        }
    }

    fn char_to_tile_type(c: char) -> TileType {
        match c {
            '.' => TileType::Empty,
            '/' => TileType::UpReflector,
            '\\' => TileType::DownReflector,
            '|' => TileType::VerticalSplitter,
            '-' => TileType::HorizontalSplitter,
            _ => panic!("Invalid tile type {}", c),
        }
    }

    fn input_to_grid(input: String) -> Grid<Tile> {
        Grid::from_vec(
            input
                .lines()
                .flat_map(|line| {
                    line
                        .chars()
                        .map(Day16::char_to_tile_type)
                        .map(Tile::new)
                        .collect::<Vec<Tile>>()
                })
                .collect::<Vec<Tile>>(),
            input
                .lines()
                .last()
                .expect("No lines in input")
                .chars()
                .count()
        )
    }

    fn ray(grid: &mut Grid<Tile>, location: (isize, isize), direction: (isize, isize)) -> (isize, isize) {
        let mut current_location = location;
        let mut current_direction = direction;

        loop {
            current_location = (current_location.0 + current_direction.0, current_location.1 + current_direction.1);

            if current_location.1 >= grid.cols() as isize || current_location.0 >= grid.rows() as isize || current_location.0 < 0 || current_location.1 < 0 {
                break;
            }

            let tile = grid[(current_location.1 as usize, current_location.0 as usize)];

            grid[(current_location.1 as usize, current_location.0 as usize)].energized = true;

            match tile.tile_type {
                TileType::Empty => continue,
                TileType::UpReflector => {
                    match current_direction {
                        (1, 0) => current_direction = (0, -1),
                        (0, 1) => current_direction = (-1, 0),
                        (-1, 0) => current_direction = (0, 1),
                        (0, -1) => current_direction = (1, 0),
                        _ => panic!("Invalid direction"),
                    }
                },
                TileType::DownReflector => {
                    match current_direction {
                        (1, 0) => current_direction = (0, 1),
                        (0, 1) => current_direction = (1, 0),
                        (-1, 0) => current_direction = (0, -1),
                        (0, -1) => current_direction = (-1, 0),
                        _ => panic!("Invalid direction"),
                    }
                },
                TileType::VerticalSplitter => {
                    if grid[(current_location.1 as usize, current_location.0 as usize)].already_used {
                        break;
                    }
                    grid[(current_location.1 as usize, current_location.0 as usize)].already_used = true;
                    match current_direction {
                        (1, 0) => {
                            Day16::ray(grid, current_location, (0, 1));
                            Day16::ray(grid, current_location, (0, -1));
                        },
                        (-1, 0) => {
                            Day16::ray(grid, current_location, (0, 1));
                            Day16::ray(grid, current_location, (0, -1));
                        },
                        (0, 1) => continue,
                        (0, -1) => continue,
                        _ => panic!("Invalid direction"),
                    }
                    break;
                },
                TileType::HorizontalSplitter => {
                    if grid[(current_location.1 as usize, current_location.0 as usize)].already_used {
                        break;
                    }
                    grid[(current_location.1 as usize, current_location.0 as usize)].already_used = true;
                    match current_direction {
                        (1, 0) => continue,
                        (-1, 0) => continue,
                        (0, 1) => {
                            Day16::ray(grid, current_location, (1, 0));
                            Day16::ray(grid, current_location, (-1, 0));
                        },
                        (0, -1) => {
                            Day16::ray(grid, current_location, (1, 0));
                            Day16::ray(grid, current_location, (-1, 0));
                        },
                        _ => panic!("Invalid direction"),
                    }
                    break;
                },
            }
        }

        current_location
    }
}

impl Solver for Day16 {
    fn day(&self) -> usize {
        16
    }

    fn name(&self) -> String {
        String::from("The Floor Will Be Lava")
    }

    fn solve_first(&self) -> u128 {
        let mut grid = Day16::input_to_grid(self.input_first());

        let current_location = (-1, 0);
        let current_direction = (1, 0);

        grid[(0, 0)].energized = true;

        Day16::ray(&mut grid, current_location, current_direction);

        grid
            .iter()
            .filter(|tile| tile.energized)
            .count() as u128
    }

    fn solve_second(&self) -> u128 {
        let grid = Day16::input_to_grid(self.input_second());

        (0..grid.cols())
            .map(|x| {
                let mut local_grid = grid.clone();
                local_grid[(0, x)].energized = true;
                Day16::ray(&mut local_grid, (x as isize, -1), (0, 1));
                local_grid
                    .iter()
                    .filter(|tile| tile.energized)
                    .count() as u128
            })
            .chain(
                (0..grid.cols())
                    .map(|x| {
                        let mut local_grid = grid.clone();
                        local_grid[(grid.rows() - 1, x)].energized = true;
                        Day16::ray(&mut local_grid, (x as isize, grid.rows() as isize), (0, -1));
                        local_grid
                            .iter()
                            .filter(|tile| tile.energized)
                            .count() as u128
                    })
            )
            .chain(
                (0..grid.rows())
                    .map(|y| {
                        let mut local_grid = grid.clone();
                        local_grid[(y, 0)].energized = true;
                        Day16::ray(&mut local_grid, (-1, y as isize), (1, 0));
                        local_grid
                            .iter()
                            .filter(|tile| tile.energized)
                            .count() as u128
                    })
            )
            .chain(
                (0..grid.rows())
                    .map(|y| {
                        let mut local_grid = grid.clone();
                        local_grid[(y, grid.cols() - 1)].energized = true;
                        Day16::ray(&mut local_grid, (grid.cols() as isize, y as isize), (-1, 0));
                        local_grid
                            .iter()
                            .filter(|tile| tile.energized)
                            .count() as u128
                    })
            )
            .sorted()
            .last()
            .unwrap_or(0)
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day16.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day16.txt").to_string()
    }
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    tile_type: TileType,
    energized: bool,
    already_used: bool,
}

impl Tile {
    fn new(tile_type: TileType) -> Tile {
        Tile {
            tile_type,
            energized: false,
            already_used: false,
        }
    }

}

#[derive(Clone, Copy, Debug)]
enum TileType {
    Empty,
    UpReflector,
    DownReflector,
    VerticalSplitter,
    HorizontalSplitter,
}