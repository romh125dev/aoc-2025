fn main() {
    let test = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let invalid_ids = get_invalids(test);
    let result: i64 = invalid_ids.iter().sum();
    println!("Result: {}", result);

    let input = "78847-119454,636-933,7143759788-7143793713,9960235-10043487,44480-68595,23468-43311,89-123,785189-1014654,3829443354-3829647366,647009-692765,2-20,30-42,120909-197026,5477469-5677783,9191900808-9191943802,1045643-1169377,46347154-46441299,2349460-2379599,719196-779497,483556-641804,265244-450847,210541-230207,195-275,75702340-75883143,58-84,2152-3237,3367-5895,1552-2029,9575-13844,6048-8966,419388311-419470147,936-1409,9292901468-9292987321";
    let invalid_ids = get_invalids(input);
    let result: i64 = invalid_ids.iter().sum();
    println!("Result: {}", result);
}

fn get_invalids(input: &str)->Vec<i64>{
    let mut result = Vec::new();
    let ranges: Vec<&str> = input.split(',').collect();
    for range in &ranges {
        let v: Vec<&str> = range.split('-').collect();
        let start: i64 = v[0].parse().unwrap();
        let end: i64 = v[1].parse().unwrap();
        println!("Analysing range: {} - {}", start, end);
        for id in start..end+1{
            if is_invalid(&id.to_string()){
                println!("\tFound invalid id: {}", id);
                result.push(id);
            }
        }
    }
    result
}

fn is_invalid_part1(id: &str) -> bool {
    let result: bool;
    if id.len()%2 > 0 {
        result = false;
    } else {
        let no1 = &id[0..id.len()/2];
        let no2 = &id[id.len()/2..id.len()];
        result = no1 == no2;
    }
    result
}

fn is_invalid(id: &str) -> bool {
    // All the possible sizes
    for size in 1..id.len()/2+1 {
        //println!("\tId {}, size {}", id, size);
        if id.len() % size == 0 {
            // It's divisible, now get all the chunks
            let mut chunk = &id[0..size];
            let mut j = size;
            let mut same = true;
            while j < id.len() {
                let next_chunk = &id[j..j+size];
                //println!("\t\t\tComparing chunk {} vs {}", chunk, next_chunk);
                same = chunk == next_chunk;
                if !same {
                    break;
                }
                chunk = next_chunk;
                j = j+size;
            }
            if same {
                return same;
            }
        }
    }
    return false;
}