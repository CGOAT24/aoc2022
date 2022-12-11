use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

struct Elf {
    number: u32,
    calories: u32
}

fn read_file(filename: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut vec = Vec::from([0]);
    let mut position = 0;

    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            vec.push(0);
            position += 1;
        }
        else {
            vec[position] += line.parse::<u32>().unwrap();
        }
    }
    Ok(vec)
}

fn get_highest(vec: Vec<u32>) -> Elf {
    let mut elf = Elf { number: 0, calories: 0 };
    let mut i = 0;
    while i < vec.len() {
        if vec[i] > elf.calories {
            elf = Elf { number: i as u32, calories: vec[i] };
        }
        i += 1;
    }
    elf
}

fn main() {
    let mut vec = read_file("input.txt").unwrap();
    vec.sort();
    vec.reverse();
    let mut res = 0;
    for i in 0..3 {
        res += vec[i];
    }
    println!("{}", res);
}
