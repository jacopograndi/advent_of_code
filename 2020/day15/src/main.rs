use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Sequence {
    nums: Vec<i64>,
}

impl FromStr for Sequence {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Sequence {
            nums: s
                .trim_end_matches("\n")
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect(),
        })
    }
}

impl Sequence {
    fn last(&self) -> i64 {
        self.nums.last().unwrap().clone()
    }

    fn next(&self) -> Sequence {
        let last = self.last();
        let next = if let Some((i, _)) = self
            .nums
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .find(|(_, x)| x == &&last)
        {
            self.nums.len() as i64 - 1 - i as i64
        } else {
            0
        };
        Sequence {
            nums: self.nums.iter().chain([&next]).cloned().collect(),
        }
    }
}

#[derive(Debug)]
struct FastSequence {
    past: HashMap<i64, i64>,
    current: i64,
    step: i64,
}

impl FastSequence {
    fn from_seq(seq: &Sequence) -> Self {
        let mut fast = FastSequence {
            past: HashMap::new(),
            current: 0,
            step: 0,
        };
        for (i, num) in seq.nums.iter().enumerate() {
            fast.current = *num;
            if i <= seq.nums.len() - 2 {
                fast.step += 1;
                fast.past.insert(*num, i as i64);
            }
        }
        fast
    }

    fn next(&mut self) {
        if let Some(last) = self.past.get_mut(&self.current) {
            self.current = self.step - *last;
            *last = self.step;
        } else {
            self.past.insert(self.current, self.step);
            self.current = 0;
        }
        self.step += 1;
    }
}

fn run(seq: &Sequence, max: i64) -> Sequence {
    let mut curr = seq.clone();
    for _ in 0..(max - seq.nums.len() as i64) {
        curr = curr.next();
    }
    curr
}

fn run_map(seq: &Sequence, max: i64) -> i64 {
    let mut fast = FastSequence::from_seq(seq);
    for _ in 0..(max - seq.nums.len() as i64) {
        fast.next();
    }
    fast.current
}

fn main() {
    let seq: Sequence = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    println!("The 2020th number spoken is {}", run(&seq, 2020).last());
    println!(
        "The 30000000th number spoken is {}",
        run_map(&seq, 30000000)
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    fn get_simple_examples() -> Vec<(String, i64)> {
        vec![
            ("1,3,2".to_string(), 1),
            ("2,1,3".to_string(), 10),
            ("1,2,3".to_string(), 27),
            ("2,3,1".to_string(), 78),
            ("3,2,1".to_string(), 438),
            ("3,1,2".to_string(), 1836),
        ]
    }

    fn get_hard_examples() -> Vec<(String, i64)> {
        vec![
            ("0,3,6".to_string(), 175594),
            ("1,3,2".to_string(), 2578),
            ("2,1,3".to_string(), 3544142),
            ("1,2,3".to_string(), 261214),
            ("2,3,1".to_string(), 6895259),
            ("3,2,1".to_string(), 18),
            ("3,1,2".to_string(), 362),
        ]
    }

    fn base_example() {
        assert_eq![0, run(&"0,3,6".parse().unwrap(), 10).last()];
        assert_eq![0, run_map(&"0,3,6".parse().unwrap(), 10)];
    }

    #[test]
    fn simple_examples() {
        for (ex, res) in get_simple_examples().iter() {
            dbg!(ex);
            let seq: Sequence = ex.parse().unwrap();
            let mut curr = seq.clone();
            let mut fast = FastSequence::from_seq(&seq);
            for i in (seq.nums.len() as i64)..2020 {
                curr = curr.next();
                fast.next();
                assert_eq![curr.last(), fast.current];
            }
            assert_eq![*res, curr.last()];
            assert_eq![*res, fast.current];
        }
    }

    #[test]
    fn hard_examples() {
        for (ex, res) in get_hard_examples().iter() {
            dbg!(ex);
            assert_eq![res, &run_map(&ex.parse().unwrap(), 30000000)];
        }
    }
}
