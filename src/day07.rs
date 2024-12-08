fn can_be_solved(result: usize, operands: &Vec<usize>, use_concat_operator: bool) -> bool {
    if operands.len() == 0 {
        return false;
    }
    let op1 = *operands.get(0).unwrap();
    if operands.len() == 1 {
        return result == op1;
    }
    let op2 = *operands.get(1).unwrap();
    let rest = operands[2..].to_vec();
    let solvable = can_be_solved(result, &[&[op1 + op2], &rest[..]].concat(), use_concat_operator) ||
        can_be_solved(result, &[&[op1 * op2], &rest[..]].concat(), use_concat_operator);
    if use_concat_operator && !solvable {
        let op1op2 = format!("{}{}", op1.to_string(), op2.to_string()).parse().unwrap();
        return can_be_solved(result, &[&[op1op2], &rest[..]].concat(), use_concat_operator);
    }
    return solvable;
}

pub fn sum_solvable_equations(input: &str, use_concat_operator: bool) -> usize {
    let equations: Vec<(usize, Vec<usize>)> = input.split('\n').map(|line| {
        let mut it = line.split(": ");
        let left = it.next().unwrap().parse().unwrap();
        let right = it.next().unwrap().split_ascii_whitespace().map(|num| num.parse().unwrap()).collect();
        return (left, right);
    }).collect::<Vec<_>>();

    return equations.iter().map(|equation| {
        if can_be_solved(equation.0, &equation.1, use_concat_operator) { equation.0 } else { 0 }
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

        assert_eq!(sum_solvable_equations(sample_input, false), 3749);
        assert_eq!(sum_solvable_equations(sample_input, true), 11387);
    }
}