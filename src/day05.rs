use std::cmp::Ordering;
use std::collections::HashMap;

fn parse_rules(rules: &str) -> HashMap<u8, Vec<u8>> {
    let mut result = HashMap::new();
    rules.split('\n').for_each(|line| {
        let [before, after] = line.split('|').take(2).map(|num| num.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap();
        result.entry(before).or_insert(Vec::new()).push(after);
    });
    return result;
}

fn parse_updates(updates: &str) -> Vec<Vec<u8>> {
    return updates.split('\n').map(|update| {
        return update.split(',').map(|num| num.parse().unwrap()).collect::<Vec<u8>>();
    }).collect();
}

pub fn day05(input: &str, count_sorted: bool) -> u32 {
    let [rules, updates] = input.split("\n\n").take(2).collect::<Vec<&str>>().try_into().unwrap();
    let is_before = parse_rules(rules);
    let mut result = 0;
    parse_updates(updates).iter().for_each(|update| {
        let mut sorted_update = update.clone();
        sorted_update.sort_by(|a,b| {
            if is_before.get(a).is_some_and(|later_pages| later_pages.contains(b)) {
                return Ordering::Less
            }
            if is_before.get(b).is_some_and(|later_pages| later_pages.contains(a)) {
                return Ordering::Greater
            }
            return Ordering::Equal;
        });
        if (count_sorted && *update == sorted_update) || (!count_sorted && *update != sorted_update) {
            result += sorted_update[sorted_update.len()/2] as u32;
        }
    });
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(day05(sample_input, true), 143);
        assert_eq!(day05(sample_input, false), 123);
    }
}