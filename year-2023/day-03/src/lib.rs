use std::cmp;
use std::fs;

fn part1(input: &str) -> i32 {
    let mut part_number_sum = 0;
    input.lines().enumerate().for_each(|(index, line)| {
        let mut mutable_line = line;
        while let Some(number_start_index) = mutable_line.find(|c: char| c.is_numeric()) {
            let number_end_index = mutable_line[number_start_index..].find(|c: char| !c.is_numeric()).unwrap_or(mutable_line.len() - number_start_index);
            let number_string = &mutable_line[number_start_index..number_start_index + number_end_index];

            let diff: usize = line.len() - mutable_line.len();
            let search_start_index: usize = cmp::max((number_start_index + diff) as i32 - 1, 0) as usize;
            let search_end_index: usize = cmp::min((number_start_index + number_end_index) + diff + 1, line.len());
            let neighbor_range = search_start_index..search_end_index;

            let mut is_part_number = false;
            let number = number_string.parse::<i32>().unwrap();

            if let Some(previous_line) = input.lines().nth((index as i32 - 1) as usize) {
                if previous_line[neighbor_range.clone()].contains(|c: char| !c.is_numeric() && c != '.') {
                    is_part_number = true;
                }
            }

            if line[neighbor_range.clone()].contains(|c: char| !c.is_numeric() && c != '.') {
                is_part_number = true;
            }

            if let Some(next_line) = input.lines().nth(index + 1) {
                if next_line[neighbor_range.clone()].contains(|c: char| !c.is_numeric() && c != '.') {
                    is_part_number = true;
                }
            }

            if is_part_number {
                part_number_sum += number;
            }

            mutable_line = &mutable_line[number_start_index + number_end_index..];
        }
    });
    return part_number_sum;
}

fn part2(input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&contents), 4361);
    }

    #[test]
    fn test_input_part1() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part1(&contents.trim()), 527369);
    }
}