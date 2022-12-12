use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut arrs = Vec::new();
    let group_size = 3;
    for _ in 0..group_size {
        arrs.push([false; 53]);
    }

    let mut running_sum: usize = 0;
    if let Ok(lines) = read_lines("./day3.txt") {
        let mut j = 0;
        for line in lines {
            if let Ok(line_s) = line {
                let byte_line = line_s.as_bytes();
                for i in 0..byte_line.len() {
                    arrs[j][priority(byte_line[i])] = true;
                }
                j += 1;
                if j == 3 {
                    j = 0;
                    for i in 1..53 {
                        if arrs[0][i] && arrs[1][i] && arrs[2][i] {
                            running_sum += i;
                        }
                    }
                    for arr in arrs.iter_mut() {
                        arr.fill(false);
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
