use std::collections::HashSet;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

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

#[derive(Clone, Debug)]
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

pub fn visited_positions(input: &str) -> HashSet<(i16, i16)> {
    let lab_map = parse_lab_map(input);
    let mut guard = lab_map.guard.clone();
    let mut visited = HashSet::new();
    while !guard.is_out_of_bounds(lab_map.size) {
        visited.insert(guard.position);
        guard = guard.step(&lab_map.walls);
    }
    return visited;
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
    let lab_map1 = parse_lab_map(input);
    let lab_map2 = parse_lab_map(input);
    let lab_map3 = parse_lab_map(input);
    let lab_map4 = parse_lab_map(input);
    let mut positions1 = visited_positions(input).into_iter().collect::<Vec<_>>();
    let mut positions2 = positions1.split_off(positions1.len() / 4);
    let mut positions3 = positions2.split_off(positions2.len() / 3);
    let positions4 = positions3.split_off(positions3.len() / 2);

    let (tx1, rx): (Sender<usize>, Receiver<_>) = mpsc::channel();
    let tx2: Sender<usize> = tx1.clone();
    let tx3 = tx1.clone();
    let tx4 = tx1.clone();

    thread::spawn(move || {
        let result = positions1.iter().map(|position| {
            if has_loop(&lab_map1, *position) { 1 } else { 0 }
        }).sum();
        tx1.send(result).unwrap();
    });
    thread::spawn(move || {
        let result = positions2.iter().map(|position| {
            if has_loop(&lab_map2, *position) { 1 } else { 0 }
        }).sum();
        tx2.send(result).unwrap();
    });
    thread::spawn(move || {
        let result = positions3.iter().map(|position| {
            if has_loop(&lab_map3, *position) { 1 } else { 0 }
        }).sum();
        tx3.send(result).unwrap();
    });
    thread::spawn(move || {
        let result = positions4.iter().map(|position| {
            if has_loop(&lab_map4, *position) { 1 } else { 0 }
        }).sum();
        tx4.send(result).unwrap();
    });

    return rx.iter().sum();
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

        assert_eq!(visited_positions(sample_input).len(), 41);
        assert_eq!(loop_obstructions(sample_input), 6);
    }
}