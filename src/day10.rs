use std::collections::{HashMap, HashSet};

pub fn trailhead_scores(input: &str) -> (usize, usize) {
    let mut pos_to_height: HashMap<(i16, i16), u8> = HashMap::new();
    let mut height_to_pos: HashMap<u8, Vec<(i16, i16)>> = HashMap::new();

    input.split('\n').into_iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            let height = ch.to_digit(10).unwrap() as u8;
            let pos = (x as i16, y as i16);
            pos_to_height.insert(pos, height);
            height_to_pos.entry(height).or_insert(Vec::new()).push(pos);
        });
    });

    let mut pos_to_tops: HashMap<(i16, i16), HashSet<(i16, i16)>> = HashMap::new();
    let mut pos_to_rating: HashMap<(i16, i16), usize> = HashMap::new();
    height_to_pos.get(&9).unwrap().iter().for_each(|pos| {
        pos_to_tops.entry(*pos).or_insert(HashSet::new()).insert(*pos);
        pos_to_rating.insert(*pos, 1);
    });
    for height in (0..9).rev() {
        height_to_pos.get(&height).unwrap().iter().for_each(|(x, y)| {
            vec![(*x - 1, *y), (*x + 1, *y), (*x, *y - 1), (*x, *y + 1)].iter().for_each(|neighbour| {
                if *pos_to_height.get(neighbour).unwrap_or(&11) == height + 1 {
                    let neighbour_tops = pos_to_tops.get(neighbour).unwrap_or(&HashSet::new()).clone();
                    let tops = pos_to_tops.entry((*x, *y)).or_insert(HashSet::new());
                    let neighbour_rating = pos_to_rating.get(neighbour).unwrap_or(&0).clone();
                    *pos_to_rating.entry((*x, *y)).or_default() += neighbour_rating;
                    neighbour_tops.iter().for_each(|neigbour_top| {
                        tops.insert(*neigbour_top);
                    });
                }
            });
        });
    }
    let part1 = height_to_pos.get(&0).unwrap().iter()
        .map(|pos| pos_to_tops.get(pos).unwrap_or(&HashSet::new()).len())
        .sum();
    let part2 = height_to_pos.get(&0).unwrap().iter()
        .map(|pos| pos_to_rating.get(pos).unwrap_or(&0))
        .sum();
    return (part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(trailhead_scores(sample_input), (36, 81));
    }
}