use std::fs::File;
use std::io::{Read};
use regex::Regex;

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
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let r: i32 = input.lines().map(|line| {
        re.find_iter(line).map(|m| m.as_str())
            .map(|m| {
                m.replace("mul(", "").replace(")", "")
            })
            .map(|m| {
                m.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            })
            .map(|v| v[0] * v[1])
            .sum::<i32>()
    }).sum();

    return r.to_string()
}


fn process_puzzle_2(input: &str) -> String {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();

    
    let mut enabled = true;
    let r: i32 = input.lines().map(|line| {
        re.find_iter(line).map(|m| m.as_str())
            .map(|m| {
                if m == "do()" {
                    enabled = true;
                } else if m == "don't()" {
                    enabled = false;
                }

                if enabled &&  m.starts_with("mul(") {
                    let numbers = m.replace("mul(", "").replace(")", "");
                    let v = numbers.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                    return v[0] * v[1];
                } else {
                    return 0;
                }
            })
            .sum::<i32>()
    }).sum();

    return r.to_string()
}