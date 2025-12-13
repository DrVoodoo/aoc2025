use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Line2D {
    start: Point2D,
    end: Point2D,
}

impl Line2D {
    pub fn new(start: Point2D, end: Point2D) -> Self {
        Line2D { start, end }
    }

    pub fn intersects(&self, other: &Line2D) -> bool {
        let d1 = (self.end.x - self.start.x) * (other.start.y - self.start.y)
            - (self.end.y - self.start.y) * (other.start.x - self.start.x);
        let d2 = (self.end.x - self.start.x) * (other.end.y - self.start.y)
            - (self.end.y - self.start.y) * (other.end.x - self.start.x);
        let d3 = (other.end.x - other.start.x) * (self.start.y - other.start.y)
            - (other.end.y - other.start.y) * (self.start.x - other.start.x);
        let d4 = (other.end.x - other.start.x) * (self.end.y - other.start.y)
            - (other.end.y - other.start.y) * (self.end.x - other.start.x);

        if ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))
        {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Point2D>,
}

impl Polygon {
    pub fn new(vertices: Vec<Point2D>) -> Self {
        Polygon { vertices }
    }

    // Check if a point is inside the polygon using the ray-casting algorithm
    // Source: https://www.xjavascript.com/blog/check-if-polygon-is-inside-a-polygon/
    pub fn contains(&self, point: &Point2D) -> bool {
        let mut inside = false;
        let n = self.vertices.len();

        for i in 0..n {
            let j = (i + n - 1) % n;

            let pi = &self.vertices[i];
            let pj = &self.vertices[j];

            // TODO: On edge check

            if (pi.y > point.y) != (pj.y > point.y) {
                let x_intersect = (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x;
                if point.x < x_intersect {
                    inside = !inside;
                }
            }
        }

        inside
    }
}

fn part2(points: Vec<Point2D>) -> isize {
    let polygon = Polygon::new(points.clone());

    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            // All 4 vertices must be within the polygon
            // Because the polygons are on a grid of points, this means we can ignore the edge checks
            // This will skip rectangles with width/height = 1, but those won't be max area (we assume)
            let vertices = [
                Point2D::new(
                    points[i].x.min(points[j].x) + 1,
                    points[i].y.min(points[j].y) + 1,
                ),
                Point2D::new(
                    points[i].x.min(points[j].x) + 1,
                    points[i].y.max(points[j].y) - 1,
                ),
                Point2D::new(
                    points[i].x.max(points[j].x) - 1,
                    points[i].y.min(points[j].y) + 1,
                ),
                Point2D::new(
                    points[i].x.max(points[j].x) - 1,
                    points[i].y.max(points[j].y) - 1,
                ),
            ];
            if !vertices.iter().all(|v| polygon.contains(v)) {
                continue;
            }

            // No edge of the rectangle can intersect with any edge of the polygon
            let rectangle_edges = vec![
                Line2D::new(vertices[0], vertices[1]),
                Line2D::new(vertices[1], vertices[3]),
                Line2D::new(vertices[3], vertices[2]),
                Line2D::new(vertices[2], vertices[0]),
            ];

            let mut intersects = false;
            'outer: for rect_edge in &rectangle_edges {
                for i in 0..points.len() {
                    let poly_edge = Line2D::new(points[i], points[(i + 1) % points.len()]);
                    if rect_edge.intersects(&poly_edge) {
                        intersects = true;
                        break 'outer;
                    }
                }
            }
            if intersects {
                continue;
            }

            let xd = (points[i].x - points[j].x).abs() + 1;
            let yd = (points[i].y - points[j].y).abs() + 1;
            let area = xd * yd;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn main() -> io::Result<()> {
    let points = parse_file()?;

    println!("area: {}", part2(points));

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl Point2D {
    pub fn new(x: isize, y: isize) -> Self {
        Point2D { x, y }
    }
}

impl From<&str> for Point2D {
    fn from(s: &str) -> Self {
        let coords: Vec<isize> = s
            .split(',')
            .map(|part| part.trim().parse().unwrap())
            .collect();
        Point2D {
            x: coords[0],
            y: coords[1],
        }
    }
}

impl Point2D {
    pub fn distance_squared(&self, other: &Point2D) -> isize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}

fn parse_file() -> io::Result<Vec<Point2D>> {
    let file = File::open("day09/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut tiles: Vec<Point2D> = Vec::new();

    while reader.read_line(&mut line)? != 0 {
        let parts: Vec<i64> = line.trim().split(',').map(|p| p.parse().unwrap()).collect();
        let tile = Point2D {
            x: parts[0] as isize,
            y: parts[1] as isize
        };
        tiles.push(tile);    
        line.clear();    
    }

    Ok(tiles)
}