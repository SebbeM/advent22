use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let file = File::open("input/day2").expect("File not found");
    let reader = BufReader::new(file);

    for line in  reader.lines() {
        let line = line.unwrap();

        calculate_game(line);
    }
}

fn calculate_game(signs: String) {
    let signs = signs.split(' ').collect::<Vec<&str>>();
    let opponent = signs[0];
    let me = signs[1];

    println!("Opponent plays: {}, I play: {}", opponent, me);
}