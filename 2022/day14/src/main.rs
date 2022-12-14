use std::{
    collections::HashSet,
    fmt::Display,
    fs,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl FromStr for Vec2 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((l, r)) = s.split_once(",") {
            Ok(Vec2 {
                x: l.parse().unwrap(),
                y: r.parse().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

impl Vec2 {
    fn distance(&self, oth: &Vec2) -> i32 {
        (self.x - oth.x).abs() + (self.y - oth.y).abs()
    }
    fn len(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
    fn norm(&self) -> Self {
        let len = self.len();
        if len > 0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self { x: 0, y: 0 }
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cave {
    rocks: HashSet<Vec2>,
    pourer: Vec2,
    sand: HashSet<Vec2>,
}

impl FromStr for Cave {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave = Cave {
            rocks: HashSet::new(),
            pourer: Vec2 { x: 500, y: 0 },
            sand: HashSet::new(),
        };
        let paths: Vec<&str> = s.split("\n").filter(|x| !x.is_empty()).collect();
        for path in &paths {
            let points: Vec<Vec2> = path
                .split(" -> ")
                .map(|x| x.parse::<Vec2>().unwrap())
                .collect();
            for i in 1..points.len() {
                let from = &points[i - 1].clone();
                let to = &points[i].clone();
                let diff = to.clone() - from.clone();
                let norm = diff.norm();
                for j in 0..=from.distance(&to) {
                    cave.rocks.insert(Vec2 {
                        x: norm.x * j + from.x,
                        y: norm.y * j + from.y,
                    });
                }
            }
        }
        Ok(cave)
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min = Vec2 {
            x: i32::MAX,
            y: i32::MAX,
        };
        let mut max = Vec2 {
            x: i32::MIN,
            y: i32::MIN,
        };
        for rock in &self.rocks {
            min.x = i32::min(min.x, rock.x);
            min.y = i32::min(min.y, rock.y);
            max.x = i32::max(max.x, rock.x);
            max.y = i32::max(max.y, rock.y);
        }
        min.x = i32::min(min.x, self.pourer.x);
        min.y = i32::min(min.y, self.pourer.y);
        max.x = i32::max(max.x, self.pourer.x);
        max.y = i32::max(max.y, self.pourer.y);
        let mut out = String::new();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let pos = Vec2 { x, y };
                if self.pourer == pos {
                    out += "+"
                } else {
                    if self.rocks.contains(&pos) {
                        out += "#"
                    } else if self.sand.contains(&pos) {
                        out += "O"
                    } else {
                        out += "."
                    }
                }
            }
            out += "\n"
        }
        write!(f, "{out}")
    }
}

#[derive(Eq, PartialEq)]
enum PourStop {
    FallThrough,
    Blocking,
}

impl Cave {
    fn pour(&mut self, stopper: &PourStop) -> bool {
        const MOVES: [Vec2; 3] = [
            Vec2 { x: 0, y: 1 },
            Vec2 { x: -1, y: 1 },
            Vec2 { x: 1, y: 1 },
        ];
        let mut particle = self.pourer.clone();
        if stopper == &PourStop::Blocking {
            if self.sand.contains(&self.pourer) {
                return false;
            }
        }
        for _ in 0..1000000 {
            let mut moved = false;
            for mov in &MOVES {
                let pos = particle.clone() + mov.clone();
                if !self.sand.contains(&pos) && !self.rocks.contains(&pos) {
                    particle = particle + mov.clone();
                    moved = true;
                    break;
                }
            }
            if !moved {
                self.sand.insert(particle);
                return true;
            }
        }
        false
    }

    fn run(&mut self, stopper: &PourStop) -> i32 {
        let mut iter = 0;
        loop {
            if !self.pour(stopper) {
                break iter;
            }
            iter += 1;
        }
    }

    fn add_floor(&mut self) {
        let mut min = Vec2 {
            x: i32::MAX,
            y: i32::MAX,
        };
        let mut max = Vec2 {
            x: i32::MIN,
            y: i32::MIN,
        };
        for rock in &self.rocks {
            min.x = i32::min(min.x, rock.x);
            min.y = i32::min(min.y, rock.y);
            max.x = i32::max(max.x, rock.x);
            max.y = i32::max(max.y, rock.y);
        }
        let mid = (min.x + max.x) / 2;
        self.rocks.insert(Vec2 {
            x: mid,
            y: max.y + 2,
        });
        for j in 1..20000 {
            self.rocks.insert(Vec2 {
                x: mid + j,
                y: max.y + 2,
            });
            self.rocks.insert(Vec2 {
                x: mid - j,
                y: max.y + 2,
            });
        }
    }
}

fn main() {
    let mut cave: Cave = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    let iter = cave.run(&PourStop::FallThrough);
    println!("Sand falls to infinity after: {iter}");
    let mut cave: Cave = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    cave.add_floor();
    let iter = cave.run(&PourStop::Blocking);
    println!("Sand blocks source after: {iter}");
}

#[cfg(test)]
mod test {
    use crate::*;

    const MINICAVE: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn minicave_flood_floored() {
        let mut cave: Cave = MINICAVE.parse().unwrap();
        cave.add_floor();
        let iter = cave.run(&PourStop::Blocking);
        println!("{cave}");
        assert_eq!(iter, 93);
    }

    #[test]
    fn pour_until_stopped() {
        let mut cave: Cave = MINICAVE.parse().unwrap();
        let iter = cave.run(&PourStop::FallThrough);
        println!("{cave}");
        assert_eq!(iter, 24);
    }

    #[test]
    fn minicave() {
        let cave: Cave = MINICAVE.parse().unwrap();
        println!("{cave}");
        assert_eq!(cave.rocks.len(), 20);
    }
}
