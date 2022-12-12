use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MaxCalorieFinder {
    max_elements: usize,
    heap: BinaryHeap<Reverse<u32>>,
}
impl MaxCalorieFinder {
    fn new(max_elements: usize) -> Self {
        MaxCalorieFinder {
            max_elements,
            heap: BinaryHeap::new(),
        }
    }
    fn add(&mut self, calorie: u32) {
        if self.heap.len() < self.max_elements {
            self.heap.push(Reverse(calorie));
        } else if let Some(Reverse(min)) = self.heap.peek() {
            if calorie > *min {
                self.heap.pop();
                self.heap.push(Reverse(calorie));
            }
        }
    }
    fn sum(&self) -> u32 {
        self.heap.iter().map(|Reverse(x)| x).sum()
    }
}

fn main() {
    let mut counter = MaxCalorieFinder::new(3);
    if let Ok(lines) = read_lines("./day1_calories.txt") {
        let mut curr_sum: u32 = 0;
        for line in lines {
            if let Ok(cals) = line {
                if cals.is_empty() {
                    counter.add(curr_sum);
                    curr_sum = 0;
                } else {
                    curr_sum += cals.parse::<u32>().unwrap();
                }
            }
        }
        counter.add(curr_sum);
        let sum = counter.sum();
        println!("Sum of max 3 is {sum}");
    } else {
        eprintln!("Failed to Open File!");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
