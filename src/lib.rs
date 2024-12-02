use std::collections::HashMap;

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
}
