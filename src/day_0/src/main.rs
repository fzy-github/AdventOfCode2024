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

   let sum: u32 = contents.lines()
   .map(|line| {
        let first_num = line.chars().find(|c| c.is_numeric());
        let last_num = line.chars().rev().find(|c| c.is_numeric());
        // println!("First: {:?}, Last: {:?}", first_num, last_num);
        match (first_num, last_num) {
            (Some(f), Some(l)) => {
                let combined = format!("{}{}", f, l);
                combined.parse::<u32>().unwrap_or(0)
            },
            _ => 0,
        }
    })
    .sum();

    println!("Sum: {}", sum);
}
