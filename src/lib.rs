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
}
