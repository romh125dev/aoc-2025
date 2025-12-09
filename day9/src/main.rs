use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

fn main() {
    let test: Vec<(u64, u64)> = vec![
        (7,1),
        (11,1),
        (11,7),
        (9,7),
        (9,5),
        (2,5),
        (2,3),
        (7,3),
    ];
    println!("Test: {}", find_max_area(&test));

    let input = read_file("input.txt");
    println!("Puzzle result {}", find_max_area(&input));
}

fn find_max_area(grid: &Vec<(u64, u64)>) -> u64 {
    // Pretty trivial - just calculate euclidean distance
    let mut max_area: u64 = 0;
    for (a,b) in grid {
        for (n,m) in grid {
            if a!=n && n!= m {
                let l: i64 = *m as i64 - *b as i64 + 1;
                let h = *n as i64 - *a as i64 + 1;
                let area = (l*h).abs() as u64;
                if area>max_area {
                    max_area = area;
                }
            }
        }
    }
    
    max_area
}

fn read_file(path: &str) -> Vec<(u64,u64)>{
    let file = File::open(path);
    if let Err(e) = file {
        eprintln!("Error opening file {}: {}", path, e);
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        if let Ok(l) = line {
            let split: Vec<&str> = l.split(',').collect();
            let start = split[0].parse().unwrap();
            let end = split[1].parse().unwrap();
            ranges.push((start,end));
        }
    }
    println!("Read file of lenght {}", ranges.len());
    
    ranges
}