use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::Play::{Paper, Rock, Scissors};

enum Play {
    Rock,
    Paper,
    Scissors
}

struct Round {
    opponent: Play,
    result: Play
}

fn ascii_to_play(ascii: u8) -> Play {
    if ascii == 65 || ascii == 88 {
        return Rock;
    }
    else if ascii == 66 || ascii == 89 {
        return Paper;
    }
    return Scissors;
}

fn read_file(filename: &str) -> Result<Vec<Round>, Box<dyn Error>> {
    let mut vec: Vec<Round> = Vec::new();

    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let opponent = ascii_to_play(line.to_string().as_bytes()[0]);
        let user = ascii_to_play(line.to_string().as_bytes()[2]);

        vec.push(Round { opponent, result: user });
    }
    return Ok(vec);
}

fn get_points(vec: Vec<Round>) -> u32 {
    let mut sum = 0;

    let mut i: usize = 0;
    while i < vec.len() {
        let round = vec.get(i).unwrap();

        /*
            ROCK        (1pt)   -> LOSE
            PAPER       (2pts)  -> DRAW
            SCISSORS    (3pts)  -> WIN
        */

        match round.opponent {
            Rock => {
                match round.result {
                    Rock => sum += 3,       //lose (0 + 3)
                    Scissors => sum += 8,   //win (6 + 2)
                    Paper => sum += 4       //draw (3 + 1)
                }
            },
            Paper => {
                match round.result {
                    Paper => sum += 5,      //draw (3 + 2)
                    Rock => sum += 1,       //lose (0 + 1)
                    Scissors => sum += 9    //win (6 + 3)
                }
            },
            Scissors => {
                match round.result {
                    Scissors => sum += 7,   //win (6 + 1)
                    Paper => sum += 6,      //draw (3 + 3)
                    Rock => sum += 2        //lose (0 + 2)
                }
            }
        }
        i += 1;
    }
    sum
}

fn main() {
    let vec = read_file("input.txt").unwrap();
    println!("{}", get_points(vec));
}
