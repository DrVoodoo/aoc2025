use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut rolls_of_paper = 0;
    let mut grid = parse_file()?;

    let length = grid[0].len() as i32;
    let height = grid.len() as i32;

    let mut has_lifted = true;

    while has_lifted == true {
        has_lifted = false;
        for y in 0..height {
            for x in 0..length {
                let value = grid[y as usize][x as usize];
                
                if value == '@' {
                    let collision_grid = get_collision_grid(&grid, x, y, length, height);
                    if can_forklift_paper_roll(collision_grid) {
                        rolls_of_paper += 1;
                        grid[y as usize][x as usize] = 'x'; // mark as lifted
                        has_lifted = true;
                    }
                }
            }
        }
    }

    println!("Total paper rolls fork lifted: {}", rolls_of_paper);

    Ok(())
}

fn get_collision_grid(grid: &Vec<Vec<char>>, x: i32, y: i32, length: i32, height: i32) -> Vec<Vec<char>> {
    let x_start: i32 = x as i32 - 1;
    let x_end: i32 = x as i32 + 1;
    let y_start: i32 = y as i32 - 1;
    let y_end: i32 = y as i32 + 1;
    let mut collision_grid: Vec<Vec<char>> = Vec::new();

    for i in y_start..=y_end {
        let mut x_row: Vec<char> = Vec::new();
        for j in x_start..=x_end {
            if i < 0 || j < 0 {
                x_row.push('.');
            }
            else if i >= height || j >= length {
                x_row.push('.');
            }
            else {
                x_row.push(grid[i as usize][j as usize]);
            }
        }
        collision_grid.push(x_row);
    }

    collision_grid
}

fn can_forklift_paper_roll(collision_grid: Vec<Vec<char>>) -> bool {
    let mut total_paper_rolls = 0;
    for y in collision_grid.iter() {
        for x in y.iter() {
            if *x == '@' {
                total_paper_rolls += 1;
            }
        }
    }
    
    total_paper_rolls -= 1; // remove the center one

    if total_paper_rolls < 4 {
        return true;
    }

    false
}

fn parse_file() -> io::Result<Vec<Vec<char>>> {
    let file = File::open("day04/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut values: Vec<String> = Vec::new();

    // read the lines
    while reader.read_line(&mut line)? != 0 {
        values.push(line.trim().to_string());
        line.clear();
    }

    // creat a structure from the lines
    let mut grid: Vec<Vec<char>> = Vec::new();

    for i in values.iter() {
        let chars: Vec<char> = i.chars().collect();
        grid.push(chars);
    }   

    Ok(grid)
}