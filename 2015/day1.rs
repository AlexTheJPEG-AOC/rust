use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file = match File::open("input1.txt") {
        Ok(file) => file,
        Err(_) => panic!("Failed to read input file."),
    };
    let mut buffer = BufReader::new(file);
    let mut instructions = String::new();
    buffer.read_line(&mut instructions).expect("Failed to read line");

    let mut floor: i32 = 0;
    let mut basement_index: i32 = 0;

    for (i, c) in instructions.chars().enumerate() {
        if c == '(' { floor += 1; }
        else { floor -= 1; }

        if basement_index == 0 && floor == -1 {
            basement_index = (i as i32) + 1;
        }
    }

    println!("{}\n{}", floor, basement_index);
}
