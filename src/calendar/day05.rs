use regex::Regex;
use lazy_static::lazy_static;
use crate::traits::solver::Solver;

pub struct Day05;

impl Day05 {

}

impl Solver for Day05 {
    fn day(&self) -> usize {
        5
    }

    fn name(&self) -> String {
        String::from("If You Give A Seed A Fertilizer")
    }

    fn solve_first(&self) -> u128 {
        lazy_static! {
            static ref MAPPING_REGEX: Regex = Regex::new(r"(?P<source>\w*)-to-(?P<destination>\w*) map:\n(?P<values>((\d+) (\d+) (\d+)\n?)+)").unwrap();
            static ref VALUES_REGEX: Regex = Regex::new(r"(?P<destination_range_start>\d+) (?P<source_range_start>\d+) (?P<range_size>\d+)").unwrap();
        }

        let groups = self.input_first()
            .replace("\r\n", "\n")
            .split("\n\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(String::from)
            .collect::<Vec<String>>();

        let mut seeds = Vec::new();
        let mut pipeline = Pipeline::default();

        for group in groups {
            if group.contains("seeds:") {
                group
                    .split(" ")
                    .skip(1)
                    .map(|seed_id| seed_id.parse::<u64>().unwrap())
                    .for_each(|seed_id| seeds.push(seed_id));
            }
            if group.contains("map:") {
                let captures = MAPPING_REGEX.captures(&group).expect("No captures");

                let mut transformation = Transformation::default();

                captures
                    .name("values")
                    .expect("No values")
                    .as_str()
                    .trim()
                    .split("\n")
                    .map(|line| VALUES_REGEX.captures(line).expect("No captures"))
                    .map(|x| RangeMapper {
                        source_range_start: x.name("source_range_start").expect("No source_range_start").as_str().parse::<u64>().unwrap(),
                        destination_range_start: x.name("destination_range_start").expect("No destination_range_start").as_str().parse::<u64>().unwrap(),
                        range_size: x.name("range_size").expect("No range_size").as_str().parse::<u64>().unwrap(),
                    })
                    .for_each(|x| {
                        transformation.transforms.push(x);
                    });

                pipeline.transformations.push(transformation);
            }
        }

        seeds
            .iter()
            .map(|seed|
                pipeline.exec(*seed)
            )
            .min()
            .expect("No min") as u128
    }

    fn solve_second(&self) -> u128 {
        lazy_static! {
            static ref MAPPING_REGEX: Regex = Regex::new(r"(?P<source>\w*)-to-(?P<destination>\w*) map:\n(?P<values>((\d+) (\d+) (\d+)\n?)+)").unwrap();
            static ref VALUES_REGEX: Regex = Regex::new(r"(?P<destination_range_start>\d+) (?P<source_range_start>\d+) (?P<range_size>\d+)").unwrap();
        }

        let groups = self.input_second()
            .replace("\r\n", "\n")
            .split("\n\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(String::from)
            .collect::<Vec<String>>();

        let mut seed_ranges = Vec::new();
        let mut pipeline = Pipeline::default();

        for group in groups {
            if group.contains("seeds:") {
                let mut seed_intermediate = group
                    .split(" ")
                    .skip(1)
                    .map(|seed_id| seed_id.parse::<u64>().unwrap());

                loop {
                    let seed_start = seed_intermediate.next();
                    match seed_start {
                        None => break,
                        Some(seed_start) => {
                            seed_ranges.push((seed_start, seed_intermediate.next().expect("No seed_size")));
                        }
                    }
                }
            }
            if group.contains("map:") {
                let captures = MAPPING_REGEX.captures(&group).expect("No captures");

                let mut transformation = Transformation::default();

                captures
                    .name("values")
                    .expect("No values")
                    .as_str()
                    .trim()
                    .split("\n")
                    .map(|line| VALUES_REGEX.captures(line).expect("No captures"))
                    .map(|x| RangeMapper {
                        source_range_start: x.name("source_range_start").expect("No source_range_start").as_str().parse::<u64>().unwrap(),
                        destination_range_start: x.name("destination_range_start").expect("No destination_range_start").as_str().parse::<u64>().unwrap(),
                        range_size: x.name("range_size").expect("No range_size").as_str().parse::<u64>().unwrap(),
                    })
                    .for_each(|x| {
                        transformation.transforms.push(x);
                    });

                pipeline.transformations.push(transformation);
            }
        }

        ((0..)
            .map(|i| (i, pipeline.exec_reverse(i)))
            .find(|(_, seed)| {
                seed_ranges
                    .iter()
                    .filter(|(start, size)| start <= seed && seed <= &(start + size))
                    .count() != 0
            })
            .expect("No last")
            .0 - 1) as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day05a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day05b.txt").to_string()
    }
}

struct Pipeline {
    transformations: Vec<Transformation>
}

impl Default for Pipeline {
    fn default() -> Self {
        Pipeline {
            transformations: Vec::new()
        }
    }
}

impl Pipeline {
    fn exec(&self, seed: u64) -> u64 {
        let mut result = seed;
        for transformation in &self.transformations {
            result = transformation.transform(result);
        }
        result
    }
    fn exec_reverse(&self, seed: u64) -> u64 {
        let mut result = seed;
        for transformation in self.transformations.iter().rev() {
            result = transformation.transform_reverse(result);
        }
        result
    }
}

struct Transformation {
    transforms: Vec<RangeMapper>
}

impl Default for Transformation {
    fn default() -> Self {
        Transformation {
            transforms: Vec::new()
        }
    }
}

impl Transformation {
    fn transform(&self, seed: u64) -> u64 {
        match self.transforms
            .iter()
            .filter(|range| range.source_range_start < seed && seed < range.source_range_start + range.range_size)
            .last() {

            None => seed,
            Some(concerned_range) => {
                let delta = seed - concerned_range.source_range_start;
                concerned_range.destination_range_start + delta
            }
        }
    }

    fn transform_reverse(&self, seed: u64) -> u64 {
        match self.transforms
            .iter()
            .filter(|range| range.destination_range_start < seed && seed < range.destination_range_start + range.range_size)
            .last() {

            None => seed,
            Some(concerned_range) => {
                let delta = seed - concerned_range.destination_range_start;
                concerned_range.source_range_start + delta
            }
        }
    }
}

struct RangeMapper {
    source_range_start: u64,
    destination_range_start: u64,
    range_size: u64,
}

impl Clone for RangeMapper {
    fn clone(&self) -> Self {
        RangeMapper {
            source_range_start: self.source_range_start,
            destination_range_start: self.destination_range_start,
            range_size: self.range_size,
        }
    }
}

impl Copy for RangeMapper {

}
