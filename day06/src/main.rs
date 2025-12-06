use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let data = parse_file()?;
    let length = 3753;// temporary hack :) data[0].len();
    let mut total_result: u64 = 0;

    let mut current_operator = data[4][0];
    let mut current_total: u64 = 0;
    for i in (0..(length-1)) {
        let mut combined: Vec<char> = Vec::new();
        combined.push(data[0][i]);
        combined.push(data[1][i]);
        combined.push(data[2][i]);
        combined.push(data[3][i]);
        let operator = data[4][i];
        
        if is_new_calculation(combined.clone(), operator) {
            // dont go over index
            if length-1 >= i+1 {
                current_operator = data[4][i+1];
            }
            total_result += current_total;
            current_total = 0;
        } else {
            let number = combine_numbers(combined);
            if current_operator == '+' {
                current_total += number;
            } else if current_operator == '*' {
                // first number
                if current_total == 0 {
                    current_total = number;
                } else {
                    current_total *= number;
                }
            }
        }
    }

    // add the last sum here because we add when we find new operator and the last value will be missed
    if current_total > 0 {
        total_result += current_total; 
    }
    println!("Total result: {}", total_result);

    Ok(())
}

fn combine_numbers(combined: Vec<char>) -> u64 {
    let only_numbers: String = combined
        .iter()
        .filter(|&&c| c != ' ')
        .map(|&c| c.to_string())
        .collect();

    let number: u64 = only_numbers.parse().expect("not a valid number");
    number
}

fn is_new_calculation(combined: Vec<char>, operator: char) -> bool {
    if combined.iter().all(|c| *c == ' ') &&
        operator == ' ' {
            return true;
        }
    
    false
}

fn parse_file() -> io::Result<Vec<Vec<char>>> {
    let file = File::open("day06/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut lines: Vec<Vec<char>> = Vec::new();

    while reader.read_line(&mut line)? != 0 {
        lines.push(line.chars().collect());
        line.clear();
    }

    Ok(lines)
}

