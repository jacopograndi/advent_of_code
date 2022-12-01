use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let elves: Vec<String> = input
        .split("\n\n")
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect();
    let mut sums: Vec<i64> = elves
        .iter()
        .map(|e| {
            e.split("\n")
                .filter(|x| !x.is_empty())
                .map(|c| c.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .collect();
    println!("The max elf is carrying {}", sums.iter().max().unwrap());
    sums.sort();
    let mut iter = sums.iter().rev();
    println!(
        "The max 3 elfs are carrying {}",
        iter.next().unwrap() + iter.next().unwrap() + iter.next().unwrap()
    );
}
