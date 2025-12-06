use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let data = parse_file()?;

    let mut total_result: u64 = 0;

    for i in 0..data.operations.len() {
        let op = data.operations[i];
        let a = data.first_numbers[i];
        let b = data.second_numbers[i];
        let c = data.third_numbers[i];
        let d = data.fourth_numbers[i];

        let result = match op {
            '+' => a + b + c + d,
            '*' => a * b * c * d,
            _ => {
                println!("Unknown operation '{}' at index {}", op, i);
                continue;
            }
        };

        total_result += result;
    }

    println!("Total result: {}", total_result);

    Ok(())
}

fn parse_file() -> io::Result<MathData> {
    let file = File::open("day06/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut lines: Vec<String> = Vec::new();

    while reader.read_line(&mut line)? != 0 {
        lines.push(line.trim().to_string());
        line.clear();
    }

    let math_data = MathData {
        first_numbers: lines[0]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>(),
        second_numbers: lines[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>(),
        third_numbers: lines[2]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>(),
        fourth_numbers: lines[3]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>(),
        operations: lines[4]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<char>().unwrap())
            .collect::<Vec<char>>(),
    };

    Ok(math_data)
}

struct MathData {
    first_numbers: Vec<u64>,
    second_numbers: Vec<u64>,
    third_numbers: Vec<u64>,
    fourth_numbers: Vec<u64>,
    operations: Vec<char>,
}
