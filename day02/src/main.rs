use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let ranges = parse_file()?;

    let mut total_count: u64 = 0;
    for range in ranges {
        for i in range.first..=range.last {
            let s = i.to_string();
            let mid = s.len() / 2;

            let first_half = &s[..mid];
            let second_half = &s[mid..];

            if first_half == second_half {
                total_count += i;
            }
        }
    }

    println!("Total sum of invalid product IDs: {}", total_count);

    Ok(())
}

fn parse_file() -> io::Result<Vec<ProductIdRange>> {
    let file = File::open("day02/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut data = String::new();
    reader.read_line(&mut data)?;
    let ranges_data: Vec<&str> = data.split(',').collect();
    
    let mut ranges: Vec<ProductIdRange> = Vec::new();

    for range_data in ranges_data {
        let range_values: Vec<&str> = range_data.split('-').collect();
        let first: u64 = range_values[0].trim().parse().unwrap();
        let last: u64 = range_values[1].trim().parse().unwrap();
        ranges.push(ProductIdRange { first, last });
    }

    Ok(ranges)
}

struct ProductIdRange {
    first: u64,
    last: u64,
}