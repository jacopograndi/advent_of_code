fn main() {
    let mut sled_valids = 0;
    let mut toboggan_valids = 0;
    for line in include_str!("../input0.txt").trim().split("\n") {
        let (rule, pswd) = line.split_once(": ").unwrap();
        let (range, letter) = rule.split_once(" ").unwrap();

        let letter = letter.chars().nth(0).unwrap();

        let (lo, hi) = range.split_once("-").unwrap();
        let lo: i32 = lo.parse().unwrap();
        let hi: i32 = hi.parse().unwrap();

        let count: i32 = pswd.chars().map(|c| (c == letter) as i32).sum();
        if lo <= count && count <= hi {
            sled_valids += 1;
        }

        let tobo: i32 = pswd
            .chars()
            .enumerate()
            .map(|(i, c)| (i as i32 + 1, c))
            .map(|(i, c)| (c == letter && (hi == i || lo == i)) as i32)
            .sum();
        if tobo == 1 {
            toboggan_valids += 1;
        }
    }
    println!("sled     valid passwords: {}", sled_valids);
    println!("toboggan valid passwords: {}", toboggan_valids);
}
