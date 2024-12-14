use regex::Regex;

pub fn min_tokens(input: &str, prize_offset: usize) -> u64 {
    let machine_regex = Regex::new(r"Button A.*\+([0-9]+),.*\+([0-9]+)\nButton B.*\+([0-9]+),.*\+([0-9]+)\nPrize.*=([0-9]+),.*=([0-9]+)").unwrap();
    let mut result = 0;
    for (_, [ax, ay, bx, by, x, y]) in machine_regex.captures_iter(input).map(|c| c.extract()) {
        let (ax, ay) = (ax.parse::<f64>().unwrap(), ay.parse::<f64>().unwrap());
        let (bx, by) = (bx.parse::<f64>().unwrap(), by.parse::<f64>().unwrap());
        let x: f64 = x.parse::<f64>().unwrap() + prize_offset as f64;
        let y: f64 = y.parse::<f64>().unwrap() + prize_offset as f64;

        // found by solving two equations for a and b
        //   a*ax + b*bx = x
        //   a*ay + b*by = y
        let a = (x - y*bx/by) / (ax - ay*bx/by);
        let b = (y - ay*a) / by;

        let (a,b, ax, ay, bx, by, x, y) = (a.round() as u64, b.round() as u64, ax as u64, ay as u64, bx as u64, by as u64, x as u64, y as u64);
        if a * ax + b * bx == x && a * ay + b * by == y {
            result += 3 * a + b;
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
        assert_eq!(min_tokens(sample_input, 0 ), 480);
        assert_eq!(min_tokens(sample_input, 10000000000000), 875318608908);
    }
}