use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use grid::Grid;

#[derive(PartialEq, Copy, Clone)]
enum Command {
    ADDX,
    NOOP
}

struct Operation {
    command: Command,
    parameter: i32,
    cycles: u32
}

impl Clone for Operation {
    fn clone(&self) -> Self {
        Operation {
            command: self.command,
            parameter: self.parameter,
            cycles: self.cycles
        }
    }
}

struct Screen {
    grid: Grid<String>
}

impl Screen {
    fn new() -> Self {
        Default::default()
    }

    fn lit_pixel(&mut self, x: usize, y: usize) {
        self.grid[x][y] = "██".to_string();
    }

    fn row_count(&self) -> usize {
        self.grid.rows()
    }

    fn col_count(&self) -> usize {
        self.grid.cols()
    }

    fn print(&self) {
        for i in 0..self.row_count() {
            for j in 0..self.col_count() {
                print!("{}", self.grid[i][j]);
            }
            println!();
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        let mut grid = Grid::new(6, 40);
        for i in 0..grid.rows() {
            for j in 0..grid.cols() {
                grid[i][j] = "░░".to_string();
            }
        }
        Screen { grid }
    }
}

fn read_file(filename: &str) -> Result<Vec<Operation>, Box<dyn Error>>{
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    let mut vec = Vec::new();
    for line in reader.lines() {
        let line = line?;

        let res: Vec<&str> = line.split(" ").collect();
        let command = match res[0] {
            "addx" => Command::ADDX,
            "noop" => Command::NOOP,
            _ => panic!("unexpected value")
        };

        if res.len() > 1 {
            let parameter = res[1].parse::<i32>().unwrap();
            vec.push(Operation { command, parameter, cycles: 2 });
        }
        else {
            vec.push(Operation { command, parameter: 0, cycles: 1 });
        }
    }
    Ok(vec)
}

fn part_1(mut vec: Vec<Operation>) -> i32 {
    let mut reg_x: i32 = 1;
    let mut signal_strength: Vec<i32> = Vec::new();
    let mut cycle = 0;

    for elem in vec.iter_mut() {
        while elem.cycles > 0 {
            elem.cycles -= 1;
            cycle += 1;

            if cycle % 40 - 20 == 0 {
                signal_strength.push(reg_x * cycle);
            }
        }

        if elem.command == Command::ADDX {
            reg_x += elem.parameter;
        }
    }

    signal_strength.iter().sum()
}

fn part_2(vec: Vec<Operation>) {
    let mut screen = Screen::new();
    let mut pixel: i32 = 0;
    let mut reg_x: i32 = 1;
    let mut cycle = 0;
    let mut col = 0;
    let mut row = 0;

    for elem in vec {
        for _ in 0..elem.cycles {
            row = cycle / 40;
            if (pixel - reg_x).abs() <= 1 {
                screen.lit_pixel(row, col);
            }

            pixel += 1;
            pixel = pixel % 40;
            cycle += 1;
            col = if col >= 39 { 0 } else { col + 1 };

        }
        reg_x += elem.parameter;
    }
    screen.print();
}


fn main() {
    let vec = read_file("input.txt").unwrap();
    println!("part1: {}", part_1(vec.clone()));
    println!("part2:");
    part_2(vec.clone());
}
