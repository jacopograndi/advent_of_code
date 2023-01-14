use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Self::Right => Point(1, 0),
            Self::Up => Point(0, -1),
            Self::Left => Point(-1, 0),
            Self::Down => Point(0, 1),
        }
    }

    fn rotate_clockwise(&self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
        }
    }

    fn rotate_counterclockwise(&self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
        }
    }

    fn opposite(&self) -> Self {
        self.rotate_clockwise().rotate_clockwise()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Walkable,
    Wall,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Map {
    tiles: HashMap<Point, Tile>,
    size: Point,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<&str> = s.split("\n").collect();
        let size = Point(
            rows.iter().map(|l| l.len()).max().unwrap() as i32,
            rows.len() as i32,
        );
        let mut tiles = HashMap::new();
        for y in 0..size.1 {
            for x in 0..size.0 {
                match rows[y as usize].chars().nth(x as usize) {
                    Some('.') => tiles.insert(Point(x, y), Tile::Walkable),
                    Some('#') => tiles.insert(Point(x, y), Tile::Wall),
                    _ => None,
                };
            }
        }
        Ok(Map { tiles, size })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Face {
    id: u32,
    bounds: (Point, Point),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Edge {
    from: (u32, Direction),
    to: (u32, Direction),
    flip: bool,
}

impl Edge {
    fn flip(&self) -> Self {
        Self {
            from: (self.to.0, self.to.1.clone()),
            to: (self.from.0, self.from.1.clone()),
            flip: self.flip,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cube {
    faces: Vec<Face>,
    edges: Vec<Edge>,
}

impl Cube {
    fn insert_flipped(&mut self) {
        let flipped: Vec<Edge> = self.edges.iter().map(Edge::flip).collect();
        for flip in flipped.iter() {
            if !self.edges.contains(flip) {
                self.edges.push(flip.clone());
            }
        }
    }
}

impl Map {
    fn topleft(&self) -> Point {
        assert!(self.tiles.iter().find(|(p, _)| p.1 == 0).is_some());
        Point(
            self.tiles
                .iter()
                .filter(|(p, _)| p.1 == 0)
                .map(|(p, _)| p.0)
                .min()
                .unwrap(),
            0,
        )
    }

    fn spawn_point(&self) -> Point {
        self.tiles
            .iter()
            .filter(|(p, t)| matches!(t, Tile::Walkable) && p.1 == 0)
            .min_by(|(a, _), (b, _)| a.0.cmp(&b.0))
            .unwrap()
            .0
            .clone()
    }

    fn get_cube_side_length(&self) -> i32 {
        (0..self.size.1)
            .map(|y| self.tiles.iter().filter(|(p, _)| p.1 == y).count())
            .min()
            .unwrap() as i32
    }

    fn get_cube(&self) -> Cube {
        let sidelen = self.get_cube_side_length();
        let topleft = self.topleft();
        let mut faces = Vec::<Face>::new();
        let mut frontier = Vec::<Face>::new();
        let mut edges = Vec::<Edge>::new();
        let mut serial = 0;
        let face = Face {
            id: serial,
            bounds: (
                topleft.clone(),
                Point(topleft.0 + sidelen, topleft.1 + sidelen),
            ),
        };
        serial += 1;
        frontier.push(face.clone());
        loop {
            if frontier.len() == 0 {
                break;
            }
            let last = frontier.remove(0);
            faces.push(last.clone());

            for dir in [Direction::Right, Direction::Down, Direction::Left] {
                let delta = dir.to_point();
                let sample = Point(
                    delta.0 * sidelen + last.bounds.0 .0,
                    delta.1 * sidelen + last.bounds.0 .1,
                );
                if self.tiles.contains_key(&sample) {
                    let bounds = (
                        sample.clone(),
                        Point(sample.0 + sidelen, sample.1 + sidelen),
                    );
                    if frontier.iter().find(|f| f.bounds == bounds).is_none()
                        && faces.iter().find(|f| f.bounds == bounds).is_none()
                    {
                        let edge = Edge {
                            from: (last.id, dir.clone()),
                            to: (serial, dir.opposite()),
                            flip: false,
                        };
                        edges.push(edge.clone());
                        edges.push(edge.flip());
                        let face = Face { id: serial, bounds };
                        serial += 1;
                        frontier.push(face);
                    }
                }
            }
        }
        Cube { faces, edges }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Action {
    MoveForward(i32),
    RotateClockwise,
    RotateCounterclockwise,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Action::RotateClockwise),
            "L" => Ok(Action::RotateCounterclockwise),
            _ => Ok(Action::MoveForward(s.parse().unwrap())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cursor {
    pos: Point,
    dir: Direction,
    path_progress: usize,
    trail: Vec<Cursor>,
}

impl Cursor {
    fn final_password(&self) -> i64 {
        1000 * (self.pos.1 + 1) as i64
            + 4 * (self.pos.0 + 1) as i64
            + match self.dir {
                Direction::Right => 0,
                Direction::Up => 3,
                Direction::Left => 2,
                Direction::Down => 1,
            }
    }
}

trait Puzzle {
    fn get_map(&self) -> &Map;
    fn get_path(&self) -> &Vec<Action>;

    fn solve(&self) -> Cursor {
        let cursor = Cursor {
            pos: self.get_map().spawn_point(),
            dir: Direction::Right,
            path_progress: 0,
            trail: vec![],
        };
        self.follow_path(cursor)
    }

    fn follow_path(&self, cursor: Cursor) -> Cursor {
        if cursor.path_progress >= self.get_path().len() {
            let mut next = cursor.clone();
            {
                let mut no_trail = cursor.clone();
                no_trail.trail.clear();
                next.trail.push(no_trail);
            }
            println!("{}", self.view(&next));
            next
        } else {
            self.follow_path(match self.get_path()[cursor.path_progress] {
                Action::MoveForward(amt) => self.move_forward(cursor, amt),
                Action::RotateCounterclockwise => self.rotate_counterclockwise(cursor),
                Action::RotateClockwise => self.rotate_clockwise(cursor),
            })
        }
    }

    fn move_forward(&self, cursor: Cursor, amt: i32) -> Cursor {
        if amt <= 0 {
            Cursor {
                pos: cursor.pos.clone(),
                dir: cursor.dir.clone(),
                path_progress: cursor.path_progress + 1,
                trail: cursor.trail.clone(),
            }
        } else {
            if let Some(dest) = self.get_star(&cursor).get(&cursor.dir) {
                let mut next = Cursor {
                    pos: dest.0.clone(),
                    dir: dest.1.clone(),
                    path_progress: cursor.path_progress,
                    trail: cursor.trail.clone(),
                };
                {
                    let mut no_trail = cursor.clone();
                    no_trail.trail.clear();
                    next.trail.push(no_trail);
                }
                self.move_forward(next, amt - 1)
            } else {
                Cursor {
                    pos: cursor.pos.clone(),
                    dir: cursor.dir.clone(),
                    path_progress: cursor.path_progress + 1,
                    trail: cursor.trail.clone(),
                }
            }
        }
    }

    fn rotate_clockwise(&self, cursor: Cursor) -> Cursor {
        Cursor {
            pos: cursor.pos.clone(),
            dir: cursor.dir.rotate_clockwise(),
            path_progress: cursor.path_progress + 1,
            trail: cursor.trail.clone(),
        }
    }

    fn rotate_counterclockwise(&self, cursor: Cursor) -> Cursor {
        Cursor {
            pos: cursor.pos.clone(),
            dir: cursor.dir.rotate_counterclockwise(),
            path_progress: cursor.path_progress + 1,
            trail: cursor.trail.clone(),
        }
    }

    fn get_star(&self, cursor: &Cursor) -> HashMap<Direction, (Point, Direction)>;
    fn view(&self, cursor: &Cursor) -> String;
}

fn parse_puzzle(s: &str) -> (Map, Vec<Action>) {
    let (map, path) = s.split_once("\n\n").unwrap();
    let path = path.trim();
    let mut actions = vec![];
    let mut current = String::new();
    for c in path.chars() {
        if c.is_alphabetic() {
            actions.push(current.clone());
            current.clear();
            actions.push(c.to_string());
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        actions.push(current.clone());
    }
    (
        map.parse().unwrap(),
        actions.iter().map(|part| part.parse().unwrap()).collect(),
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PuzzleWrap {
    map: Map,
    path: Vec<Action>,
}

impl FromStr for PuzzleWrap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map, path) = parse_puzzle(s);
        Ok(PuzzleWrap { map, path })
    }
}

impl Puzzle for PuzzleWrap {
    fn get_map(&self) -> &Map {
        &self.map
    }

    fn get_path(&self) -> &Vec<Action> {
        &self.path
    }

    fn get_star(&self, cursor: &Cursor) -> HashMap<Direction, (Point, Direction)> {
        let pos = &cursor.pos;
        const DIRECTIONS: [Direction; 4] = [
            Direction::Right,
            Direction::Up,
            Direction::Left,
            Direction::Down,
        ];
        let mut star = HashMap::<Direction, (Point, Direction)>::new();
        for dir in DIRECTIONS.iter() {
            let delta = dir.to_point();
            let sample = Point(pos.0 + delta.0, pos.1 + delta.1);
            if let Some(tile) = self.map.tiles.get(&sample) {
                if matches!(tile, Tile::Walkable) {
                    star.insert(dir.clone(), (sample, dir.clone()));
                }
            } else {
                // look backwards
                let rev = dir.rotate_clockwise().rotate_clockwise();
                let rev_delta = rev.to_point();
                let mut rev_sample = pos.clone();
                // until there are no more tiles
                while let Some(_) = self.map.tiles.get(&rev_sample) {
                    rev_sample = Point(rev_sample.0 + rev_delta.0, rev_sample.1 + rev_delta.1);
                }
                // back one and check if it's a wall
                let wrapped = Point(rev_sample.0 + delta.0, rev_sample.1 + delta.1);
                if let Some(tile) = self.map.tiles.get(&wrapped) {
                    if matches!(tile, Tile::Walkable) {
                        star.insert(dir.clone(), (wrapped, dir.clone()));
                    }
                }
            }
        }
        star
    }

    fn view(&self, cursor: &Cursor) -> String {
        let mut s = String::new();
        for y in 0..self.map.size.1 {
            for x in 0..self.map.size.0 {
                let point = Point(x, y);
                if let Some(past) = cursor.trail.iter().find(|c| c.pos == point) {
                    s += match past.dir {
                        Direction::Right => ">",
                        Direction::Up => "^",
                        Direction::Left => "<",
                        Direction::Down => "v",
                    }
                } else if let Some(tile) = self.map.tiles.get(&Point(x, y)) {
                    s += match tile {
                        Tile::Walkable => ".",
                        Tile::Wall => "#",
                    };
                } else {
                    s += " ";
                }
            }
            s += "\n";
        }
        s
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PuzzleCube {
    map: Map,
    path: Vec<Action>,
    cube: Cube,
}

impl FromStr for PuzzleCube {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map, path) = parse_puzzle(s);
        Ok(PuzzleCube {
            map: map.clone(),
            path,
            cube: map.get_cube(),
        })
    }
}

impl Puzzle for PuzzleCube {
    fn get_map(&self) -> &Map {
        &self.map
    }

    fn get_path(&self) -> &Vec<Action> {
        &self.path
    }

    fn solve(&self) -> Cursor {
        let cursor = Cursor {
            pos: self.get_map().spawn_point(),
            dir: Direction::Right,
            path_progress: 0,
            trail: vec![],
        };
        self.follow_path(cursor)
    }

    fn get_star(&self, cursor: &Cursor) -> HashMap<Direction, (Point, Direction)> {
        let pos = &cursor.pos;
        let cube = &self.cube;
        const DIRECTIONS: [Direction; 4] = [
            Direction::Right,
            Direction::Up,
            Direction::Left,
            Direction::Down,
        ];
        let mut star = HashMap::new();
        for dir in DIRECTIONS.iter() {
            let delta = dir.to_point();
            let sample = Point(pos.0 + delta.0, pos.1 + delta.1);
            if let Some(tile) = self.map.tiles.get(&sample) {
                if matches!(tile, Tile::Walkable) {
                    star.insert(dir.clone(), (sample, dir.clone()));
                }
            } else {
                let face_from = cube
                    .faces
                    .iter()
                    .find(|f| {
                        f.bounds.0 .0 <= pos.0
                            && pos.0 < f.bounds.1 .0
                            && f.bounds.0 .1 <= pos.1
                            && pos.1 < f.bounds.1 .1
                    })
                    .unwrap();
                let edge = cube
                    .edges
                    .iter()
                    .find(|e| &e.from.1 == dir && e.from.0 == face_from.id)
                    .unwrap();
                let face_to = cube.faces.iter().find(|f| f.id == edge.to.0).unwrap();
                let sidelen = self.map.get_cube_side_length() - 1;
                let relative = Point(pos.0 - face_from.bounds.0 .0, pos.1 - face_from.bounds.0 .1);
                let mut amt = match edge.from.1 {
                    Direction::Right => relative.1,
                    Direction::Up => relative.0,
                    Direction::Left => relative.1,
                    Direction::Down => relative.0,
                };
                if edge.flip {
                    amt = sidelen - amt;
                }
                let delta_to = match edge.to.1 {
                    Direction::Up => Point(amt, 0),
                    Direction::Down => Point(amt, sidelen),
                    Direction::Left => Point(0, amt),
                    Direction::Right => Point(sidelen, amt),
                };
                let cubed = Point(
                    face_to.bounds.0 .0 + delta_to.0,
                    face_to.bounds.0 .1 + delta_to.1,
                );
                if let Some(tile) = self.map.tiles.get(&cubed) {
                    if matches!(tile, Tile::Walkable) {
                        star.insert(dir.clone(), (cubed, edge.to.1.opposite()));
                    }
                }
            }
        }
        star
    }

    fn view(&self, cursor: &Cursor) -> String {
        let mut s = String::new();
        for y in 0..self.map.size.1 {
            for x in 0..self.map.size.0 {
                let point = Point(x, y);
                if let Some(past) = cursor.trail.iter().find(|c| c.pos == point) {
                    s += match past.dir {
                        Direction::Right => ">",
                        Direction::Up => "^",
                        Direction::Left => "<",
                        Direction::Down => "v",
                    }
                } else if let Some(tile) = self.map.tiles.get(&Point(x, y)) {
                    s += match tile {
                        Tile::Walkable => ".",
                        Tile::Wall => "#",
                    };
                } else {
                    s += " ";
                }
            }
            s += "\n";
        }
        s
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let puzzle: PuzzleWrap = input.parse().unwrap();
    let cursor = puzzle.solve();
    println!("{}", puzzle.view(&cursor));
    println!("final password wrap: {}", cursor.final_password());
    let mut puzzle: PuzzleCube = input.parse().unwrap();
    puzzle.cube.edges.push(Edge {
        from: (1, Direction::Down),
        to: (2, Direction::Right),
        flip: false,
    });
    puzzle.cube.edges.push(Edge {
        from: (2, Direction::Left),
        to: (4, Direction::Up),
        flip: false,
    });
    puzzle.cube.edges.push(Edge {
        from: (3, Direction::Down),
        to: (5, Direction::Right),
        flip: false,
    });
    puzzle.cube.edges.push(Edge {
        from: (0, Direction::Left),
        to: (4, Direction::Left),
        flip: true,
    });
    puzzle.cube.edges.push(Edge {
        from: (1, Direction::Right),
        to: (3, Direction::Right),
        flip: true,
    });
    puzzle.cube.edges.push(Edge {
        from: (0, Direction::Up),
        to: (5, Direction::Left),
        flip: false,
    });
    puzzle.cube.edges.push(Edge {
        from: (1, Direction::Up),
        to: (5, Direction::Down),
        flip: false,
    });
    puzzle.cube.insert_flipped();
    assert_eq!(puzzle.cube.edges.len(), 4 * 6);
    let cursor = puzzle.solve();
    println!("{}", puzzle.view(&cursor));
    println!("final password cube: {}", cursor.final_password());
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").unwrap();
        let puzzle: PuzzleWrap = input.parse().unwrap();
        let cursor = puzzle.solve();
        assert_eq!(cursor.final_password(), 6032);
    }

    #[test]
    fn example_cube() {
        let input = fs::read_to_string("example.txt").unwrap();
        let mut puzzle: PuzzleCube = input.parse().unwrap();
        dbg!(&puzzle.cube);
        puzzle.cube.edges.push(Edge {
            from: (0, Direction::Left),
            to: (3, Direction::Up),
            flip: false,
        });
        puzzle.cube.edges.push(Edge {
            from: (1, Direction::Right),
            to: (4, Direction::Up),
            flip: true,
        });
        puzzle.cube.edges.push(Edge {
            from: (3, Direction::Down),
            to: (2, Direction::Right),
            flip: true,
        });
        puzzle.cube.edges.push(Edge {
            from: (0, Direction::Up),
            to: (5, Direction::Up),
            flip: true,
        });
        puzzle.cube.edges.push(Edge {
            from: (0, Direction::Right),
            to: (4, Direction::Right),
            flip: true,
        });
        puzzle.cube.edges.push(Edge {
            from: (2, Direction::Down),
            to: (5, Direction::Down),
            flip: true,
        });
        puzzle.cube.edges.push(Edge {
            from: (4, Direction::Down),
            to: (5, Direction::Left),
            flip: true,
        });
        puzzle.cube.insert_flipped();
        assert_eq!(puzzle.cube.edges.len(), 4 * 6);
        let cursor = puzzle.solve();
        assert_eq!(cursor.final_password(), 5031);
    }
}
