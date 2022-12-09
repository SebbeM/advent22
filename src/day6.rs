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
    let mut char_buf = Vec::new();
    let mut chars = string.chars();
    let marker_len = 14;
    for i in 0..string.len() {
        let char = chars.next().expect("Could not read char");
        char_buf.insert(0, char);
        char_buf.truncate(marker_len);
        println!("Char buffer: {:?}", char_buf);
        let mut has_double = false;
        for pick in &char_buf {
            let mut count = 0;
                for comp in &char_buf {
                if pick == comp {
                    count += 1;
                }
            }
            if count > 1 {
                has_double = true;
                break
            }
        }
        if !has_double && char_buf.len() == marker_len {
            return Some(i + 1);
        }
    }
    None
}