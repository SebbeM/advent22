use std::fs::File;
use std::io::{BufRead, BufReader};

const PRIORITIES: &str = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn run() {
    let file = File::open("input/day3").expect("File not found");
    let reader = BufReader::new(file);
    let mut item_sum = 0;
    let mut badge_sum = 0;
    let mut line_buffer: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let halves = line.split_at(line.len() / 2);
        for char in halves.0.chars() {
            if halves.1.contains(char) {
                let priority = PRIORITIES.find(char).expect("No priority found for item");
                println!("Character {} is in both halves and its priority is: {}", char, priority);
                item_sum += priority;
                break // Exactly one item type may be in both halves
            }
        }
        line_buffer.push(line);
        println!("Line buffer is {} long", line_buffer.len());
        if line_buffer.len() < 3 { continue } // Wait for the buffer to fill before checking for badges

        for char in line_buffer[0].chars() {
            if line_buffer[1].contains(char) && line_buffer[2].contains(char) {
                let priority = PRIORITIES.find(char).expect("No priority found for badge");
                println!("Character {} is in all three rucksacks and its priority is: {}", char, priority);
                badge_sum += priority;
                break // Exactly one item may be in all three rucksacks
            }
        }
        line_buffer.clear();
    }
    println!("Item sum is: {}, badge sum is: {}", item_sum, badge_sum);
}