use std::fs;
use std::process;

fn main() {
    let raw = fs::read_to_string("input.txt").unwrap_or_else(|err| {
        println!("Failed to open file. {}", err);
        process::exit(1);
    } );

    let nums : Vec<i32> = raw.lines()
        .map(|x| x.parse().unwrap())
        .collect();
  
    let res : Vec<i32> = nums.iter()
        .map(|&x| nums.iter().map(move |&y| (x, y)))
        .flatten()
        .filter(|(x, y)| x+y == 2020)
        .map(|(x, y)| x*y)
        .collect();

    println!("x+y = 2020, x*y = {}", res.first().unwrap());
}
