use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut test: Vec<Vec<char>> = vec![
        "..@@.@@@@.".chars().collect(),
        "@@@.@.@.@@".chars().collect(),
        "@@@@@.@.@@".chars().collect(),
        "@.@@@@..@.".chars().collect(),
        "@@.@@@@.@@".chars().collect(),
        ".@@@@@@@.@".chars().collect(),
        ".@.@.@.@@@".chars().collect(),
        "@.@@@.@@@@".chars().collect(),
        ".@@@@@@@@.".chars().collect(),
        "@.@.@@@.@.".chars().collect(),
    ];
    println!("Test: {}", find_rolls(&test).len());

    let mut input = read_file("input.txt");
    println!("Puzzle result {}", find_rolls(&input).len());

    // Part 2
    let result = remove_rolls(&mut input);
    println!("Total rolls removed {}", result);
}

fn remove_rolls(grid: &mut Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    let mut result = (grid.len() * grid[0].len()) as u32;
    while result > 0 {
        let to_remove = find_rolls(grid);
        for item in to_remove.iter() {
            grid[item[0]][item[1]] = '.';
        }
        result = to_remove.len() as u32;
        println!("Removed {} rolls", result);
        total += result;
    }
    total    
}

fn find_rolls(grid: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut valid_rolls: Vec<Vec<usize>> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '@' {
                //println!("Found roll at: {},{}", i, j);
                let mut adjacent_rolls = 0;
                let min_row = if i>0 {i-1} else {0};
                let max_row = if i<grid.len()-1 {i+1} else {grid.len()-1};
                let min_col = if j>0 {j-1} else {0};
                let max_col = if j<row.len()-1 {j+1} else {row.len()-1};
                //println!("\tScanning adjacent rows {} to {} and columns {} to {}", min_row, max_row, min_col, max_col);
                for adj_row in min_row..max_row+1 {
                    for adj_col in min_col..max_col+1 {
                        let adj = &grid[adj_row][adj_col];
                        if adj_row == i && adj_col == j{
                            //println!("\tSkipping own element: {},{}", adj_row, adj_col);
                        } else if *adj == '@' {
                            adjacent_rolls += 1;
                            //println!("\tFound adjacent roll at: {},{}. Total so far: {}", adj_row, adj_col, adjacent_rolls);
                        }
                    }
                }
                if adjacent_rolls < 4 {
                    println!("\tRoll: {},{} is valid", i, j);
                    valid_rolls.push([i, j].to_vec());
                }
            }
        }
    }

    valid_rolls
}

fn read_file(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path);
    if let Err(e) = file {
        eprintln!("Error opening file {}: {}", path, e);
        return Vec::new();
    }

    let mut lines = Vec::new();
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        if let Ok(l) = line {
            lines.push(l.chars().collect());
        }
    }
    println!("Read file: {:?} -> {:?}", lines[0], lines[lines.len()-1]);
    
    lines
}