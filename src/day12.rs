use std::collections::{HashMap, HashSet};

const NEIGHBOURS: [(i16, i16); 4] = [(-1 , 0), (1, 0), (0, -1), (0, 1)];
const NOTHING: char = ' ';

fn expand_region(start_pos: (i16, i16), pos_to_plant: &HashMap<(i16, i16), char>) -> (HashSet<(i16, i16)>, usize) {
    let mut circumference = 0;
    let mut region = HashSet::new();
    let mut frontier = Vec::new();
    frontier.push(start_pos);
    let region_plant = *pos_to_plant.get(&start_pos).unwrap();

    while !frontier.is_empty() {
        let current: (i16,i16) = frontier.pop().unwrap();
        NEIGHBOURS.iter().map(|(dx, dy)| (current.0 + dx, current.1 + dy)).for_each(|neighbour| {
            let neighbour_plant = *pos_to_plant.get(&neighbour).unwrap_or(&NOTHING);
            if neighbour_plant != region_plant {
                circumference += 1;
            } else if neighbour_plant == region_plant && !region.contains(&neighbour) && !frontier.contains(&neighbour) {
                frontier.push(neighbour);
            }
        });
        region.insert(current);
    }
    return (region, circumference);
}
pub fn fence_price(input: &str) -> usize {
    let mut pos_to_plant: HashMap<(i16, i16), char> = HashMap::new();
    input.split('\n').enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, plant)| {
            pos_to_plant.insert((x as i16, y as i16), plant);
        })
    });
    let mut remaining_pos = pos_to_plant.keys().cloned().collect::<HashSet<_>>();
    let mut result = 0;
    while !remaining_pos.is_empty() {
        let next = remaining_pos.iter().next().cloned().unwrap();
        let (region, circumference) = expand_region(next, &pos_to_plant);
        region.iter().for_each(|region_pos| {
           remaining_pos.take(region_pos);
        });
        result += region.len() * circumference;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(fence_price(sample_input), 1930);
    }
}