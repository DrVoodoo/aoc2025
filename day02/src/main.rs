use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let ranges = parse_file()?;
    
    let mut total_count: u64 = 0;
    for range in ranges {
        for i in range.first..=range.last {
            if invalid_product_id(i) {
                total_count += i;
            }
        }
    }

    println!("Total sum of invalid product IDs: {}", total_count);

    Ok(())
}

fn invalid_product_id(value: u64) -> bool {
    let s = value.to_string();
    let len = s.to_string().len();
    
    for j in 1..len {
        if j != len && len % j == 0 {
            let chunks: Vec<String> = s.chars()
                .collect::<Vec<_>>()
                .chunks(j)
                .map(|c| c.iter().collect())
                .collect();

            let all_equal = chunks.windows(2).all(|w| w[0] == w[1]);
            if all_equal {
                return true;
            }
        }
    }

    return false;
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