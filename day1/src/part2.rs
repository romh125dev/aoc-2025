use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let test: Vec<String> = vec![
        "L68".to_string(),
        "L30".to_string(),
        "R48".to_string(),
        "L5".to_string(),
        "R60".to_string(),
        "L55".to_string(),
        "L1".to_string(),
        "L99".to_string(),
        "R14".to_string(),
        "L82".to_string(),
    ];
    let result: u32 = decode(test);
    println!("Test: {}", result);

    let input = read_file("input.txt");
    let result2: u32 = decode(input);
    println!("Puzzle result: {}", result2);
}

fn decode(rotations: Vec<String>) -> u32{
    let mut pos: i32 = 50;
    let mut result: u32 = 0;
    for i in 0..rotations.len() {
        println!("Iteration {}:", i);
        let mut rotation = rotations[i].chars();
        let dir: char = rotation.next().unwrap();
        let rot: i32 = rotation.as_str().parse().unwrap();
        println!("\tRotating {} positions in direction {}", rot, dir);

        let delta: i32 = {
            if dir == 'L' {
                -1
            } else {
                1
            }
        };
        for _ in 0..rot {
            let mut new_pos: i32 = pos;
            new_pos += delta;
            if new_pos < 0 {
                new_pos = ((new_pos % 100) + 100) % 100;
            } else {
                new_pos = new_pos % 100;
            }
            if new_pos == 0 {
                result += 1;
            }

            pos = new_pos;
        }

        println!("\tNew position {}", pos);
        println!("\tClicks on 0: {}", result);
    }
    
    result
}

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path);
    if let Err(e) = file {
        eprintln!("Error opening file {}: {}", path, e);
        return Vec::new();
    }

    let mut lines = Vec::new();
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        if let Ok(l) = line {
            lines.push(l);
        }
    }
    println!("Read file: {} -> {}", lines[0], lines[lines.len()-1]);
    return lines;
}