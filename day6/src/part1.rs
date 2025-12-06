use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let test1: Vec<Vec<String>> = vec![
        ["123", "328", "51", "64"].map(String::from).to_vec(), 
        ["45", "64", "387", "23"].map(String::from).to_vec(), 
        ["6", "98", "215", "314"].map(String::from).to_vec()
    ];
    let test2: Vec<String> = [
        "*", "+", "*", "+"]
    .map(String::from).to_vec();
    println!("Test: {}", solve_problems(&test1, &test2));

    let input = read_file("input.txt");
    println!("Puzzle result {}", solve_problems(&input[0..input.len()-1].to_vec(), &input[input.len()-1]));
}


fn solve_problems(inputs: &Vec<Vec<String>>, operations: &Vec<String>) -> u64 {
    let mut result = 0;
    for j in 0..inputs[0].len(){
        let mut solution: u64 = inputs[0][j].parse().unwrap();
        let op = &operations[j];
        for i in 1..inputs.len() {
            let num: u64 = inputs[i][j].parse().unwrap();
            if op == "+" {
                solution += num;
            } else if op == "*" {
                solution *= num;
            } else {
                eprintln!("Unsupported operation {}", op)
            }
        }
        println!("\tProblem {} solution: {}", j, solution);
        result += solution;
    }

    result
}

fn read_file(path: &str) -> Vec<Vec<String>>{
    let file = File::open(path);
    if let Err(e) = file {
        eprintln!("Error opening file {}: {}", path, e);
        return Vec::new();
    }

    let mut lines = Vec::new();
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        if let Ok(l) = line {
            let split: Vec<String> = l.split_whitespace().map(|s| s.to_string()).collect();
            lines.push(split);
        }
    }
    println!("Problems read");

    lines
}