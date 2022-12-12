use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn get_final_boxes() -> Option<Vec<i32>> {
    let lines = read_lines("./day4.txt")?;
    let mut i = 0;
    let queues = Vec::new();
    let first_line = lines.next();
    let box_width = 4;

    // Create a queue for each box
    // Find the number of boxes by dividing the length of the first line by the box width

    let num_boxes = first_line.len() / box_width;
    for i in 0..num_boxes {
        queues.push(VecDeque::new());
    }

    // second character of each box section is the id of the box

    // Loop through each line
    for line in lines {
        // Loop through each box
        for i in 0..num_boxes {
            // Get the id of the box
            let box_id = line[i * box_width + 1];
            // Get the queue for the box
            let queue = queues[i];
            // Add the id to the queue
            queue.push_back(box_id);
        }
    }

    println!("Pairs with full containment {num_contained}");
    Vec
}

fn main() {

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
