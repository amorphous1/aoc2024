use std::cmp::min;

use regex::Regex;

pub fn min_tokens(input: &str) -> usize {
    let machine_regex = Regex::new(r"Button A.*\+([0-9]+),.*\+([0-9]+)\nButton B.*\+([0-9]+),.*\+([0-9]+)\nPrize.*=([0-9]+),.*=([0-9]+)").unwrap();
    let mut result = 0;
    for (_, [ax, ay, bx, by, x, y]) in machine_regex.captures_iter(input).map(|c| c.extract()) {
        let ax: usize = ax.parse().unwrap();
        let ay: usize = ay.parse().unwrap();
        let bx: usize = bx.parse().unwrap();
        let by: usize = by.parse().unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        let max_push_a = min(100, min(x / ax, y / ay));
        let mut min_tokens = usize::MAX;
        for push_a in 0..max_push_a + 1 {
            let (remaining_x, remaining_y) = (x - ax * push_a, y - ay * push_a);
            let push_b = remaining_x / bx;
            if push_b * bx == remaining_x && push_b * by == remaining_y {
                min_tokens = min(min_tokens, 3 * push_a + push_b);
            }
        }
        if min_tokens < usize::MAX {
            result += min_tokens;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(min_tokens(sample_input), 480);
    }
}