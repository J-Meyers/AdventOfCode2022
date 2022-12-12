use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn main() {
    let mut max_val: u32= 0;
    if let Ok(lines) = read_lines("./day1_calories.txt") {
        let mut curr_sum: u32 = 0;
        for line in lines {
            if let Ok(cals) = line {
                if cals.is_empty() {
                    max_val = cmp::max(curr_sum, max_val);
                    curr_sum = 0;
                } else {
                    curr_sum += cals.parse::<u32>().unwrap();
                }
            }
        }
        max_val = cmp::max(curr_sum, max_val);
        println!("Max value is {max_val}");
    } else {
        eprintln!("Failed to Open File!");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
