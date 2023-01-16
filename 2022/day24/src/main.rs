// I DID THIS IN LUDUM DARE FIFTYYYYY
// lol xdd

use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

#[derive(Clone, Debug, Eq, PartialEq, Default, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist(&self, oth: &Vec2) -> i32 {
        i32::abs(self.x - oth.x) + i32::abs(self.y - oth.y)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Direction::Right),
            "<" => Ok(Direction::Left),
            "^" => Ok(Direction::Up),
            "v" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn to_vec2(&self) -> Vec2 {
        match self {
            Direction::Right => Vec2::new(1, 0),
            Direction::Left => Vec2::new(-1, 0),
            Direction::Up => Vec2::new(0, -1),
            Direction::Down => Vec2::new(0, 1),
        }
    }

    fn from_vec2(vec: &Vec2) -> Self {
        match vec {
            Vec2 { x: 1, y: 0 } => Direction::Right,
            Vec2 { x: -1, y: 0 } => Direction::Left,
            Vec2 { x: 0, y: -1 } => Direction::Up,
            Vec2 { x: 0, y: 1 } => Direction::Down,
            _ => panic!(),
        }
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Right => ">".to_string(),
            Direction::Left => "<".to_string(),
            Direction::Up => "^".to_string(),
            Direction::Down => "v".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Blizzard {
    pos: Vec2,
    vel: Vec2,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Valley {
    start: Vec2,
    end: Vec2,
    walls: HashSet<Vec2>,
    blizzards: Vec<Blizzard>,
    blizzard_mask: HashSet<Vec2>,
    size: Vec2,
    stars: HashMap<Vec2, Vec<Vec2>>,
}

impl FromStr for Valley {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut valley = Valley::default();
        let lines: Vec<&str> = s.split("\n").filter(|l| !l.is_empty()).collect();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    valley.walls.insert(Vec2::new(x as i32, y as i32));
                } else if let Ok(dir) = c.to_string().parse::<Direction>() {
                    valley.blizzards.push(Blizzard {
                        pos: Vec2::new(x as i32, y as i32),
                        vel: dir.to_vec2(),
                    });
                }
                valley.size.x = x as i32;
            }
            valley.size.y = y as i32;
        }

        for x in 0..valley.size.x {
            let sample = Vec2::new(x, 0);
            if !valley.walls.contains(&sample) {
                valley.start = sample;
            }
            let sample = Vec2::new(x, valley.size.y - 1);
            if !valley.walls.contains(&sample) {
                valley.end = sample;
            }
        }
        valley.fill_mask();
        Ok(valley)
    }
}

impl ToString for Valley {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..=self.size.y {
            for x in 0..=self.size.x {
                let sample = Vec2::new(x, y);
                let blizzes = self.blizzards.iter().filter(|b| b.pos == sample).count();
                if blizzes > 1 {
                    s += &blizzes.to_string();
                } else if let Some(b) = self.blizzards.iter().find(|b| b.pos == sample) {
                    s += &Direction::from_vec2(&b.vel).to_string();
                } else if self.walls.contains(&sample) {
                    s += "#";
                } else {
                    s += ".";
                }
            }
            s += "\n";
        }
        s
    }
}

impl Valley {
    fn blow_blizzards(&mut self) {
        for mut blizzard in self.blizzards.iter_mut() {
            blizzard.pos.x += blizzard.vel.x;
            blizzard.pos.y += blizzard.vel.y;
            if blizzard.pos.x >= self.size.x {
                blizzard.pos.x = 1;
            }
            if blizzard.pos.y >= self.size.y {
                blizzard.pos.y = 1;
            }
            if blizzard.pos.x <= 0 {
                blizzard.pos.x = self.size.x - 1;
            }
            if blizzard.pos.y <= 0 {
                blizzard.pos.y = self.size.y - 1;
            }
        }
        self.fill_mask();
    }

    fn fill_mask(&mut self) {
        self.blizzard_mask.clear();
        for blizzard in self.blizzards.iter_mut() {
            self.blizzard_mask.insert(blizzard.pos.clone());
        }
    }
}

const AVAILABLE_MOVES: [Vec2; 5] = [
    Vec2 { x: 0, y: 0 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: -1, y: 0 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: 0, y: -1 },
];

struct ValleyCache {
    valleys: Vec<Valley>,
}

impl ValleyCache {
    fn new(valley: &Valley) -> ValleyCache {
        ValleyCache {
            valleys: vec![valley.clone()],
        }
    }

    fn get(&mut self, time: i32) -> &Valley {
        while (self.valleys.len() as i32) <= time {
            let mut last = self.valleys.last().unwrap().clone();
            last.blow_blizzards();
            self.valleys.push(last);
        }
        &self.valleys[time as usize]
    }

    fn get_star(&mut self, time: i32, pos: &Vec2) -> &Vec<Vec2> {
        self.get(time);
        if self.valleys[time as usize].stars.is_empty() {
            let next = self.get(time + 1).clone();
            let last = &mut self.valleys[time as usize];
            for y in 0..=last.size.y + 1 {
                for x in 0..=last.size.x + 1 {
                    let mut stars = vec![];
                    let cur = Vec2::new(x, y);
                    for mov in AVAILABLE_MOVES {
                        let sample = Vec2::new(x + mov.x, y + mov.y);
                        if !next.walls.contains(&sample)
                            && !next.blizzard_mask.contains(&sample)
                            && sample.y >= 0
                            && sample.y <= last.size.y
                        {
                            stars.push(sample.clone());
                        }
                    }
                    last.stars.insert(cur.clone(), stars);
                }
            }
        }
        if let Some(star) = self.valleys[time as usize].stars.get(pos) {
            star
        } else {
            dbg!(pos);
            unreachable!()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SimNode {
    pos: Vec2,
    time: i32,
}

impl SimNode {
    fn view(&self, valley: &Valley) -> String {
        let mut s = valley.to_string();
        let i = (self.pos.x + self.pos.y * (valley.size.x + 2)) as usize;
        if s.chars().nth(i) == Some('.') {
            s.replace_range(i..=i, "E");
        } else {
            s.replace_range(i..=i, "!");
        }
        s
    }
}

fn shortest_path(init: &Valley, node: SimNode, target: &Vec2) -> SimNode {
    let mut cache = ValleyCache::new(init);
    let mut frontier = vec![node];
    let mut maxtime = 0;
    loop {
        if frontier.len() == 0 {
            unreachable!();
        }
        frontier.sort_by(|a, b| (a.time + a.pos.dist(target)).cmp(&(b.time + b.pos.dist(target))));
        let step = frontier.remove(0);
        if &step.pos == target {
            let node = SimNode {
                pos: step.pos.clone(),
                time: step.time + 1,
            };
            break node;
        }
        if step.time > maxtime {
            maxtime = step.time;
        }
        for mov in cache.get_star(step.time, &step.pos) {
            let node = SimNode {
                pos: mov.clone(),
                time: step.time + 1,
            };
            if !frontier.contains(&node) {
                frontier.push(node.clone());
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let valley: Valley = input.parse().unwrap();
    let trip_to_end = shortest_path(
        &valley,
        SimNode {
            pos: valley.start.clone(),
            time: 0,
        },
        &valley.end,
    );
    println!("shortest path: {}", trip_to_end.time);
    let trip_to_start = shortest_path(&valley, trip_to_end, &valley.start);
    println!("shortest path back: {}", trip_to_start.time);
    let trip_to_start_again = shortest_path(&valley, trip_to_start, &valley.end);
    println!("shortest path back and forth: {}", trip_to_start_again.time);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn parse() {
        let input = include_str!("example.txt");
        let valley: Valley = input.parse().unwrap();
        println!("{}", valley.to_string());
        assert_eq!(valley.to_string(), input);
    }

    #[test]
    fn shortest_path_test() {
        let input = include_str!("example.txt");
        let valley: Valley = input.parse().unwrap();
        assert_eq!(
            shortest_path(
                &valley,
                SimNode {
                    pos: valley.start.clone(),
                    time: 0
                },
                &valley.end,
            )
            .time,
            18
        );
    }

    #[test]
    fn shortest_path_back_and_forth() {
        let input = include_str!("example.txt");
        let valley: Valley = input.parse().unwrap();
        let trip_to_end = shortest_path(
            &valley,
            SimNode {
                pos: valley.start.clone(),
                time: 0,
            },
            &valley.end,
        );
        assert_eq!(trip_to_end.time, 18);
        let trip_to_start = shortest_path(&valley, trip_to_end, &valley.start);
        assert_eq!(trip_to_start.time, 42);
        let trip_to_start_again = shortest_path(&valley, trip_to_start, &valley.end);
        assert_eq!(trip_to_start_again.time, 54);
    }
}
