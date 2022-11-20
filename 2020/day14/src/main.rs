use std::collections::HashMap;
use std::vec;
use std::{fs, str::FromStr};

trait Mask {
    fn apply(&self, addr: i64, num: i64) -> Vec<(i64, i64)>;
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct SimpleMask {
    lo: i64,
    hi: i64,
}

impl FromStr for SimpleMask {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = SimpleMask { lo: 0, hi: 0 };
        for (i, c) in s.trim().char_indices() {
            let j = 35 - i;
            match c {
                '0' => mask.lo |= 1 << j,
                '1' => mask.hi |= 1 << j,
                _ => (),
            }
        }
        Ok(mask)
    }
}

impl Mask for SimpleMask {
    fn apply(&self, addr: i64, num: i64) -> Vec<(i64, i64)> {
        vec![(addr, (num | self.hi) & !self.lo)]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct FloatingMask {
    lo: i64,
    hi: i64,
    floating: i64,
}

impl FromStr for FloatingMask {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = FloatingMask {
            lo: 0,
            hi: 0,
            floating: 0,
        };
        for (i, c) in s.trim().char_indices() {
            let j = 35 - i;
            match c {
                '0' => mask.lo |= 1 << j,
                '1' => mask.hi |= 1 << j,
                'X' => mask.floating |= 1 << j,
                _ => (),
            }
        }
        Ok(mask)
    }
}

impl Mask for FloatingMask {
    fn apply(&self, addr: i64, num: i64) -> Vec<(i64, i64)> {
        let mut floated = vec![];
        for i in (0..36).rev() {
            if (self.floating >> i) & 1 == 1 {
                floated.push(i);
            }
        }
        let masked = addr | self.hi;
        let mut ret = vec![];
        floated.sort();
        for i in 0..(2_i64.pow(floated.len() as u32)) {
            let mut newmask = masked;
            for j in 0..floated.len() {
                if (i >> j) & 1 == 1 {
                    newmask |= 2_i64.pow(floated[j] as u32);
                } else {
                    newmask &= !2_i64.pow(floated[j] as u32);
                }
            }
            ret.push((newmask, num));
        }
        ret
    }
}

#[derive(Clone, Debug)]
enum Instruction<T: Mask> {
    SetMask(T),
    Write { addr: i64, num: i64 },
}

impl<T: Mask> FromStr for Instruction<T>
where
    T: Clone,
    String: From<<T as FromStr>::Err>,
    T: FromStr,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once("=").ok_or("'=' not found")?;
        match left.trim() {
            "mask" => Ok(Self::SetMask(right.parse()?)),
            _ => Ok(Self::Write {
                addr: left
                    .trim()
                    .strip_prefix("mem[")
                    .ok_or("no mem[ found")?
                    .strip_suffix("]")
                    .ok_or("no ] found")?
                    .parse()
                    .unwrap(),
                num: right.trim().parse().unwrap(),
            }),
        }
    }
}

#[derive(Clone, Debug)]
struct Program<T: Mask> {
    instr: Vec<Instruction<T>>,
    mem: HashMap<i64, i64>,
    mask: T,
}

impl<T: Mask> FromStr for Program<T>
where
    T: Clone,
    String: From<<T as FromStr>::Err>,
    T: FromStr,
    T: Default,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Program {
            instr: s
                .split("\n")
                .filter_map(|l| l.trim().parse::<Instruction<T>>().ok())
                .collect(),
            mem: HashMap::new(),
            mask: T::default(),
        })
    }
}

impl<T: Mask> Program<T>
where
    T: Clone,
{
    fn apply(&self, instr: Instruction<T>) -> Program<T> {
        let mut next = self.clone();
        match instr {
            Instruction::SetMask(m) => next.mask = m.clone(),
            Instruction::Write { addr, num } => {
                for (a, v) in self.mask.apply(addr, num) {
                    next.mem.insert(a, v);
                }
            }
        };
        next.instr.remove(0);
        next
    }

    fn pop(&self) -> Option<Program<T>> {
        if let Some(instr) = self.instr.first() {
            Some(self.clone().apply(instr.clone()))
        } else {
            None
        }
    }

    fn exec(&self) -> Program<T> {
        let mut curr = self.clone();
        while let Some(next) = curr.pop() {
            curr = next;
        }
        curr
    }

    fn sum(&self) -> i64 {
        self.mem.iter().fold(0, |acc, (_, v)| acc + v)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let program: Program<SimpleMask> = input.parse().unwrap();
    println!("The simple sum is {}", program.exec().sum());
    let program: Program<FloatingMask> = input.parse().unwrap();
    println!("The floating sum is {}", program.exec().sum());
}

#[cfg(test)]
mod test {
    use crate::*;

    const MASK: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    const PROGRAM: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0";
    const PROGRAM_FLOAT: &str = "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1";

    #[test]
    fn parse_mask() {
        assert_eq!(Ok(SimpleMask { hi: 64, lo: 2 }), MASK.parse::<SimpleMask>());
    }

    #[test]
    fn apply_mask() {
        let mask = MASK.parse::<SimpleMask>().unwrap();
        assert_eq!(vec![(0, 73)], mask.apply(0, 11));
        assert_eq!(vec![(0, 101)], mask.apply(0, 101));
        assert_eq!(vec![(0, 64)], mask.apply(0, 0));
    }

    #[test]
    fn exec_simple_program() {
        let program: Program<SimpleMask> = PROGRAM.parse().unwrap();
        assert_eq!(165, program.exec().sum());
    }

    #[test]
    fn exec_floating_program() {
        let program: Program<FloatingMask> = PROGRAM_FLOAT.parse().unwrap();
        assert_eq!(208, program.exec().sum());
    }
}

// first try second part!
