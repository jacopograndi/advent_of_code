use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct HeightMap {
    area: Vec<u8>,
    size: (u32, u32),
    start: (u32, u32),
    end: (u32, u32),
}

impl FromStr for HeightMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").filter(|x| !x.is_empty()).collect();
        let mut map = HeightMap {
            area: vec![],
            size: (lines[0].len() as u32, lines.len() as u32),
            start: (0, 0),
            end: (0, 0),
        };
        for y in 0..map.size.1 {
            for x in 0..map.size.0 {
                let spot = lines[y as usize].chars().nth(x as usize).unwrap();
                match spot {
                    'S' => {
                        map.start = (x, y);
                        map.area.push('a' as u8);
                    }
                    'E' => {
                        map.end = (x, y);
                        map.area.push('z' as u8);
                    }
                    c => {
                        map.area.push(c as u8);
                    }
                }
            }
        }
        Ok(map)
    }
}

impl HeightMap {
    fn search(&self) -> Option<Vec<usize>> {
        let mut frontier = Vec::<usize>::new();
        let mut visited = Vec::<usize>::new();
        let mut previous = vec![None; self.area.len()];
        frontier.push(self.xy_to_i(self.start));
        loop {
            if frontier.len() == 0 {
                return None;
            }
            let first = frontier.remove(0);
            if first == self.xy_to_i(self.end) {
                let mut trace = first;
                let mut path = vec![];
                while trace != self.xy_to_i(self.start) {
                    trace = previous[trace].unwrap();
                    path.push(trace);
                }
                self.view(&previous, &path);
                println!();
                return Some(path);
            }
            let moves = self.get_moves(first);
            let novel = moves
                .iter()
                .filter(|m| !visited.contains(m) && !frontier.contains(m))
                .cloned()
                .collect::<Vec<usize>>();
            for n in novel.iter() {
                previous[*n] = Some(first);
            }
            frontier.extend(novel);
            visited.push(first);
        }
    }

    fn get_moves(&self, i: usize) -> Vec<usize> {
        self.get_star(i)
            .iter()
            .filter(|&&j| self.can_walk(i, j))
            .cloned()
            .collect()
    }

    fn can_walk(&self, from: usize, to: usize) -> bool {
        let height_from = self.area[from];
        let height_to = self.area[to];
        height_to as i32 - height_from as i32 <= 1
    }

    fn get_star(&self, i: usize) -> Vec<usize> {
        const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let (x, y) = self.i_to_xy(i);
        let mut star = vec![];
        for (dx, dy) in DIRS {
            let (w, h) = (x as i32 + dx, y as i32 + dy);
            if w >= 0 && w < self.size.0 as i32 && h >= 0 && h < self.size.1 as i32 {
                star.push(self.xy_to_i((w as u32, h as u32)));
            }
        }
        star
    }

    fn xy_to_i(&self, xy: (u32, u32)) -> usize {
        (xy.0 + xy.1 * self.size.0) as usize
    }

    fn i_to_xy(&self, i: usize) -> (u32, u32) {
        (i as u32 % self.size.0, i as u32 / self.size.0)
    }

    fn distance(&self, from: (u32, u32), to: (u32, u32)) -> u32 {
        i32::abs(from.0 as i32 - to.0 as i32) as u32 + i32::abs(from.1 as i32 - to.1 as i32) as u32
    }

    fn view(&self, previous: &Vec<Option<usize>>, path: &Vec<usize>) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let i = self.xy_to_i((x, y));
                if self.start == (x, y) {
                    print!("S");
                } else if self.end == (x, y) {
                    print!("E");
                } else if path.contains(&i) {
                    if let Some(prev) = previous[i] {
                        let (a, b) = self.i_to_xy(prev);
                        let diff = (a as i32 - x as i32, b as i32 - y as i32);
                        match diff {
                            (1, 0) => print!("<"),
                            (-1, 0) => print!(">"),
                            (0, 1) => print!("^"),
                            (0, -1) => print!("v"),
                            _ => unreachable!(),
                        }
                    } else {
                        dbg!(i);
                    }
                } else {
                    print!("{}", self.area[i] as char);
                }
            }
            println!();
        }
    }
}

fn search_all(map: &HeightMap) -> u32 {
    map.area
        .iter()
        .enumerate()
        .filter(|&(_, &spot)| spot == 'a' as u8)
        .filter_map(|(i, _)| {
            let mut anya = (*map).clone();
            anya.start = anya.i_to_xy(i);
            anya.search()
        })
        .map(|path| path.len() as u32)
        .min()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map: HeightMap = input.parse().unwrap();
    println!("The shortest path is {}", map.search().unwrap().len());
    println!("The shortest path from any a is {}", search_all(&map));
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn parse() {
        let input = fs::read_to_string("test.txt").unwrap();
        let map: HeightMap = input.parse().unwrap();
        dbg!(map);
    }

    #[test]
    fn breadth_search() {
        let input = fs::read_to_string("test.txt").unwrap();
        let map: HeightMap = input.parse().unwrap();
        assert_eq!(map.search().unwrap().len(), 31);
    }

    #[test]
    fn breadth_search_all() {
        let input = fs::read_to_string("test.txt").unwrap();
        let map: HeightMap = input.parse().unwrap();
        assert_eq!(search_all(&map), 29);
    }
}
