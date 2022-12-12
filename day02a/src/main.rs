use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut running_score: u32 = 0;
    if let Ok(lines) = read_lines("./day2.txt") {
        for line in lines {
            if let Ok(line_s) = line {
                running_score += score(line_s);
            }
        }
        println!("My score would be {running_score}");
    } else {
        eprintln!("Failed to Open File!");
    }
}

fn score(line: String) -> u32 {
    let them = line.as_bytes()[0];
    let me = line.as_bytes()[2];
    let mut score: u32 = (me - b'W') as u32;
    if them - b'A' == me - b'X' {
        score += 3;
    } else {
        match (them, me) {
            (b'A', b'Y') | (b'B', b'Z') |(b'C', b'X') => score += 6,
            _ => (),
        }
    }
    score
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
