use std::collections::HashMap;
use std::fs::File;
use std::i32;
use std::io::Read;

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
        Ok(_) => {}
        Err(error) => eprintln!("Error reading file {}: {}", filename, error),
    }

    let result_puzzle_1 = process_puzzle_1(&contents);
    println!("Result for puzzle 1: {}", result_puzzle_1);

    let result_puzzle_2 = process_puzzle_2(&contents);
    println!("Result for puzzle 2: {}", result_puzzle_2);
}

fn process_puzzle_1(input: &str) -> String {
    let (rules, pages) = parse(input);

    let valid = pages
        .iter()
        .filter(|page| {
            let mut position_map: HashMap<i32, i32> = HashMap::new();

            page.iter().enumerate().for_each(|(index, &value)| {
                position_map.insert(value, index as i32);
            });

            for rule in rules.iter() {
                let before = rule[0];
                let after = rule[1];

                if position_map.contains_key(&before) && position_map.contains_key(&after) {
                    if position_map.get(&before).unwrap() > position_map.get(&after).unwrap() {
                        return false;
                    }
                }
            }
            return true;
        })
        .collect::<Vec<&Vec<i32>>>();

    let result = valid
        .iter()
        .map(|v| {
            let len = v.len();
            let middle = len / 2;
            v[middle]
        })
        .sum::<i32>();

    return result.to_string();
}

fn process_puzzle_2(input: &str) -> String {
    let (rules, pages) = parse(input);

    let invalid = pages
        .iter()
        .filter(|page| {
            let mut position_map: HashMap<i32, i32> = HashMap::new();

            page.iter().enumerate().for_each(|(index, &value)| {
                position_map.insert(value, index as i32);
            });

            for rule in rules.iter() {
                let before = rule[0];
                let after = rule[1];

                if position_map.contains_key(&before) && position_map.contains_key(&after) {
                    if position_map.get(&before).unwrap() > position_map.get(&after).unwrap() {
                        return true;
                    }
                }
            }
            return false;
        })
        .collect::<Vec<&Vec<i32>>>();

    let corrected = invalid.iter().map(|v| {

    });

    // println!("{:?}", corrected);

    // let result = corrected
    //     .map(|v| {
    //         let len = v.len();
    //         let middle = len / 2;
    //         v[middle]
    //     })
    //     .sum::<i32>();

    // return result.to_string();
    return String::new();
}

fn parse(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules = vec![];
    let mut pages = vec![];
    let mut is_rules = true;

    input.lines().for_each(|line| {
        if line.is_empty() {
            is_rules = false;
        } else if is_rules {
            rules.push(
                line.split("|")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        } else {
            pages.push(
                line.split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    });

    return (rules, pages);
}
