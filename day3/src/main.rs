use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let test: Vec<String> = vec![
        "987654321111111".to_string(),
        "811111111111119".to_string(),
        "234234234234278".to_string(),
        "818181911112111".to_string(),
    ];
    let test_result = total_joltage(&test,2);
    println!("Test part 1: {}", test_result);

    let input = read_file("input.txt");
    let result = total_joltage(&input,2);
    println!("Puzzle result: {}", result);

    // Part 2
    println!("Test part 2: {}", total_joltage(&test,12));
    println!("Puzzle result part 2: {}", total_joltage(&input,12));
}

fn total_joltage(banks: &Vec<String>, batteries: usize) -> u64 {
    let mut result = 0;
    for bank in banks {
        let mut joltage: u64 = 0;
        let mut next_index = 0;
        for i in 0..batteries {
            // Find the largest number...
            let unit_position = batteries - i - 1;
            let max_len = bank.len()-unit_position;
            let subbank = &bank[next_index..max_len];
            //println!("\tUnit {}, using subbank {} to max len {}", unit_position, subbank, max_len);
            let (max, index) = find_max(&subbank);
            joltage += max * 10_u64.pow(unit_position as u32);
            next_index += index + 1;
        }
        println!("Bank {} size {} joltage: {}", bank, batteries, joltage);
        result += joltage;
    }
    
    result
}

fn find_max(numstr: &str) -> (u64, usize) {
    let mut max = 0;
    let mut index = 0;
    for (i, num) in numstr.char_indices(){
        let value: u64 = num.to_string().parse().unwrap();
        if value > max {
            max = value;
            index = i;
        }
    }
    (max, index)
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
    
    lines
}