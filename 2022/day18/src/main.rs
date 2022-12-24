use std::{collections::HashSet, fs, io, str::FromStr};

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Vec3(i32, i32, i32);

impl FromStr for Vec3 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(",").collect();
        Ok(Vec3(
            coords[0].parse().unwrap(),
            coords[1].parse().unwrap(),
            coords[2].parse().unwrap(),
        ))
    }
}

impl Vec3 {
    fn distance(&self, oth: &Vec3) -> i32 {
        i32::abs(self.0 - oth.0) + i32::abs(self.1 - oth.1) + i32::abs(self.2 - oth.2)
    }
}

#[derive(Clone, Debug)]
struct Droplet {
    cubes: HashSet<Vec3>,
}

impl FromStr for Droplet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Droplet {
            cubes: s
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|l| l.parse().unwrap())
                .collect(),
        })
    }
}

const DIRS: [Vec3; 6] = [
    Vec3(1, 0, 0),
    Vec3(-1, 0, 0),
    Vec3(0, 1, 0),
    Vec3(0, -1, 0),
    Vec3(0, 0, 1),
    Vec3(0, 0, -1),
];

impl Droplet {
    fn surface_area(&self) -> i32 {
        self.cubes.iter().map(|pos| 6 - self.count_nears(pos)).sum()
    }

    fn count_nears(&self, pos: &Vec3) -> i32 {
        DIRS.iter()
            .filter(|dir| {
                self.cubes
                    .contains(&Vec3(pos.0 + dir.0, pos.1 + dir.1, pos.2 + dir.2))
            })
            .count() as i32
    }

    fn bounds(&self) -> (Vec3, Vec3) {
        let mut min = Vec3(i32::MAX, i32::MAX, i32::MAX);
        let mut max = Vec3(i32::MIN, i32::MIN, i32::MIN);
        for cube in &self.cubes {
            min.0 = min.0.min(cube.0);
            min.1 = min.1.min(cube.1);
            min.2 = min.2.min(cube.2);
            max.0 = max.0.max(cube.0);
            max.1 = max.1.max(cube.1);
            max.2 = max.2.max(cube.2);
        }
        (min, max)
    }

    fn fill_interior(&self) -> Droplet {
        let (min, max) = self.bounds();
        let mut trapped = Vec::<Vec3>::new();
        let outside_min = self.get_outside_surface(&min, min.distance(&max));
        let outside_max = self.get_outside_surface(&max, min.distance(&max));
        let outside: HashSet<Vec3> = outside_min.union(&outside_max).cloned().collect();
        for z in min.2..=max.2 {
            for y in min.1..=max.1 {
                for x in min.0..=max.0 {
                    let probe = Vec3(x, y, z);
                    if !self.cubes.contains(&probe) {
                        if !outside.contains(&probe) {
                            trapped.push(probe.clone());
                        }
                    }
                }
            }
        }
        let mut filled = self.clone();
        filled.cubes.extend(trapped);
        filled
    }

    fn get_outside_surface(&self, start: &Vec3, maxrange: i32) -> HashSet<Vec3> {
        let mut frontier = vec![start.clone()];
        let mut visited = HashSet::<Vec3>::new();
        loop {
            if frontier.len() == 0 {
                break visited;
            }
            let cur = frontier.remove(0);
            visited.insert(cur.clone());
            for dir in &DIRS {
                let next = Vec3(cur.0 + dir.0, cur.1 + dir.1, cur.2 + dir.2);
                if !visited.contains(&next)
                    && !frontier.contains(&next)
                    && !self.cubes.contains(&next)
                    && next.distance(start) < maxrange
                {
                    frontier.push(next);
                }
            }
        }
    }

    fn can_escape(&self, start: &Vec3) -> bool {
        let (min, max) = self.bounds();
        let threshold = min.distance(&max);
        let mut frontier = vec![start.clone()];
        let mut visited = Vec::<Vec3>::new();
        loop {
            if frontier.len() == 0 {
                break false;
            }
            let cur = frontier.remove(0);
            if cur.distance(start) > threshold || frontier.len() > 200 {
                break true;
            }
            visited.push(cur.clone());
            for dir in &DIRS {
                let next = Vec3(cur.0 + dir.0, cur.1 + dir.1, cur.2 + dir.2);
                if !visited.contains(&next)
                    && !frontier.contains(&next)
                    && !self.cubes.contains(&next)
                {
                    frontier.push(next);
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let droplet: Droplet = input.parse().unwrap();
    println!("The droplet surface area is: {}", droplet.surface_area());
    println!(
        "The filled droplet surface area is: {}",
        droplet.fill_interior().surface_area()
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn one_cube_droplet() {
        let droplet = Droplet {
            cubes: HashSet::from([Vec3(1, 1, 1)]),
        };
        assert_eq!(droplet.surface_area(), 6);
    }

    #[test]
    fn two_cubes_droplet() {
        let droplet = Droplet {
            cubes: HashSet::from([Vec3(1, 1, 1), Vec3(2, 1, 1)]),
        };
        assert_eq!(droplet.surface_area(), 10);
    }

    #[test]
    fn example_droplet() {
        let input = include_str!("example.txt");
        let droplet: Droplet = input.parse().unwrap();
        assert_eq!(droplet.surface_area(), 64);
    }

    #[test]
    fn example_droplet_without_interior() {
        let input = include_str!("example.txt");
        let droplet: Droplet = input.parse().unwrap();
        assert_eq!(droplet.fill_interior().surface_area(), 58);
    }
}
