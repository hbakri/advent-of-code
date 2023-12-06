use std::fs;

fn part1(input: &str) -> i32 {
    let mut part_number_sum = 0;

    input.lines().enumerate().for_each(|(index, line)| {
        let mut current_index = 0;

        while let Some(relative_number_start_index) = line[current_index..].find(|c: char| c.is_numeric()) {
            let number_start_index = current_index + relative_number_start_index;
            let number_string = line[number_start_index..]
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>();
            let number = number_string.parse::<i32>().unwrap();

            let horizontal_check_range = number_start_index.saturating_sub(1)..(number_start_index + number_string.len() + 1).min(line.len());
            let vertical_check_range = index.saturating_sub(1)..=(index + 1).min(input.lines().count());
            let is_adjacent_to_symbol = input.lines()
                .enumerate()
                .filter(|&(index, _)| vertical_check_range.contains(&index))
                .any(|(_, neighbor_line)| {
                    neighbor_line.get(horizontal_check_range.clone())
                        .map_or(false, |segment| segment.chars().any(|c| !c.is_numeric() && c != '.'))
                });

            if is_adjacent_to_symbol {
                part_number_sum += number;
            }

            current_index = number_start_index + number_string.len();
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
