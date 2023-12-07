use std::fs;

fn get_part_numbers_with_positions(input: &str) -> Vec<(String, (usize, usize))> {
    let mut part_numbers: Vec<(String, (usize, usize))> = Vec::new();

    input.lines().enumerate().for_each(|(index, line)| {
        let mut current_index = 0;

        while let Some(relative_number_start_index) = line[current_index..].find(|c: char| c.is_numeric()) {
            let number_start_index = current_index + relative_number_start_index;
            let number_string = line[number_start_index..]
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>();

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
                part_numbers.push((number_string.clone(), (index, number_start_index)));
            }

            current_index = number_start_index + number_string.len();
        }
    });
    return part_numbers;
}

fn part1(input: &str) -> i32 {
    let part_numbers_with_positions = get_part_numbers_with_positions(input);
    return part_numbers_with_positions
        .iter()
        .map(|(number_string, _)| number_string.parse::<i32>().unwrap())
        .sum();
}

fn part2(input: &str) -> i32 {
    let part_numbers_with_positions = get_part_numbers_with_positions(input);
    let mut product = 0;
    input.lines().enumerate().for_each(|(index, line)| {
        let mut current_index = 0;

        while let Some(relative_gear_start_index) = line[current_index..].find(|c: char| c == '*') {
            let gear_start_index = current_index + relative_gear_start_index;

            let horizontal_check_range = gear_start_index.saturating_sub(1)..=(gear_start_index + 1).min(line.len());
            let vertical_check_range = index.saturating_sub(1)..=(index + 1).min(input.lines().count());

            let adjacent_numbers = part_numbers_with_positions
                .iter()
                .filter(|(number_string, (number_index, number_start_index))| {
                    vertical_check_range.contains(number_index) &&
                        (horizontal_check_range.contains(number_start_index) || horizontal_check_range.contains(&(number_start_index + number_string.len() - 1)))
                })
                .map(|(number_string, _)| number_string.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if adjacent_numbers.len() == 2 {
                product += adjacent_numbers.iter().product::<i32>();
            }

            current_index = gear_start_index + 1;
        }
    });
    return product;
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

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part2(&contents), 467835);
    }

    #[test]
    fn test_input_part2() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part2(&contents.trim()), 73074886);
    }
}
