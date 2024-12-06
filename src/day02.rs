fn is_safe_without_dampener(report: &Vec<i32>) -> bool {
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
        if is_safe_without_dampener(&dampened_report) {
            return true;
        }
    }
    return false;
}

pub fn is_safe(input: &str, use_dampener: bool) -> usize {
    return input.split('\n').filter(|line| {
        let report = line.split_ascii_whitespace().map(|num| num.parse().unwrap()).collect();
        return if use_dampener { is_safe_using_dampener(&report) } else { is_safe_without_dampener(&report) };
    }).count();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(is_safe(sample_input, false), 2);
        assert_eq!(is_safe(sample_input, true), 4);
    }
}