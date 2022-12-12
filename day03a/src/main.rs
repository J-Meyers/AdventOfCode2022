use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut larr: [bool; 53] = [false; 53];
    let mut rarr: [bool; 53] = [false; 53];
    let mut running_sum: usize = 0;
    if let Ok(lines) = read_lines("./day3.txt") {
        for line in lines {
            if let Ok(line_s) = line {
                // Fill the array with 0
                larr.fill(false);
                rarr.fill(false);

                let byte_line = line_s.as_bytes();
                let size = byte_line.len() / 2;
                for i in 0..size {
                    larr[priority(byte_line[i])] = true;
                }
                for i in size..2*size {
                    rarr[priority(byte_line[i])] = true;
                }
                for i in 1..53 {
                    if larr[i] && rarr[i] {
                        running_sum += i;
                    }
                }
            }
        }
        println!("Sum of overlapped priorities {running_sum}");
    } else {
        eprintln!("Failed to Open File!");
    }
}

fn priority(c: u8) -> usize {
    if c >= b'a' {
        (c - b'a' + 1) as usize
    } else {
        (c - b'A' + 27) as usize
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
