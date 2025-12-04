use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let values = parse_file()?;
    let mut total_battery_value = 0;

    for val in &values {
        let battery_value = get_battery_value(val.to_string())?;
        total_battery_value += battery_value.parse::<i32>().unwrap_or(0);
    }

    println!("Total battery value: {}", total_battery_value);

    Ok(())
}

fn get_battery_value(val: String) -> io::Result<String> {
    let excluded = [90, 80, 70, 60, 50, 40, 30, 20, 10];

    for i in (11..=99).rev() {
        if excluded.contains(&i) {
            continue;
        }

        let first_find = (i - (i % 10)) / 10;
        let second_find = i % 10;
        let mut first = 0;
        let mut last = 0;
        for c in val.chars() {
            let d = c.to_digit(10).unwrap() as i32;
            
            if first == 0 {
                if d == first_find {
                    first = d;
                }
            } else {
                if d == second_find {
                    last = d;
                }
            }
            if first != 0 && last != 0 {
                return Ok(format!("{}{}", first, last));
            }
        }
    }
    Ok("".to_string())
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