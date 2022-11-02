use std::fs;

struct Seat {
    x: i32,
    y: i32,
}

impl Seat {
    fn from_string(raw: String) -> Self {
        let (row, column) = raw.split_at(7);
        Seat {
            x: decode(column.to_string(), 'R'),
            y: decode(row.to_string(), 'B'),
        }
    }

    fn id(&self) -> i32 {
        self.x + self.y * 8
    }
}

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();

    let mut seats = vec![];

    let mut max_id = 0;
    for line in raw.split("\n") {
        if line.len() > 0 {
            let seat = Seat::from_string(line.to_string());
            max_id = max_id.max(seat.id());
            seats.push(seat);
        }
    }

    println!("The max seat id is: {}", max_id);

    seats.sort_by(|a, b| a.id().cmp(&b.id()));
    for i in 0..(seats.len() - 1) {
        if seats[i].id() == seats[i + 1].id() - 2 {
            println!("The target seat id is: {}", seats[i].id() + 1);
            break;
        }
    }
}

fn decode(raw: String, upper: char) -> i32 {
    let mut sum = 0;
    let rev: Vec<char> = raw.chars().rev().collect();
    for i in 0..raw.len() {
        if rev[i] == upper {
            sum += 2i32.pow(i as u32);
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn decode_simple_example() {
        assert_eq!(decode("FBFBBFF".to_string(), 'B'), 44);
        assert_eq!(decode("RLR".to_string(), 'R'), 5);
    }

    #[test]
    fn decode_complex_examples() {
        assert_eq!(Seat::from_string("BFFFBBFRRR".to_string()).id(), 567);
        assert_eq!(Seat::from_string("FFFBBBFRRR".to_string()).id(), 119);
        assert_eq!(Seat::from_string("BBFFBBFRLL".to_string()).id(), 820);
    }
}
