use std::fs::File;
use std::io::{Read};
use std::iter::zip;

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

    let safe_count: i32 = input.lines().map(|line| {
        let parts = line.split_whitespace().map(|s| s.parse::<i32>());
        let numbers = parts.map(|r| r.unwrap()).collect::<Vec<i32>>();
        let mut isIncreasing: Option<bool> = None;
        for (l, r) in zip(numbers.iter(), numbers.iter().skip(1)) {
            let dist = l - r;
            
            if isIncreasing.is_none() {
                isIncreasing = Some(dist > 0);
            }

            let isIncreasing = isIncreasing.unwrap();
            if (isIncreasing && dist < 0) || (!isIncreasing && dist > 0) {
                return 0;
            }

            if dist == 0  || dist.abs() > 3 {
                return 0;
            }

        }
        return 1;
    })
    .sum();

    return safe_count.to_string();
}


fn process_puzzle_2(input: &str) -> String {

    fn is_safe(numbers: &Vec<i32>) -> bool {
        let mut isIncreasing: Option<bool> = None;
        for (l, r) in zip(numbers.iter(), numbers.iter().skip(1)) {
            let dist = l - r;
            
            if isIncreasing.is_none() {
                isIncreasing = Some(dist > 0);
            }

            let isIncreasing = isIncreasing.unwrap();
            if (isIncreasing && dist < 0) || (!isIncreasing && dist > 0) {
                return false;
            }

            if dist == 0  || dist.abs() > 3 {
                return false;
            }

        }
        return true;
    }

    let safe_count: i32 = input.lines().map(|line| {
        let parts = line.split_whitespace().map(|s| s.parse::<i32>());
        let numbers = parts.map(|r| r.unwrap()).collect::<Vec<i32>>();

        if is_safe(&numbers) {
            return 1;
        }

        for i in 0..numbers.len() {
            let mut new_numbers = numbers.clone();
            new_numbers.remove(i);
            if is_safe(&new_numbers) {
                return 1;
            }
        }
        return 0;

    })
    .sum();

    return safe_count.to_string();
}