use regex::Regex;

pub fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut result = 0;
    for (_, [factor1, factor2]) in re.captures_iter(input).map(|c| c.extract()) {
        let factor1: u32 = factor1.parse().unwrap();
        let factor2: u32 = factor2.parse().unwrap();
        result += factor1 * factor2;
    }
    return result;
}

pub fn part2(input: &str) -> u32 {
    let mut result = 0;
    input.split("do()").for_each(|do_block| {
        result += part1(do_block.split("don't()").into_iter().next().unwrap());
    });
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
        assert_eq!(part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
    }
}