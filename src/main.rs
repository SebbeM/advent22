use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(file: File) -> [i32; 3] {
    let reader = BufReader::new(file);
    let mut total = 0;
    let mut max_list = [0; 3];

    for line in  reader.lines() {
        let line = line.unwrap();
        if let Ok(result) = line.parse::<i32>() {
            total += result;
            println!("Result: {}, Total: {}", result, total);
        } else {
            println!("No number");
            if (total > max_list[2]) {
                max_list = put_max(max_list, total);
            }
            total = 0;
        }
    }
    max_list
}

fn put_max(mut max_list: [i32; 3], new_val: i32) -> [i32; 3] {
    let mut new_list = max_list.clone();
    for (i, val) in max_list.iter_mut().enumerate() {
        println!("Index: {}, Value: {}, New Value: {}", i, val, new_val);
        if new_val > *val {
            new_list[i] = new_val;
            put_max(new_list, *val);
            break;
        }
    }
    new_list
}

fn main() {
    let file = File::open("input").expect("File not found");

    let res = read_file(file);

    println!("Max values: {:?}", res);
    println!("Sum: {}", res[0] + res[1] + res[2])
}
