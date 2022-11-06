use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Bag {
    name: String,
}

impl FromStr for Bag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let col = s
            .trim()
            .strip_suffix("bags")
            .or_else(|| s.trim().strip_suffix("bag"))
            .unwrap();
        Ok(Bag {
            name: col.trim().to_string(),
        })
    }
}

#[derive(Debug, PartialEq)]
struct Rule {
    bag: Bag,
    contents: HashMap<Bag, i32>,
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rhs, lhs) = s.split_once("contain").unwrap();

        let contents = if lhs.contains("no other bags") {
            HashMap::new()
        } else {
            let mut map = HashMap::new();
            for pair in lhs.split(",") {
                let (num, bag) = pair.trim().split_once(" ").unwrap();
                map.insert(Bag::from_str(bag).unwrap(), num.parse().unwrap());
            }
            map
        };

        Ok(Rule {
            bag: Bag::from_str(rhs).unwrap(),
            contents,
        })
    }
}

fn solve(raw: String) -> (i32, u64) {
    let mut rules: Vec<Rule> = vec![];
    for line in raw.split("\n") {
        if line.len() > 0 {
            let rule = Rule::from_str(line.trim_end_matches('.')).unwrap();
            rules.push(rule);
        }
    }

    let shinygolds: Vec<&Rule> = rules
        .iter()
        .filter(|r| {
            r.contents.contains_key(&Bag {
                name: "shiny gold".to_string(),
            })
        })
        .collect();

    let mut frontier = HashMap::<Bag, i32>::new();
    for shinygold in shinygolds.clone() {
        frontier.insert(shinygold.bag.clone(), 1);
    }

    loop {
        let mut explored = Vec::<&Rule>::new();
        for (bag, _) in frontier.iter() {
            for rule in rules.iter().filter(|r| r.contents.contains_key(bag)) {
                if !explored.contains(&&rule) && !frontier.contains_key(&rule.bag) {
                    explored.push(&rule);
                }
            }
        }

        if explored.is_empty() {
            break;
        }

        for rule in explored {
            frontier.insert(rule.bag.clone(), 1);
        }
    }

    let mut leaves = 0u64;
    let mut expand = Vec::<(Bag, i32)>::new();
    expand.push((
        Bag {
            name: "shiny gold".to_string(),
        },
        1,
    ));

    loop {
        let mut added = Vec::<(Bag, i32)>::new();
        if let Some((bag, num)) = expand.pop() {
            if !(bag.name == "shiny gold") {
                leaves += num as u64;
            }
            if let Some(rule) = rules.iter().find(|r| r.bag == bag) {
                let mut contents = rule.contents.iter().collect::<Vec<(&Bag, &i32)>>();
                contents.sort_by(|a, b| a.0.name.cmp(&b.0.name));

                for (inbag, innum) in contents {
                    added.push((inbag.clone().clone(), *innum * num));
                }
            }
        } else {
            break;
        }
        for (bag, num) in added.iter() {
            expand.push((bag.clone(), *num));
        }
    }

    (frontier.len() as i32, leaves)
}

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let (holders, leaves) = solve(raw);

    println!(
        "Bags that can hold at least one shiny gold bag: {}",
        holders
    );

    println!("Bags that needs to be held in a shiny gold bag: {}", leaves);
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn example_first() {
        let raw = fs::read_to_string("example.txt").unwrap();
        let (holders, leaves) = solve(raw);
        assert_eq!(holders, 4);
        assert_eq!(leaves, 32);
    }

    #[test]
    fn example_second() {
        let raw = fs::read_to_string("example2.txt").unwrap();
        let (holders, leaves) = solve(raw);
        assert_eq!(holders, 0);
        assert_eq!(leaves, 126);
    }
}
