use std::{fs, io::Lines, str::FromStr};

#[derive(Debug, Clone)]
struct Move {
    amt: i32,
    start: usize,
    end: usize,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        Ok(Move {
            amt: parts[1].parse().unwrap(),
            start: parts[3].parse::<usize>().unwrap() - 1,
            end: parts[5].parse::<usize>().unwrap() - 1,
        })
    }
}

#[derive(Debug, Clone)]
struct CargoShip {
    crates: Vec<Vec<char>>,
    crane: Vec<Move>,
}

impl FromStr for CargoShip {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (str_crates, str_crane) = s.split_once("\n\n").ok_or("no separator")?;
        let lines_crates: Vec<&str> = str_crates.split("\n").filter(|x| !x.is_empty()).collect();
        let mut crates = vec![];
        for i in 0..lines_crates[0].len() {
            if lines_crates.last().unwrap().chars().nth(i) != Some(' ') {
                let mut stack = vec![];
                for line in lines_crates.iter().rev().skip(1) {
                    if let Some(content) = line.chars().nth(i) {
                        if content != ' ' {
                            stack.push(content);
                        }
                    }
                }
                crates.push(stack);
            }
        }
        Ok(CargoShip {
            crates,
            crane: str_crane
                .split("\n")
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<Move>().unwrap())
                .collect(),
        })
    }
}

impl CargoShip {
    fn crane_op9000(&self, mov: &Move) -> Self {
        let mut ship: CargoShip = self.clone();
        for n in 0..mov.amt {
            let content = ship.crates[mov.start].pop().unwrap();
            ship.crates[mov.end].push(content);
        }
        ship
    }

    fn crane_op9001(&self, mov: &Move) -> Self {
        let mut ship: CargoShip = self.clone();
        let mut moved = vec![];
        for n in 0..mov.amt {
            let content = ship.crates[mov.start].pop().unwrap();
            moved.push(content);
        }
        for n in 0..mov.amt {
            let content = moved.pop().unwrap();
            ship.crates[mov.end].push(content);
        }
        ship
    }

    fn run(&self, f: fn(&Self, &Move) -> Self) -> Self {
        let mut ship: CargoShip = self.clone();
        for mov in self.crane.iter() {
            ship = f(&ship.clone(), mov);
        }
        ship
    }

    fn top(&self) -> String {
        let mut s = String::new();
        for stack in self.crates.iter() {
            s += &stack.last().unwrap().to_string();
        }
        s
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let ship = input.parse::<CargoShip>().unwrap();
    let fin = ship.run(CargoShip::crane_op9000);
    println!("9000: The top crates are: {}", fin.top());
    let fin = ship.run(CargoShip::crane_op9001);
    println!("9001: The top crates are: {}", fin.top());
}

#[cfg(test)]
mod test {
    use crate::*;

    const EXAMPLE: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn parse() {
        let ship: CargoShip = EXAMPLE.parse().unwrap();
        assert_eq!(
            ship.crates,
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],]
        );
        assert_eq!(ship.crane.len(), 4);
    }

    #[test]
    fn run_ship() {
        let ship: CargoShip = EXAMPLE.parse().unwrap();
        let fin = ship.run(CargoShip::crane_op9000);
        assert_eq!(
            fin.crates,
            vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z'],]
        );
    }
}
