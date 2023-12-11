use itertools::Itertools;
use crate::traits::solver::Solver;

pub struct Day10;

impl Day10 {
    fn char_to_pipe(c: char) -> PipeType {
        match c {
            '|' => PipeType::NorthSouth,
            '-' => PipeType::EastWest,
            'L' => PipeType::NorthEast,
            'J' => PipeType::NorthWest,
            '7' => PipeType::SouthWest,
            'F' => PipeType::SouthEast,
            '.' => PipeType::Ground,
            'S' => PipeType::Start,
            _ => {
                panic!("Unknown pipe type: {}", c);
            }
        }
    }

    fn pipe_to_deltas(pipe_type: PipeType) -> Vec<(isize, isize)> {
        match pipe_type {
            PipeType::NorthSouth => vec![(0, 1), (0, -1)],
            PipeType::EastWest => vec![(1, 0), (-1, 0)],
            PipeType::NorthEast => vec![(0, -1), (1, 0)],
            PipeType::NorthWest => vec![(0, -1), (-1, 0)],
            PipeType::SouthEast => vec![(0, 1), (1, 0)],
            PipeType::SouthWest => vec![(0, 1), (-1, 0)],
            PipeType::Ground => vec![],
            PipeType::Start => vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
        }
    }
}

impl Solver for Day10 {
    fn day(&self) -> usize {
        10
    }

    fn name(&self) -> String {
        String::from("Pipe Maze")
    }

    fn solve_first(&self) -> u128 {
        let pipes =  self.input_first()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line|
                line
                    .chars()
                    .map(|c| Day10::char_to_pipe(c))
                    .collect::<Vec<PipeType>>()
            )
            .enumerate()
            .flat_map(|(y, row)|
                row
                    .iter()
                    .enumerate()
                    .map(|(x, pipe_type)|
                        Pipe::new(
                            pipe_type.clone(),
                            x,
                            y,
                        )
                    )
                    .collect::<Vec<Pipe>>()
            )
            .collect::<Vec<Pipe>>();

        let mut maze = Maze::new(pipes);
        maze.compute_distances();
        let distance = maze.get_max_distance();

        distance as u128
    }

    fn solve_second(&self) -> u128 {
        let pipes =  self.input_second()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line|
                line
                    .chars()
                    .map(|c| Day10::char_to_pipe(c))
                    .collect::<Vec<PipeType>>()
            )
            .enumerate()
            .flat_map(|(y, row)|
                row
                    .iter()
                    .enumerate()
                    .map(|(x, pipe_type)|
                        Pipe::new(
                            pipe_type.clone(),
                            x,
                            y,
                        )
                    )
                    .collect::<Vec<Pipe>>()
            )
            .collect::<Vec<Pipe>>();

        let mut maze = Maze::new(pipes);
        maze.compute_distances();
        maze.replace_start();

        maze.count_inner() as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day10a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day10b.txt").to_string()
    }
}

#[derive(Clone)]
struct Maze {
    pipes: Vec<Pipe>,
    width: usize,
    height: usize
}

impl Maze {
    fn new(pipes: Vec<Pipe>) -> Maze {
        Maze {
            pipes: pipes.clone(),
            width: pipes.iter().map(|pipe| pipe.x).max().unwrap() + 1,
            height: pipes.iter().map(|pipe| pipe.y).max().unwrap() + 1
        }
    }

    fn get_start_index(&self) -> usize {
        self.pipes
            .iter()
            .find_position(|pipe| pipe.pipe_type == PipeType::Start)
            .expect("No start pipe found")
            .0
    }

    fn get_index(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    fn get_neighbors_indices(&self, pipe: &Pipe) -> Vec<usize> {
        Day10::pipe_to_deltas(pipe.pipe_type)
            .iter()
            .filter_map(|(dx, dy)| {
                let x = pipe.x as isize + dx;
                let y = pipe.y as isize + dy;
                if x < 0 || y < 0 {
                    return None;
                }
                if x >= self.width as isize || y >= self.height as isize {
                    return None;
                }
                Some((x as usize, y as usize))
            })
            .map(|(x, y)| self.get_index((x, y)))
            .collect::<Vec<usize>>()
    }

    fn compute_distances(&mut self) {
        let start_index = self.get_start_index();
        self.pipes[start_index].distance = 0;

        let mut next_pipes = Vec::new();
        next_pipes.push(start_index);

        while let Some(pipe_index) = next_pipes.pop() {
            let pipe = &self.pipes[pipe_index].clone();

            let neighbor_indices = self.get_neighbors_indices(pipe);
            for neighbor_index in neighbor_indices {
                let neighbor = &self.pipes[neighbor_index];

                if pipe.pipe_type == PipeType::Start {
                    let is_valid_neighbor = self.get_neighbors_indices(neighbor)
                        .iter()
                        .any(|neighbor_neighbor_index| self.pipes[*neighbor_neighbor_index].pipe_type == PipeType::Start);
                    if !is_valid_neighbor {
                        continue;
                    }
                }

                if neighbor.pipe_type == PipeType::Ground {
                    continue;
                }
                if neighbor.distance == -1 || neighbor.distance > pipe.distance + 1 {
                    self.pipes[neighbor_index].distance = pipe.distance + 1;
                    next_pipes.push(neighbor_index);
                }
            }
        }
    }

    fn get_max_distance(&self) -> isize {
        self.pipes
            .iter()
            .filter(|pipe| pipe.pipe_type != PipeType::Ground)
            .map(|pipe| pipe.distance)
            .max()
            .unwrap()
    }

    fn replace_start(&mut self) {
        let start_index = self.get_start_index();
        // ugly hardcoded replacement
        self.pipes[start_index].pipe_type = PipeType::SouthEast;
    }

    fn is_inside(&self, (x, y): (usize, usize)) -> bool {
        let pipe_index = self.get_index((x, y));
        let pipe = self.pipes[pipe_index];
        if pipe.distance != -1 {
            return false;
        }

        (0 .. x)
            .filter(|x| {
                let current_pipe = self.pipes[self.get_index((*x, pipe.y))];
                if (current_pipe.pipe_type == PipeType::NorthSouth ||
                    current_pipe.pipe_type == PipeType::NorthEast ||
                    current_pipe.pipe_type == PipeType::NorthWest ||
                    current_pipe.pipe_type == PipeType::Start)
                    && current_pipe.distance != -1 {
                    return true;
                }

                return false;
                })
            .count() % 2 == 1
    }

    fn count_inner(&self) -> usize {
        self.pipes
            .iter()
            .filter(|pipe|
                self.is_inside((pipe.x, pipe.y))
            )
            .count()

    }
}

#[derive(Clone, Copy)]
struct Pipe {
    pipe_type: PipeType,
    x: usize,
    y: usize,
    distance: isize
}

impl Pipe {
    fn new(pipe_type: PipeType, x: usize, y: usize) -> Pipe {
        Pipe {
            pipe_type,
            x,
            y,
            distance: -1
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PipeType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start
}