use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let file = File::open("input/day4").expect("File not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
    }
}