use std::fs;

fn get_cards(input: &str) -> Vec<(u32, Vec<u32>, Vec<u32>)> {
    return input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let card_number = parts[0].replace("Card", "").trim().parse::<u32>().unwrap();
            let numbers: Vec<&str> = parts[1].split(" | ").collect();
            let group1 = numbers[0].split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
            let group2 = numbers[1].split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
            return (card_number, group1, group2);
        })
        .collect();
}

fn part1(input: &str) -> u32 {
    return get_cards(input)
        .iter()
        .map(|(_, winning_numbers, numbers)| {
            return numbers.iter().map(|&number| winning_numbers.contains(&number)).filter(|&b| b).count() as u32;
        })
        .filter(|&count| count > 0)
        .map(|count| 2_u32.pow(count - 1))
        .sum();
}

fn part2(input: &str) -> usize {
    let mut card_copies = get_cards(input)
        .iter()
        .map(|(_, winning_numbers, numbers)| {
            return (1, numbers.iter().map(|&number| winning_numbers.contains(&number)).filter(|&b| b).count());
        })
        .collect::<Vec<(usize, usize)>>();

    let mut scratchcards = 0;
    for index in 0..card_copies.len() {
        let (card_number, winning_numbers) = card_copies[index];
        if winning_numbers > 0 {
            for i in index+1..=index+winning_numbers {
                let (other_card_number, other_count) = card_copies[i];
                card_copies[i] = (other_card_number+card_number, other_count);
            }
        }
        scratchcards += card_number;
    }

    return scratchcards;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&contents), 13);
    }

    #[test]
    fn input_part1() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part1(&contents.trim()), 23673);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part2(&contents), 30);
    }

    #[test]
    fn input_part2() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part2(&contents.trim()), 12263631);
    }
}
