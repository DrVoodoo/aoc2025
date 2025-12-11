use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let points = parse_file()?;
    let mut point_distances: Vec<PointDistance> = Vec::new();
    let number_of_pairs = 1000;//10;
    let number_of_largest_circuits = 3;

    for i in 0..points.len()-2 {
        for j in (i+1)..points.len() {
            let point1 = points[i];
            let point2 = points[j];
            let distance = shortest_distance(point1, point2);
            let jd = PointDistance {
                point1: point1.clone(),
                point2: point2.clone(),
                distance: distance
            };
            point_distances.push(jd);
        }
    }

    point_distances.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let mut circuits: Vec<Circuit> = Vec::new();
    for point_distance in point_distances.iter().take(number_of_pairs) {
        if connected_in_same_circuit(point_distance, &circuits) {
            continue;
        }

        if no_connections(point_distance, &circuits) {
            circuits.push(Circuit { connections: vec![point_distance.clone()] });
            continue;
        }

        let has_p1_connection = has_connection_in_circuits(&point_distance.point1, &circuits);
        let has_p2_connection = has_connection_in_circuits(&point_distance.point2, &circuits);
        if has_p1_connection && !has_p2_connection {
            for circuit in circuits.iter_mut() {
                if circuit.connections.iter().any(|c| is_same_point(point_distance.point1, c.point1)) ||
                    circuit.connections.iter().any(|c| is_same_point(point_distance.point1, c.point2)) {
                        circuit.connections.push(point_distance.clone());
                    }
            }
            continue;
        }
        if !has_p1_connection && has_p2_connection {
            for circuit in circuits.iter_mut() {
                if circuit.connections.iter().any(|c| is_same_point(point_distance.point2, c.point1)) ||
                    circuit.connections.iter().any(|c| is_same_point(point_distance.point2, c.point2)) {
                        circuit.connections.push(point_distance.clone());
                    }
            }
            continue;
        }
        // merge circuits
        let mut new_circuits: Vec<Circuit> = Vec::new();
        let mut new_connections: Vec<PointDistance> = Vec::new();
        for circuit in circuits.iter() {
            if has_connection_in_circuit(&point_distance.point1, circuit) {
                new_connections = [new_connections, circuit.connections.clone()].concat();
                continue;
            }
            if has_connection_in_circuit(&point_distance.point2, circuit) {
                new_connections = [new_connections, circuit.connections.clone()].concat();
                continue;
            }
            new_circuits.push(Circuit { connections: circuit.connections.clone() });
        }
        new_connections.push(*point_distance);
        new_circuits.push(Circuit { connections: new_connections });
        circuits = new_circuits;
    }

    circuits.sort_by(|a, b| b.connections.len().cmp(&a.connections.len()));

    let mut answer = 1;
    for i in 0..number_of_largest_circuits {
        answer *= circuits[i].connections.len()+1;
    }
    println!("Answer {}", answer);

    Ok(())
}

fn connected_in_same_circuit(point_distance: &PointDistance, circuits: &Vec<Circuit>) -> bool {
    for circuit in circuits {
        if (circuit.connections.iter().any(|c| is_same_point(c.point1, point_distance.point1)) || 
            circuit.connections.iter().any(|c| is_same_point(c.point2, point_distance.point1))) &&
           (circuit.connections.iter().any(|c| is_same_point(c.point1, point_distance.point2)) || 
            circuit.connections.iter().any(|c| is_same_point(c.point2, point_distance.point2))){
                return true;
            }
    }
    false
}

fn no_connections(point_distance: &PointDistance, circuits: &Vec<Circuit>) -> bool {
    for circuit in circuits {
        if circuit.connections.iter().any(|c| is_same_point(c.point1, point_distance.point1)) || 
           circuit.connections.iter().any(|c| is_same_point(c.point2, point_distance.point1)) ||
           circuit.connections.iter().any(|c| is_same_point(c.point1, point_distance.point2)) || 
           circuit.connections.iter().any(|c| is_same_point(c.point2, point_distance.point2)) {
                return false;
            }
    }
    true
}

fn has_connection_in_circuits(point: &Point, circuits: &Vec<Circuit>) -> bool {
    for circuit in circuits { 
        if circuit.connections.iter().any(|c| is_same_point(c.point1, *point)) ||
            circuit.connections.iter().any(|c| is_same_point(c.point2, *point)) {
                return true
        }
    }
    false
}

fn has_connection_in_circuit(point: &Point, circuit: &Circuit) -> bool {
    circuit.connections.iter().any(|c| is_same_point(c.point1, *point)) || circuit.connections.iter().any(|c| is_same_point(c.point2, *point))
}

fn is_same_point(p1: Point, p2: Point) -> bool {
    p1.x == p2.x && p1.y == p2.y && p1.z == p1.z
}

fn shortest_distance(p1: Point, p2: Point) -> f64 {
    (((p1.x-p2.x).pow(2)+(p1.y-p2.y).pow(2)+(p1.z-p2.z).pow(2)) as f64).sqrt()
}

fn parse_file() -> io::Result<Vec<Point>> {
    let file = File::open("day08/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut points: Vec<Point> = Vec::new();

    while reader.read_line(&mut line)? != 0 {
        let position: Vec<u32> = line.trim().split(',').map(|p| p.parse().unwrap()).collect();
        let point = Point {
            x: position[0] as i64,
            y: position[1] as i64,
            z: position[2] as i64
        };
        points.push(point);    
        line.clear();    
    }

    Ok(points)
}

struct Circuit {
    connections: Vec<PointDistance>
}

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Copy, Clone)]
struct PointDistance {
    point1: Point,
    point2: Point,
    distance: f64,
}