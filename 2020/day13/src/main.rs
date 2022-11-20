use std::{fs, str::FromStr, vec};

/// first part was really easy.

#[derive(Debug)]
struct Schedule {
    target: u32,
    ids: Vec<i32>,
}

impl FromStr for Schedule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once("\n").ok_or(())?;
        Ok(Schedule {
            target: l.parse().or(Err(()))?,
            ids: r.split(",").filter_map(|x| x.parse::<i32>().ok()).collect(),
        })
    }
}

impl Schedule {
    fn first_after_target(&self) -> Vec<i32> {
        let mut res = vec![];
        for id in self.ids.iter() {
            let div = self.target as i32 / id;
            let rem = self.target as i32 % id;
            let after = if rem == 0 { div * id } else { (div + 1) * id };
            res.push(after);
        }
        res
    }

    fn score(&self) -> i32 {
        let (i, &earliest) = self
            .first_after_target()
            .iter()
            .enumerate()
            .min_by(|x, y| x.1.cmp(&y.1))
            .unwrap();
        self.ids[i] * (earliest - self.target as i32)
    }
}

/// second part ws very difficult.

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bus {
    id: i64,
    off: i64,
}

impl Bus {
    fn is_sync(&self, time: i64) -> bool {
        (time + self.off) % self.id == 0
    }
}

fn parse_buses(s: String) -> Vec<Bus> {
    s.trim_end_matches("\n")
        .split(",")
        .enumerate()
        .filter_map(|(i, x)| {
            dbg!(x);
            if let Ok(n) = x.parse::<i64>() {
                Some(Bus {
                    id: n,
                    off: i as i64,
                })
            } else {
                None
            }
        })
        .collect()
}

/// finds the first time a+rem is divisible by b.
/// the iteration is minimal, max iterations is |b|
/// s is the offset added to the iter var, is used to find the "wavelength"
fn get_off(a: i64, b: i64, rem: i64, s: i64) -> Result<i64, String> {
    for j in 0..b {
        let i: i128 = (j * a + s).into();
        if (i + rem as i128) % b as i128 == 0 {
            return Ok(i as i64);
        }
    }
    Err(format!("not found number {a} and {b} with rem: {rem}"))
}

/// fold on buses to basically get 11...1 in base (buses_0, buses_1, ..., buses_n)
fn slide(buses: Vec<Bus>) -> Result<i64, String> {
    let mut rem = 0;
    let mut delta = buses[0].id;
    for i in 1..buses.len() {
        let off = get_off(delta, buses[i].id, buses[i].off, 0)?;
        // keep first offset, others are only used to find the wavelenght
        if i == 1 {
            rem += off;
        }
        for _ in 0..i32::MAX {
            // cycle by previous wavelength until rem divides by buses.
            if buses[0..i + 1].iter().all(|bus| bus.is_sync(rem)) {
                break;
            }
            rem += delta;
        }
        // calculate next wavelength
        let next_delta = get_off(delta, buses[i].id, buses[i].off, off + delta)? - off;
        delta = next_delta;
    }
    Ok(rem)
}

/// just check every i32 * first bus id as an easy optimization
fn bruteforce(buses: Vec<Bus>) -> Result<i64, String> {
    for i in 0..i32::MAX as i64 {
        let j = i * buses[0].id;
        if buses.iter().all(|bus| bus.is_sync(j)) {
            return Ok(j);
        }
    }
    Err("the timestamp is too big".to_string())
}

fn main() {
    // just panic if something fails
    let input = fs::read_to_string("input.txt").unwrap();
    let sched = input.parse::<Schedule>().unwrap();
    let score = sched.score();
    println!("id * delta: {}", score);
    let only_second = input.split_once("\n").unwrap().1;
    let buses = parse_buses(only_second.to_string());
    let early = slide(buses).unwrap();
    println!("earliest: {}", early);
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT_0: &str = "939\n7,13,x,x,59,x,31,19";
    const BUSES_0: &str = "7,13,x,x,59,x,31,19";
    const BUSES_1: &str = "17,x,13,19";
    const BUSES_2: &str = "67,7,59,61";
    const BUSES_3: &str = "67,x,7,59,61";
    const BUSES_4: &str = "67,7,x,59,61";
    const BUSES_5: &str = "1789,37,47,1889";
    const BUSES_6: &str = "11,7,9,x,x,x,5";
    const BUSES_7: &str = "7,5,x,x,x,x,x,x,x,x,3";
    const BUSES_8: &str =
        "41,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,379";
    const BUSES_9: &str =
        "41,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,379,x,x,x,x,x,x,x,23";

    #[test]
    fn parse() {
        let sched = INPUT_0.parse::<Schedule>().unwrap();
        assert_eq!(sched.target, 939);
        assert_eq!(sched.ids.len(), 5);
    }

    #[test]
    fn find_earliest() {
        let sched = INPUT_0.parse::<Schedule>().unwrap();
        assert_eq!(944, *sched.first_after_target().iter().min().unwrap());
    }

    #[test]
    fn slideit_example() {
        assert_eq!(Ok(1068781), slide(parse_buses(BUSES_0.to_string())));
        assert_eq!(Ok(3417), slide(parse_buses(BUSES_1.to_string())));
        assert_eq!(Ok(754018), slide(parse_buses(BUSES_2.to_string())));
        assert_eq!(Ok(779210), slide(parse_buses(BUSES_3.to_string())));
        assert_eq!(Ok(1261476), slide(parse_buses(BUSES_4.to_string())));
        assert_eq!(Ok(1202161486), slide(parse_buses(BUSES_5.to_string())));
    }

    #[test]
    fn slideit_versus() {
        assert_eq!(
            bruteforce(parse_buses(BUSES_6.to_string())),
            slide(parse_buses(BUSES_6.to_string()))
        );
        assert_eq!(
            bruteforce(parse_buses(BUSES_7.to_string())),
            slide(parse_buses(BUSES_7.to_string()))
        );
        assert_eq!(
            bruteforce(parse_buses(BUSES_8.to_string())),
            slide(parse_buses(BUSES_8.to_string()))
        );
        assert_eq!(
            bruteforce(parse_buses(BUSES_9.to_string())),
            slide(parse_buses(BUSES_9.to_string()))
        );
    }
}
