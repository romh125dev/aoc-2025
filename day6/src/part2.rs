use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let test1: Vec<String> = vec![        
        "123 328  51 64 ".to_string(), 
        " 45 64  387 23 ".to_string(), 
        "  6 98  215 314".to_string()
    ];
    let test2: Vec<String> = "*   +   *   +  ".split_whitespace().map(String::from).collect();
    println!("Test: {}", solve_problems(&test1, &test2));

    let input = read_file("input.txt");
    let input1 = &input[0..input.len()-1].to_vec();
    let input2: Vec<String> = input[input.len()-1].split_whitespace().map(String::from).collect();
    println!("Puzzle result {}", solve_problems(input1, &input2));
}


fn solve_problems(inputs: &Vec<String>, operations: &Vec<String>) -> u64 {
    let mut result = 0;
    let mut p = 0;

    // We'll go character by character
    let mut nums: Vec<u64> = Vec::new();
    for j in (0..inputs[0].len()).rev() {
        // We read the digits of the column row by row
        let mut col: Vec<&str> = Vec::new();
        for i in 0..inputs.len() {
            let digit: &str = &inputs[i][j..j+1];
            col.push(digit);
        }
        println!("Column {}: {:?}", j, col);
        let col_num = col.concat().trim().parse::<u64>();
        let mut parsed = false;
        if !col_num.is_ok(){
            parsed = true;
        } else if j==0 {
            nums.push(col_num.unwrap());
            parsed = true;
        } else {
            nums.push(col_num.unwrap());
        }
        if parsed {
            // This problem is parsed, let's calculate!
            let op = &operations[operations.len()-p-1];
            println!("\tProblem {}: numbers: {:?}, operation {}", p, &nums, op);
            let mut solution: u64 = if op =="+" {0} else {1};
            for num in &nums {
                if op == "+" {
                    solution += num;
                } else {
                    solution *= num;
                }
            }
            println!("\tProblem {} solution: {}", p, solution);
            result += solution;
        
            // Reset vars
            nums.clear();
            p += 1;
        }    
    }

    result
}

fn read_file(path: &str) -> Vec<String>{
    // Back to basics - just read the full line
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
    println!("Problems read");

    lines
}