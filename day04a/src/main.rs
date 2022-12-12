use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut num_contained: usize = 0;
    if let Ok(lines) = read_lines("./day4.txt") {
        for line in lines {
            if let Ok(line_s) = line {
                // Input in form 2-4,6-8
                // Another example 256-1298,980-1234
                // Want lhs = (2,4) and rhs = (6,8)
                // Want lhs = (256,1298) and rhs = (980,1234)
                let mut lhs: (usize, usize) = (0, 0);
                let mut rhs: (usize, usize) = (0, 0);
                // Can assume line is ascii
                let mut i = 0;
                {
                    // Lambda function which takes a stop char, increments i and returns a single usize

                    let mut get_num = |stop_char| {
                        let mut num = 0;
                        while line_s.as_bytes()[i] != stop_char {
                            num = num * 10 + (line_s.as_bytes()[i] - b'0') as usize;
                            i += 1;
                        }
                        i += 1;
                        num
                    };

                    // Parse lhs
                    lhs.0 = get_num(b'-');
                    lhs.1 = get_num(b',');
                    // Parse rhs
                    rhs.0 = get_num(b'-');
                }
                // Specialization of the above for the last number with no stop char
                let mut get_last_num = || {
                    let mut num = 0;
                    while i < line_s.len() {
                        num = num * 10 + (line_s.as_bytes()[i] - b'0') as usize;
                        i += 1;
                    }
                    num
                };

                rhs.1 = get_last_num();

                if contains(lhs, rhs) {
                    num_contained += 1;
                }
            }
        }
        println!("Pairs with full containment {num_contained}");
    } else {
        eprintln!("Failed to Open File!");
    }
}

fn contains(lhs: (usize, usize), rhs: (usize, usize)) -> bool {
    if lhs.0 <= rhs.0 && lhs.1 >= rhs.1 {
        true
    } else if rhs.0 <= lhs.0 && rhs.1 >= lhs.1 {
        true
    } else {
        false
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
