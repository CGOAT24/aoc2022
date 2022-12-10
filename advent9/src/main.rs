use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io;
use std::f32;
use std::hash::Hash;
use std::io::BufRead;

#[derive(Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Clone)]
struct Movement {
    direction: Direction,
    length: i32
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

impl Position {
    fn update(&mut self, dir: Direction) {
        match dir {
            Direction::UP => self.y += 1,
            Direction::DOWN => self.y -= 1,
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1
        }
    }

    fn distance_with(&self, pos: Position) -> f32 {
        let x1 = self.x;
        let x2 = pos.x;
        let y1 = self.y;
        let y2 = pos.y;

        let mut x: f32 = (x2 - x1).abs() as f32;
        x *= x;

        let mut y: f32 = (y2 - y1).abs() as f32;
        y *= y;

        f32::sqrt(x + y)
    }

    fn adjust_to_head(&mut self, head: Position) {
        if self.distance_with(head.clone()) > f32::sqrt(2.0) {
            self.x += 1 * (head.x - self.x).signum();
            self.y += 1 * (head.y - self.y).signum();
        }
    }
}

fn read_file(filename: &str) -> Result<Vec<Movement>, Box<dyn Error>> {
    let mut vec = Vec::new();
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let res: Vec<&str> = line.split(" ").collect();
        let length = res[1].parse::<i32>().unwrap();
        let direction = match res[0] {
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => panic!("unexpected value")
        };

        vec.push(Movement { direction, length });
    }
    Ok(vec)
}

fn get_positions<const NUM_KNOTS: usize>(vec: Vec<Movement>) -> HashSet<Position> {
    let mut knots: [Position ; NUM_KNOTS] = [Default::default() ; NUM_KNOTS];
    let mut positions = HashSet::new();

    for mov in vec.iter() {
        for _ in 0..mov.length {
            knots[0].update(mov.direction.clone());

            for i in 1..knots.len() {
                knots[i].adjust_to_head(knots[i - 1].clone());
                positions.insert(knots[knots.len() - 1].clone());
            }
        }
    }
    positions
}

fn main() {
    let vec = read_file("input.txt").unwrap();
    println!("part1: {}", get_positions::<2>(vec.clone()).len());
    println!("part2: {}", get_positions::<10>(vec.clone()).len());
}