use std::collections::{HashMap, HashSet};

pub fn unique_antinodes(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect()).collect();
    let mut antennas: HashMap<char, Vec<(i16, i16)>> = HashMap::new();
    for y in 0..grid.len() {
        for x in 0.. grid.get(y).unwrap().len() {
            if grid[y][x] != '.' {
                antennas.entry(grid[y][x]).or_insert(Vec::new()).push((x as i16, y as i16));
            }
        }
    }
    let grid_size = grid.len() as i16;

    let mut antinodes: HashSet<(i16, i16)> = HashSet::new();
    for antenna in antennas {
        let locations = antenna.1;
        for a in 0..locations.len()-1 {
            for b in a+1..locations.len() {
                let antinode1 = (locations[a].0 + locations[a].0 - locations[b].0, locations[a].1 + locations[a].1 - locations[b].1);
                let antinode2 = (locations[b].0 + locations[b].0 - locations[a].0, locations[b].1 + locations[b].1 - locations[a].1);
                if antinode1.0 >=0 && antinode1.0 < grid_size && antinode1.1 >=0 && antinode1.1 < grid_size {
                    antinodes.insert(antinode1);
                }
                if antinode2.0 >=0 && antinode2.0 < grid_size && antinode2.1 >=0 && antinode2.1 < grid_size {
                    antinodes.insert(antinode2);
                }
            }
        }
    }
    return antinodes.len();
}

pub fn unique_antinodes2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect()).collect();
    let mut antennas: HashMap<char, Vec<(i16, i16)>> = HashMap::new();
    for y in 0..grid.len() {
        for x in 0.. grid.get(y).unwrap().len() {
            if grid[y][x] != '.' {
                antennas.entry(grid[y][x]).or_insert(Vec::new()).push((x as i16, y as i16));
            }
        }
    }
    let grid_size = grid.len() as i16;

    let mut antinodes: HashSet<(i16, i16)> = HashSet::new();
    for antenna in antennas {
        let locations = antenna.1;
        for a in 0..locations.len()-1 {
            for b in a+1..locations.len() {
                let mut in_bounds = true;
                let mut step = -1;
                while in_bounds {
                    let antinode = (locations[a].0 + step * (locations[a].0 - locations[b].0), locations[a].1 + step * (locations[a].1 - locations[b].1));
                    if antinode.0 >=0 && antinode.0 < grid_size && antinode.1 >=0 && antinode.1 < grid_size {
                        antinodes.insert(antinode);
                        step -= 1;
                    } else {
                        in_bounds = false;
                    }

                }
                let mut in_bounds = true;
                let mut step = 0;
                while in_bounds {
                    let antinode = (locations[a].0 + step * (locations[a].0 - locations[b].0), locations[a].1 + step * (locations[a].1 - locations[b].1));
                    if antinode.0 >=0 && antinode.0 < grid_size && antinode.1 >=0 && antinode.1 < grid_size {
                        antinodes.insert(antinode);
                        step += 1;
                    } else {
                        in_bounds = false;
                    }

                }
            }
        }
    }
    return antinodes.len();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(unique_antinodes(sample_input), 14);
        assert_eq!(unique_antinodes2(sample_input), 34);
    }
}