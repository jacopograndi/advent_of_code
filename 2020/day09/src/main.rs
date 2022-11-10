use std::fs;

// sliding window with variable width
// the next number in the sequence is the sum of two in the window
// k[n] = k[i] + k[k] s.t. k-w < i,j < k, i != j, w = window width

fn parse_seq(input: String) -> Vec<i64> {
    let mut seq = Vec::new();
    for line in input.split("\n") {
        if let Ok(num) = line.trim().parse::<i64>() {
            seq.push(num);
        }
    }
    seq
}

fn check_in_window(window: &[i64], num: i64) -> bool {
    for s in 0..window.len() {
        for t in 0..window.len() {
            if s != t && window[s] != window[t] && window[s] + window[t] == num {
                return true;
            }
        }
    }
    false
}

fn find_weak(seq: &Vec<i64>, window_width: u32) -> Option<i64> {
    for i in (window_width)..(seq.len() as u32) {
        if !check_in_window(
            &seq[(i - window_width) as usize..i as usize],
            seq[i as usize],
        ) {
            return Some(seq[i as usize]);
        }
    }
    None
}

fn find_sequence(seq: &Vec<i64>, target: i64) -> Option<i64> {
    let index = seq
        .iter()
        .enumerate()
        .find(|(_, n)| **n == target)
        .unwrap()
        .0;
    for amp in 2..index - 1 {
        for i in 0..(index - amp) {
            let range = &seq[i as usize..(i + amp) as usize];
            let sum: i64 = range.iter().sum();
            if sum == target {
                let max = range.iter().max().unwrap();
                let min = range.iter().min().unwrap();
                return Some(max + min);
            }
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let seq = parse_seq(input.to_string());
    let solved = find_weak(&seq, 25);
    if let Some(num) = solved {
        println!("Num which isn't a sum of a pair in the window: {}", num);
        if let Some(range) = find_sequence(&seq, num) {
            println!("The sum of the lo..hi that sums to {}: {}", num, range);
        } else {
            println!("No number found");
        }
    } else {
        println!("No number found");
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example() {
        let input = "35
            20
            15
            25
            47
            40
            62
            55
            65
            95
            102
            117
            150
            182
            127
            219
            299
            277
            309
            576";
        let seq = parse_seq(input.to_string());
        let solved = find_weak(&seq, 5);
        assert_eq!(solved, Some(127));
        let range = find_sequence(&seq, 127);
        assert_eq!(range, Some(62));
    }
}
