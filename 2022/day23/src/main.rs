use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn splat(n: i32) -> Self {
        Self { x: n, y: n }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Field {
    map: HashSet<Vec2>,
}

impl FromStr for Field {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Field {
            map: HashSet::from_iter(
                s.split("\n")
                    .filter(|l| !l.is_empty())
                    .enumerate()
                    .map(|(y, l)| {
                        l.chars()
                            .enumerate()
                            .filter(|(_, c)| c == &'#')
                            .map(|(x, _)| Vec2::new(x as i32, y as i32))
                            .collect::<Vec<Vec2>>()
                    })
                    .flatten(),
            ),
        })
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        let mut s = String::new();
        let (min, max) = self.bounds();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if self.map.contains(&Vec2::new(x, y)) {
                    s += "#";
                } else {
                    s += "."
                }
            }
            s += "\n";
        }
        s
    }
}

impl Field {
    fn bounds(&self) -> (Vec2, Vec2) {
        let mut min = Vec2::splat(i32::MAX);
        let mut max = Vec2::splat(i32::MIN);
        for pos in self.map.iter() {
            min.x = min.x.min(pos.x);
            min.y = min.y.min(pos.y);
            max.x = max.x.max(pos.x);
            max.y = max.y.max(pos.y);
        }
        (min, max)
    }

    fn empty_in_bound(&self) -> i32 {
        let (min, max) = self.bounds();
        let all = (max.y - min.y + 1) * (max.x - min.x + 1);
        all - self.map.len() as i32
    }
}

struct Spreader {
    look_order: Vec<Vec<Vec2>>,
}

impl Spreader {
    fn new() -> Self {
        Self {
            look_order: vec![
                vec![Vec2::new(0, -1), Vec2::new(1, -1), Vec2::new(-1, -1)],
                vec![Vec2::new(0, 1), Vec2::new(1, 1), Vec2::new(-1, 1)],
                vec![Vec2::new(-1, 0), Vec2::new(-1, -1), Vec2::new(-1, 1)],
                vec![Vec2::new(1, 0), Vec2::new(1, -1), Vec2::new(1, 1)],
            ],
        }
    }

    fn look(&mut self, field: &Field) -> HashMap<Vec2, Vec2> {
        let mut looks = HashMap::<Vec2, Vec2>::new();
        let mut dups = Vec::<Vec2>::new();
        for pos in field.map.iter() {
            if !field.map.iter().any(|oth| {
                !(i32::abs(oth.x - pos.x) == 2 || i32::abs(oth.y - pos.y) == 2)
                    && i32::abs(oth.x - pos.x) + i32::abs(oth.y - pos.y) <= 2
                    && pos != oth
            }) {
                continue;
            }
            let mut view: Option<Vec2> = None;
            for dirs in self.look_order.iter() {
                if !dirs
                    .iter()
                    .any(|dir| field.map.contains(&Vec2::new(pos.x + dir.x, pos.y + dir.y)))
                {
                    view = Some(dirs[0].clone());
                    break;
                }
            }
            if let Some(dir) = view {
                let to = Vec2::new(pos.x + dir.x, pos.y + dir.y);
                if looks.values().any(|v| v == &to) {
                    dups.push(to.clone());
                } else {
                    looks.insert(pos.clone(), to.clone());
                }
            }
        }
        for dup in dups {
            let keys: Vec<Vec2> = looks
                .iter()
                .filter_map(|(k, v)| if v == &dup { Some(k) } else { None })
                .cloned()
                .collect();
            for key in keys.iter() {
                looks.remove(&key);
            }
        }
        looks
    }

    fn spread(&mut self, field: &mut Field, looks: HashMap<Vec2, Vec2>) {
        for (from, to) in looks.iter() {
            field.map.remove(from);
            field.map.insert(to.clone());
        }
    }

    fn sim(field: &mut Field, stopper: Stopper) -> i32 {
        let mut spreader = Spreader::new();
        let mut iters = 0;
        let mut dirty = true;
        loop {
            match stopper {
                Stopper::Num(n) => {
                    if iters >= n {
                        break iters;
                    }
                }
                Stopper::Stasis => {
                    if !dirty {
                        break iters;
                    }
                }
            }
            let looks = spreader.look(field);
            if looks.is_empty() {
                dirty = false;
            }
            spreader.spread(field, looks);
            spreader.look_order.rotate_left(1);
            iters += 1;
            dbg!(iters);
        }
    }
}

enum Stopper {
    Num(i32),
    Stasis,
}

fn main() {
    let blue_field: Field = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    let mut field = blue_field.clone();
    Spreader::sim(&mut field, Stopper::Num(10));
    println!("after 10 rounds, {} spaces", field.empty_in_bound());

    let mut field = blue_field.clone();
    let iters = Spreader::sim(&mut field, Stopper::Stasis);
    println!("after {} rounds it terminates", iters);
}

#[cfg(test)]
mod test {

    use crate::*;

    const EXAMPLE_SMALL: &str = "##\n#.\n..\n##\n";
    #[test]
    fn example_small() {
        let mut field: Field = EXAMPLE_SMALL.parse().unwrap();
        Spreader::sim(&mut field, Stopper::Num(3));
        assert_eq!(field.empty_in_bound(), 25);
    }

    #[test]
    fn example_mid() {
        let mut field: Field = include_str!("example-mid.txt").parse().unwrap();
        Spreader::sim(&mut field, Stopper::Num(10));
        assert_eq!(field.empty_in_bound(), 110);
    }

    #[test]
    fn example_stasis() {
        let mut field: Field = include_str!("example-mid.txt").parse().unwrap();
        let iters = Spreader::sim(&mut field, Stopper::Stasis);
        assert_eq!(iters, 20);
    }
}
