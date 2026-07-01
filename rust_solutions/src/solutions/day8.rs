use std::collections::HashMap;

const NUM_CONNECTIONS: usize = 1000;

#[derive(Debug, PartialEq, PartialOrd, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Coord {
    fn from_csv_str(csv: &str) -> Option<Coord> {
        let coords: Vec<&str> = csv.split(',').collect();
        if coords.len() != 3 {
            return None;
        }

        Some(Coord {
            x: coords[0].parse().unwrap(),
            y: coords[1].parse().unwrap(),
            z: coords[2].parse().unwrap(),
        })
    }
}

impl Eq for Coord {}

pub fn sol(input: &str) -> usize {
    let coords: Vec<_> = input
        .lines()
        .filter_map(|mut line| {
            line = line.trim();
            if line.is_empty() {
                return None;
            }

            return Coord::from_csv_str(line);
        })
        .collect();

    // Calculate distances between all pairs of points
    let mut distances = vec![];
    for (i, coord) in coords.iter().enumerate() {
        for rest in &coords[i + 1..] {
            let dist =
                (coord.x - rest.x).pow(2) + (coord.y - rest.y).pow(2) + (coord.z - rest.z).pow(2);

            distances.push((coord, rest, dist));
        }
    }

    distances.sort_by_key(|k| k.2);

    // Start connecting the n shortest-distance pairs in circuits
    let mut num_connections = NUM_CONNECTIONS;
    let mut id = 0;
    let mut positions: HashMap<&Coord, usize> = HashMap::new();

    for (conn1, conn2, _) in distances {
        let first = positions.get(conn1).copied();
        let second = positions.get(conn2).copied();

        if first.is_some() && second.is_some() {
            let circuit_one = first.unwrap();
            let circuit_two = second.unwrap();

            if circuit_one != circuit_two {
                for (_, circuit) in &mut positions {
                    if *circuit == circuit_two {
                        *circuit = circuit_one;
                    }
                }
            }
        } else if first.is_some() {
            positions.insert(conn2, first.unwrap());
        } else if second.is_some() {
            positions.insert(conn1, second.unwrap());
        } else {
            positions.insert(conn1, id);
            positions.insert(conn2, id);
            id += 1;
        }

        num_connections -= 1;
        if num_connections == 0 {
            break;
        }
    }

    // Calculate product of sizes of three largest circuits
    let frequencies: HashMap<usize, usize> =
        positions.into_iter().fold(HashMap::new(), |mut map, item| {
            *map.entry(item.1).or_default() += 1;
            map
        });

    let mut counts: Vec<usize> = frequencies.values().map(|v| *v).collect();
    counts.sort_by(|a, b| b.cmp(a));

    counts.iter().take(3).product()
}
