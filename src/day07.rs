fn can_be_solved(result: usize, operands: &Vec<usize>) -> bool {
    if operands.len() == 0 {
        return false;
    }
    let op1 = *operands.get(0).unwrap();
    if operands.len() == 1 {
        return result == op1;
    }
    let op2 = *operands.get(1).unwrap();
    let rest = operands[2..].to_vec();
    return can_be_solved(result, &[&[op1 + op2], &rest[..]].concat()) ||
        can_be_solved(result, &[&[op1 * op2], &rest[..]].concat());
}

pub fn part1(input: &str) -> usize {
    let equations: Vec<(usize, Vec<usize>)> = input.split('\n').map(|line| {
        let mut it = line.split(": ");
        let left = it.next().unwrap().parse().unwrap();
        let right = it.next().unwrap().split_ascii_whitespace().map(|num| num.parse().unwrap()).collect();
        return (left, right);
    }).collect::<Vec<_>>();

    return equations.iter().map(|equation| {
        if can_be_solved(equation.0, &equation.1) { equation.0 } else { 0 }
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(part1(sample_input), 3749);
    }
}