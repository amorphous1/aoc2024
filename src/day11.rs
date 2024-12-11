fn count_stones(stone: usize, steps: u16) -> usize {
    if steps == 0 {
        return 1;
    }
    if stone == 0 {
        return count_stones(1, steps - 1);
    }
    let stone_text = stone.to_string();
    if stone_text.len() % 2 == 0 {
        return count_stones(stone_text[..stone_text.len() / 2].parse().unwrap(), steps - 1) +
            count_stones(stone_text[stone_text.len() / 2..].parse().unwrap(), steps - 1);
    }
    return count_stones(stone * 2024, steps - 1);
}
pub fn day11(input: &str, steps: u16) -> usize {
    let stones = input.split_ascii_whitespace().map(|num| num.parse().unwrap()).collect::<Vec<usize>>();
    return stones.iter().map(|stone| count_stones(*stone, steps)).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "125 17";
        assert_eq!(day11(sample_input, 25), 55312);
    }
}