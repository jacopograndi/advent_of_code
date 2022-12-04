use std::{fs, str::FromStr};

struct Lohi {
    lo: i32,
    hi: i32,
}

impl FromStr for Lohi {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lo, hi) = s.trim().split_once("-").ok_or("no -")?;
        Ok(Lohi {
            lo: lo.trim().parse().unwrap(),
            hi: hi.trim().parse().unwrap(),
        })
    }
}

impl Lohi {
    fn contains_whole(&self, other: &Lohi) -> bool {
        (other.lo..(other.hi + 1)).all(|n| (self.lo..(self.hi + 1)).contains(&n))
    }

    fn is_intersecting(&self, other: &Lohi) -> bool {
        (other.lo..(other.hi + 1)).any(|n| (self.lo..(self.hi + 1)).contains(&n))
    }
}

fn parse_lohis(s: &str) -> Vec<(Lohi, Lohi)> {
    s.split("\n")
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l.split_once(",").unwrap();
            (a.parse::<Lohi>().unwrap(), b.parse::<Lohi>().unwrap())
        })
        .collect()
}

fn sum_lohis(lohis: &Vec<(Lohi, Lohi)>, f: fn(&Lohi, &Lohi) -> bool) -> i32 {
    lohis
        .iter()
        .map(|(a, b)| f(a, b) || f(b, a))
        .filter(|x| *x)
        .count() as i32
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lohis = parse_lohis(&input);
    let sum = sum_lohis(&lohis, Lohi::contains_whole);
    println!("Fully contained: {}", sum);
    let sum = sum_lohis(&lohis, Lohi::is_intersecting);
    println!("Fully contained: {}", sum);
}

#[cfg(test)]
mod test {
    use crate::*;

    const EXAMPLE: &str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

    #[test]
    fn parse() {
        assert_eq!(parse_lohis(EXAMPLE).len(), 6);
    }

    #[test]
    fn simple() {
        assert_eq!(sum_lohis(parse_lohis(EXAMPLE), Lohi::contains_whole), 2);
    }
}
