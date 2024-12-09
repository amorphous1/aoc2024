use std::cmp::min;

pub fn checksum(input: &str) -> usize {
    let mut result = 0;
    let disk_map = input.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
    let mut file_map = disk_map.iter().step_by(2).map(|num| *num).collect::<Vec<usize>>();
    let mut free_map = disk_map.iter().skip(1).step_by(2).map(|num| *num).collect::<Vec<usize>>();
    let mut left_id = 0;
    let mut right_id = file_map.len() - 1;
    let mut block = 0;
    while left_id <= right_id {
        result += left_id * (block..block + file_map[left_id]).sum::<usize>();
        block += file_map[left_id];
        // println!("processed file - block={}, left_id={}, right_id={}, result={}", block, left_id, right_id, result);

        // fill empty space after left_id
        while free_map[left_id] > 0 && right_id > left_id {
            let blocks = min(free_map[left_id], file_map[right_id]);
            free_map[left_id] -= blocks;
            file_map[right_id] -= blocks;
            result += right_id * (block..block + blocks).sum::<usize>();
            block += blocks;
            if file_map[right_id] == 0 {
                right_id -= 1;
            }
            // println!("processed empty space - block={}, left_id={}, right_id={}, result={}", block, left_id, right_id, result);
        }
        left_id += 1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "2333133121414131402";
        assert_eq!(checksum(sample_input), 1928);
    }
}