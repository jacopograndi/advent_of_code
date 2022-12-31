use std::{fs, str::FromStr};

const SHORT_TIME_LIMIT: u32 = 24;
const LONG_TIME_LIMIT: u32 = 32;
const MAX_FRONTIER: usize = 100;
const CHUNK_SIZE: u32 = 8;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
struct State {
    time: u32,
    resources: [u32; 4],
    collectors: [u32; 4],
    path: Box<Vec<State>>,
}

impl State {
    fn new() -> Self {
        Self {
            time: 0,
            resources: [0; 4],
            collectors: [1, 0, 0, 0],
            path: Box::new(vec![]),
        }
    }

    fn tick(&self) -> Self {
        let mut res = self.resources.clone();
        for i in 0..4 {
            res[i] += self.collectors[i];
        }
        Self {
            time: self.time + 1,
            resources: res,
            collectors: self.collectors,
            path: self.path.clone(),
        }
    }

    fn score(&self) -> u32 {
        (0..4 as u32)
            .map(|i| {
                self.collectors[i as usize] * (i + 1) * (i + 5) * 200
                    + self.resources[1] * 1
                    + self.resources[2] * 1000
                    + self.resources[3] * 10000000
            })
            .sum()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    costs: [[u32; 4]; 4],
}

impl FromStr for Blueprint {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, body) = s.split_once(":").unwrap();
        let id = head.split_once(" ").unwrap().1.parse().unwrap();
        let costs = body
            .split(".")
            .filter(|l| l.len() > 2)
            .map(|res| {
                let (_, prices_str) = res.split_once("costs ").unwrap();
                let mut prices = [0, 0, 0, 0];
                for p in prices_str.split("and") {
                    let (quantity_str, name) = p.trim().split_once(" ").unwrap();
                    let quantity = quantity_str.parse().unwrap();
                    match name {
                        "ore" => prices[0] = quantity,
                        "clay" => prices[1] = quantity,
                        "obsidian" => prices[2] = quantity,
                        "geode" => prices[3] = quantity,
                        _ => unreachable!(),
                    }
                }
                prices
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok(Blueprint { id, costs })
    }
}

impl Blueprint {
    fn quality(&self) -> u32 {
        self.id
            * self
                .simulate(&State::new(), SHORT_TIME_LIMIT, MAX_FRONTIER, CHUNK_SIZE)
                .resources[3]
    }

    fn simulate(
        &self,
        state: &State,
        time_limit: u32,
        max_frontier: usize,
        chunk_size: u32,
    ) -> State {
        let mut starters = vec![state.clone()];
        let mut frontier = vec![];
        let mut time = time_limit;
        while time > 0 {
            if starters.len() > max_frontier {
                starters.sort_by(|a, b| b.cmp(a));
                starters.truncate(max_frontier);
            }
            let chunk = chunk_size.min(time);
            for _ in 0..chunk {
                loop {
                    let Some(first) = starters.pop() else { break; };
                    let star = self.star(&first);
                    frontier.extend(star);
                }
                starters.append(&mut frontier);
            }
            time -= chunk;
        }
        starters.sort_by(|a, b| b.cmp(a));
        starters[0].clone()
    }

    fn star(&self, state: &State) -> Vec<State> {
        let mut moves = vec![];
        for i in (0..4).rev() {
            if i == 3 || state.collectors[i] < self.costs.iter().map(|x| x[i]).max().unwrap() {
                if (0..4).all(|j| state.resources[j] >= self.costs[i][j]) {
                    let mut buy = state.tick();
                    for j in 0..4 {
                        buy.resources[j] -= self.costs[i][j];
                    }
                    buy.collectors[i] += 1;
                    moves.push(buy);
                }
            }
        }
        moves.push(state.tick());
        moves
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let blueprints: Vec<Blueprint> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Blueprint>().unwrap())
        .collect();
    let sum: u64 = blueprints.iter().map(|bp| bp.quality() as u64).sum();
    println!("Sum of qualities: {}", sum);
    let left: Vec<u64> = blueprints
        .iter()
        .take(3)
        .map(|bp| {
            bp.simulate(&State::new(), LONG_TIME_LIMIT, 100, 10)
                .resources[3] as u64
        })
        .collect();
    println!(
        "Sum of first 3 blueprints geodes after 32 minutes: {}",
        left.iter().product::<u64>()
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn quality_level_blueprint_1() {
        let blueprint = Blueprint {
            id: 1,
            costs: [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]],
        };
        assert_eq!(9, blueprint.quality());
    }

    #[test]
    fn quality_level_blueprint_2() {
        let blueprint = Blueprint {
            id: 2,
            costs: [[2, 0, 0, 0], [3, 0, 0, 0], [3, 8, 0, 0], [3, 0, 12, 0]],
        };
        assert_eq!(24, blueprint.quality());
    }

    #[test]
    fn quality_level_blueprint_1_long() {
        let blueprint = Blueprint {
            id: 1,
            costs: [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]],
        };
        assert_eq!(
            56,
            blueprint
                .simulate(&State::new(), LONG_TIME_LIMIT, 400, 3)
                .resources[3]
        );
    }

    #[test]
    fn quality_level_blueprint_2_long() {
        let blueprint = Blueprint {
            id: 2,
            costs: [[2, 0, 0, 0], [3, 0, 0, 0], [3, 8, 0, 0], [3, 0, 12, 0]],
        };
        assert_eq!(
            62,
            blueprint
                .simulate(&State::new(), LONG_TIME_LIMIT, 400, 3)
                .resources[3]
        );
    }
}
