use std::fs;

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap();
    let input: Vec<i32> = raw
        .split("\n")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    let res = solve(&input);
    println!("The number of 1 diff * 3 diff is: {}", res);
    println!("The number of permutations is: {}", get_perm(&input));
}

fn get_lohi(adas: &[i32]) -> Vec<i32> {
    let mut lohi = adas.to_vec().clone();
    lohi.sort();
    lohi.push(lohi.last().unwrap() + 3);
    let mut res = vec![0];
    res.append(&mut lohi);
    res
}

fn solve(adas: &[i32]) -> i32 {
    let lohi = get_lohi(adas);
    let mut prev = 0;
    let mut count_1 = 0;
    let mut count_3 = 0;
    for ada in lohi.iter() {
        match ada - prev {
            1 => count_1 += 1,
            3 => count_3 += 1,
            _ => (),
        }
        prev = *ada;
    }
    count_1 * count_3
}

fn find_sub_perms(sub: Vec<i32>) -> i64 {
    // bruteforce find permutation number
    let mut perms = (sub.len() == 1) as i64;
    let mut iter = sub.iter();
    if let Some(first) = sub.first() {
        for _ in 1..4 {
            if let Some(next) = iter.next() {
                //dbg!(next);
                if next - first <= 3 {
                    perms += find_sub_perms(iter.clone().map(|v| v.clone()).collect());
                }
            }
        }
    }
    perms
}

fn get_perm(adas: &[i32]) -> i64 {
    let lohi = get_lohi(adas);
    let mut perms = 1;
    let mut prev = 0;

    // look ahead numbers up to distance 3
    // see how many holes
    // advance to highest

    let mut subs = vec![];
    let mut sub = vec![];

    for ada in lohi {
        if ada - prev == 3 {
            subs.push(sub.clone());
            sub.clear();
        }
        sub.push(ada);
        prev = ada
    }

    for sub in subs {
        perms *= find_sub_perms(sub);
    }
    perms
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT_0: [i32; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    const INPUT_1: [i32; 31] = [
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    #[test]
    fn simple_0() {
        assert_eq!(solve(&INPUT_0), 35);
    }

    #[test]
    fn simple_1() {
        assert_eq!(solve(&INPUT_1), 220);
    }

    #[test]
    fn complex_0() {
        assert_eq!(get_perm(&INPUT_0), 8);
    }

    #[test]
    fn complex_1() {
        assert_eq!(get_perm(&INPUT_1), 19208);
    }

    #[test]
    fn perms() {
        assert_eq!(find_sub_perms(vec![1, 2, 3, 4]), 4);
    }
}
