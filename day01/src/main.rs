use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let values = parse_file()?;
    let mut final_code = 0;
    let mut dial = 50;

    for Code { turn, number} in &values {
        let mut counter: i32 = *number;
        if turn == "L" {
            while counter > 0 {
                dial -= 1;
                if dial == 0 {
                    final_code += 1;
                }
                if dial < 0 {
                    dial = 99;
                }
                counter -= 1;                
            }
        } else if turn == "R" {
            while counter > 0 {
                dial += 1;
                if dial == 100 {
                    final_code += 1;
                    dial = 0;
                }
                counter -= 1;                
            }
        }
    }

    println!("Final code is {}", final_code);
    Ok(())
}

fn parse_file() -> io::Result<Vec<Code>> {
    let file = File::open("day01/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut values: Vec<Code> = Vec::new();

    while reader.read_line(&mut line)? != 0 {
        let turn = line[..1].to_string();
        let number: i32 = line[1..].trim().parse().unwrap();
        values.push(Code { turn, number });
        line.clear();
    }

    Ok(values)
}

struct Code {
    turn: String,
    number: i32,
}