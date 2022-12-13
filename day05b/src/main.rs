use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

struct CargoMover {
    boxes: Vec<VecDeque<char>>,
    num_boxes: usize,
    box_width: usize,
    parsing_boxes: bool,
    skip: bool,
}
impl CargoMover {
    fn new() -> CargoMover {
        CargoMover {
            boxes: Vec::new(),
            num_boxes: 0,
            box_width: 4,
            parsing_boxes: true,
            skip: true,
        }
    }

    // Main function
    fn compute_end_boxes(&mut self, filename: String) -> String {
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(line_s) = line {
                    if self.parsing_boxes {
                        self.parse_boxes(line_s);
                    } else {
                        self.parse_commands(line_s);
                    }
                }
            }
            self.compute_result()
        } else {
            "Error reading file".to_string()
        }
    }

    fn parse_boxes(&mut self, line_s: String) {
        if self.num_boxes == 0 {
            // ceiling of division of line_s.len / box_width is num_boxes
            self.num_boxes = (line_s.len() + self.box_width - 1) / self.box_width;
            for _ in 0..self.num_boxes {
                self.boxes.push(VecDeque::new());
            }
            assert!(self.num_boxes > 0);
        }
        // Check if still parsing boxes
        // Check if the character at position 1 is a digit
        let c = line_s.as_bytes()[1];
        if c >= b'0' && c <= b'9' {
            self.parsing_boxes = false;
        } else {
            // Parse the line
            // Loop through num_boxes
            // Multiply index * box_width, add 1 to get the id index of the box
            // if the index is not " ", then parse the capital letter, and add it to the
            //  back of the boxes queue
            for i in 0..self.num_boxes {
                let id_index = i * self.box_width + 1;
                if line_s.as_bytes()[id_index] != b' ' {
                    let letter = line_s.as_bytes()[id_index] as char;
                    self.boxes[i].push_back(letter);
                }
            }
        }
    }

    fn parse_commands(&mut self, line_s: String) {
        if self.skip {
            self.skip = false;
            return;
        }
        let split_line: Vec<&str> = line_s.split_whitespace().collect();
        let num_boxes_to_move = split_line[1].parse::<usize>().unwrap();
        let src_box = split_line[3].parse::<usize>().unwrap();
        let dst_box = split_line[5].parse::<usize>().unwrap();

        self.move_boxes(num_boxes_to_move, src_box, dst_box);
    }

    fn move_boxes(&mut self, num_boxes_to_move: usize, src_box: usize, dst_box: usize) {
        let mut temp_vec: VecDeque<char> = VecDeque::new();
        temp_vec.reserve(num_boxes_to_move);
        for _ in 0..num_boxes_to_move {
            let letter = self.boxes[src_box - 1].pop_front().unwrap();
            temp_vec.push_front(letter);
        }
        for letter in temp_vec {
            self.boxes[dst_box - 1].push_front(letter);
        }
    }

    fn compute_result(&self) -> String {
        let mut res = String::new();
        // Result is the top letter of each box
        for box_ in &self.boxes {
            res.push(box_.front().unwrap().clone());
        }
        res
    }
}

fn main() {
    let mut cargo_mover = CargoMover::new();
    let res = cargo_mover.compute_end_boxes("./input.txt".to_string());
    println!("{}", res);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
