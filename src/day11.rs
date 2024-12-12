use std::collections::HashMap;

fn count_stones(stone: usize, steps: u16, memo: &mut HashMap<(usize, u16), usize>) -> usize {
    if memo.contains_key(&(stone, steps)) {
        return *memo.get(&(stone, steps)).unwrap();
    }
    if steps == 0 {
        return 1;
    }
    if stone == 0 {
        return count_stones(1, steps - 1, memo);
    }
    let stone_text = stone.to_string();
    if stone_text.len() % 2 == 0 {
        let left_stone = stone_text[..stone_text.len() / 2].parse().unwrap();
        let left_count = count_stones(left_stone, steps - 1, memo);
        memo.insert((left_stone, steps - 1), left_count);
        let right_stone = stone_text[stone_text.len() / 2..].parse().unwrap();
        let right_count = count_stones(right_stone, steps - 1, memo);
        memo.insert((right_stone, steps - 1), right_count);
        return left_count + right_count;
    }
    return count_stones(stone * 2024, steps - 1, memo);
}
pub fn day11(input: &str, steps: u16) -> usize {
    let mut memo = HashMap::new();
    return input.split_ascii_whitespace()
        .map(|stone| count_stones(stone.parse().unwrap(), steps, &mut memo))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(day11("125 17", 25), 55312);
    }
}