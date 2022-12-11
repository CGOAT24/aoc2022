use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Range;

fn parse_line(line: String) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for c in line.chars().into_iter() {
        vec.push(c.to_string().parse::<u8>().unwrap());
    }
    return vec;
}

fn read_file(filename: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut vec: Vec<Vec<u8>> = Vec::new();
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        vec.push(parse_line(line));
    }
    return Ok(vec);
}

fn is_visible(vec: Vec<u8>, index: usize) -> bool {
    if index == 0 || index == vec.len() - 1 {
        return true;
    }

    let val = vec[index];
    let mut visible_left = true;
    let mut visible_right = true;
    for i in 0..index {
        if vec[i] >= val && i != index {
            visible_left = false;
        }
    }

    for i in index..vec.len() {
        if vec[i] >= val && i != index {
            visible_right = false;
        }
    }
    return visible_left || visible_right;
}

fn get_column(vec: Vec<Vec<u8>>, index: usize) -> Vec<u8> {
    let mut column = Vec::new();

    for i in 0..vec.len() {
        column.push(vec[i][index]);
    }
    return column;
}

//part1
fn verify_visibility(trees: Vec<Vec<u8>>) -> u32 {
    let mut count = 0;

    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            let row = trees[i].clone();
            let column = get_column(trees.clone(), j);

            if i == 0 || j == 0 {
                count += 1;
            }
            else if is_visible(row, j) || is_visible(column, i) {
                count += 1;
            }
        }
    }
    return count;
}

fn get_score(trees: Vec<Vec<u8>>, i: usize, j: usize) -> u32 {
    let tree = trees[i][j];

    //left
    let mut l_score = 0;
    for counter in (0..j).rev() {
        l_score += 1;

        if trees[i][counter] >= tree {
            break;
        }
    }

    //right
    let mut r_score = 0;
    for counter in (j + 1)..trees[i].len() {
        r_score += 1;

        if trees[i][counter] >= tree {
            break;
        }
    }

    //top
    let mut t_score = 0;
    for counter in (0..i).rev() {
        t_score += 1;

        if trees[counter][j] >= tree {
            break;
        }
    }

    //bottom
    let mut b_score = 0;
    for counter in (i + 1)..trees[i].len() {
        b_score += 1;

        if trees[counter][j] >= tree {
            break;
        }
    }

    return l_score * r_score * t_score * b_score;

}

//part2
fn get_highest_scenic_score(trees: Vec<Vec<u8>>) -> u32 {
    let mut highest_score = 0;

    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            if i > 0 && j > 0 && i < 99 && j < 99 {
                let score = get_score(trees.clone(), i, j);
                if score > highest_score {
                    highest_score = score;
                }
            }
        }
    }
    return highest_score;
}

fn main() {
    let trees = read_file("input.txt").unwrap();
    let score = get_highest_scenic_score(trees);
    println!("{}", score);
}
