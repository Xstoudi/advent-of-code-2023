use std::fmt::Display;
use crate::traits::solver::Solver;

pub struct Day11;

impl Day11 {

}

impl Solver for Day11 {
    fn day(&self) -> usize {
        11
    }

    fn name(&self) -> String {
        String::from("Cosmic Expansion")
    }

    fn solve_first(&self) -> u128 {
        let objects = self.input_first()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line|
                line
                    .chars()
                    .map(|c| match c {
                        '.' => CosmosObjectType::Empty,
                        '#' => CosmosObjectType::Galaxy,
                        _ => panic!("Unknown cosmos object type: {}", c)
                    })
                    .collect::<Vec<CosmosObjectType>>()
            )
            .enumerate()
            .flat_map(|(y, row)|
                row
                    .iter()
                    .enumerate()
                    .map(|(x, object_type)|
                        CosmosObject::new(
                            x,
                            y,
                            object_type.clone()
                        )
                    )
                    .collect::<Vec<CosmosObject>>()
            )
            .collect::<Vec<CosmosObject>>();

        let mut cosmos = Cosmos::new(objects);
        cosmos.expand();
        cosmos
            .get_galaxies()
            .iter()
            .enumerate()
            .flat_map(|(index, galaxy)|
                cosmos
                    .get_galaxies()
                    .iter()
                    .skip(index + 1)
                    .map(|other_galaxy|
                        cosmos.distance_between(galaxy, other_galaxy, 1)
                    )
                    .collect::<Vec<usize>>()
            )
            .sum::<usize>() as u128
    }

    fn solve_second(&self) -> u128 {
        let objects = self.input_second()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line|
                line
                    .chars()
                    .map(|c| match c {
                        '.' => CosmosObjectType::Empty,
                        '#' => CosmosObjectType::Galaxy,
                        _ => panic!("Unknown cosmos object type: {}", c)
                    })
                    .collect::<Vec<CosmosObjectType>>()
            )
            .enumerate()
            .flat_map(|(y, row)|
                row
                    .iter()
                    .enumerate()
                    .map(|(x, object_type)|
                        CosmosObject::new(
                            x,
                            y,
                            object_type.clone()
                        )
                    )
                    .collect::<Vec<CosmosObject>>()
            )
            .collect::<Vec<CosmosObject>>();

        let mut cosmos = Cosmos::new(objects);
        cosmos.expand();

        cosmos
            .get_galaxies()
            .iter()
            .enumerate()
            .flat_map(|(index, galaxy)|
                cosmos
                    .get_galaxies()
                    .iter()
                    .skip(index + 1)
                    .map(|other_galaxy|
                        cosmos.distance_between(galaxy, other_galaxy, 999999)
                    )
                    .collect::<Vec<usize>>()
            )
            .sum::<usize>() as u128

    }

    fn input_first(&self) -> String {
        include_str!("../resource/day11.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day11.txt").to_string()
    }
}

#[derive(Clone, PartialEq, Debug)]
enum CosmosObjectType {
    Empty,
    Galaxy,
    Expansion
}

struct Cosmos {
    width: usize,
    height: usize,
    map: Vec<CosmosObject>,
}

impl Cosmos {
    fn new(map: Vec<CosmosObject>) -> Cosmos {
        Cosmos {
            width: map.iter().map(|object| object.x).max().unwrap() + 1,
            height: map.iter().map(|object| object.y).max().unwrap() + 1,
            map: map.clone()
        }
    }

    fn expand(&mut self) {
        let mut current_y = 0;
        while current_y < self.height {
            if !self.is_row_empty(current_y) {
                current_y += 1;
                continue;
            }
            (0..self.width)
                .for_each(|x| {
                    self.map.insert(self.get_index(x, current_y), CosmosObject::new(
                        x,
                        current_y,
                        CosmosObjectType::Expansion,
                    ))
                });
            self.height += 1;
            current_y += 2;
        }

        let mut current_x = 0;
        while current_x < self.width {
            if !self.is_column_empty(current_x) {
                current_x += 1;
                continue;
            }
            (0..self.height)
                .rev()
                .for_each(|y| {
                    self.map.insert(
                        self.get_index(current_x, y),
                        CosmosObject::new(
                            current_x,
                            y,
                            CosmosObjectType::Expansion,
                        )
                    )
                });
            self.width += 1;
            current_x += 2;
        }

        self.map = self.map
            .iter()
            .enumerate()
            .map(|(index, object)| {
                let (x, y) = self.get_position(index);
                CosmosObject::new(
                    x,
                    y,
                    object.object_type.clone(),
                )
            })
            .collect::<Vec<CosmosObject>>();
    }

    fn is_row_empty(&self, y: usize) -> bool {
        for x in 0..self.width {
            if self.get_object(x, y).object_type != CosmosObjectType::Empty
                && self.get_object(x, y).object_type != CosmosObjectType::Expansion {
                return false;
            }
        }
        true
    }

    fn is_column_empty(&self, x: usize) -> bool {
        for y in 0..self.height {
            if self.get_object(x, y).object_type != CosmosObjectType::Empty
                && self.get_object(x, y).object_type != CosmosObjectType::Expansion {
                return false;
            }
        }
        true
    }

    fn get_object(&self, x: usize, y: usize) -> CosmosObject {
        self.map[self.get_index(x, y)].clone()
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_position(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn get_galaxies(&self) -> Vec<CosmosObject> {
        self.map
            .iter()
            .filter(|object| object.object_type == CosmosObjectType::Galaxy)
            .map(|object| object.clone())
            .collect::<Vec<CosmosObject>>()
    }

    fn distance_between(&self, object1: &CosmosObject, object2: &CosmosObject, expansion_factor: usize) -> usize {
        let mut current_position: (isize, isize) = (object1.x as isize, object1.y as isize);

        let x_move: isize = if object1.x < object2.x {
            1
        } else if object1.x > object2.x {
            -1
        } else {
            0
        };

        let y_move: isize = if object1.y < object2.y {
            1
        } else if object1.y > object2.y {
            -1
        } else {
            0
        };

        let mut distance = 0;

        while current_position.0 != object2.x as isize {
            current_position.0 += x_move;
            distance += match self.get_object(current_position.0 as usize, current_position.1 as usize).object_type {
                CosmosObjectType::Empty => 1,
                CosmosObjectType::Galaxy => 1,
                CosmosObjectType::Expansion => expansion_factor
            };
            // println!("Current position : {:?}, distance traveled so far: {}", current_position, distance);

        }

        while current_position.1 != object2.y as isize {
            current_position.1 += y_move;
            distance += match self.get_object(current_position.0 as usize, current_position.1 as usize).object_type {
                CosmosObjectType::Empty => 1,
                CosmosObjectType::Galaxy => 1,
                CosmosObjectType::Expansion => expansion_factor
            };
            // println!("Current position : {:?}, distance traveled so far: {}", current_position, distance);

        }

        // println!("Found distance for {:?} and {:?} : {}", (object1.x, object1.y), (object2.x, object2.y), distance);

        distance
    }
}

impl Display for Cosmos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                result.push(match self.get_object(x, y).object_type {
                    CosmosObjectType::Empty => '.',
                    CosmosObjectType::Galaxy => '#',
                    CosmosObjectType::Expansion => 'E'
                });
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

#[derive(Clone, Debug)]
struct CosmosObject {
    x: usize,
    y: usize,
    object_type: CosmosObjectType
}

impl CosmosObject {
    fn new(x: usize, y: usize, object_type: CosmosObjectType) -> CosmosObject {
        CosmosObject {
            x,
            y,
            object_type
        }
    }
}
