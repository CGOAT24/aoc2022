use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn read_file(filename: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;

        //first split
        let mut i = 0;
        while i < line.len() {
            if line.chars().nth(i).unwrap() == ',' {
                break;
            }
            i += 1;
        }
        let (work1, mut work2) = line.split_at(i);

        //second split
        i = 0;
        while i < work1.len() {
            if work1.chars().nth(i).unwrap() == '-' {
                break;
            }
            i += 1;
        }
        let (str_lower1, str_upper1) = work1.split_at(i);

        let upper1 = parse(str_upper1);
        let lower1 = parse(str_lower1);

        i = 0;
        while i < work2.len() {
            if work2.chars().nth(i).unwrap() == '-' {
                break;
            }
            i += 1;
        }
        let (str_lower2, str_upper2) = work2.split_at(i);

        let upper2 = parse(str_upper2);
        let lower2 = parse(str_lower2);

        if analyze(lower1, upper1, lower2, upper2) {
            println!("[{}-{}] [{}-{}]", lower1, upper1, lower2, upper2);
            sum += 1;
        }
    }
    return Ok(sum);
}

fn analyze(lower1: u32, upper1: u32, lower2: u32, upper2: u32) -> bool {
    if upper1 >= lower2 && lower1 <= upper2 {
        return true;
    }

    if upper2 >= lower1 && lower2 <= upper1 {
        return true;
    }
    return false
}

fn parse(val: &str) -> u32 {
    let mut chars = val.chars();
    let first = chars.clone().nth(0).unwrap();
    if first == ',' || first == '-' {
        chars.next();
    }
    chars.as_str().parse::<u32>().unwrap()
}

fn main() {
    let res = read_file("input.txt").unwrap();
    println!("{}", res);
}
