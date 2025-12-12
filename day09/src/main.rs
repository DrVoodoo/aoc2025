use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let tiles = parse_file()?;
    let mut tlie_areas: Vec<TileArea> = Vec::new();

    for i in 0..tiles.len()-2 {
        for j in (i+1)..tiles.len() {
            let tile1 = tiles[i];
            let tile2 = tiles[j];
            let area = tile_area(tile1, tile2);
            let tile_area = TileArea {
                tile1,
                tile2,
                area
            };
            tlie_areas.push(tile_area);
        }
    }

    tlie_areas.sort_by(|a, b| a.area.partial_cmp(&b.area).unwrap());

    println!("Answer {}", tlie_areas.last().unwrap().area);

    Ok(())
}

fn tile_area(tile1: Tile, tile2: Tile) -> i64 {
    // add 1 to count the nodes and not edges
    let x = (tile1.x - tile2.x).abs()+1;
    let y = (tile1.y - tile2.y).abs()+1;

    x*y
}

fn parse_file() -> io::Result<Vec<Tile>> {
    let file = File::open("day09/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut tiles: Vec<Tile> = Vec::new();

    while reader.read_line(&mut line)? != 0 {
        let parts: Vec<i64> = line.trim().split(',').map(|p| p.parse().unwrap()).collect();
        let tile = Tile {
            x: parts[0] as i64,
            y: parts[1] as i64
        };
        tiles.push(tile);    
        line.clear();    
    }

    Ok(tiles)
}

struct TileArea {
    tile1: Tile,
    tile2: Tile,
    area: i64
}

#[derive(Copy, Clone)]
struct Tile {
    x: i64,
    y: i64
}