use std::fs;

fn part1(input: &str) -> u32 {
    return input.lines().map(|line| {
        let first_digit = line.chars().find(|&c| c.is_numeric()).unwrap();
        let last_digit = line.chars().rfind(|&c| c.is_numeric()).unwrap();

        return format!("{}{}", first_digit, last_digit).parse::<u32>().unwrap();
    }).sum();
}

fn part2(input: &str) -> u32 {
    let digits_to_words = [
        ("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'), ("five", '5'),
        ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9'), ("zero", '0'),
    ];
    return input.lines().map(|line| {
        let first_digit_numeric = line.chars()
            .enumerate()
            .find(|&(_, c)| c.is_numeric());
        let first_digit_word = digits_to_words
            .iter()
            .filter_map(|&(word, value)| {
                line.find(word).map(|index| (index, value))
            })
            .min_by_key(|&(index, _)| index);
        let first_digit = [first_digit_numeric, first_digit_word]
            .iter()
            .filter_map(|&option| option)
            .min_by_key(|&(index, _)| index)
            .unwrap()
            .1;

        let last_digit_numeric = line.chars()
            .rev()
            .enumerate()
            .find(|&(_, c)| c.is_numeric())
            .map(|(index, c)| (line.len() - index - 1, c));
        let last_digit_word = digits_to_words
            .iter()
            .filter_map(|&(word, value)| {
                line.rfind(word).map(|index| (index, value))
            })
            .max_by_key(|&(index, _)| index);
        let last_digit = [last_digit_numeric, last_digit_word]
            .iter()
            .filter_map(|&option| option)
            .max_by_key(|&(index, _)| index)
            .unwrap()
            .1;

        return format!("{}{}", first_digit, last_digit).parse::<u32>().unwrap();
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("test-part1.txt").unwrap();
        assert_eq!(part1(&contents), 142);
    }

    #[test]
    fn test_input_part1() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part1(&contents.trim()), 54388);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("test-part2.txt").unwrap();
        assert_eq!(part2(&contents), 281);
    }

    #[test]
    fn test_input_part2() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part2(&contents.trim()), 53515);
    }
}
