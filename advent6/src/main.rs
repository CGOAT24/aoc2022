use std::fs;
use std::collections::HashSet;
use substring::Substring;
use std::env;
use chrono::{DateTime, Duration, Local};

fn read_file(filename: &str) -> std::io::Result<String> {
    return fs::read_to_string(filename);
}

fn is_unique(text: &str) -> bool {
    let mut set = HashSet::new();
    for i in 0..text.len() {
        set.insert(text.substring(i, i + 1));
    }
    return set.len() == text.len();
}

fn find_unique(text: String, length: u32) -> u32 {
    for i in 0..text.len() {
        if is_unique(text.substring(i, i + length as usize)) {
            return i as u32 + length;
        }
    }
    return 0;
}

fn main() {
    let start: DateTime<Local> = Local::now();

    let mut length: u32 = 14;
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        length = args[1].parse::<u32>().unwrap();
    }

    let file: String = read_file("input.txt").unwrap();
    let pos: u32 = find_unique(file, length);
    println!("{}", pos);
    let end: DateTime<Local> = Local::now();
    let time: Duration = end - start;
    println!("Time elapsed: {}ms", time.num_milliseconds());
}