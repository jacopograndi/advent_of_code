use core::str::FromStr;
use std::fs;

#[derive(Clone)]
enum ActCode {
    North,
    South,
    East,
    West,
    Forward,
    Left,
    Right,
}

impl FromStr for ActCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(ActCode::North),
            "S" => Ok(ActCode::South),
            "E" => Ok(ActCode::East),
            "W" => Ok(ActCode::West),
            "F" => Ok(ActCode::Forward),
            "L" => Ok(ActCode::Left),
            "R" => Ok(ActCode::Right),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct Action {
    code: ActCode,
    amt: i32,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, a) = s.split_at(1);
        Ok(Action {
            code: c.parse()?,
            amt: a.parse().unwrap(),
        })
    }
}

#[derive(Clone)]
struct Instructions {
    actions: Vec<Action>,
}

impl FromStr for Instructions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instructions {
            actions: s
                .split("\n")
                .filter(|l| !l.is_empty())
                .map(|l| l.trim().parse::<Action>().unwrap())
                .collect(),
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Heading {
    x: i32,
    y: i32,
}

impl Default for Heading {
    fn default() -> Self {
        Heading::from_angle(0)
    }
}

impl Heading {
    fn from_angle(angle: i32) -> Heading {
        let clamped = if angle % 360 < 0 {
            angle % 360 + 360
        } else {
            angle % 360
        };
        match clamped {
            0 => Heading { x: 1, y: 0 },
            90 => Heading { x: 0, y: 1 },
            180 => Heading { x: -1, y: 0 },
            270 => Heading { x: 0, y: -1 },
            _ => unreachable!(),
        }
    }

    fn to_angle(&self) -> i32 {
        match self {
            Heading { x: 1, y: 0 } => 0,
            Heading { x: 0, y: 1 } => 90,
            Heading { x: -1, y: 0 } => 180,
            Heading { x: 0, y: -1 } => 270,
            _ => unreachable!(),
        }
    }

    fn turn(&self, angle: i32) -> Heading {
        Heading::from_angle(angle + self.to_angle())
    }
}

trait Boat {
    fn cruise(&self, action: &Action) -> Self;
    fn distance(&self) -> i32;
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
struct SimpleBoat {
    x: i32,
    y: i32,
    heading: Heading,
}

impl Boat for SimpleBoat {
    fn cruise(&self, action: &Action) -> Self {
        match action.code {
            ActCode::Forward => self.forward(action.amt, &self.heading),
            ActCode::Right => self.turn(-action.amt),
            ActCode::Left => self.turn(action.amt),
            ActCode::East => self.forward(action.amt, &Heading::from_angle(0)),
            ActCode::North => self.forward(action.amt, &Heading::from_angle(90)),
            ActCode::West => self.forward(action.amt, &Heading::from_angle(180)),
            ActCode::South => self.forward(action.amt, &Heading::from_angle(270)),
        }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl SimpleBoat {
    fn turn(&self, angle: i32) -> SimpleBoat {
        SimpleBoat {
            x: self.x,
            y: self.y,
            heading: self.heading.turn(angle),
        }
    }

    fn forward(&self, amt: i32, head: &Heading) -> SimpleBoat {
        SimpleBoat {
            x: self.x + head.x * amt,
            y: self.y + head.y * amt,
            heading: self.heading.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Default for Waypoint {
    fn default() -> Self {
        Waypoint { x: 10, y: 1 }
    }
}

impl Waypoint {
    fn rotate(&self, angle: i32) -> Waypoint {
        let clamped = if angle % 360 < 0 {
            angle % 360 + 360
        } else {
            angle % 360
        };
        match clamped {
            0 => Waypoint {
                x: self.x,
                y: self.y,
            },
            90 => Waypoint {
                x: -self.y,
                y: self.x,
            },
            180 => Waypoint {
                x: -self.x,
                y: -self.y,
            },
            270 => Waypoint {
                x: self.y,
                y: -self.x,
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
struct WaypointBoat {
    x: i32,
    y: i32,
    waypoint: Waypoint,
}

impl Boat for WaypointBoat {
    fn cruise(&self, action: &Action) -> Self {
        match action.code {
            ActCode::Forward => self.forward(action.amt),
            ActCode::Right => self.rotate_wp(-action.amt),
            ActCode::Left => self.rotate_wp(action.amt),
            ActCode::East => self.move_wp(action.amt, &Heading::from_angle(0)),
            ActCode::North => self.move_wp(action.amt, &Heading::from_angle(90)),
            ActCode::West => self.move_wp(action.amt, &Heading::from_angle(180)),
            ActCode::South => self.move_wp(action.amt, &Heading::from_angle(270)),
        }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl WaypointBoat {
    fn forward(&self, amt: i32) -> WaypointBoat {
        WaypointBoat {
            x: self.x + self.waypoint.x * amt,
            y: self.y + self.waypoint.y * amt,
            waypoint: self.waypoint.clone(),
        }
    }

    fn rotate_wp(&self, angle: i32) -> WaypointBoat {
        WaypointBoat {
            x: self.x,
            y: self.y,
            waypoint: self.waypoint.rotate(angle),
        }
    }

    fn move_wp(&self, amt: i32, head: &Heading) -> WaypointBoat {
        WaypointBoat {
            x: self.x,
            y: self.y,
            waypoint: Waypoint {
                x: self.waypoint.x + head.x * amt,
                y: self.waypoint.y + head.y * amt,
            },
        }
    }
}

struct Navigation<T: Boat> {
    instructions: Instructions,
    boat: T,
}

impl<T: Boat> Navigation<T>
where
    T: Clone,
{
    fn cruise(&self) -> T {
        self.instructions
            .actions
            .iter()
            .fold(self.boat.clone(), |acc, i| acc.cruise(i))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let instructions = input.parse::<Instructions>().unwrap();
    let nav = Navigation {
        instructions: instructions.clone(),
        boat: SimpleBoat::default(),
    };
    let boat = nav.cruise();
    println!("the boat made it this far: {}", boat.distance());
    let nav = Navigation {
        instructions,
        boat: WaypointBoat::default(),
    };
    let boat = nav.cruise();
    println!("the waypoint boat made it this far: {}", boat.distance());
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT_0: &str = "
        F10
        N3
        F7
        R90
        F11";

    #[test]
    fn parse() {
        let instructions = INPUT_0.parse::<Instructions>().unwrap();
        assert_eq!(5, instructions.actions.len());
    }

    #[test]
    fn exec_instructions() {
        let nav = Navigation {
            instructions: INPUT_0.parse::<Instructions>().unwrap(),
            boat: SimpleBoat::default(),
        };
        let boat = nav.cruise();
        assert_eq!(
            SimpleBoat {
                x: 17,
                y: -8,
                heading: Heading::from_angle(270)
            },
            boat,
        );
        assert_eq!(boat.distance(), 25);
    }

    #[test]
    fn exec_waypoint_instructions() {
        let nav = Navigation {
            instructions: INPUT_0.parse::<Instructions>().unwrap(),
            boat: WaypointBoat::default(),
        };
        let boat = nav.cruise();
        assert_eq!(boat.distance(), 286);
    }
}
