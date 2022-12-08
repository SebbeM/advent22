use std::fs::File;
use std::io::{BufRead, BufReader, Split};

pub fn run() {
    let file = File::open("input/day4").expect("File not found");
    let reader = BufReader::new(file);
    let mut contained_ranges = 0;
    let mut overlapping_ranges = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if ranges_contained(&line) { contained_ranges += 1 };
        if ranges_overlap(&line) {overlapping_ranges += 1};
    }
    println!("Number of fully contained ranges: {}", contained_ranges);
    println!("Number of overlapping ranges: {}", overlapping_ranges);
}

fn ranges_contained(line: &String) -> bool {
    let elf_pair: Vec<&str> = line.split(",").collect::<Vec<&str>>();

    let range_0 = get_range(elf_pair[0]);
    let range_1 = get_range(elf_pair[1]);

    if range_0.0 >= range_1.0 && range_0.1 <= range_1.1 {
        println!("Range {} is contained in range {}", elf_pair[0], elf_pair[1]);
        return true
    } else if range_0.0 <= range_1.0 && range_0.1 >= range_1.1 {
        println!("Range {} is contained in range {}", elf_pair[1], elf_pair[0]);
        return true
    }
    false
}

fn ranges_overlap(line: &String) -> bool {
    let elf_pair: Vec<&str> = line.split(",").collect::<Vec<&str>>();

    let range_0 = get_range(elf_pair[0]);
    let range_1 = get_range(elf_pair[1]);

    if (range_0.0 >= range_1.0 && range_0.0 <= range_1.1) ||
        (range_0.1 >= range_1.0 && range_0.1 <= range_1.1) ||
        (range_1.0 >= range_0.0 && range_1.0 <= range_0.1) ||
        (range_1.1 >= range_0.0 && range_1.1 <= range_0.1) {
        println!("Range {} overlaps with range {}", elf_pair[0], elf_pair[1]);
        return true
    }
    false
}

fn get_range(string: &str) -> (i8, i8) {
    let mut range: Vec<&str> = string.split("-").collect();
    let range: (i8, i8) = (range[0].parse().expect("First number could not be parsed"),
                           range[1].parse().expect("Second number could not be parsed"));
    range
}