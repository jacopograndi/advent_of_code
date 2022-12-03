use std::fs;

fn parse_sacks(s: &str) -> Vec<String> {
    s.split("\n")
        .map(|l| l.trim().trim_end_matches("\n").to_string())
        .filter(|x| !x.is_empty())
        .collect()
}

fn group_triple(sacks: &Vec<String>) -> Vec<Vec<String>> {
    let mut groups = vec![];
    assert_eq!(sacks.len() % 3, 0);
    for i in (0..sacks.len()).step_by(3) {
        groups.push(vec![
            sacks[i].clone(),
            sacks[i + 1].clone(),
            sacks[i + 2].clone(),
        ]);
    }
    groups
}

fn group_pockets(sacks: &Vec<String>) -> Vec<Vec<String>> {
    sacks
        .iter()
        .map(|s| {
            let (l, r) = s.split_at(s.len() / 2);
            vec![l.to_string(), r.to_string()]
        })
        .collect()
}

fn score_item(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn groups_sum(sacks: Vec<Vec<String>>) -> u32 {
    sacks
        .iter()
        .map(|parts| {
            parts[0]
                .chars()
                .find(|c| parts.iter().all(|part| part.contains(*c)))
                .unwrap()
        })
        .map(score_item)
        .sum::<u32>()
}

fn main() {
    let sacks = parse_sacks(&fs::read_to_string("input.txt").unwrap());
    let sum = groups_sum(group_pockets(&sacks));
    println!("sacks sum: {}", sum);
    let sum = groups_sum(group_triple(&sacks));
    println!("triple sum: {}", sum);
}

#[cfg(test)]
mod test {
    use crate::*;

    const EXAMPLE_0: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn example() {
        let sacks = parse_sacks(EXAMPLE_0);
        let sum = groups_sum(group_pockets(&sacks));
        assert_eq!(sum, 157);
    }
    #[test]
    fn example_groups() {
        let sacks = parse_sacks(EXAMPLE_0);
        let sum = groups_sum(group_triple(&sacks));
        assert_eq!(sum, 70);
    }
}
