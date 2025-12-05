use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let values = parse_file()?;
    let mut total_battery_value: u64 = 0;

    for val in &values {
        let battery_value = get_twelwe_cell_battery(val.to_string())?;
        total_battery_value += battery_value.parse::<u64>().unwrap_or(0);
    }

    println!("Total battery value: {}", total_battery_value);

    Ok(())
}

fn get_twelwe_cell_battery(val: String) -> io::Result<String> {
    let mut counter = 0;
    let mut result = String::new();
    let mut left_to_choose_from = val;

    while counter < 12 {
        for i in (0..=9).rev() {
            let index = index_of_choosen_battery(i, left_to_choose_from.to_string(), 12 - counter).unwrap();
        
            if index != -1 {
                left_to_choose_from = left_to_choose_from.get(((index+1) as usize)..).unwrap().to_string();
                counter += 1;
                result.push_str(&i.to_string());
                break;
            }
        }    
    }
    
    Ok(result.to_string())
}

fn index_of_choosen_battery(number_to_find: i32, val: String, choices_left: i32) -> io::Result<i32> {
    let choices = val.len() as i32;
    for i in 0..=choices-1 {
        let c = val.chars().nth(i as usize).unwrap().to_digit(10).unwrap() as i32;
        if c == number_to_find {
            if choices-i >= choices_left {
                return Ok(i);
            } else {
                return Ok(-1);
            }
        }
    }

    Ok(-1)
}

fn parse_file() -> io::Result<Vec<String>> {
    let file = File::open("day03/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut values: Vec<String> = Vec::new();

    while reader.read_line(&mut line)? != 0 {
        values.push(line.trim().to_string());
        line.clear();
    }

    Ok(values)
}