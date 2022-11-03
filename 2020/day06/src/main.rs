use std::collections::HashSet;
use std::fs;

fn answer_union(group: String) -> HashSet<char> {
    let mut set = HashSet::new();
    for line in group.split("\n") {
        set.extend(&HashSet::<char>::from_iter(line.chars()));
    }
    set
}

fn answer_inters(group: String) -> HashSet<char> {
    let mut set = HashSet::new();
    for line in group.split("\n") {
        let subset = HashSet::<char>::from_iter(line.chars());
        if set.len() == 0 {
            set.extend(&subset);
        } else {
            set = set.into_iter().filter(|e| subset.contains(e)).collect();
            if set.len() == 0 {
                break;
            }
        }
    }
    set
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut sum_union = 0;
    let mut sum_inters = 0;
    for group in file.split("\n\n") {
        sum_union += answer_union(group.to_string()).len();
        sum_inters += answer_inters(group.to_string()).len();
    }
    println!("The sum of the groups questions is: {}", sum_union);
    println!("The sum of the groups consensus is: {}", sum_inters);
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn set_test() {
        let mut set = HashSet::new();
        set.insert('a');
        set.insert('a');
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn simple_union() {
        let file = fs::read_to_string("example.txt").unwrap();
        let mut sum = 0;
        for group in file.split("\n\n") {
            sum += answer_union(group.to_string()).len();
        }
        assert_eq!(sum, 11);
    }

    #[test]
    fn simple_inters() {
        let file = fs::read_to_string("example.txt").unwrap();
        let mut sum = 0;
        for group in file.split("\n\n") {
            sum += answer_inters(group.to_string()).len();
        }
        assert_eq!(sum, 6);
    }
}
