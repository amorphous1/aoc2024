use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Guard {
    position: (i16, i16),
    orientation: usize,
}

#[derive(Debug)]
struct LabMap {
    size: i16,
    walls: HashSet<(i16, i16)>,
    guard: Guard,
}

fn parse_lab_map(input: &str) -> LabMap {
    let grid: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect()).collect();
    let mut guard_position: (i16, i16) = (-1, -1);
    let mut walls = HashSet::new();
    for y in 0..grid.len() {
        for x in 0.. grid.get(y).unwrap().len() {
            if grid[y][x] == '^' {
                guard_position = (x as i16, y as i16);
            }
            else if grid[y][x] == '#' {
                walls.insert((x as i16, y as i16));
            }
        }
    }
    return LabMap {
        size: grid.len() as i16,
        walls,
        guard: Guard { position: guard_position, orientation: 0 },
    }
}

pub fn visited_positions(input: &str) -> usize {
    let lab_map = parse_lab_map(input);
    let mut guard = lab_map.guard.clone();
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    println!("{:?}", lab_map);
    let mut visited = HashSet::new();
    loop {
        visited.insert(guard.position);
        while lab_map.walls.contains(&(guard.position.0 + directions[guard.orientation].0, guard.position.1 + directions[guard.orientation].1)) {
            guard.orientation = (guard.orientation + 1) % 4;
        }
        guard = Guard {
            position: (guard.position.0 + directions[guard.orientation].0, guard.position.1 + directions[guard.orientation].1),
            orientation: guard.orientation,
        };
        if guard.position.0 < 0 || guard.position.0 >= lab_map.size || guard.position.1 < 0 || guard.position.1 >= lab_map.size {
            break;
        }
    }
    return visited.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(visited_positions(sample_input), 41);
    }
}