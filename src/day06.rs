use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    position: (i16, i16),
    orientation: usize,
}

impl Guard {
    const  DIRECTIONS: [(i16, i16); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    fn maybe_step(&self, new_orientation: usize) -> (i16, i16) {
        return (self.position.0 + Self::DIRECTIONS[new_orientation].0, self.position.1 + Self::DIRECTIONS[new_orientation].1);
    }

    pub fn step(&self, walls: &HashSet<(i16, i16)>) -> Guard {
        let mut  new_orientation = self.orientation;
        while walls.contains(&self.maybe_step(new_orientation)) {
            new_orientation = (new_orientation + 1) % Self::DIRECTIONS.len();
        }
        return  Guard {
            position: self.maybe_step(new_orientation),
            orientation: new_orientation,
        };
    }

    pub fn is_out_of_bounds(&self, size: i16) -> bool {
        return  self.position.0 < 0 || self.position.0 >= size || self.position.1 < 0 || self.position.1 >= size;
    }
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
    let mut visited = HashSet::new();
    while !guard.is_out_of_bounds(lab_map.size) {
        visited.insert(guard.position);
        guard = guard.step(&lab_map.walls);
    }
    return visited.len();
}

fn has_loop(lab_map: &LabMap, obstruction: (i16, i16)) -> bool {
    if obstruction == lab_map.guard.position || lab_map.walls.contains(&obstruction) {
        return false
    }
    let mut guard = lab_map.guard.clone();
    let mut walls = lab_map.walls.clone();
    walls.insert(obstruction);
    let mut visited = HashSet::new();
    while !guard.is_out_of_bounds(lab_map.size) {
        visited.insert(guard.clone());
        guard = guard.step(&walls);
        if visited.contains(&guard) {
            return true;
        }
    }
    return false;
}

pub fn loop_obstructions(input: &str) -> usize {
    let lab_map = parse_lab_map(input);
    let mut result = 0;
    for y in 0..lab_map.size {
        for x in 0..lab_map.size {
            if has_loop(&lab_map, (x, y)) {
                result += 1;
            }
        }
    }
    return result;
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
        assert_eq!(loop_obstructions(sample_input), 6);
    }
}