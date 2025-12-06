use core::num;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let database = parse_file()?;
    
    //let mut fresh_ingredient_count = 0;
    // for ingriedient in database.ingredients.iter() {
    //     if is_ingriedient_fresh(*ingriedient, &database.fresh_id_ranges) {
    //         fresh_ingredient_count += 1;
    //     }
    // }

    // println!("Total fresh ingridients: {}", fresh_ingredient_count);

    let mut fresh_id_ranges = database.fresh_id_ranges;
    fresh_id_ranges.sort_by_key(|r| (r.start, r.end));

    let mut counter = 0;
    let mut merged_ranges: Vec<FreshIdRange> = Vec::new();

    while counter < fresh_id_ranges.len() - 1 {
        let ranges_to_merge = get_ranges_to_merge(&fresh_id_ranges, counter);
        counter += ranges_to_merge.len();
        let merged_range = merge_ranges(ranges_to_merge);
        merged_ranges.push(merged_range);
    }

    let mut number_of_fresh_ids = 0;
    for range in merged_ranges.iter() {
        number_of_fresh_ids += range.end - range.start + 1;
    }

    print!("Number of fresh ids: {}", number_of_fresh_ids);

    Ok(())
}

fn merge_ranges(mut ranges_to_merge: Vec<FreshIdRange>) -> FreshIdRange {
    let mut start = 0;
    let mut end = 0;
    for range in ranges_to_merge.iter() {
        if range.start < start || start == 0 {
            start = range.start;
        }
        if range.end > end {
            end = range.end;
        }
    }
    
    FreshIdRange { start, end }
}

fn get_ranges_to_merge(fresh_id_ranges: &[FreshIdRange], counter: usize) -> Vec<FreshIdRange> {
    let mut ranges_to_merge: Vec<FreshIdRange> = Vec::new();
    
    for range in fresh_id_ranges.iter().skip(counter) {
        if ranges_to_merge.is_empty() {
            ranges_to_merge.push(FreshIdRange { start: range.start, end: range.end });
        } else {
            let last_range = ranges_to_merge.last().unwrap();
            if range.start <= last_range.end {
                ranges_to_merge.push(FreshIdRange { start: range.start, end: range.end });
            } else {
                break;
            }
        }
    }
    ranges_to_merge
}

fn parse_file() -> io::Result<Database> {
    let file = File::open("day05/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    // read the lines
    let mut processing_id_ranges = true;
    let mut fresh_id_ranges: Vec<FreshIdRange> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    while reader.read_line(&mut line)? != 0 {
        if line.trim().is_empty() {
            processing_id_ranges = false;
        } else if processing_id_ranges {
           let parts: Vec<&str> = line.trim().split('-').collect();
           let start: u64 = parts[0].parse().unwrap();
           let end: u64 = parts[1].parse().unwrap();
           let fresh_id_range = FreshIdRange { start, end };
            fresh_id_ranges.push(fresh_id_range);
        } else {
            ingredients.push(line.trim().parse().unwrap());
        }    
        line.clear();    
    }

    let database = Database {
        fresh_id_ranges,
        ingredients,
    };

    Ok(database)
}

struct Database {
    fresh_id_ranges: Vec<FreshIdRange>,
    ingredients: Vec<u64>,
}

struct FreshIdRange {
    start: u64,
    end: u64,
}