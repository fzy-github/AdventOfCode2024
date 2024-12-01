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
    return String::new();
}