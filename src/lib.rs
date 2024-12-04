use std::collections::HashMap;

use regex::Regex;

pub fn day01a(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    input.split('\n').for_each(|line| {
        let mut it = line.split_ascii_whitespace();
        left.push(it.next().unwrap().parse().unwrap());
        right.push(it.next().unwrap().parse().unwrap());
    });
    left.sort();
    right.sort();

    let mut result: u32 = 0;
    for (index, elem) in left.iter().enumerate() {
        result += elem.abs_diff(right[index]);
    }
    return result;
}

pub fn day01b(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right_counts: HashMap<u32, u32> = HashMap::new();
    input.split('\n').for_each(|line| {
        let mut it = line.split_ascii_whitespace();
        left.push(it.next().unwrap().parse().unwrap());
        let right_elem = it.next().unwrap().parse().unwrap();
        let existing_count = right_counts.get(&right_elem).cloned().unwrap_or(0);
        right_counts.insert(right_elem, existing_count + 1);
    });

    let mut result: u32 = 0;
    for  elem in left.iter() {
        let elem_count = right_counts.get(elem).cloned().unwrap_or(0);
        result += elem * elem_count;
    }
    return result;
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut last_sig = (report[1] - report[0]).signum();
    for i  in 1..report.len() {
        let delta = report[i] - report[i-1];
        if delta.signum() != last_sig || delta.abs() < 1 || delta.abs() > 3 {
            return false;
        }
        last_sig = delta.signum();
    }
    return true;
}

fn is_safe_using_dampener(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let dampened_report = [&report[..i], &report[i+1..]].concat();
        if is_safe(&dampened_report) {
            return true;
        }
    }
    return false;
}

pub fn day02(input: &str, use_dampener: bool) -> usize {
    return input.split('\n').filter(|line| {
        let report = line.split_ascii_whitespace().map(|num| num.parse().unwrap()).collect();
        return if use_dampener { is_safe_using_dampener(&report) } else { is_safe(&report) };
    }).count();
}

pub fn day03a(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut result = 0;
    for (_, [factor1, factor2]) in re.captures_iter(input).map(|c| c.extract()) {
        let factor1: u32 = factor1.parse().unwrap();
        let factor2: u32 = factor2.parse().unwrap();
        result += factor1 * factor2;
    }
    return result;
}

pub fn day03b(input: &str) -> u32 {
    let mut result = 0;
    input.split("do()").for_each(|do_block| {
        result += day03a(do_block.split("don't()").into_iter().next().unwrap());
    });
    return result;
}

pub fn day04a(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.split('\n').map(|line| line.chars().collect()).collect();
    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut result = 0;
    let directions = vec![(-1,-1), (0,-1), (1,-1), (1,0), (1,1), (0,1), (-1,1), (-1,0)];

    for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            result += find_in(&grid, &xmas, x, y, &directions);
        }
    }
    return result;
}

fn find_in(grid: &Vec<Vec<char>>, term: &Vec<char>, x: i32, y: i32, directions: &Vec<(i32,i32)>) -> u32 {
    if y < 0 || y >= grid.len() as i32 || x < 0 || x >= grid[y as usize].len() as i32 || grid[y as usize][x as usize] != term[0] {
        return 0;
    }
    if term.len() == 1 {
        return 1;
    }
    return directions.iter()
        .map(|dir| find_in(grid, &term[1..].to_vec(), x + dir.0, y + dir.1, &vec![*dir]))
        .sum();
}

pub fn day04b(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.split('\n').map(|line| line.chars().collect()).collect();
    let mut result = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x] == 'A' &&
                (grid[y-1][x-1] == 'M' && grid[y+1][x+1] == 'S' || grid[y-1][x-1] == 'S' && grid[y+1][x+1] == 'M') &&
                (grid[y-1][x+1] == 'M' && grid[y+1][x-1] == 'S' || grid[y-1][x+1] == 'S' && grid[y+1][x-1] == 'M') {
                result += 1;
            }
        }
    }
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_samples() {
        let sample_input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(day01a(sample_input), 11);
        assert_eq!(day01b(sample_input), 31);
    }

    #[test]
    fn day02_samples() {
        let sample_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(day02(sample_input, false), 2);
        assert_eq!(day02(sample_input, true), 4);
    }

    #[test]
    fn day03_samples() {
        assert_eq!(day03a("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
        assert_eq!(day03b("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
    }

    #[test]
    fn day04_samples() {
        let sample_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(day04a(sample_input), 18);
        assert_eq!(day04b(sample_input), 9);
    }

}
