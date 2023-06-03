#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::str::FromStr;

// copying the stack a lot
// using less the heap
// = fast

const TILE_WIDTH: usize = 10;
const TILE_HEIGHT: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile<const WIDTH: usize, const HEIGHT: usize>
where
    [(); WIDTH * HEIGHT]: Sized,
{
    id: u64,
    pixels: [bool; WIDTH * HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> FromStr for Tile<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]: Sized,
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, img) = s.split_once("\n").ok_or(())?;
        let mut pixels = [true; WIDTH * HEIGHT];
        for (y, line) in img.split("\n").filter(|l| !l.is_empty()).enumerate() {
            for (x, c) in line.chars().enumerate() {
                pixels[x + y * WIDTH] = match c {
                    '.' => false,
                    '#' => true,
                    _ => return Err(()),
                }
            }
        }
        Ok(Tile {
            id: head
                .trim_start_matches("Tile ")
                .trim_end_matches(":")
                .to_string()
                .parse()
                .or(Err(()))?,
            pixels,
        })
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ToString for Tile<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]: Sized,
{
    fn to_string(&self) -> String {
        let mut s = format!("Tile {}:\n", self.id);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                s += if self.pixels[x + y * WIDTH] { "#" } else { "." }
            }
            s += "\n";
        }
        s
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Tile<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]: Sized,
{
    fn rotate_right(&self) -> Self {
        let mut rotated = Tile {
            id: self.id,
            pixels: [false; WIDTH * HEIGHT],
        };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                rotated.pixels[x + y * HEIGHT] = self.pixels[(WIDTH - 1 - x) * HEIGHT + y];
            }
        }
        rotated
    }

    fn mirror_horizontal(&self) -> Self {
        let mut mirrored = Tile {
            id: self.id,
            pixels: [false; WIDTH * HEIGHT],
        };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                mirrored.pixels[x + y * HEIGHT] = self.pixels[(WIDTH - 1 - x) + y * HEIGHT];
            }
        }
        mirrored
    }

    fn matches(&self, oth: &Tile<WIDTH, HEIGHT>, dir: &Direction) -> bool {
        match dir {
            Direction::Right => {
                (0..HEIGHT).all(|y| self.pixels[WIDTH - 1 + y * WIDTH] == oth.pixels[y * WIDTH])
            }
            Direction::Left => {
                (0..HEIGHT).all(|y| oth.pixels[WIDTH - 1 + y * WIDTH] == self.pixels[y * WIDTH])
            }
            Direction::Down => {
                (0..WIDTH).all(|x| self.pixels[x] == oth.pixels[(WIDTH - 1) * WIDTH + x])
            }
            Direction::Up => {
                (0..WIDTH).all(|x| oth.pixels[x] == self.pixels[(WIDTH - 1) * WIDTH + x])
            }
        }
    }

    fn all(&self) -> Vec<Self> {
        vec![
            self.clone(),
            self.rotate_right(),
            self.rotate_right().rotate_right(),
            self.rotate_right().rotate_right().rotate_right(),
            self.mirror_horizontal(),
            self.rotate_right().mirror_horizontal(),
            self.rotate_right().rotate_right().mirror_horizontal(),
            self.rotate_right()
                .rotate_right()
                .rotate_right()
                .mirror_horizontal(),
        ]
    }

    fn sum(&self) -> u64 {
        self.pixels.iter().map(|p| *p as u64).sum()
    }

    fn get_monster_spared(&self) -> Option<u64> {
        const MONSTER: &str = "Tile 0:
..................#.
#....##....##....###
.#..#..#..#..#..#...
";
        let monster: Tile<20, 3> = MONSTER.parse().unwrap();
        let mut num = 0;
        for y in 0..HEIGHT - 3 {
            for x in 0..WIDTH - 19 {
                let mut found = true;
                for my in 0..3 {
                    for mx in 0..20 {
                        if monster.pixels[mx + my * 20] {
                            if !self.pixels[(x + mx) + (y + my) * WIDTH] {
                                found = false;
                            }
                        }
                    }
                }
                if found {
                    num += 1;
                }
            }
        }
        if num > 0 {
            Some(self.sum() - num * monster.sum())
        } else {
            None
        }
    }

    #[cfg(feature = "image")]
    fn to_image(&self) -> image::RgbImage {
        let mut raw = image::RgbImage::new(WIDTH as u32, HEIGHT as u32);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                raw.put_pixel(
                    x as u32,
                    y as u32,
                    if self.pixels[x + y * WIDTH] {
                        image::Rgb([0, 0, 0])
                    } else {
                        image::Rgb([255, 255, 255])
                    },
                );
            }
        }
        raw
    }
}

#[cfg(test)]
mod test_tile {

    use crate::*;

    const TEST_TILE: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###
";

    #[test]
    fn tile_serde() {
        let tile = TEST_TILE.parse::<Tile<TILE_WIDTH, TILE_HEIGHT>>().unwrap();
        assert_eq!(tile.to_string(), TEST_TILE);
    }

    #[test]
    fn tile_rotate() {
        let tile = TEST_TILE.parse::<Tile<TILE_WIDTH, TILE_HEIGHT>>().unwrap();
        assert_eq!(
            tile.rotate_right()
                .rotate_right()
                .rotate_right()
                .rotate_right()
                .to_string(),
            TEST_TILE
        );
    }

    #[test]
    fn tile_mirror() {
        let tile = TEST_TILE.parse::<Tile<TILE_WIDTH, TILE_HEIGHT>>().unwrap();
        assert_eq!(
            tile.mirror_horizontal().mirror_horizontal().to_string(),
            TEST_TILE
        );
    }

    #[test]
    fn matches() {
        let tile = TEST_TILE.parse::<Tile<TILE_WIDTH, TILE_HEIGHT>>().unwrap();
        let matching: [(
            (Tile<TILE_WIDTH, TILE_HEIGHT>, Tile<TILE_WIDTH, TILE_HEIGHT>),
            Direction,
        ); 4] = [
            ((tile.clone(), tile.mirror_horizontal()), Direction::Right),
            ((tile.clone(), tile.mirror_horizontal()), Direction::Left),
            (
                (tile.rotate_right(), tile.mirror_horizontal().rotate_right()),
                Direction::Up,
            ),
            (
                (tile.rotate_right(), tile.mirror_horizontal().rotate_right()),
                Direction::Down,
            ),
        ];
        for ((t, s), dir) in &matching {
            println!("{}", t.to_string());
            println!("{:?}", dir);
            println!("{}", s.to_string());
            println!();
            assert!(t.matches(&s, &dir));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl TryFrom<(&i64, &i64)> for Direction {
    type Error = ();
    fn try_from(value: (&i64, &i64)) -> Result<Self, Self::Error> {
        match value {
            (1, 0) => Ok(Direction::Right),
            (0, 1) => Ok(Direction::Up),
            (-1, 0) => Ok(Direction::Left),
            (0, -1) => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

impl From<&Direction> for (i64, i64) {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, -1),
        }
    }
}

const DIRS: [Direction; 4] = [
    Direction::Right,
    Direction::Up,
    Direction::Left,
    Direction::Down,
];

#[derive(Clone, Debug)]
struct Assembler<'a> {
    placed: Vec<Tile<TILE_WIDTH, TILE_HEIGHT>>,
    left: Vec<Tile<TILE_WIDTH, TILE_HEIGHT>>,
    image: &'a Image,
}

impl<'a> Assembler<'a> {
    fn new(image: &'a Image) -> Assembler<'a> {
        Self {
            placed: vec![],
            left: image.tiles.clone(),
            image,
        }
    }

    fn assemble(&self) -> Result<Image, String> {
        let mut front = vec![self.clone()];
        loop {
            let mut back = vec![];
            if !front.is_empty() {
                let asm = front.pop().unwrap();
                if let Some(complete) = asm.done() {
                    break Ok(complete);
                } else {
                    back.append(&mut asm.next());
                }
            } else {
                break Err(format!("no image found"));
            }
            front.append(&mut back);
        }
    }

    fn done(&self) -> Option<Image> {
        if self.left.len() == 0 && self.placed.len() == self.image.tiles.len() {
            Some(Image::new(self.placed.clone()))
        } else {
            None
        }
    }

    fn next(self) -> Vec<Assembler<'a>> {
        if self.left.is_empty() {
            return vec![self];
        } else {
            let mut ret = vec![];
            let len = self.placed.len();
            let next = (
                (len % self.image.side) as i64,
                (len / self.image.side) as i64,
            );
            for left in self.left.iter() {
                for l in left.all().iter() {
                    let star = self.get_star(next);
                    if star.is_empty() {
                        let mut new = self.clone();
                        new.place(l);
                        ret.push(new);
                    }
                    if self
                        .get_star(next)
                        .iter()
                        .all(|(near, dir)| l.matches(near, dir))
                    {
                        let mut new = self.clone();
                        new.place(l);
                        ret.push(new);
                    }
                }
            }
            ret
        }
    }

    fn place(&mut self, tile: &Tile<TILE_WIDTH, TILE_HEIGHT>) {
        self.placed.push(tile.clone());
        self.left.retain(|lf| lf.id != tile.id);
    }

    fn get_star(&self, pos: (i64, i64)) -> Vec<(&Tile<TILE_WIDTH, TILE_HEIGHT>, &Direction)> {
        DIRS.iter()
            .filter_map(|dir| {
                let (dx, dy) = dir.into();
                let (px, py) = (pos.0 + dx, pos.1 + dy);
                if !(0..self.image.side as i64).contains(&px) {
                    return None;
                }
                if !(0..self.image.side as i64).contains(&py) {
                    return None;
                }
                let i = px + py * self.image.side as i64;
                if let Some(tile) = self.placed.get(i as usize) {
                    Some((tile, dir))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Image {
    tiles: Vec<Tile<TILE_WIDTH, TILE_HEIGHT>>,
    side: usize,
}

impl FromStr for Image {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .split("\n\n")
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<Tile<TILE_WIDTH, TILE_HEIGHT>>().unwrap())
            .collect();
        Ok(Image::new(tiles))
    }
}

impl Image {
    fn new(tiles: Vec<Tile<TILE_WIDTH, TILE_HEIGHT>>) -> Image {
        let side = (tiles.len() as f64).sqrt() as usize;
        Image { tiles, side }
    }

    fn score(&self) -> u64 {
        let s = self.side;
        self.tiles[0].id
            * self.tiles[s - 1].id
            * self.tiles[s * s - 1].id
            * self.tiles[s * s - s].id
    }
}

#[cfg(test)]
mod test_assembler {
    use crate::*;

    #[test]
    fn example() {
        let inp = include_str!("example.txt");
        let img: Image = inp.parse().unwrap();
        let a = Assembler::new(&img);
        let assembled = a.assemble().unwrap();
        assert_eq!(20899048083289, assembled.score());
        let mut bigtile: Tile<24, 24> = Tile {
            id: assembled.score(),
            pixels: [false; 24 * 24],
        };
        print!("{}", assembled.tiles[0].to_string());
        for (i, tile) in assembled.tiles.iter().enumerate() {
            let (offx, offy) = (i % assembled.side, i / assembled.side);
            for y in 0..TILE_HEIGHT - 2 {
                for x in 0..TILE_WIDTH - 2 {
                    bigtile.pixels
                        [(offx * (TILE_WIDTH - 2)) + x + (offy * (TILE_HEIGHT - 2) + y) * 24] =
                        tile.pixels[x + 1 + (y + 1) * TILE_WIDTH];
                }
            }
        }
        let num = bigtile
            .all()
            .iter()
            .find_map(|img| img.get_monster_spared())
            .unwrap();
        assert_eq!(273, num);
    }
}

fn main() {
    let inp = include_str!("input.txt");
    let img: Image = inp.parse().unwrap();
    let a = Assembler::new(&img);
    if let Ok(completed) = a.assemble() {
        println!("Score: {}", completed.score());
        const BS: usize = 8 * 12;
        let mut bigtile: Tile<BS, BS> = Tile {
            id: completed.score(),
            pixels: [false; BS * BS],
        };
        for (i, tile) in completed.tiles.iter().enumerate() {
            let (offx, offy) = (i % completed.side, i / completed.side);
            for y in 0..TILE_HEIGHT - 2 {
                for x in 0..TILE_WIDTH - 2 {
                    bigtile.pixels
                        [(offx * (TILE_WIDTH - 2)) + x + (offy * (TILE_HEIGHT - 2) + y) * BS] =
                        tile.pixels[x + 1 + (y + 1) * TILE_WIDTH];
                }
            }
        }
        #[cfg(feature = "image")]
        {
            let img = bigtile.to_image();
            let fout = &mut std::fs::File::create(&std::path::Path::new("img.png")).unwrap();
            img.write_to(fout, image::ImageFormat::Png).unwrap();
        }
        let num = bigtile
            .all()
            .iter()
            .find_map(|img| img.get_monster_spared())
            .unwrap();
        println!("Not covered by dargon: {}", num);
    } else {
        println!("Score: no");
    }
}
