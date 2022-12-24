use std::{collections::HashSet, fs, str::FromStr};

#[derive(Clone, Debug)]
struct Rock {
    shape: Vec<Vec<bool>>,
    size: (u32, u32),
}

impl FromStr for Rock {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").filter(|l| !l.is_empty()).collect();
        Ok(Rock {
            shape: lines
                .iter()
                .map(|l| {
                    l.chars()
                        .map(|c| if c == '#' { true } else { false })
                        .collect()
                })
                .collect(),
            size: (lines[0].len() as u32, lines.len() as u32),
        })
    }
}

impl ToString for Rock {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.size.1 as usize {
            for x in 0..self.size.0 as usize {
                s += if self.shape[y][x] { "#" } else { "." };
            }
            s += "\n";
        }
        s
    }
}

fn get_rocks() -> Vec<Rock> {
    let raw = fs::read_to_string("rocks.txt").unwrap();
    raw.split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<Rock>().unwrap())
        .collect()
}

#[derive(Clone, Debug)]
struct Chamber {
    falling: Option<(Rock, (i32, i32))>,
    occupied: HashSet<(i32, i32)>,
}

impl Chamber {
    fn new() -> Chamber {
        Chamber {
            falling: None,
            occupied: HashSet::new(),
        }
    }
    fn height(&self) -> i32 {
        if self.occupied.len() > 0 {
            *self.occupied.iter().map(|(_, y)| y).max().unwrap() + 1
        } else {
            0
        }
    }
    fn spawn(&mut self, rock: &Rock) {
        while self.occupied.len() > 1000 {
            let value = self.occupied.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
            self.occupied.remove(&value.clone());
        }
        self.falling = Some((rock.clone(), (2, self.height() + 3)))
    }

    fn fall(&mut self) {
        if let Some((rock, pos)) = &self.falling {
            if pos.1 > 0 && !self.is_obstructed(rock, &(pos.0, pos.1 - 1)) {
                self.falling = Some((rock.clone(), (pos.0, pos.1 - 1)));
            } else {
                self.occupied.extend(Chamber::to_world(rock, pos));
                self.falling = None;
            }
        }
    }

    fn jet(&mut self, jet: char) {
        if let Some((rock, pos)) = &self.falling {
            let next = match jet {
                '>' => (pos.0 + 1, pos.1),
                '<' => (pos.0 - 1, pos.1),
                _ => {
                    dbg!(jet);
                    unreachable!()
                }
            };
            if !self.is_obstructed(rock, &next) {
                self.falling = Some((rock.clone(), next));
            }
        }
    }

    fn is_obstructed(&self, rock: &Rock, pos: &(i32, i32)) -> bool {
        return pos.0 < 0
            || (pos.0 + rock.size.0 as i32) > 7
            || Chamber::to_world(rock, pos)
                .iter()
                .any(|s| self.occupied.contains(s));
    }

    fn to_world(rock: &Rock, pos: &(i32, i32)) -> Vec<(i32, i32)> {
        let mut points = vec![];
        for y in 0..rock.size.1 {
            for x in 0..rock.size.0 {
                if rock.shape[y as usize][x as usize] {
                    points.push((x as i32 + pos.0, rock.size.1 as i32 - 1 - y as i32 + pos.1));
                }
            }
        }
        points
    }
}

impl ToString for Chamber {
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut max = self.height();
        let mut floating = vec![];
        if let Some((rock, pos)) = &self.falling {
            max = max.max(pos.1 + rock.size.1 as i32 - 1);
            floating = Chamber::to_world(rock, pos);
        }
        for y in (0..=max).rev() {
            s += "|";
            for x in 0..7 {
                if floating.contains(&(x, y)) {
                    s += "@";
                } else {
                    s += if self.occupied.contains(&(x, y)) {
                        "#"
                    } else {
                        "."
                    };
                }
            }
            s += "|";
            s += "\n";
        }
        s += "+-------+\n";
        s
    }
}

enum Stopper {
    Steps(i32),
    Period,
}

const GIANT_STEPS: u64 = 1000000000000;

/// returns the chamber height after the stopping condition is met
///
/// periodicity:
/// chamber rocks: inital rocks + periodic section + end section
/// each section is characterized by how many rocks have fallen and how high it is.
/// chamber rocks height after 1T rocks:
///     initial rocks height +
///     periodic section height n times +
///     end section height
/// s.t. n = (1T - initial rocks number) / periodic section rocks number
/// and the end section is ((1T - initial rocks number) % periodic section rocks number) rocks long
fn simulate(chamber: &Chamber, rocks: &Vec<Rock>, jets: &str, stopper: Stopper) -> u64 {
    let mut rock_iter = rocks.iter().cycle();
    let mut jets_iter = jets.chars().cycle();
    let mut i = 0;
    let mut j = 0;
    let mut js = vec![];
    let mut chamber = chamber.clone();
    // just wait the transient out
    let stable_threshold = rocks.len() * jets.len() * 50;
    loop {
        match stopper {
            Stopper::Steps(steps) => {
                if i >= steps {
                    return chamber.height() as u64;
                }
            }
            Stopper::Period => {
                if i >= stable_threshold as i32 {
                    if let Some((first_rocks, _, first_height)) =
                        js.iter().rev().find(|&(first_i, first_j, _)| {
                            &j % jets.len() == first_j % jets.len()
                                && &i % rocks.len() as i32 == first_i % rocks.len() as i32
                        })
                    {
                        let period_height = chamber.height() - first_height;
                        let period_rocks = i as u64 - *first_rocks as u64;
                        let n = (GIANT_STEPS - *first_rocks as u64) / period_rocks;
                        let height = *first_height as u64 + period_height as u64 * n;
                        let rem = (GIANT_STEPS - *first_rocks as u64) % period_rocks;
                        if rem == 0 {
                            return height;
                        }
                    }
                    js.push((i, j, chamber.height()));
                }
            }
        };
        let rock = rock_iter.next().unwrap();
        chamber.spawn(rock);
        loop {
            let jet = jets_iter.next().unwrap();
            chamber.jet(jet);
            chamber.fall();
            j += 1;
            if chamber.falling.is_none() {
                break;
            }
        }
        i += 1;
    }
}

fn main() {
    let input = &fs::read_to_string("input.txt")
        .unwrap()
        .trim_end_matches("\n")
        .to_string();
    let h = simulate(&Chamber::new(), &get_rocks(), input, Stopper::Steps(2022));
    println!("Pile height after 2022 rocks: {}", h);

    let big_h = simulate(&Chamber::new(), &get_rocks(), input, Stopper::Period);
    println!("Pile height after 1 tera rocks: {}", big_h);
}

#[cfg(test)]
mod test {
    use crate::*;

    const JETS: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn tower_height() {
        assert_eq!(
            simulate(&Chamber::new(), &get_rocks(), JETS, Stopper::Steps(2022)),
            3068
        );
    }
    #[test]
    fn tower_giant() {
        let input = JETS;
        let h = simulate(&Chamber::new(), &get_rocks(), input, Stopper::Period);
        assert_eq!(h, 1514285714288);
    }
}
