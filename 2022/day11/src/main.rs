use std::{fs, str::FromStr};

#[derive(Debug, Clone)]
enum Operator {
    Sum,
    Product,
}

impl FromStr for Operator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Sum),
            "*" => Ok(Operator::Product),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Immediate(i64),
}

impl FromStr for Operand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            _ => Ok(Operand::Immediate(s.parse().unwrap())),
        }
    }
}

impl Operand {
    fn exec(&self, old: i64) -> i64 {
        match self {
            Operand::Old => old,
            Operand::Immediate(v) => *v,
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    lhs: Operand,
    operator: Operator,
    rhs: Operand,
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let expr = s.trim_start_matches("Operation:").trim();
        let (_, second) = expr.split_once("=").unwrap();
        let (lhs, rest) = second.trim().split_once(" ").unwrap();
        let (op, rhs) = rest.split_once(" ").unwrap();
        Ok(Operation {
            lhs: lhs.trim().parse().unwrap(),
            operator: op.trim().parse().unwrap(),
            rhs: rhs.trim().parse().unwrap(),
        })
    }
}

impl Operation {
    fn exec(&self, old: i64) -> i64 {
        match self.operator {
            Operator::Sum => self.lhs.exec(old) + self.rhs.exec(old),
            Operator::Product => self.lhs.exec(old) * self.rhs.exec(old),
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    modulo: u64,
    pass: usize,
    fail: usize,
}

impl FromStr for Test {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let body = s.trim().trim_start_matches("Test: ");
        let (cond, rest) = body.trim().split_once("\n").unwrap();
        let (pass, fail) = rest.trim().split_once("\n").unwrap();
        Ok(Test {
            modulo: cond
                .trim_start_matches("divisible by")
                .trim()
                .parse()
                .unwrap(),
            pass: pass
                .trim()
                .trim_start_matches("If true: throw to monkey")
                .trim()
                .parse()
                .unwrap(),
            fail: fail
                .trim()
                .trim_start_matches("If false: throw to monkey")
                .trim()
                .parse()
                .unwrap(),
        })
    }
}

impl Test {
    fn exec(&self, item: u64) -> usize {
        if item % self.modulo == 0 {
            self.pass
        } else {
            self.fail
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

impl FromStr for Monkey {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split_once("\n").unwrap();
        let (items, rest) = rest.split_once("\n").unwrap();
        let (op, test) = rest.split_once("\n").unwrap();
        Ok(Monkey {
            id: id
                .trim_start_matches("Monkey ")
                .trim_end_matches(":")
                .parse()
                .unwrap(),
            items: items
                .trim()
                .trim_start_matches("Starting items: ")
                .split(",")
                .map(|x| x.trim().parse::<u64>().unwrap())
                .collect(),
            operation: op
                .trim()
                .trim_start_matches("Operation: ")
                .parse::<Operation>()
                .unwrap(),
            test: test.parse::<Test>().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
struct Game {
    monkeys: Vec<Monkey>,
    inspection_events: Vec<(usize, u64)>,
    very_worried: bool,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Game {
            monkeys: s
                .split("\n\n")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect(),
            inspection_events: vec![],
            very_worried: false,
        })
    }
}

impl Game {
    fn round(&mut self) {
        let mut curr = self.clone();
        let range: u64 = self.monkeys.iter().map(|x| x.test.modulo).product();
        for i in 0..self.monkeys.len() {
            let mut next = curr.clone();
            let monkey = &mut curr.monkeys[i];
            for item in monkey.items.iter() {
                next.inspection_events.push((monkey.id, *item));
                let worry = monkey.operation.exec(item.clone() as i64);
                let bored = if self.very_worried { worry } else { worry / 3 } as u64;
                let mut calc = bored;
                while calc >= range {
                    calc -= range;
                }
                next.monkeys[monkey.test.exec(calc)].items.push(calc);
                next.monkeys[i].items.remove(0);
            }
            curr = next.clone();
        }
        *self = curr.clone();
    }

    fn run(&mut self, rounds: u64) {
        for _ in 0..rounds {
            self.round();
        }
    }

    fn monkey_business(&self) -> i64 {
        let mut results = vec![0; self.monkeys.len()];
        for (id, _) in self.inspection_events.iter() {
            results[*id] += 1;
        }
        results.sort();
        results.iter().rev().take(2).product()
    }

    fn view(&self) {
        for monkey in self.monkeys.iter() {
            print!("Monkey {}: ", monkey.id);
            for item in monkey.items.iter() {
                print!("{item}, ");
            }
            println!();
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut game: Game = input.parse().unwrap();
    game.run(20);
    let business = game.monkey_business();
    println!("monkey business: {business}");

    let mut game: Game = input.parse().unwrap();
    game.very_worried = true;
    game.run(10000);
    let business = game.monkey_business();
    println!("monkey business while very worried: {business}");
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn parse_monkeys() {
        let input = fs::read_to_string("test.txt").unwrap();
        let game: Result<Game, ()> = input.parse();
        dbg!(&game);
        assert!(game.is_ok());
    }

    #[test]
    fn business() {
        let input = fs::read_to_string("test.txt").unwrap();
        let mut game: Game = input.parse().unwrap();
        game.run(20);
        let business = game.monkey_business();
        assert_eq!(business, 10605);
    }

    #[test]
    fn very_worried() {
        let input = fs::read_to_string("test.txt").unwrap();
        let mut game: Game = input.parse().unwrap();
        game.very_worried = true;
        game.run(10000);
        let business = game.monkey_business();
        assert_eq!(business, 2713310158);
    }
}
