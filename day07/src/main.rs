use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let data = parse_file()?;

    let number_of_timelines = particle_timeline_calculator(data);

    println!("Total number of timelines {}", number_of_timelines);

    Ok(())
}

fn particle_timeline_calculator(data: Vec<Vec<char>>) -> usize {
    let height = data.len();
    let width = data[0].len();

    // Create a DP table to store the computed results
    let mut dp = vec![vec![0; width]; height];

    // Fill the DP table starting from the bottom row
    for y in (0..height).rev() {
        for x in 0..width {
            let current_particle_position = data[y][x];

            if y == height - 1 {
                // Base case: on the last row, particle path count is 1 or 2
                dp[y][x] = if current_particle_position == '^' { 2 } else { 1 };
            } else if current_particle_position == '^' {
                // Calculate for splitting particles
                let mut left = 0;
                let mut right = 0;

                if x > 0 {
                    left = dp[y + 1][x - 1]; // Coming from the left
                }

                if x < width - 1 {
                    right = dp[y + 1][x + 1]; // Coming from the right
                }

                dp[y][x] = left + right;
            } else {
                // Calculate for straight path particles
                dp[y][x] = dp[y + 1][x]; // Coming straight down
            }
        }
    }

    // Find the starting position and return the number of timelines
    for x in 0..width {
        if data[0][x] == 'S' {
            return dp[0][x];
        }
    }

    0 // If no start position is found, return 0
}

fn parse_file() -> io::Result<Vec<Vec<char>>> {
    let file = File::open("day07/input.txt")?;
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