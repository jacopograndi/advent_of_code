use std::{collections::HashMap, fs, str::FromStr};

#[derive(Clone, Debug)]
struct Node {
    id: String,
    star: Vec<String>,
    flow: i32,
}

impl FromStr for Node {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(";").unwrap();
        if let &[_, id, _, _, rate] = l.split(" ").collect::<Vec<&str>>().as_slice() {
            let (_, num) = rate.split_once("=").unwrap();
            Ok(Node {
                id: id.to_string(),
                star: r
                    .trim()
                    .split(" ")
                    .skip(4)
                    .map(|x| x.trim().trim_end_matches(",").to_string())
                    .collect::<Vec<String>>(),
                flow: num.parse().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

struct Net {
    nodes: Vec<Node>,
    paths: HashMap<(String, String), Vec<String>>,
    nonzero_valves: Vec<String>,
}

impl FromStr for Net {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut net = Net {
            nodes: s
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect(),
            paths: HashMap::new(),
            nonzero_valves: vec![],
        };
        net.nonzero_valves = net
            .nodes
            .iter()
            .filter(|x| x.flow > 0)
            .map(|x| x.id.clone())
            .collect();
        net.precalc_paths();
        Ok(net)
    }
}

impl Net {
    fn precalc_paths(&mut self) {
        for from in &self.nodes {
            for to in &self.nodes {
                self.paths.insert(
                    (from.id.clone(), to.id.clone()),
                    self.shortest_path(&from.id, &to.id),
                );
            }
        }
    }

    fn shortest_path(&self, from: &String, to: &String) -> Vec<String> {
        let mut frontier = Vec::<String>::new();
        let mut visited = Vec::<String>::new();
        let mut previous = HashMap::<String, String>::new();
        frontier.push(from.to_string());
        loop {
            if frontier.len() == 0 {
                unreachable!() // do i keep it?
            }
            let first = frontier.remove(0);
            if &first == to {
                let mut trace = first;
                let mut path = vec![];
                while &trace != from {
                    path.push(trace.clone());
                    trace = previous.get(&trace).unwrap().clone();
                }
                path.reverse();
                return path;
            }
            let moves = self.nodes.iter().find(|&x| x.id == first).unwrap();
            let novel = moves
                .star
                .iter()
                .filter(|m| !visited.contains(m) && !frontier.contains(m))
                .cloned()
                .collect::<Vec<String>>();
            for n in novel.iter() {
                previous.insert(n.to_string(), first.clone());
            }
            frontier.extend(novel);
            visited.push(first);
        }
    }
}

#[derive(Clone, Debug)]
struct GameState {
    players: Vec<String>,
    opened: Vec<String>,
    pending: Vec<String>,
    plans: Vec<Vec<String>>,
    flow: i32,
    time_left: i32,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            players: vec!["AA".to_string()],
            opened: vec![],
            pending: vec![],
            plans: vec![vec![]],
            flow: 0,
            time_left: 30,
        }
    }

    fn new_with_elephant() -> GameState {
        GameState {
            players: vec!["AA".to_string(), "AA".to_string()],
            plans: vec![vec![], vec![]],
            opened: vec![],
            pending: vec![],
            flow: 0,
            time_left: 26,
        }
    }

    fn turn(&self, net: &Net) -> GameState {
        if self.is_over() {
            self.clone()
        } else if !net
            .nodes
            .iter()
            .any(|x| x.flow > 0 && !self.opened.contains(&x.id))
        {
            // run it to the end
            let mut next = self.clone();
            loop {
                next.econ(net);
                if next.is_over() {
                    break next.clone();
                }
            }
        } else {
            for i in 0..self.players.len() {
                if self.plans[i].is_empty() {
                    if !self
                        .valid_valves(net)
                        .iter()
                        .find(|&v| v == &&self.players[i])
                        .is_some()
                        && self.valid_valves(net).len() > 0
                    {
                        // plan
                        let valves = self.valid_valves(net);
                        let mut destination: Vec<(i32, String)> = valves
                            .iter()
                            .map(|&v| {
                                let node = net.nodes.iter().find(|x| x.id == **v).unwrap();
                                let score = -node.flow
                                    / net
                                        .paths
                                        .get(&(self.players[i].clone(), v.to_string()))
                                        .unwrap()
                                        .len() as i32;
                                (score, v.clone())
                            })
                            .collect();
                        destination.sort_by(|a, b| a.0.cmp(&b.0));
                        return destination
                            .iter()
                            .take(3) // HAAAXXXX!!!11!!1 ("euristic")
                            .map(|(_, valve)| {
                                let mut branch = self.clone();
                                branch.plans[i] = net
                                    .paths
                                    .get(&(branch.players[i].clone(), valve.to_string()))
                                    .cloned()
                                    .unwrap();
                                branch.turn(net)
                            })
                            .max_by(|a, b| a.flow.cmp(&b.flow))
                            .unwrap();
                    }
                }
            }
            let mut next = self.clone();
            for i in 0..self.players.len() {
                if next.plans[i].is_empty() {
                    if next
                        .valid_valves(net)
                        .iter()
                        .find(|&v| v == &&next.players[i])
                        .is_some()
                    {
                        // open
                        next.pending.push(next.players[i].clone());
                    }
                } else {
                    // move
                    let step = next.plans[i].remove(0);
                    next.players[i] = step;
                }
            }
            // flow, time
            next.econ(net);
            next.opened.append(&mut next.pending);
            //println!("{:?}", next);
            next.turn(net)
        }
    }

    fn valid_valves<'a>(&self, net: &'a Net) -> Vec<&'a String> {
        net.nonzero_valves
            .iter()
            .filter(|x| !self.opened.contains(&x))
            .collect()
    }

    fn is_over(&self) -> bool {
        self.time_left <= 0
    }

    fn econ(&mut self, net: &Net) {
        self.flow += self.get_flow(net);
        self.time_left -= 1;
    }

    fn get_flow(&self, net: &Net) -> i32 {
        net.nodes
            .iter()
            .filter(|x| self.opened.contains(&x.id))
            .map(|x| x.flow)
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let net: Net = input.parse().unwrap();
    let gst: GameState = GameState::new();
    println!("total flow after 30 turns: {}", gst.turn(&net).flow);
    let gst: GameState = GameState::new_with_elephant();
    println!(
        "total flow after 26 turns with the help of an elephant: {}",
        gst.turn(&net).flow
    );
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn shor_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        let net: Net = input.parse().unwrap();
        assert_eq!(
            net.shortest_path(&"AA".to_string(), &"HH".to_string())
                .len(),
            5
        );
        assert_eq!(
            net.shortest_path(&"CC".to_string(), &"HH".to_string())
                .len(),
            5
        );
        assert_eq!(
            net.shortest_path(&"AA".to_string(), &"BB".to_string())
                .len(),
            1
        );
        assert_eq!(
            net.shortest_path(&"AA".to_string(), &"BB".to_string()),
            vec!["BB".to_string()]
        );
        assert_eq!(
            net.shortest_path(&"CC".to_string(), &"FF".to_string())
                .len(),
            3
        );
    }

    #[test]
    fn max_outflow() {
        let input = fs::read_to_string("test.txt").unwrap();
        let net: Net = input.parse().unwrap();
        let gst: GameState = GameState::new();
        assert_eq!(gst.turn(&net).flow, 1651);
    }

    #[test]
    fn max_outflow_with_elephant() {
        let input = fs::read_to_string("test.txt").unwrap();
        let net: Net = input.parse().unwrap();
        let gst: GameState = GameState::new_with_elephant();
        assert_eq!(gst.turn(&net).flow, 1707);
    }
}
