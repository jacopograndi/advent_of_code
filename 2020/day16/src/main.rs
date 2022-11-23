use std::{collections::HashMap, fmt::Debug, fs, ops::Range, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Bound {
    name: String,
    valid: Vec<Range<i64>>,
}

impl FromStr for Bound {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, ranges) = s.split_once(":").unwrap();
        Ok(Bound {
            name: name.trim().to_string(),
            valid: ranges
                .trim()
                .split("or")
                .map(|x| {
                    let (lo, hi) = x.trim().split_once("-").unwrap();
                    Range {
                        start: lo.parse().unwrap(),
                        end: hi.parse::<i64>().unwrap() + 1,
                    }
                })
                .collect(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Ticket {
    values: Vec<i64>,
}

impl FromStr for Ticket {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            values: s
                .trim()
                .split(",")
                .map(|x| x.trim().parse().unwrap())
                .collect(),
        })
    }
}

impl Ticket {
    fn get_invalid_value(&self, bounds: &Vec<Bound>) -> Option<i64> {
        for v in self.values.iter() {
            if !bounds.iter().any(|b| b.valid.iter().any(|r| r.contains(v))) {
                return Some(*v);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Reference {
    bounds: Vec<Bound>,
    ticket: Ticket,
    nears: Vec<Ticket>,
}

impl FromStr for Reference {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split("\n\n").map(|x| x.trim().to_string()).collect();
        let (_, ticket) = parts[1].split_once("\n").unwrap();
        let (_, ranges) = parts[2].split_once("\n").unwrap();
        Ok(Reference {
            bounds: parts[0].split("\n").map(|x| x.parse().unwrap()).collect(),
            ticket: ticket.parse()?,
            nears: ranges.split("\n").map(|x| x.parse().unwrap()).collect(),
        })
    }
}

impl Reference {
    fn scanning_error(&self) -> i64 {
        let mut error = 0;
        for ticket in self.nears.iter() {
            if let Some(v) = ticket.get_invalid_value(&self.bounds) {
                error += v;
            }
        }
        error
    }

    fn remove_invalids(&self) -> Reference {
        Reference {
            bounds: self.bounds.clone(),
            ticket: self.ticket.clone(),
            nears: self
                .nears
                .iter()
                .filter(|t| t.get_invalid_value(&self.bounds).is_none())
                .cloned()
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Field {
    name: Option<String>,
    values: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq)]
struct FieldReference {
    fields: Vec<Field>,
}

impl From<&Reference> for FieldReference {
    fn from(r: &Reference) -> Self {
        let mut asm = FieldReference { fields: vec![] };
        for i in 0..r.ticket.values.len() {
            let mut field = Field {
                name: None,
                values: vec![r.ticket.values[i]],
            };
            for v in r.nears.iter() {
                field.values.push(v.values[i]);
            }
            asm.fields.push(field);
        }
        asm
    }
}

impl FieldReference {
    fn assign(&mut self, r: Reference) {
        let mut map = HashMap::<u32, Vec<u32>>::new();
        for i in 0..self.fields.len() {
            for j in 0..r.bounds.len() {
                if self.fields[i]
                    .values
                    .iter()
                    .all(|x| r.bounds[j].valid.iter().any(|r| r.contains(x)))
                {
                    if let Some(ovf) = map.get_mut(&(j as u32)) {
                        ovf.push(i as u32);
                    } else {
                        map.insert(j as u32, vec![i as u32]);
                    }
                }
            }
        }

        loop {
            let most_constrained = map.iter().min_by(|(_, v), (_, w)| v.len().cmp(&w.len()));
            if let Some((key, v)) = most_constrained {
                assert_eq!(1, v.len());
                self.fields[v[0] as usize].name = Some(r.bounds[*key as usize].name.clone());
                map = map
                    .iter()
                    .filter(|(k, _)| *k != key)
                    .map(|(k, w)| {
                        (
                            k.clone(),
                            w.iter()
                                .filter(|x| **x != v[0])
                                .cloned()
                                .collect::<Vec<u32>>(),
                        )
                    })
                    .collect();
            } else {
                break;
            }
        }
    }

    fn score(&self) -> i64 {
        self.fields
            .iter()
            .filter_map(|f| {
                if let Some(name) = &f.name {
                    if name.starts_with("departure") {
                        return Some(f.values[0]);
                    }
                }
                None
            })
            .product()
    }
}

fn main() {
    let reference = fs::read_to_string("input.txt")
        .unwrap()
        .parse::<Reference>()
        .unwrap();
    println!("The scanning error is {}", reference.scanning_error());
    let pruned_reference = reference.remove_invalids();
    let mut fieldref = FieldReference::from(&pruned_reference);
    fieldref.assign(pruned_reference);
    println!("The score is {}", fieldref.score());
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT_0: &str = "
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";

    const INPUT_1: &str = "
        class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19

        your ticket:
        11,12,13

        nearby tickets:
        3,9,18
        15,1,5
        5,14,9";

    #[test]
    fn parse_reference() {
        assert_eq!(
            INPUT_0.parse(),
            Ok(Reference {
                bounds: vec![
                    Bound {
                        name: "class".to_string(),
                        valid: vec![1..4, 5..8],
                    },
                    Bound {
                        name: "row".to_string(),
                        valid: vec![6..12, 33..45],
                    },
                    Bound {
                        name: "seat".to_string(),
                        valid: vec![13..41, 45..51],
                    },
                ],
                ticket: Ticket {
                    values: vec![7, 1, 14]
                },
                nears: vec![
                    Ticket {
                        values: vec![7, 3, 47]
                    },
                    Ticket {
                        values: vec![40, 4, 50]
                    },
                    Ticket {
                        values: vec![55, 2, 20]
                    },
                    Ticket {
                        values: vec![38, 6, 12]
                    }
                ]
            })
        );
    }

    #[test]
    fn scanning_error() {
        assert_eq!(71, INPUT_0.parse::<Reference>().unwrap().scanning_error());
    }

    #[test]
    fn remove_invalids() {
        assert_eq!(
            1,
            INPUT_0
                .parse::<Reference>()
                .unwrap()
                .remove_invalids()
                .nears
                .len()
        );
    }

    #[test]
    fn create_fields() {
        let reference = INPUT_1.parse::<Reference>().unwrap().remove_invalids();
        let fieldref = FieldReference::from(&reference);
        assert_eq!(
            fieldref,
            FieldReference {
                fields: vec![
                    Field {
                        name: None,
                        values: vec![11, 3, 15, 5],
                    },
                    Field {
                        name: None,
                        values: vec![12, 9, 1, 14],
                    },
                    Field {
                        name: None,
                        values: vec![13, 18, 5, 9],
                    }
                ]
            }
        );
    }

    #[test]
    fn assigned() {
        let reference = INPUT_1.parse::<Reference>().unwrap().remove_invalids();
        let mut fieldref = FieldReference::from(&reference);
        fieldref.assign(reference);
        assert_eq!(fieldref.fields[0].name, Some("row".to_string()));
        assert_eq!(fieldref.fields[1].name, Some("class".to_string()));
        assert_eq!(fieldref.fields[2].name, Some("seat".to_string()));
    }
}
