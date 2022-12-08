use std::{fs, str::FromStr};

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<u32>>,
}

impl FromStr for Forest {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        Ok(Forest { trees })
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_vec(&self) -> (i32, i32) {
        match self {
            &Self::Left => (-1, 0),
            &Self::Right => (1, 0),
            &Self::Up => (0, -1),
            &Self::Down => (0, 1),
        }
    }

    fn all() -> Vec<Direction> {
        vec![
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ]
    }
}

impl Forest {
    fn map_coords<T>(&self, f: fn(&Self, (usize, usize)) -> T) -> Vec<Vec<T>> {
        self.trees
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, _)| f(self, (x, y)))
                    .collect()
            })
            .collect()
    }

    fn check_view(&self, pos: (usize, usize), dir: &Direction) -> bool {
        let (dx, dy) = dir.to_vec();
        let dist = match dir {
            Direction::Right => self.trees[0].len() - pos.0 - 1,
            Direction::Left => pos.0,
            Direction::Down => self.trees.len() - pos.1 - 1,
            Direction::Up => pos.1,
        };
        for i in 1..=dist {
            let cur = (pos.0 as i32 + i as i32 * dx, pos.1 as i32 + i as i32 * dy);
            let cur = (cur.0 as usize, cur.1 as usize);
            if self.trees[cur.1][cur.0] >= self.trees[pos.1][pos.0] {
                return false;
            }
        }
        true
    }

    fn visibilty(&self, pos: (usize, usize)) -> bool {
        Direction::all().iter().any(|dir| self.check_view(pos, dir))
    }

    fn count_visible(&self) -> u32 {
        self.map_coords(Forest::visibilty)
            .iter()
            .map(|row| row.iter().map(|t| if *t { 1 } else { 0 }).sum::<u32>())
            .sum()
    }

    fn get_scenic_score(&self, pos: (usize, usize), dir: &Direction) -> u32 {
        let (dx, dy) = dir.to_vec();
        let dist = match dir {
            Direction::Right => self.trees[0].len() - pos.0 - 1,
            Direction::Left => pos.0,
            Direction::Down => self.trees.len() - pos.1 - 1,
            Direction::Up => pos.1,
        };
        for i in 1..=dist {
            let cur = (pos.0 as i32 + i as i32 * dx, pos.1 as i32 + i as i32 * dy);
            let cur = (cur.0 as usize, cur.1 as usize);
            if self.trees[cur.1][cur.0] >= self.trees[pos.1][pos.0] {
                return i as u32;
            }
        }
        dist as u32
    }

    fn scenic_score(&self, pos: (usize, usize)) -> u32 {
        Direction::all()
            .iter()
            .map(|dir| self.get_scenic_score(pos, dir))
            .product()
    }

    fn max_scenic_score(&self) -> u32 {
        *self
            .map_coords(Forest::scenic_score)
            .iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let forest: Forest = input.parse().unwrap();
    println!("Trees visible from outside: {}", forest.count_visible());
    println!(
        "Tree with the max scenic score: {}",
        forest.max_scenic_score()
    );
}

#[cfg(test)]
mod test {
    use crate::*;

    const EXAMPLE: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn parse_forest() {
        let forest: Result<Forest, String> = EXAMPLE.parse();
        dbg!(&forest);
        assert!(forest.is_ok());
    }

    #[test]
    fn map_visibility() {
        let forest: Forest = EXAMPLE.parse().unwrap();
        assert_eq!(21, forest.count_visible());
    }

    #[test]
    fn map_score() {
        let forest: Forest = EXAMPLE.parse().unwrap();
        assert_eq!(8, forest.max_scenic_score());
    }
}
