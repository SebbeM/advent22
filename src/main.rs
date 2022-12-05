use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(file: File) -> i32 {
    let reader = BufReader::new(file);
    let mut total = 0;
    let mut max = 0;

    for line in  reader.lines() {
        let line = line.unwrap();
        if let Ok(result) = line.parse::<i32>() {
            total = total + result;
            println!("Result: {}, Total: {}", result, total);
        } else {
            println!("No number");
            if total > max { max = total }
            total = 0;
        }
    }
    max
}

fn main() {
    let file = File::open("input").expect("File not found");

    let res = read_file(file);

    println!("Max value: {}", res);
}
