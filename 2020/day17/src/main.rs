use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

#[derive(Eq, Hash, PartialEq, Clone, Debug, Default)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

#[derive(Debug, Default, Clone)]
struct Core {
    cells: HashSet<Pos>,
    dimensions: i32,
}

impl FromStr for Core {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut core = Core {
            cells: HashSet::<Pos>::new(),
            dimensions: 3,
        };
        for (y, line) in s.split("\n").enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    core.cells.insert(Pos {
                        x: x as i32,
                        y: y as i32,
                        z: 0,
                        w: 0,
                    });
                }
            }
        }
        Ok(core)
    }
}

impl ToString for Core {
    fn to_string(&self) -> String {
        let (min, max) = self.bounds();
        let mut s = String::default();

        for w in min.w..max.w + 1 {
            s = format!["{s}w={w}\n"];
            for z in min.z..max.z + 1 {
                s = format!["{s}z={z}\n"];
                for y in min.y..max.y + 1 {
                    for x in min.x..max.x + 1 {
                        if self.cells.contains(&Pos { x, y, z, w }) {
                            s += "#";
                        } else {
                            s += ".";
                        }
                    }
                    s += "\n";
                }
                s += "\n";
            }
            s += "\n";
        }
        s
    }
}

impl Core {
    fn bounds(&self) -> (Pos, Pos) {
        let mut min = Pos {
            x: i32::MAX,
            y: i32::MAX,
            z: i32::MAX,
            w: i32::MAX,
        };
        let mut max = Pos {
            x: i32::MIN,
            y: i32::MIN,
            z: i32::MIN,
            w: i32::MIN,
        };
        for pos in self.cells.iter() {
            min.x = min.x.min(pos.x);
            min.y = min.y.min(pos.y);
            min.z = min.z.min(pos.z);
            min.w = min.w.min(pos.w);
            max.x = max.x.max(pos.x);
            max.y = max.y.max(pos.y);
            max.z = max.z.max(pos.z);
            max.w = max.w.max(pos.w);
        }
        (min, max)
    }

    fn step(&self) -> Core {
        let (min, max) = self.bounds();
        let mut next = Core {
            cells: HashSet::<Pos>::new(),
            dimensions: self.dimensions,
        };
        let no_w = if self.dimensions == 3 {
            0..1
        } else {
            (min.w - 1)..(max.w + 2)
        };
        for w in no_w {
            for z in (min.z - 1)..(max.z + 2) {
                for y in (min.y - 1)..(max.y + 2) {
                    for x in (min.x - 1)..(max.x + 2) {
                        let pos = Pos { x, y, z, w };
                        let count = self.count_neighs(&pos);
                        if !self.cells.contains(&pos) {
                            if count == 3 {
                                next.cells.insert(pos);
                            }
                        } else {
                            if count == 2 || count == 3 {
                                next.cells.insert(pos);
                            }
                        }
                    }
                }
            }
        }
        next
    }

    fn count_neighs(&self, pos: &Pos) -> i32 {
        let mut sum = 0;
        let no_w = if self.dimensions == 3 { 0..1 } else { -1..2 };
        for w in no_w {
            for z in -1..2 {
                for y in -1..2 {
                    for x in -1..2 {
                        let off = Pos {
                            x: pos.x + x,
                            y: pos.y + y,
                            z: pos.z + z,
                            w: pos.w + w,
                        };
                        if !(x == 0 && y == 0 && z == 0 && w == 0) && self.cells.contains(&off) {
                            sum += 1;
                        }
                    }
                }
            }
        }
        sum
    }

    fn run(&self, iters: u32) -> Core {
        println!("cycle left {}", iters);
        println!("{}", &self.to_string());
        if iters == 0 {
            self.clone()
        } else {
            self.step().run(iters - 1)
        }
    }

    fn hyper(&self) -> Core {
        Core {
            cells: self.cells.clone(),
            dimensions: 4,
        }
    }
}

fn main() {
    let core: Core = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    println!("Core after 6 steps: {}", core.run(6).cells.len());
    println!(
        "Hypercore after 6 steps: {}",
        core.hyper().run(6).cells.len()
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = ".#.\n..#\n###";

    #[test]
    fn parse() {
        let core: Core = INPUT.parse().unwrap();
        assert_eq!(core.cells.len(), 5);
    }

    #[test]
    fn run() {
        let core: Core = INPUT.parse().unwrap();
        assert_eq!(core.run(1).cells.len(), 11);
        assert_eq!(core.run(6).cells.len(), 112);
    }

    #[test]
    fn hyperrun() {
        let core: Core = INPUT.parse().unwrap();
        assert_eq!(core.hyper().run(6).cells.len(), 848);
    }
}
