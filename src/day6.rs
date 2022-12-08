use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Read, Seek, Split};

pub fn run() {
    let string: String = fs::read_to_string("input/day6")
        .expect("File could not be read");
    let pos = find_start(string).expect("No start was found");

    println!("Start position: {}", pos);
}

fn find_start(mut string: String) -> Option<usize> {
    let mut char_buf = vec!['c', 'c', 'c', 'c'];
    let mut chars = string.chars();
    for i in 0..string.len() {
        let char = chars.next().expect("Could not read char");
        char_buf.push(char);
        char_buf.truncate(4);
        print!("{}", char);
        return Some(i);
    }
    None
}