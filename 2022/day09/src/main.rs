use std::{
    collections::HashSet,
    fs,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "L" => Ok(Direction::Left),
            "D" => Ok(Direction::Down),
            oth => Err(format!("no direction named {oth}")),
        }
    }
}

impl Direction {
    fn to_vec2(&self) -> Vec2 {
        match self {
            Direction::Left => Vec2::new(-1, 0),
            Direction::Right => Vec2::new(1, 0),
            Direction::Up => Vec2::new(0, -1),
            Direction::Down => Vec2::new(0, 1),
        }
    }
}

#[derive(Debug, Clone)]
struct Move {
    dir: Direction,
    distance: i32,
}

fn parse_moves(s: &str) -> Vec<Move> {
    s.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l.trim().split_once(" ").unwrap();
            Move {
                dir: a.parse().unwrap(),
                distance: b.parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    fn map(&self, f: fn(i32) -> i32) -> Self {
        Self {
            x: f(self.x),
            y: f(self.y),
        }
    }

    fn length_manhattan(&self) -> u32 {
        let abs = self.map(i32::abs);
        (abs.x + abs.y) as u32
    }

    fn is_adjacent(&self) -> bool {
        if self.length_manhattan() < 2 {
            true
        } else {
            let abs = self.map(i32::abs);
            if abs.x > 1 || abs.y > 1 {
                false
            } else {
                true
            }
        }
    }

    fn clamped(&self) -> Self {
        Vec2 {
            x: i32::max(i32::min(self.x, 1), -1),
            y: i32::max(i32::min(self.y, 1), -1),
        }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug, Clone)]
struct Simulation {
    visits: HashSet<Vec2>,
    rope: Vec<Vec2>,
}

impl Simulation {
    fn run(moves: &Vec<Move>, length: usize) -> Simulation {
        let mut sim = Simulation {
            visits: HashSet::from([Vec2::new(0, 0)]),
            rope: vec![Vec2::new(0, 0); length],
        };
        for mov in moves.iter() {
            sim.step(mov);
        }
        sim
    }

    fn step(&mut self, mov: &Move) {
        for _ in 0..mov.distance {
            self.rope[0] = self.rope[0].clone() + mov.dir.to_vec2();
            //self.view();
            self.seek();
        }
    }

    fn seek(&mut self) {
        for i in 1..self.rope.len() {
            let delta = self.rope[i - 1].clone() - self.rope[i].clone();
            if !delta.is_adjacent() {
                self.rope[i] = self.rope[i].clone() + delta.clamped();
                if i == self.rope.len() - 1 {
                    self.visits.insert(self.rope[i].clone());
                }
            }
        }
    }

    fn visited_count(&self) -> usize {
        self.visits.len()
    }

    fn view(&self) {
        let mut min = Vec2::new(i32::MAX, i32::MAX);
        let mut max = Vec2::new(i32::MIN, i32::MIN);
        for pos in self.visits.iter() {
            min.x = i32::min(min.x, pos.x);
            min.y = i32::min(min.y, pos.y);
            max.x = i32::max(max.x, pos.x);
            max.y = i32::max(max.y, pos.y);
        }
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let pos = Vec2::new(x, y);
                if self.rope[0] == pos {
                    print!("H");
                } else if self.rope.contains(&pos) {
                    print!("T");
                } else if pos.length_manhattan() == 0 {
                    print!("s");
                } else {
                    if self.visits.contains(&Vec2::new(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let moves = parse_moves(&fs::read_to_string("input.txt").unwrap());
    let sim = Simulation::run(&moves, 2);
    println!("Visited tiles with rope len  2: {}", sim.visited_count());
    let sim_long = Simulation::run(&moves, 10);
    println!(
        "Visited tiles with rope len 10: {}",
        sim_long.visited_count()
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    const EXAMPLE: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const EXAMPLE_LARGE: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn parse() {
        let moves = parse_moves(EXAMPLE);
        dbg!(&moves);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn run_simulation() {
        let moves = parse_moves(EXAMPLE);
        let simulation = Simulation::run(&moves, 2);
        simulation.view();
        assert_eq!(simulation.visited_count(), 13);
    }

    #[test]
    fn run_simulation_large() {
        let moves = parse_moves(EXAMPLE_LARGE);
        let simulation = Simulation::run(&moves, 10);
        simulation.view();
        assert_eq!(simulation.visited_count(), 36);
    }
}
