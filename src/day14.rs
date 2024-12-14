use std::collections::HashSet;
use regex::Regex;

pub fn safety_factor(input: &str, steps: u16, width: u16, height: u16) -> u64 {
    let line_regex = Regex::new(r"p=([-0-9]+),([-0-9]+) v=([-0-9]+),([-0-9]+)").unwrap();
    let mut quadrants: Vec<u64> = vec![0, 0, 0, 0, 0];
    for (_, [px, py, vx, vy]) in line_regex.captures_iter(input).map(|c| c.extract()) {
        let (px, py, vx, vy) = (px.parse::<i64>().unwrap(), py.parse::<i64>().unwrap(), vx.parse::<i64>().unwrap(), vy.parse::<i64>().unwrap());
        let mut px = (px + vx * steps as i64) % width as i64;
        while px < 0 {
            px += width as i64;
        }
        let mut py = (py + vy * steps as i64) % height as i64;
        while py < 0 {
            py += height as i64;
        }
        let qx = (px - (width as i64 - 1) / 2).signum();
        let qy = (py - (height as i64 - 1) / 2).signum();
        let quadrant = match (qx, qy)   {
            (-1, -1) => 1,
            (1, -1) => 2,
            (-1, 1) => 3,
            (1, 1) => 4,
            _ => 0,
        };
        quadrants[quadrant] += 1;
    }
    return quadrants[1] * quadrants[2] * quadrants[3] * quadrants[4];
}

pub fn print_robots(input: &str, steps: u16, width: u16, height: u16) {
    let line_regex = Regex::new(r"p=([-0-9]+),([-0-9]+) v=([-0-9]+),([-0-9]+)").unwrap();
    let mut robots: HashSet<(i64, i64)> = HashSet::new();
    for (_, [px, py, vx, vy]) in line_regex.captures_iter(input).map(|c| c.extract()) {
        let (px, py, vx, vy) = (px.parse::<i64>().unwrap(), py.parse::<i64>().unwrap(), vx.parse::<i64>().unwrap(), vy.parse::<i64>().unwrap());
        let mut px = (px + vx * steps as i64) % width as i64;
        while px < 0 {
            px += width as i64;
        }
        let mut py = (py + vy * steps as i64) % height as i64;
        while py < 0 {
            py += height as i64;
        }
        robots.insert((px, py));
    }
    if is_print_candidate(&robots) {
        println!("{} ==================================================================================", steps);
        for y in 0..height as i64 {
            for x in 0..width as i64 {
                print!("{}", if robots.contains(&(x, y)) { '#' } else { ' ' });
            }
            println!();
        }
    }
}

fn is_print_candidate(robots: &HashSet<(i64, i64)>) -> bool {
    for (x,y) in robots {
        if robots.contains(&(*x + 1, *y)) && robots.contains(&(*x + 2, *y)) && robots.contains(&(*x + 3, *y)) && robots.contains(&(*x + 4, *y)) &&
            robots.contains(&(*x, *y + 1)) && robots.contains(&(*x, *y + 2)) && robots.contains(&(*x, *y + 3)) && robots.contains(&(*x, *y + 4)) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(safety_factor(sample_input, 100, 11, 7), 12);
    }
}