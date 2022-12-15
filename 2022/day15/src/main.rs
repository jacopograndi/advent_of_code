use std::{fs, ops::Add, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl FromStr for Vec2 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(", ").unwrap();
        Ok(Vec2 {
            x: a.trim_start_matches("x=").parse().unwrap(),
            y: b.trim_start_matches("y=").parse().unwrap(),
        })
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    fn distance(&self, oth: &Vec2) -> i32 {
        (self.x - oth.x).abs() + (self.y - oth.y).abs()
    }

    fn perpdot(&self) -> Vec2 {
        Vec2::new(-self.y, self.x)
    }
}

struct Sensor {
    pos: Vec2,
    beacon: Vec2,
}

impl FromStr for Sensor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(":").unwrap();
        Ok(Sensor {
            pos: a.split_once("at").unwrap().1.trim().parse().unwrap(),
            beacon: b.split_once("at").unwrap().1.trim().parse().unwrap(),
        })
    }
}

impl Sensor {
    fn range(&self) -> i32 {
        self.pos.distance(&self.beacon)
    }
}

fn parse_sensors(s: &str) -> Vec<Sensor> {
    s.split("\n")
        .filter(|x| !x.is_empty())
        .map(|l| l.parse::<Sensor>().unwrap())
        .collect()
}

fn cut(sensors: &Vec<Sensor>, y: i32) -> i32 {
    let maxrange = sensors.iter().map(|s| s.range()).max().unwrap();
    let minx = sensors.iter().map(|s| s.beacon.x).min().unwrap();
    let maxx = sensors.iter().map(|s| s.beacon.x).max().unwrap();
    ((minx - maxrange)..(maxx + maxrange))
        .filter(|&x| {
            sensors.iter().any(|s| {
                Vec2::new(x, y).distance(&s.pos) <= s.range() && Vec2::new(x, y) != s.beacon
            })
        })
        .count() as i32
}

fn check_all_range(sensors: &Vec<Sensor>, pos: &Vec2, min: &Vec2, max: &Vec2) -> bool {
    (min.x..=max.x).contains(&pos.x)
        && (min.y..=max.y).contains(&pos.y)
        && sensors.iter().all(|s| pos.distance(&s.pos) > s.range())
}

/// iterates along the borders of the sensor
fn search(sensors: &Vec<Sensor>, min: &Vec2, max: &Vec2) -> Option<Vec2> {
    for s in sensors.iter() {
        let mut pos = s.pos.clone();
        let mut arm = Vec2::new(0, s.range() + 1);
        let mut dir = Vec2::new(1, -1);
        for _ in 0..4 {
            let sample = pos.clone() + arm.clone();
            if check_all_range(sensors, &sample, min, max) {
                let sample = pos.clone() + arm.clone();
                return Some(sample);
            }
            for _ in 0..s.range() {
                pos = pos.clone() + Vec2::new(1, -1);
                let sample = pos.clone() + arm.clone();
                if check_all_range(sensors, &sample, min, max) {
                    return Some(sample);
                }
            }
            arm = arm.perpdot();
            dir = dir.perpdot();
        }
    }
    return None;
}

fn freq(pos: &Vec2) -> i64 {
    pos.x as i64 * 4000000 + pos.y as i64
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sensors = parse_sensors(&input);
    println!("cut count at y=2000000: {}", cut(&sensors, 2000000));
    let found = search(&sensors, &Vec2::new(0, 0), &Vec2::new(4000000, 4000000)).unwrap();
    println!(
        "distress beacon pos: x={}, y={}, freq: {}",
        found.x,
        found.y,
        freq(&found)
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn find_beacon() {
        let input = fs::read_to_string("test.txt").unwrap();
        let sensors = parse_sensors(&input);
        assert_eq!(
            search(&sensors, &Vec2::new(0, 0), &Vec2::new(20, 20)).unwrap(),
            Vec2::new(14, 11)
        );
        assert_eq!(
            freq(&search(&sensors, &Vec2::new(0, 0), &Vec2::new(20, 20)).unwrap()),
            56000011
        );
    }

    #[test]
    fn cut_count() {
        let input = fs::read_to_string("test.txt").unwrap();
        let sensors = parse_sensors(&input);
        assert_eq!(cut(&sensors, 10), 26);
    }
}
