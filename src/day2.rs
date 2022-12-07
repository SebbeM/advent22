use std::fs::File;
use std::io::{BufRead, BufReader};
use phf::{phf_map};

// Matrix as [me][opponent]
const RESULT_MATRIX: [[i32; 3]; 3] = [[3, 0, 6],
                                    [6, 3, 0],
                                    [0, 6, 3]];

const SIGN_MAP: phf::Map<&str, i32> = phf_map![
    "A" => 0, "X" => 0,
    "B" => 1, "Y" => 1,
    "C" => 2, "Z" => 2];

// Matrix as [me][opponent]
const STRATEGY_MATRIX: [[i32; 3]; 3] = [   [2, 0, 1],
                            [0, 1, 2],
                            [1, 2, 0]];

pub fn run() {
    let file = File::open("input/day2").expect("File not found");
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in  reader.lines() {
        let line = line.unwrap();

        result += calculate_game(line);
    }
    println!("Result: {}", result)
}

fn calculate_game(signs: String) -> i32 {
    let signs = signs.split(' ').collect::<Vec<&str>>();
    let opponent = SIGN_MAP.get(signs[0]).unwrap();
    let mut me = SIGN_MAP.get(signs[1]).unwrap();
    // Change sign according to strategy
    me = &STRATEGY_MATRIX[*me as usize][*opponent as usize];

    println!("Opponent plays: {}({}), I play: {}({})", signs[0], opponent, signs[1], me);

    let mut score = *me + 1;
    score += RESULT_MATRIX[*me as usize][*opponent as usize];
    score
}