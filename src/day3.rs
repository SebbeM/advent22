use std::fs::File;
use std::io::{BufRead, BufReader};

const PRIORITIES: &str = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn run() {
    let file = File::open("input/day3").expect("File not found");
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in  reader.lines() {
        let line = line.unwrap();
        let halves = line.split_at(line.len() / 2);

        for char in halves.0.chars() {
            if halves.1.contains(char) {
                let priority = PRIORITIES.find(char).expect("No priority found");
                println!("Character {} is in both halves and its priority is: {}", char, priority);
                sum += priority;
                break // Exactly one item type may be in both halves
            }
        }
    }
    println!("Sum is: {}", sum);
}