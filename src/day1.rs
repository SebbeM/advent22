use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let file = File::open("input").expect("File not found");

    let reader = BufReader::new(file);
    let mut total = 0;
    let mut sorted_list = vec![0, 3];

    for line in  reader.lines() {
        let line = line.unwrap();
        if let Ok(result) = line.parse::<i32>() {
            total += result;
            println!("Result: {}, Total: {}", result, total);
        } else {
            println!("No number");
            for i in 0..sorted_list.len() {
                if total > sorted_list[i] {
                    sorted_list.insert(i, total);
                    sorted_list.truncate(3); // Keep list length at 3 to reduce run time
                    break
                }
            }
            total = 0;
        }
    }

    println!("Max values: {:?}", sorted_list);
    println!("Sum: {}", sorted_list[0] + sorted_list[1] + sorted_list[2])
}