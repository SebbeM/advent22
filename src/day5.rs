use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, Split};

const POWER_LEVEL: i32 = 9001;

pub fn run() {
    let file = File::open("input/day5").expect("File not found");

    let mut stacks: Vec<Vec<char>> = parse_stacks(file);
    println!("Stacks after: {:?}", stacks);
    let mut tops = String::new();
    for stack in stacks {
        tops.push(*stack.last().expect("Could not get value"));
    }
    println!("Tops: {}", tops);
}

fn parse_stacks(file: File) -> Vec<Vec<char>> {
    let mut reader = BufReader::new(file).lines();
    let mut line = reader.next()
        .expect("Line could not be read").ok().unwrap_or_default();
    
    let mut no_of_stacks: usize = (line.len() + 1) / 4; // len() seems to be 1 short for some reason
    let mut stacks = vec![vec![]; no_of_stacks];
    println!("Line length: {}. Number of stacks: {}", line.len(), no_of_stacks);

    while line.contains("["){
        for stack in 0..no_of_stacks {
            let string_index = 1 + stack * 4;
            let content = line.chars().nth(string_index).unwrap_or_default();
            if content != ' ' {
                stacks[stack].insert(0, content);
            }
        }
        if let Some(Ok(next)) = reader.next() {
            line = next;
        } else { break }
    }
    println!("Stacks: {:?}", stacks);
    // Skip line with stack numbers and blank line
    line = reader.next().expect("No lines could be read").ok().unwrap_or_default();
    line = reader.next().expect("No lines could be read").ok().unwrap_or_default();

    while line.contains("move"){
        let args = line.split(" ").collect::<Vec<&str>>();
        let mut amount: usize = args[1].parse().unwrap();
        let from: usize = args[3].parse().unwrap();
        let to: usize = args[5].parse().unwrap();

        if POWER_LEVEL > 9000 {
            amount = &stacks[from -1].len() - amount;
            let mut items = stacks[from - 1].split_off(amount);
            println!("Moving {:?}", items);
            stacks[to - 1].append(&mut items);
        } else {
            for _ in 0..amount {
                let item = stacks[from - 1].pop().expect("No item found");
                stacks[to - 1].push(item);
            }
        }
        if let Some(Ok(next)) = reader.next() {
            line = next;
        } else { break }
    }

    stacks
}