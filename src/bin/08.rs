use advent_of_code::vector::{linear_distance, Vector3d};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    solve_part1(input, 1000)
}

fn solve_part1(input: &str, no_of_connections: usize) -> Option<u64> {
    let junction_boxes = parse_junction_boxes(input);

    let mut connections = get_all_distances(&junction_boxes);
    connections.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    connections.truncate(no_of_connections);

    // group to circuits
    let mut circuits: Vec<Vec<&Vector3d>> = Vec::new();
    for connection in connections {
        let mut circuit_a_index: Option<usize> = None;
        let mut circuit_b_index: Option<usize> = None;

        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&connection.a) {
                circuit_a_index = Some(i);
            }
            if circuit.contains(&connection.b) {
                circuit_b_index = Some(i);
            }
        }

        if circuit_a_index == None && circuit_b_index == None {
            circuits.push(vec![connection.a, connection.b]);
        } else if circuit_a_index.is_some() && circuit_b_index.is_none() {
            circuits[circuit_a_index.unwrap()].push(&connection.b);
        } else if circuit_a_index.is_none() && circuit_b_index.is_some() {
            circuits[circuit_b_index.unwrap()].push(&connection.a);
        } else {
            let a = circuit_a_index.unwrap();
            let b = circuit_b_index.unwrap();

            if a == b {
                continue;
            }

            let (a, b) = if a < b { (a, b) } else { (b, a) };
            let (left, right) = circuits.split_at_mut(b);
            let circuit_a = &mut left[a];
            let circuit_b = &mut right[0];
            circuit_a.extend(circuit_b.iter().cloned());
            circuits.remove(b);
        }
    }

    // get biggest 3 and multiply
    let mut circuit_sizes: Vec<u64> = circuits
        .iter()
        .map(|circuit| circuit.len() as u64)
        .collect();
    circuit_sizes.sort();
    circuit_sizes.reverse();
    circuit_sizes.truncate(3);

    Some(circuit_sizes.iter().fold(1, |acc, next| acc * next))
}

fn get_all_distances(junction_boxes: &'_ Vec<Vector3d>) -> Vec<Connection<'_>> {
    let mut connections: Vec<Connection> = Vec::new();
    for i in 0..junction_boxes.len() - 1 {
        for j in i + 1..junction_boxes.len() {
            let a = &junction_boxes[i];
            let b = &junction_boxes[j];
            connections.push(Connection {
                a: &a,
                b: &b,
                distance: linear_distance(a, b),
            });
        }
    }
    connections
}

fn parse_junction_boxes(input: &str) -> Vec<Vector3d> {
    input
        .lines()
        .map(|line| Vector3d::from_str(line))
        .collect::<Vec<Vector3d>>()
}

pub fn part_two(input: &str) -> Option<u64> {
    let junction_boxes = parse_junction_boxes(input);

    let mut connections = get_all_distances(&junction_boxes);
    connections.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    // group to circuits
    let mut circuits: Vec<Vec<&Vector3d>> = Vec::new();
    let mut last_connection: Option<Connection> = None;
    for connection in connections {
        let mut circuit_a_index: Option<usize> = None;
        let mut circuit_b_index: Option<usize> = None;

        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&connection.a) {
                circuit_a_index = Some(i);
            }
            if circuit.contains(&connection.b) {
                circuit_b_index = Some(i);
            }
        }

        if circuit_a_index == None && circuit_b_index == None {
            circuits.push(vec![connection.a, connection.b]);
        } else if circuit_a_index.is_some() && circuit_b_index.is_none() {
            circuits[circuit_a_index.unwrap()].push(&connection.b);
            last_connection = Some(connection);
        } else if circuit_a_index.is_none() && circuit_b_index.is_some() {
            circuits[circuit_b_index.unwrap()].push(&connection.a);
            last_connection = Some(connection);
        } else {
            let a = circuit_a_index.unwrap();
            let b = circuit_b_index.unwrap();

            if a == b {
                continue;
            }

            last_connection = Some(connection);

            let (a, b) = if a < b { (a, b) } else { (b, a) };
            let (left, right) = circuits.split_at_mut(b);
            let circuit_a = &mut left[a];
            let circuit_b = &mut right[0];
            circuit_a.extend(circuit_b.iter().cloned());
            circuits.remove(b);
        }
    }

    last_connection.map(|connection| (connection.a.x * connection.b.x) as u64)
}

#[derive(Debug)]
struct Connection<'a> {
    a: &'a Vector3d,
    b: &'a Vector3d,
    distance: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part1(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
