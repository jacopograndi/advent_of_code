use std::{fs, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Occupied,
    Floor,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Tile::Empty,
            "." => Tile::Floor,
            "#" => Tile::Occupied,
            _ => unreachable!(),
        })
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Empty => "L".to_string(),
            Tile::Floor => ".".to_string(),
            Tile::Occupied => "#".to_string(),
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq)]
struct Ferry {
    layout: Vec<Tile>,
    size_x: u32,
    size_y: u32,
}

impl FromStr for Ferry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ferry = Ferry::default();
        let lines = s.split("\n").filter(|l| l.len() > 0);
        ferry.size_y = lines.clone().count() as u32;
        for line in lines {
            ferry.size_x = line.trim().len() as u32;
            for c in line.trim().chars() {
                ferry.layout.push(c.to_string().parse().unwrap());
            }
        }
        Ok(ferry)
    }
}

impl ToString for Ferry {
    fn to_string(&self) -> String {
        let mut res = String::default();
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let i = x + y * self.size_x;
                res += &self.layout[i as usize].to_string();
            }
            res += "\n";
        }
        res
    }
}

impl Ferry {
    fn occupied(&self) -> u32 {
        self.layout.iter().filter(|x| **x == Tile::Occupied).count() as u32
    }
    fn star(&self, x: u32, y: u32, range: bool) -> Vec<&Tile> {
        let mut star = vec![];
        let mut spin = vec![];
        for s in -1..2 as i32 {
            for t in -1..2 as i32 {
                if s != 0 || t != 0 {
                    spin.push((s, t));
                }
            }
        }
        for (s, t) in spin {
            if !range {
                let xs = x as i32 + s;
                let yt = y as i32 + t;
                let i: usize = (xs + yt * self.size_x as i32) as usize;
                if (0..self.size_x as i32).contains(&xs) && (0..self.size_y as i32).contains(&yt) {
                    star.push(&self.layout[i]);
                }
            } else {
                for d in 1..self.size_x as i32 {
                    let xs = x as i32 + s * d;
                    let yt = y as i32 + t * d;
                    let i: usize = (xs + yt * self.size_x as i32) as usize;
                    if (0..self.size_x as i32).contains(&xs)
                        && (0..self.size_y as i32).contains(&yt)
                        && self.layout[i] != Tile::Floor
                    {
                        star.push(&self.layout[i]);
                        break;
                    }
                }
            }
        }
        star
    }

    fn round(&self, tolerance: i32, range: bool) -> Self {
        let mut next = self.clone();
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let i = (x + y * self.size_x) as usize;
                if self.layout[i] == Tile::Empty {
                    if self
                        .star(x, y, range)
                        .iter()
                        .filter(|x| **x == &Tile::Occupied)
                        .count()
                        == 0
                    {
                        next.layout[i] = Tile::Occupied;
                    }
                } else if self.layout[i] == Tile::Occupied {
                    if self
                        .star(x, y, range)
                        .iter()
                        .filter(|x| **x == &Tile::Occupied)
                        .count()
                        >= tolerance as usize
                    {
                        next.layout[i] = Tile::Empty;
                    }
                }
            }
        }
        next
    }
}

fn engine(ferry: Ferry, tol: i32, range: bool) -> (i32, Ferry) {
    let mut i = 0;
    let mut next = ferry.clone();
    loop {
        let prev = next.clone();
        next = next.round(tol, range);
        if next == prev {
            break;
        }
        i += 1;
    }
    (i, next)
}

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let ferry = raw.parse::<Ferry>().unwrap();
    let (i, no_range) = engine(ferry.clone(), 4, false);
    println!("Occupied seats after {} rounds: {}", i, no_range.occupied());
    let (i, range) = engine(ferry.clone(), 5, true);
    println!("Occupied seats after {} rounds: {}", i, range.occupied());
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT_0: &str = "
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";

    #[test]
    fn parsing() {
        if let Ok(ferry) = INPUT_0.parse::<Ferry>() {
            assert_eq!(ferry.size_x, 10);
            assert_eq!(ferry.size_y, 10);
            assert_eq!(
                ferry.layout.iter().filter(|x| **x == Tile::Empty).count(),
                71
            );
        }
    }

    #[test]
    fn one_round() {
        if let Ok(ferry) = INPUT_0.parse::<Ferry>() {
            assert_eq!(ferry.round(4, false).occupied(), 71);
        }
    }

    #[test]
    fn engine_no_range() {
        if let Ok(ferry) = INPUT_0.parse::<Ferry>() {
            let (_, ferry) = engine(ferry, 4, false);
            assert_eq!(ferry.occupied(), 37);
        }
    }

    #[test]
    fn engine_ranged() {
        if let Ok(ferry) = INPUT_0.parse::<Ferry>() {
            let (_, ferry) = engine(ferry, 5, true);
            assert_eq!(ferry.occupied(), 26);
        }
    }
}
