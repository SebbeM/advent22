use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(file: File) -> Vec<i32> {
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
    sorted_list
}

fn main() {
    let file = File::open("input").expect("File not found");

    let res = read_file(file);

    println!("Max values: {:?}", res);
    println!("Sum: {}", res[0] + res[1] + res[2])
}
