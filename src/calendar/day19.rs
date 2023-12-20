// Greatly inspired from https://gist.github.com/samueltardieu/63e7d1f8edbb520334c2dc10c0afb545 as I was stuck on part2
// I notably learned using arrays instead of vectors and the existence of tuple struct, which I don't really like
use hashbrown::HashMap;
use crate::traits::solver::Solver;

pub struct Day19;

impl Day19 {
    fn parse(input: String) -> (HashMap<String, Workflow>, Vec<Rating>) {
        let cleaned = input.replace("\r", "");
        let (workflows, ratings) = cleaned
            .split_once("\n\n")
            .expect("Failed to split input in workflows and ratings");
        let workflows = workflows
            .lines()
            .map(|l| {
                let (name, branches) = l.split_once('{').expect("Failed to split workflow in name and branches");
                let branches = branches[..branches.len() - 1]
                    .split(',')
                    .map(|b| b.to_string())
                    .map(|b| {
                        if let Some((cond, target)) = b.split_once(':') {
                            let property = cond.as_bytes()[0];
                            let value = b"xmas"
                                .iter()
                                .position(|b| *b == property)
                                .expect(&*format!("Failed to find property {}", property));

                            let operation = cond.as_bytes()[1];
                            let limit = cond[2..].parse().expect("Cannot parse branch limit");
                            Branch::new(
                                target.to_string(),
                                Some((value, operation, limit)),
                            )
                        } else {
                            Branch::new(b, None)
                        }
                    })
                    .collect::<Vec<Branch>>();
                (name.to_string(), branches)
            })
            .collect();
        let ratings = ratings
            .lines()
            .map(|l| {
                l[1..l.len() - 1]
                    .split(',')
                    .map(|s|
                        s[2..]
                            .parse()
                            .expect("Failed to parse rating")
                    )
                    .collect()
            })
            .collect();
        (workflows, ratings)
    }

    fn process_rule(
        workflows: &HashMap<String, Vec<Branch>>,
        rule: String,
        mut possible: [(i64, i64); 4],
    ) -> i64 {
        match rule.as_str() {
            "A" => {
                return possible
                    .into_iter()
                    .map(|(l, h)| i64::from(h - l))
                    .product::<i64>()
            }
            "R" => return 0,
            _ => (),
        }
        let mut t = 0;
        for r in &workflows[&rule] {
            let short = |target| t + Day19::process_rule(workflows, target, possible);
            let target = r.target.clone();
            if let Some((index, operation, limit)) = r.condition {
                match (possible[index], operation) {
                    ((_, u), b'<') if u <= limit => {
                        return short(target);
                    }
                    ((l, _), b'>') if l > limit => {
                        return short(target);
                    }
                    ((l, u), b'<') if l < limit => {
                        possible[index] = (l, limit);
                        t += Day19::process_rule(workflows, target, possible);
                        possible[index] = (limit, u);
                    }
                    ((l, u), b'>') if u >= limit => {
                        possible[index] = (limit + 1, u);
                        t += Day19::process_rule(workflows, target, possible);
                        possible[index] = (l, limit + 1);
                    }
                    _ => (),
                }
            } else {
                return short(target);
            }
        }
        t
    }

    fn run(workflows: &HashMap<String, Workflow>, r: &[i64]) -> bool {
        Day19::process_rule(
            workflows,
            String::from("in"),
            [
                (r[0], r[0] + 1),
                (r[1], r[1] + 1),
                (r[2], r[2] + 1),
                (r[3], r[3] + 1),
            ],
        ) == 1
    }
}

impl Solver for Day19 {
    fn day(&self) -> usize {
        19
    }

    fn name(&self) -> String {
        String::from("Aplenty")
    }

    fn solve_first(&self) -> u128 {
        let (workflows, ratings) = Day19::parse(self.input_first());
        ratings
            .into_iter()
            .filter_map(
                |rating|
                    Day19::run(&workflows, &rating)
                        .then(||
                            rating
                                .into_iter()
                                .map(i64::from)
                                .sum::<i64>()
                        )
            )
            .sum::<i64>() as u128
    }

    fn solve_second(&self) -> u128 {
        Day19::process_rule(
            &Day19::parse(
                self.input_second()).0,
                "in".to_string(),
            [(1, 4001); 4]
        ) as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day19.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day19.txt").to_string()
    }
}
type Rating = Vec<i64>;
#[derive(Debug, Clone)]
struct Branch {
    target: String,
    condition: Option<(usize, u8, i64)>,
}

impl Branch {
    fn new(target: String, condition: Option<(usize, u8, i64)>) -> Self {
        Self { target, condition }
    }
}

type Workflow = Vec<Branch>;