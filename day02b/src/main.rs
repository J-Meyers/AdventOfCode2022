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

// X lose, Y draw, Z win
fn score(line: String) -> u32 {
    let them = line.as_bytes()[0];
    let outcome = line.as_bytes()[2];
    let mut score: u32 = 0;
    match outcome {
        b'Y' => score += 3,
        b'Z' => score += 6,
        _ => ()
    }
    if outcome == b'Y' {
        score += 1 + (them - b'A') as u32;
    }
    match (them, outcome) {
        (b'A', b'X') => score += 3,
        (b'A', b'Z') => score += 2,
        (b'B', b'X') => score += 1,
        (b'B', b'Z') => score += 3,
        (b'C', b'X') => score += 2,
        (b'C', b'Z') => score += 1,
        _ => ()
    }
    score
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
