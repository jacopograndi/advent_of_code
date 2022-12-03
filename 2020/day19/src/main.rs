use std::{collections::HashMap, fs, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule {
    Seq(Vec<usize>),
    Many(Vec<Rule>),
    Term(String),
}

impl FromStr for Rule {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next() == Some('"') {
            Ok(Rule::Term(
                s.chars().skip(1).next().ok_or("no char")?.to_string(),
            ))
        } else {
            let rules: Vec<&str> = s.split("|").collect();
            if rules.len() > 1 {
                let mut many = vec![];
                for rule in rules {
                    many.push(Rule::Seq(
                        rule.trim()
                            .split(" ")
                            .map(|id| id.trim().parse().unwrap())
                            .collect(),
                    ));
                }
                Ok(Rule::Many(many))
            } else {
                Ok(Rule::Seq(
                    s.trim()
                        .split(" ")
                        .map(|id| id.trim().parse().unwrap())
                        .collect(),
                ))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grammar {
    rules: HashMap<usize, Rule>,
}

impl FromStr for Grammar {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grammar = Grammar {
            rules: HashMap::new(),
        };
        for l in s.split("\n") {
            let (id, v) = l.split_once(":").unwrap();
            grammar.rules.insert(
                id.trim().parse::<usize>().unwrap(),
                v.trim().parse::<Rule>().unwrap(),
            );
        }
        Ok(grammar)
    }
}

impl Grammar {
    fn validate(&self, s: &str) -> bool {
        let sol = self.valid_rec(&self.rules[&0], s, 0, 0);
        if sol.len() > 0 {
            let len = sol[0].clone();
            len == s.len()
        } else {
            false
        }
    }

    fn valid_rec(&self, rule: &Rule, s: &str, j: usize, lv: usize) -> Vec<usize> {
        match rule {
            Rule::Many(rules) => {
                let mut conts = vec![];
                for rule in rules {
                    let sol = self.valid_rec(rule, s, j, lv + 1);
                    conts.append(&mut sol.clone());
                }
                conts
            }
            Rule::Seq(seq) => {
                let mut conts = vec![j];
                let mut nextconts = vec![];
                for step in seq {
                    while conts.len() > 0 {
                        let firstcont = conts.remove(0);
                        let sol = self.valid_rec(&self.rules[step], s, firstcont, lv);
                        nextconts.append(&mut sol.clone());
                    }
                    conts = nextconts.clone();
                    nextconts.clear();
                }
                conts
            }
            Rule::Term(c) => {
                if s.chars().nth(j) == c.chars().next() {
                    vec![j + 1]
                } else {
                    vec![]
                }
            }
        }
    }
}

#[derive(Debug)]
struct Exercise {
    grammar: Grammar,
    words: Vec<String>,
}

impl FromStr for Exercise {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, msgs) = s.split_once("\n\n").ok_or("no empty line")?;
        Ok(Exercise {
            grammar: rules.parse()?,
            words: msgs
                .split("\n")
                .filter(|x| x.len() > 0)
                .map(|l| l.trim().to_string())
                .collect(),
        })
    }
}

fn main() {
    let exercise: Exercise = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    let val = exercise
        .words
        .iter()
        .filter(|w| exercise.grammar.validate(w))
        .count();
    println!("Valid words: {val}");
    let ex_loop: Exercise = fs::read_to_string("input_2.txt").unwrap().parse().unwrap();
    let val = ex_loop
        .words
        .iter()
        .filter(|w| ex_loop.grammar.validate(w))
        .count();
    println!("Valid words: {val}");
}

#[cfg(test)]
mod test {
    use crate::*;

    const EX_TREE: &str = r#"0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb"#;

    #[test]
    fn validate_tree() {
        let answers: Vec<bool> = vec![true, false, true, false, false];
        let exercise: Exercise = EX_TREE.parse().unwrap();
        for (word, ans) in exercise.words.iter().zip(answers.iter()) {
            dbg!(word, ans);
            assert_eq!(exercise.grammar.validate(word), *ans);
        }
    }

    const EX_CYCLE: &str = r#"0: 4 1 5
        0: 8 11
        1: "a"
        2: 1 24 | 14 4
        3: 5 14 | 16 1
        4: 1 1
        5: 1 14 | 15 1
        6: 14 14 | 1 14
        7: 14 5 | 1 21
        8: 42 | 42 8
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        11: 42 31 | 42 11 31
        12: 24 14 | 19 1
        13: 14 3 | 1 12
        14: "b"
        15: 1 | 14
        16: 15 1 | 14 14
        17: 14 2 | 1 7
        18: 15 15
        19: 14 1 | 14 14
        20: 14 14 | 1 15
        21: 14 1 | 1 14
        22: 14 14
        23: 25 1 | 22 14
        24: 14 1
        25: 1 1 | 1 14
        26: 14 22 | 1 20
        27: 1 6 | 14 18
        28: 16 1
        31: 14 17 | 1 13
        42: 9 14 | 10 1

        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        bbabbbbaabaabba
        babbbbaabbbbbabbbbbbaabaaabaaa
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        bbbbbbbaaaabbbbaaabbabaaa
        bbbababbbbaaaaaaaabbababaaababaabab
        ababaaaaaabaaab
        ababaaaaabbbaba
        baabbaaaabbaaaababbaababb
        abbbbabbbbaaaababbbbbbaaaababb
        aaaaabbaabaaaaababaa
        aaaabbaaaabbaaa
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        babaaabbbaaabaababbaabababaaab
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    #[test]
    fn validate_graph() {
        let answers: Vec<bool> = vec![
            false, true, true, true, true, true, true, true, true, true, true, false, true, false,
            true,
        ];

        let exercise: Exercise = EX_CYCLE.parse().unwrap();
        for (word, ans) in exercise.words.iter().zip(answers.iter()) {
            dbg!(word, ans);
            assert_eq!(exercise.grammar.validate(word), *ans);
        }
    }
}
