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
    
    let nx = nums.clone().into_iter();
    let ny = nums.clone().into_iter();
    let nz = nums.clone().into_iter();
    let cube = nx.flat_map(|x| {
        ny.clone().flat_map({
            let nz = &nz;
            move |y| nz.clone().map(move |z| (x, y, z))
        })
    });
        
    let res : Vec<i32> = cube
        .filter(|(x, y, z)| x+y+z == 2020)
        .map(|(x, y, z)| x*y*z)
        .collect();

    println!("x+y+z = 2020, x*y*z = {}", res.first().unwrap());
}
