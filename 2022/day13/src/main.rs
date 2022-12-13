use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
enum List {
    Value(i32),
    List(Box<Vec<List>>),
}

impl FromStr for List {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("[") {
            let mut indent = 0;
            let mut elements = vec![];
            let mut start = 0;
            for (i, c) in s.chars().enumerate() {
                if c == '[' {
                    if indent == 0 {
                        start = i + 1;
                    }
                    indent += 1
                }
                if c == ']' {
                    indent -= 1;
                    if indent <= 0 && start < i {
                        elements.push(s[start..i].to_string());
                        break;
                    }
                }
                if c == ',' {
                    if indent == 1 {
                        elements.push(s[start..i].to_string());
                        start = i + 1;
                    }
                }
            }
            let mut list = vec![];
            for el in elements {
                list.push(el.parse().unwrap());
            }
            Ok(List::List(Box::new(list)))
        } else {
            Ok(List::Value(s.parse().unwrap()))
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            List::Value(v) => match other {
                List::Value(w) => v.cmp(w),
                List::List(_) => List::List(Box::new(vec![self.clone()])).cmp(other),
            },
            List::List(list) => match other {
                List::Value(_) => self.cmp(&List::List(Box::new(vec![other.clone()]))),
                List::List(oth) => {
                    let mut i: usize = 0;
                    loop {
                        if i as i32 > list.len() as i32 - 1 {
                            break Ordering::Less;
                        }
                        if i as i32 > oth.len() as i32 - 1 {
                            break Ordering::Greater;
                        }
                        let ord = list[i].cmp(&oth[i]);
                        if ord != Ordering::Equal {
                            return ord;
                        }
                        i += 1;
                    }
                }
            },
        }
    }
}

struct Pairs {
    packets: Vec<(List, List)>,
}

impl FromStr for Pairs {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pairs {
            packets: s
                .split("\n\n")
                .filter(|x| !x.is_empty())
                .map(|pair| {
                    let (l, r) = pair.split_once("\n").unwrap();
                    (l.parse().unwrap(), r.parse().unwrap())
                })
                .collect(),
        })
    }
}

impl Pairs {
    fn sum_valid_indexes(&self) -> i32 {
        self.packets
            .iter()
            .enumerate()
            .filter(|(_, p)| p.0.cmp(&p.1) == Ordering::Less)
            .map(|(i, _)| i as i32 + 1)
            .sum()
    }
}

struct Packets {
    packets: Vec<List>,
}

impl FromStr for Packets {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut packets: Vec<List> = s
            .replace("\n\n", "\n")
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|packet| packet.parse().unwrap())
            .collect();
        packets.push("[[2]]".parse().unwrap());
        packets.push("[[6]]".parse().unwrap());
        Ok(Packets { packets })
    }
}

impl Packets {
    fn separators_index_product(&self) -> i32 {
        let mut packs = self.packets.clone();
        packs.sort_by(|a, b| a.cmp(&b));
        let sep0: List = "[[2]]".parse().unwrap();
        let sep1: List = "[[6]]".parse().unwrap();
        let i0 = packs
            .iter()
            .enumerate()
            .find(|&(_, p)| *p == sep0)
            .unwrap()
            .0
            + 1;
        let i1 = packs
            .iter()
            .enumerate()
            .find(|&(_, p)| *p == sep1)
            .unwrap()
            .0
            + 1;
        i0 as i32 * i1 as i32
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let pairs: Pairs = input.parse().unwrap();
    println!("sum of valid pair's indexes: {}", pairs.sum_valid_indexes());
    let packets: Packets = input.parse().unwrap();
    println!(
        "sorted packets dividers product: {}",
        packets.separators_index_product()
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn sorted_packets_dividers_product() {
        let input = fs::read_to_string("test.txt").unwrap();
        let packets: Packets = input.parse().unwrap();
        assert_eq!(packets.separators_index_product(), 140);
    }

    #[test]
    fn valid_packets_sum_indexes() {
        let input = fs::read_to_string("test.txt").unwrap();
        let pairs: Pairs = input.parse().unwrap();
        assert_eq!(pairs.sum_valid_indexes(), 13);
    }

    #[test]
    fn impl_parse() {
        assert_eq!(
            "[9]".parse::<List>(),
            Ok(List::List(Box::new(vec![List::Value(9)])))
        );
        assert_eq!(
            "[[1],[2,3,4]]".parse::<List>(),
            Ok(List::List(Box::new(vec![
                List::List(Box::new(vec![List::Value(1)])),
                List::List(Box::new(vec![
                    List::Value(2),
                    List::Value(3),
                    List::Value(4)
                ]))
            ])))
        );
        assert_eq!("[]".parse::<List>(), Ok(List::List(Box::new(vec![]))));
        assert_eq!(
            "[[]]".parse::<List>(),
            Ok(List::List(Box::new(vec![List::List(Box::new(vec![]))])))
        );
    }
}
