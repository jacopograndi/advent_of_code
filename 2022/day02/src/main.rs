use std::{cmp::Ordering, fs};

trait Score {
    fn score(&self) -> i32;
}

#[derive(PartialEq, Eq, PartialOrd)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Score for Shape {
    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Rock => match other {
                Self::Paper => Ordering::Less,
                Self::Rock => Ordering::Equal,
                Self::Scissor => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Scissor => Ordering::Less,
                Self::Paper => Ordering::Equal,
                Self::Rock => Ordering::Greater,
            },
            Self::Scissor => match other {
                Self::Rock => Ordering::Less,
                Self::Scissor => Ordering::Equal,
                Self::Paper => Ordering::Greater,
            },
        }
    }
}

struct Round {
    op: Shape,
    me: Shape,
}

impl Round {
    fn parse(s: &str) -> Result<Self, String> {
        let (ops, mes) = s.split_once(" ").ok_or("no spearator")?;
        Ok(Round {
            op: match ops {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissor,
                _ => unreachable!(),
            },
            me: match mes {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissor,
                _ => unreachable!(),
            },
        })
    }

    fn parse_alt(s: &str) -> Result<Self, String> {
        let (ops, mes) = s.split_once(" ").ok_or("no spearator")?;
        match ops {
            "A" => Ok(Round {
                op: Shape::Rock,
                me: match mes {
                    "X" => Shape::Scissor,
                    "Y" => Shape::Rock,
                    "Z" => Shape::Paper,
                    _ => unreachable!(),
                },
            }),
            "B" => Ok(Round {
                op: Shape::Paper,
                me: match mes {
                    "X" => Shape::Rock,
                    "Y" => Shape::Paper,
                    "Z" => Shape::Scissor,
                    _ => unreachable!(),
                },
            }),
            "C" => Ok(Round {
                op: Shape::Scissor,
                me: match mes {
                    "X" => Shape::Paper,
                    "Y" => Shape::Scissor,
                    "Z" => Shape::Rock,
                    _ => unreachable!(),
                },
            }),
            _ => unreachable!(),
        }
    }
}

impl Score for Round {
    fn score(&self) -> i32 {
        self.me.score()
            + match self.me.cmp(&self.op) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let parsesum = |f: fn(&str) -> Result<Round, String>| {
        input
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| f(l).unwrap())
            .map(|r| r.score())
            .sum::<i32>()
    };
    println!("Sum of all scores is: {}", parsesum(Round::parse));
    println!("Sum of all alt scores is: {}", parsesum(Round::parse_alt));
}

// first try both time, good compiler
