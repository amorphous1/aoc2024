use std::collections::{HashMap, HashSet};

const NEIGHBOURS: [(i16, i16); 4] = [(-1 , 0), (1, 0), (0, -1), (0, 1)];
const EMPTY: char = ' ';

pub fn fence_price(input: &str, bulk_discount: bool) -> usize {
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
        let (region, plant) = expand_region(next, &pos_to_plant);
        region.iter().for_each(|region_pos| {
           remaining_pos.take(region_pos);
        });
        let multiplier = if bulk_discount { sides(&region, plant, &pos_to_plant) } else { circumference(&region, plant, &pos_to_plant) };
        result += region.len() * multiplier;
    }
    return result;
}

fn expand_region(start_pos: (i16, i16), pos_to_plant: &HashMap<(i16, i16), char>) -> (HashSet<(i16, i16)>, char) {
    let mut region = HashSet::new();
    let mut frontier = Vec::new();
    frontier.push(start_pos);
    let region_plant = *pos_to_plant.get(&start_pos).unwrap();

    while !frontier.is_empty() {
        let current: (i16,i16) = frontier.pop().unwrap();
        NEIGHBOURS.iter().map(|(dx, dy)| (current.0 + dx, current.1 + dy)).for_each(|neighbour| {
            let neighbour_plant = *pos_to_plant.get(&neighbour).unwrap_or(&EMPTY);
            if neighbour_plant == region_plant && !region.contains(&neighbour) && !frontier.contains(&neighbour) {
                frontier.push(neighbour);
            }
        });
        region.insert(current);
    }
    return (region, region_plant);
}
fn circumference(region: &HashSet<(i16, i16)>, region_plant: char, pos_to_plant: &HashMap<(i16, i16), char>) -> usize {
    let mut result = 0;
    region.iter().for_each(|pos| {
        NEIGHBOURS.iter().map(|(dx, dy)| (pos.0 + dx, pos.1 + dy)).for_each(|neighbour| {
            let neighbour_plant = *pos_to_plant.get(&neighbour).unwrap_or(&EMPTY);
            if neighbour_plant != region_plant {
                result += 1;
            }
        });
    });
    return result;
}

fn sides(region: &HashSet<(i16, i16)>, plant: char, pos_to_plant: &HashMap<(i16, i16), char>) -> usize {
    let mut result = 0;
    let mut sorted_region = region.iter().cloned().collect::<Vec<_>>();
    sorted_region.sort();
    sorted_region.iter().for_each(|(x, y)| {
        let top_plant = *pos_to_plant.get(&(*x, *y-1)).unwrap_or(&EMPTY);
        let left_plant = *pos_to_plant.get(&(*x-1, *y)).unwrap_or(&EMPTY);
        let right_plant = *pos_to_plant.get(&(*x+1, *y)).unwrap_or(&EMPTY);
        let bottom_plant = *pos_to_plant.get(&(*x, *y+1)).unwrap_or(&EMPTY);
        let top_left_plant = *pos_to_plant.get(&(*x-1, *y-1)).unwrap_or(&EMPTY);
        let top_right_plant = *pos_to_plant.get(&(*x+1, *y-1)).unwrap_or(&EMPTY);
        let bottom_left_plant = *pos_to_plant.get(&(*x-1, *y+1)).unwrap_or(&EMPTY);
        if left_plant != plant && !(top_plant == plant && top_left_plant != plant) {
            result += 1;
        }
        if right_plant != plant && !(top_plant == plant && top_right_plant != plant) {
            result += 1;
        }
        if top_plant != plant && !(left_plant == plant && top_left_plant != plant) {
            result += 1;
        }
        if bottom_plant != plant && !(left_plant == plant && bottom_left_plant != plant) {
            result += 1;
        }
    });
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
        assert_eq!(fence_price(sample_input, false), 1930);
        assert_eq!(fence_price(sample_input, true), 1206);
    }
}