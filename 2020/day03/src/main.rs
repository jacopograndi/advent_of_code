fn slide(lines: Vec<Vec<char>>, vx: i32, vy: i32) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut hits: i32 = 0;
    loop {
        if lines[y as usize][x as usize] == '#' {
            hits += 1;
        }
        x += vx;
        y += vy;
        if y as usize >= lines.len() {
            break;
        }
        if x as usize >= lines[y as usize].len() {
            x -= lines[y as usize].len() as i32;
        }
    }
    hits
}

fn slide_everywhere(lines: Vec<Vec<char>>) -> i32 {
    let mut hits: i32 = 1;
    for (vx, vy) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        hits *= slide(lines.clone(), vx, vy);
    }
    hits
}

fn main() {
    let lines: Vec<Vec<char>> = include_str!("../input0.txt")
        .trim()
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();
    let hits = slide(lines.clone(), 3, 1);
    println!("hits: {}", hits);
    let hits_everything = slide_everywhere(lines.clone());
    println!("hits on the five paths: {}", hits_everything);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn small_input() {
        let lines: Vec<Vec<char>> = include_str!("../test0.txt")
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(slide(lines, 3, 1), 7);
    }

    #[test]
    fn big_input() {
        let lines: Vec<Vec<char>> = include_str!("../input0.txt")
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(slide(lines, 3, 1), 220);
    }

    #[test]
    fn small_input_all_slopes() {
        let lines: Vec<Vec<char>> = include_str!("../test0.txt")
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(slide_everywhere(lines), 336);
    }
}
