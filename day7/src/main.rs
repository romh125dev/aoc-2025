use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

fn main() {
    let test: Vec<Vec<char>> = vec![
        ".......S.......".chars().collect(),
        "...............".chars().collect(),
        ".......^.......".chars().collect(),
        "...............".chars().collect(),
        "......^.^......".chars().collect(),
        "...............".chars().collect(),
        ".....^.^.^.....".chars().collect(),
        "...............".chars().collect(),
        "....^.^...^....".chars().collect(),
        "...............".chars().collect(),
        "...^.^...^.^...".chars().collect(),
        "...............".chars().collect(),
        "..^...^.....^..".chars().collect(),
        "...............".chars().collect(),
        ".^.^.^.^.^...^.".chars().collect(),
        "...............".chars().collect(),
    ];
    println!("Test: {}", find_splits(&test));

    let input = read_file("input.txt");
    println!("Puzzle result {}", find_splits(&input));

    // // Part 2
    let initial_col: usize = test[0].iter().position(|&r| r == 'S').unwrap();
    let mut paths_cache: HashMap<(usize,usize),u64> = HashMap::new();
    println!("Test part 2: {}", find_paths(&test, 1, initial_col, &mut paths_cache));

    let initial_col: usize = input[0].iter().position(|&r| r == 'S').unwrap();
    paths_cache.clear();
    println!("Puzzle part 2: {}", find_paths(&input, 1, initial_col, &mut paths_cache));
}

fn find_splits(grid: &Vec<Vec<char>>) -> u64 {
    let mut result: u64 = 0;
    // Columns with rays
    let mut rays: HashSet<usize> = HashSet::new();
    rays.insert(grid[0].iter().position(|&r| r == 'S').unwrap());
    for i in 1..grid.len(){
        let prev_rays = rays.clone();
        rays.clear();
        for j in prev_rays {
            if grid[i][j] == '.' {
                // Keep going on the same column
                rays.insert(j);
            } else {
                // Split into left-right (if possible)
                if j > 0 {
                    rays.insert(j-1);
                }
                if j < grid[i].len() - 1{
                    rays.insert(j+1);
                }
                result += 1;
            }
        }
    }

    result
}

fn find_paths(grid: &Vec<Vec<char>>, row: usize, col: usize, paths_cache: &mut HashMap<(usize,usize),u64>) -> u64 {
    let mut result: u64 = 0;
    if row+1 == grid.len(){
        // We've reached grid end on this path
        result = 1;
    } else if paths_cache.get(&(row, col)).is_some(){
        // Cache hit - use that
        result = *paths_cache.get(&(row, col)).unwrap();
    } else {
        if grid[row][col] == '.' {
            // Advance to the next row
            result = find_paths(grid, row+1, col, paths_cache);
        } else {
            // If branch exists, advance to each side
            let mut left = 0;
            let mut right = 0;
            if col > 0 {
                left = find_paths(grid, row+1, col-1, paths_cache);
            }
            if col < grid[row].len() - 1{
                right = find_paths(grid, row+1, col+1, paths_cache);
            }
            result = left + right;
        }
    }
    paths_cache.insert((row, col), result);

    result
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
    println!("Read file of {} lines", lines.len());
    
    lines
}