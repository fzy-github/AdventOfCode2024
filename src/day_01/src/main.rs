use std::fs::File;
use std::io::{Read};
use std::collections::HashMap;

fn main() {
    // read file into memory
    let filename = "input/input.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(error) => eprintln!("Error reading file {}: {}", filename, error),
    }

    let result_puzzle_1 = process_puzzle_1(&contents);
    println!("Result for puzzle 1: {}", result_puzzle_1);

    let result_puzzle_2 = process_puzzle_2(&contents);
    println!("Result for puzzle 2: {}", result_puzzle_2);
}


fn process_puzzle_1(input: &str) -> String {

    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();

    input.lines().for_each(|line| {
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>());
        match (nums.next(), nums.next()) {
            (Some(l), Some(r)) => {
                left.push(l.unwrap());
                right.push(r.unwrap());
            },
            _ => eprintln!("Error parsing line: {}", line),
        }
    });

    left.sort();
    right.sort();

    let result: i32 = std::iter::zip(left.iter(), right.iter()).map(|(l, r)| {
        let dist = l - r;
        return dist.abs();
    })
    .sum();

    return result.to_string();
}

fn process_puzzle_2(input: &str) -> String {

    let mut left = HashMap::<i32, i32>::new();
    let mut right = Vec::<i32>::new();

    input.lines().for_each(|line| {
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>());
        match (nums.next(), nums.next()) {
            (Some(l), Some(r)) => {
                left.insert(l.unwrap(), 0);
                right.push(r.unwrap());
            },
            _ => eprintln!("Error parsing line: {}", line),
        }
    });

    right.iter().for_each(|r| {
        left.entry(*r).and_modify(|e| *e += 1);
    });

    let result: i32 = left.iter().map(|(k, v)| {
        k * v
    })
    .sum();

    return result.to_string();
}