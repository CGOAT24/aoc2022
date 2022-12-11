use std::collections::LinkedList;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;
use regex::Regex;

struct Operation {
    quantity: u32,
    source: u32,
    destination: u32
}

fn read_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.len() > 0 {
            vec.push(line);
        }
    }
    Ok(vec)
}

fn parse_stack(vec: Vec<String>) -> Vec<LinkedList<String>> {
    let mut stack: Vec<LinkedList<String>> = std::vec::from_elem(LinkedList::new(), 9);

    let mut i: isize = 7;
    while i >= 0 {
        let mut index = 0;

        while index < 9 {
            let cell: &str = vec[i as usize].substring(index * 4, (index * 4) + 3);
            if cell.to_string().trim().len() > 0 {
                stack[index].push_front(cell.trim().to_string());
            }
            index += 1;
        }
        i -= 1;
    }
    stack
}

fn parse_operations(vec: Vec<String>) -> Vec<Operation> {
    let mut res: Vec<Operation> = Vec::new();

    let mut i = 9;
    while i < vec.len() {
        let line = vec.get(i).unwrap().to_string();
        let reg = Regex::new(r"(\d+)").unwrap();
        let mut reg_res = Vec::new();
        for cap in reg.captures_iter(&line) {
            let val = &cap[1];
            reg_res.push(val.to_string());

        }

        let op = Operation {
            quantity: reg_res[0].parse::<u32>().unwrap(),
            source: reg_res[1].parse::<u32>().unwrap(),
            destination: reg_res[2].parse::<u32>().unwrap(),
        };
        res.push(op);
        i += 1;
    }
    res
}

fn operate(mut stack: Vec<LinkedList<String>>, operations: Vec<Operation>) -> Vec<LinkedList<String>> {
    let mut i = 0;
    while i < operations.len() {
        let mut j = 0;
        let mut temp = LinkedList::new();
        while j < operations[i].quantity {
            let val = stack[(operations[i].source - 1) as usize].pop_front().unwrap();
            temp.push_back(val);
            //stack[(operations[i].destination - 1) as usize].push_front(val);
            j += 1;
        }

        while !temp.is_empty() {
            let val = temp.pop_back().unwrap();
            stack[(operations[i].destination - 1) as usize].push_front(val);
        }


        i += 1;
    }

    stack
}

fn print_msg(mut stack: Vec<LinkedList<String>>) {
    let mut i = 0;
    while i < stack.len() {
        print!("{}", stack[i].pop_front().unwrap());
        i += 1;
    }
}

fn main() {
    let res = read_file("input.txt").unwrap();
    let stack = parse_stack(res.clone());
    let operations = parse_operations(res.clone());
    let new_stack = operate(stack.clone(), operations);
    print_msg(new_stack);
}
