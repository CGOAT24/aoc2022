use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

struct Rucksack {
    compartment1: String,
    compartment2: String,
    both: String
}

fn read_file(filename: &str) -> Result<Vec<Rucksack>, Box<dyn Error>> {
    let mut vec = Vec::new();
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let (c1, c2) = line.split_at(line.len() / 2);
        let r = Rucksack {
            compartment1: c1.to_string(),
            compartment2: c2.to_string(),
            both: line.to_string()
        };
        vec.push(r);
    }
    Ok(vec)
}

fn verify_redundance(vec: Vec<Rucksack>) -> u32 {
    let mut sum: u32 = 0;
    /*
    for elem in vec {
        let mut i = 0;
        let size = elem.compartment1.len();
        while i < size as usize {
            let val = elem.compartment1.chars().nth(i).unwrap();
            if elem.compartment2.contains(val) {
                if val.is_uppercase() {
                    sum += 26;
                }
                sum += (val.to_lowercase().to_string().into_bytes()[0] - 96)  as u32;
                break;
            }
            i += 1;
        }
    }
    */

    let mut i = 0;
    while i < vec.len() {
        let mut j = 0;
        while j < vec[i].both.len() {
            let val = vec[i].both.chars().nth(j).unwrap();
            if vec[i + 1].both.contains(val) && vec[i + 2].both.contains(val) {
                if val.is_uppercase() {
                    sum += 26;
                }

                sum += (val.to_lowercase().to_string().into_bytes()[0] - 96) as u32;
                break;
            }
            j += 1;
        }
        i += 3;
    }
    sum
}

fn main() {
    let vec = read_file("input.txt").unwrap();
    println!("{}", verify_redundance(vec));
}
