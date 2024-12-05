use std::fs::File;
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
        Ok(_) => {},
        Err(error) => eprintln!("Error reading file {}: {}", filename, error),
    }

    let result_puzzle_1 = process_puzzle_1(&contents);
    println!("Result for puzzle 1: {}", result_puzzle_1);

    let result_puzzle_2 = process_puzzle_2(&contents);
    println!("Result for puzzle 2: {}", result_puzzle_2);
}

#[derive(Debug)]
struct Element {
    char: char,
    x: usize,
    y: usize
}

fn process_puzzle_1(input: &str) -> String {
    let grid = create_grid(input);
    // println!("{:?}", grid);

   let result: i32 = grid.iter().map(|row|{
    row.iter().map(|e| {
        check_xmass(e, &grid)
    })
    .sum::<i32>()
   })
   .sum();

    return result.to_string();
}

fn create_grid(input: &str) -> Vec<Vec<Element>> {
    let mut elements: Vec<Vec<Element>> = Vec::new();
    
    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<Element> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(Element { char: c, x: x, y: y });
        }
        elements.push(row);
    }
    elements
}

fn check_xmass(element: &Element, grid: &Vec<Vec<Element>>) -> i32 {
    let grid_height = grid.len().try_into().unwrap();
    let grid_width = grid[0].len().try_into().unwrap();

    let mut num_xmass = 0;

    // check north
    if element.y as i32 - 3 >= 0 {
        let mut text = String::new();
        text.push(grid[element.y][element.x].char);
        text.push(grid[element.y - 1][element.x].char);
        text.push(grid[element.y - 2][element.x].char);
        text.push(grid[element.y - 3][element.x].char);

        if text == "XMAS" {
            num_xmass += 1;
        }
    }

    // check north-east
    if element.y as i32 - 3 >= 0 && element.x + 3 < grid_width {
        let mut text = String::new();
        text.push(grid[element.y][element.x].char);
        text.push(grid[element.y - 1][element.x + 1].char);
        text.push(grid[element.y - 2][element.x + 2].char);
        text.push(grid[element.y - 3][element.x + 3].char);

        if text == "XMAS" {
            num_xmass += 1;
        }
    }

    // check east
    if element.x + 3 < grid_width {
        let mut text = String::new();
        text.push(grid[element.y][element.x].char);
        text.push(grid[element.y][element.x + 1].char);
        text.push(grid[element.y][element.x + 2].char);
        text.push(grid[element.y][element.x + 3].char);

        if text == "XMAS" {
            num_xmass += 1;
        }
    }

    // check south-east
    if element.y + 3 < grid_height && element.x + 3 < grid_width {
        let mut text = String::new();
        text.push(grid[element.y][element.x].char);
        text.push(grid[element.y + 1][element.x + 1].char);
        text.push(grid[element.y + 2][element.x + 2].char);
        text.push(grid[element.y + 3][element.x + 3].char);

        if text == "XMAS" {
            num_xmass += 1;
        }
    }

    // check south
    if element.y + 3 < grid_height {
        let mut text = String::new();
        text.push(grid[element.y][element.x].char);
        text.push(grid[element.y + 1][element.x].char);
        text.push(grid[element.y + 2][element.x].char);
        text.push(grid[element.y + 3][element.x].char);

        if text == "XMAS" {
            num_xmass += 1;
        }
    }

    // check south-west
    if element.y + 3 < grid_height && element.x as i32 - 3 >= 0 {
        let mut text = String::new();
        text.push(grid[element.y][element.x].char);
        text.push(grid[element.y + 1][element.x - 1].char);
        text.push(grid[element.y + 2][element.x - 2].char);
        text.push(grid[element.y + 3][element.x - 3].char);

        if text == "XMAS" {
            num_xmass += 1;
        }
    }

    // check west
    if element.x as i32 - 3 >= 0 {
        let mut text = String::new();
        text.push(grid[element.y][element.x].char);
        text.push(grid[element.y][element.x - 1].char);
        text.push(grid[element.y][element.x - 2].char);
        text.push(grid[element.y][element.x - 3].char);

        if text == "XMAS" {
            num_xmass += 1;
        }
    }

    // check north-west

    if element.y as i32 - 3 >= 0 && element.x as i32 - 3 >= 0 {
        let mut text = String::new();
        text.push(grid[element.y][element.x].char);
        text.push(grid[element.y - 1][element.x - 1].char);
        text.push(grid[element.y - 2][element.x - 2].char);
        text.push(grid[element.y - 3][element.x - 3].char);

        if text == "XMAS" {
            num_xmass += 1;
        }
    }

    num_xmass
}


fn process_puzzle_2(input: &str) -> String {

    let grid = create_grid(input);

    let result: i32 = grid.iter().map(|row|{
        row.iter().map(|e| {
            check_x_mas(e, &grid)
        })
        .sum::<i32>()
    })
    .sum();

    return result.to_string();
}

fn check_x_mas(element: &Element, grid: &Vec<Vec<Element>>) -> i32 {
    let grid_height = grid.len().try_into().unwrap();
    let grid_width = grid[0].len().try_into().unwrap();

    if element.char != 'A' {
        return 0;
    }

    if element.x as i32 - 1 < 0 || element.x + 1 >= grid_width || element.y as i32 - 1 < 0 || element.y + 1 >= grid_height {
        return 0;
    }

    let mut diag1 = String::new();
    let mut diag2 = String::new();

    diag1.push(grid[element.y - 1][element.x - 1].char);
    diag1.push(grid[element.y][element.x].char);
    diag1.push(grid[element.y + 1][element.x + 1].char);

    diag2.push(grid[element.y - 1][element.x + 1].char);
    diag2.push(grid[element.y][element.x].char);
    diag2.push(grid[element.y + 1][element.x - 1].char);

    if (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM") {
        return 1;
    }
    
    0
}