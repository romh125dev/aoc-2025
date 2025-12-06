use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let test1: Vec<(u64,u64)> = vec![
        (3,5),
        (10,14),
        (16,20),
        (12,18),
    ];
    let test2: Vec<u64> = vec![1, 5, 8, 11, 17, 32];
    println!("Test: {}", find_fresh(&test1, &test2));

    let (input1, input2) = read_file("input.txt");
    // println!("Puzzle result {}", find_fresh(&input1, &input2));

    // Part 2
    println!("Test 2: {}", find_all_fresh(&test1));
    println!("Puzzle result {}", find_all_fresh(&input1));
}


fn find_fresh(ranges: &Vec<(u64, u64)>, ingredients: &Vec<u64>) -> u32 {
    let mut result  = 0;
    for id in ingredients{
        for (start,end) in ranges{
            if id >= start && id <= end{
                println!("Ingredient {} is fresh, found in range {}-{}", id, start, end);
                result += 1;
                break;
            }            
        }
    }

    result
}

fn find_all_fresh(ranges: &Vec<(u64, u64)>) -> u64 {
    // Merge the ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut ordered: Vec<(u64, u64)> = ranges.clone();
    ordered.sort_by_key(|k| k.0);
    for (start, end) in ordered {
        let mut added = false;
        for range in merged.iter_mut(){
            let (i, j) = *range;
            let mut merge = false;
            if (i >= start && i <= end) ||
               (j >= start && j <= end) ||
               (start >= i && start <= j)  ||
               (end >= i && end <= j) {
                merge = true;
            }
            if merge {
                let min = if i < start {i} else {start};
                let max = if j > end {j} else {end};
                println!("\tMerging range {}-{} with {}-{}", start, end, i, j);
                // Replaces the element in the vector in-place
                *range = (min, max);
                println!("\tNew range {}-{}", min, max);
                added = true;
                break;
            }
        }
        if !added {
            merged.push((start, end));
        }
    }

    let mut result = 0;
    for (start, end) in merged {
        //println!("Processing range {}-{}", start, end);
        result += (end+1-start);
    }

    result
}

fn read_file(path: &str) -> (Vec<(u64,u64)>, Vec<u64>){
    let file = File::open(path);
    if let Err(e) = file {
        eprintln!("Error opening file {}: {}", path, e);
        return (Vec::new(), Vec::new());
    }

    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    let mut ranges_done = false;
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                println!("Ranges read");
                ranges_done = true;
                continue;
            }
            if !ranges_done {
                let split: Vec<&str> = l.split('-').collect();
                let start = split[0].parse().unwrap();
                let end = split[1].parse().unwrap();
                ranges.push((start,end));
            }
            else {
                let id: u64 = l.parse().unwrap();
                ingredients.push(id);
            }
        }
    }
    println!("Ingredients read");
    
    (ranges, ingredients)
}