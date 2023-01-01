use std::{collections::HashMap, fs, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operator {
    Sum,
    Product,
    Subtraction,
    Division,
    Equality,
}

impl Operator {
    fn exec(&self, l: &i64, r: &i64) -> i64 {
        match self {
            Self::Sum => l + r,
            Self::Subtraction => l - r,
            Self::Product => l * r,
            Self::Division => l / r,
            Self::Equality => *l,
        }
    }
}

impl FromStr for Operator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Sum),
            "-" => Ok(Self::Subtraction),
            "*" => Ok(Self::Product),
            "/" => Ok(Self::Division),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Operation {
    lhs: String,
    operator: Operator,
    rhs: String,
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [lhs, op, rhs]: [&str; 3] = s.splitn(3, " ").collect::<Vec<_>>().try_into().unwrap();
        Ok(Operation {
            lhs: lhs.parse().unwrap(),
            operator: op.parse().unwrap(),
            rhs: rhs.parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expression {
    Value(i64),
    Operation(Operation),
    Human,
}

impl FromStr for Expression {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.trim().parse() {
            Ok(Expression::Value(num))
        } else {
            Ok(Expression::Operation(s.trim().parse().unwrap()))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Riddle {
    monkeys: HashMap<String, Expression>,
}

impl FromStr for Riddle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Riddle {
            monkeys: s
                .split("\n")
                .filter(|l| !l.is_empty())
                .map(|l| {
                    let (name, exp) = l.split_once(":").unwrap();
                    (name.to_string(), exp.parse::<Expression>().unwrap())
                })
                .collect(),
        })
    }
}

impl Riddle {
    fn humanize(&mut self) {
        if let Some(Expression::Operation(op)) = self.monkeys.get_mut("root") {
            op.operator = Operator::Equality;
        }
        if let Some(expr) = self.monkeys.get_mut("humn") {
            *expr = Expression::Human;
        }
    }

    fn solve(&mut self) {
        loop {
            let mut next = self.clone();
            let mut dirty = false;
            for expr in self.monkeys.iter() {
                match expr.1 {
                    Expression::Value(_) => (),
                    Expression::Human => (),
                    Expression::Operation(op) => {
                        if next.exec(expr.0, op) {
                            dirty = true;
                        }
                    }
                }
            }
            *self = next;
            if !dirty {
                break;
            }
        }
    }

    fn exec(&mut self, name: &String, op: &Operation) -> bool {
        if let Some(result) = match &self.monkeys[&op.lhs] {
            Expression::Value(l) => match &self.monkeys[&op.rhs] {
                Expression::Value(r) => Some(op.operator.exec(l, r)),
                _ => None,
            },
            _ => None,
        } {
            self.monkeys
                .insert(name.to_string(), Expression::Value(result));
            true
        } else {
            false
        }
    }

    fn propagate(&mut self, name: &String, result: i64) -> i64 {
        let (lhs, operator, rhs) = match &self.monkeys[name] {
            Expression::Operation(Operation { lhs, operator, rhs }) => {
                (lhs.clone(), operator, rhs.clone())
            }
            Expression::Human => return result,
            _ => unreachable!(),
        };
        match operator {
            Operator::Equality => match (&self.monkeys[&lhs], &self.monkeys[&rhs]) {
                // x = v
                (_, Expression::Value(v)) => self.propagate(&lhs, *v),
                // v = x
                (Expression::Value(v), _) => self.propagate(&rhs, *v),
                _ => unreachable!(),
            },
            Operator::Sum => match (&self.monkeys[&lhs], &self.monkeys[&rhs]) {
                // x + v = result => x = result - v
                (_, Expression::Value(v)) => self.propagate(&lhs, result - *v),
                // v + x = result => x = result - v
                (Expression::Value(v), _) => self.propagate(&rhs, result - *v),
                _ => unreachable!(),
            },
            Operator::Subtraction => match (&self.monkeys[&lhs], &self.monkeys[&rhs]) {
                // x - v = result => x = result + v
                (_, Expression::Value(v)) => self.propagate(&lhs, result + *v),
                // v - x = result => x = v - result
                (Expression::Value(v), _) => self.propagate(&rhs, *v - result),
                _ => unreachable!(),
            },
            Operator::Product => match (&self.monkeys[&lhs], &self.monkeys[&rhs]) {
                // x * v = result => x = result / v
                (_, Expression::Value(v)) => self.propagate(&lhs, result / *v),
                // v * x = result => x = result / v
                (Expression::Value(v), _) => self.propagate(&rhs, result / *v),
                _ => unreachable!(),
            },
            Operator::Division => match (&self.monkeys[&lhs], &self.monkeys[&rhs]) {
                // x / v = result => x = result * v
                (_, Expression::Value(v)) => self.propagate(&lhs, result * *v),
                // v / x = result => x = v / result
                (Expression::Value(v), _) => self.propagate(&rhs, *v / result),
                _ => unreachable!(),
            },
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut riddle: Riddle = input.parse().unwrap();
    riddle.solve();
    let value = match riddle.monkeys["root"] {
        Expression::Value(v) => Some(v),
        _ => None,
    }
    .unwrap();
    println!("root: {}", value);

    let mut riddle: Riddle = input.parse().unwrap();
    riddle.humanize();
    riddle.solve();
    let human = riddle.propagate(&"root".to_string(), 0);
    println!("humn: {}", human);
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").unwrap();
        let mut riddle: Riddle = input.parse().unwrap();
        riddle.solve();
        let value = match riddle.monkeys["root"] {
            Expression::Value(v) => Some(v),
            _ => None,
        }
        .unwrap();
        assert_eq!(152, value);
    }

    #[test]
    fn example_as_human() {
        let input = fs::read_to_string("example.txt").unwrap();
        let mut riddle: Riddle = input.parse().unwrap();
        riddle.humanize();
        riddle.solve();
        let human = riddle.propagate(&"root".to_string(), 0);
        assert_eq!(301, human);
    }
}
