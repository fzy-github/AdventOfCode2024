use std::fs::File;
use std::io::{Read};

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

    let result = process_input(&contents);

    println!("Result: {}", result);
}


fn process_input(input: &str) -> String {

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