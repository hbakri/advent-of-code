use std::collections::HashMap;
use std::fs;
use std::cmp::Ordering;

const CARD_STRENGTHS: [(char, i32); 13] = [
    ('2', 2), ('3', 3), ('4', 4), ('5', 5), ('6', 6),
    ('7', 7), ('8', 8), ('9', 9), ('T', 10), ('J', 11),
    ('Q', 12), ('K', 13), ('A', 14)
];

fn get_card_strength(card: char) -> i32 {
    CARD_STRENGTHS.iter().find(|&&(c, _)| c == card).map(|&(_, strength)| strength).unwrap()
}

fn get_card_strength_with_jokers(card: char) -> i32 {
    if card == 'J' {
        return 1;
    }
    CARD_STRENGTHS.iter().find(|&&(c, _)| c == card).map(|&(_, strength)| strength).unwrap()
}

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand_score(hand_type: HandType) -> i32 {
    match hand_type {
        HandType::FiveOfAKind => 7,
        HandType::FourOfAKind => 6,
        HandType::FullHouse => 5,
        HandType::ThreeOfAKind => 4,
        HandType::TwoPair => 3,
        HandType::OnePair => 2,
        HandType::HighCard => 1,
    }
}

fn get_hand_bid_pairs(input: &str) -> Vec<(&str, i32)> {
    return input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let string_part = parts[0];
            let number_part = parts[1].parse::<i32>().unwrap();
            (string_part, number_part)
        })
        .collect();
}

fn get_hand_value(hand: &str) -> i32 {
    let mut card_counts = HashMap::new();
    hand.chars().for_each(|c| {
        *card_counts.entry(c).or_insert(0) += 1;
    });
    return get_hand_score(determine_hand_type(card_counts));
}

fn determine_hand_type(card_counts: HashMap<char, i32>) -> HandType {
    let mut frequencies = card_counts.values().cloned().collect::<Vec<i32>>();
    frequencies.sort();

    match frequencies.as_slice() {
        [1, 1, 1, 1, 1] => HandType::HighCard,
        [1, 1, 1, 2] => HandType::OnePair,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 3] => HandType::ThreeOfAKind,
        [2, 3] => HandType::FullHouse,
        [1, 4] => HandType::FourOfAKind,
        [5] => HandType::FiveOfAKind,
        _ => HandType::HighCard,
    }
}

fn get_hand_value_with_jokers(hand: &str) -> i32 {
    let mut card_counts = HashMap::new();
    hand.chars().for_each(|c| {
        *card_counts.entry(c).or_insert(0) += 1;
    });

    let joker_count = *card_counts.get(&'J').unwrap_or(&0);
    let mut best_hand_value = get_hand_value(hand); // Start with the value without jokers

    if joker_count > 0 {
        for &(card, _) in CARD_STRENGTHS.iter() {
            if card != 'J' {
                let mut modified_counts = card_counts.clone();
                *modified_counts.entry(card).or_insert(0) += joker_count;
                modified_counts.remove(&'J');

                let simulated_hand_value = get_hand_score(determine_hand_type(modified_counts));
                best_hand_value = std::cmp::max(best_hand_value, simulated_hand_value);
            }
        }
    }

    return best_hand_value;
}

fn compare_hands(hand1: &str, hand2: &str) -> Ordering {
    let hand1_value = get_hand_value(hand1);
    let hand2_value = get_hand_value(hand2);

    if hand1_value != hand2_value {
        return hand1_value.cmp(&hand2_value);
    }

    for (card1, card2) in hand1.chars().zip(hand2.chars()) {
        let strength1 = get_card_strength(card1);
        let strength2 = get_card_strength(card2);

        match strength1.cmp(&strength2) {
            Ordering::Equal => continue,
            other => return other,
        }
    }

    Ordering::Equal
}

fn part1(input: &str) -> i32 {
    let mut hand_bid_pairs = get_hand_bid_pairs(input);

    hand_bid_pairs.sort_by(|&(ref hand1, _), &(ref hand2, _)| {
        compare_hands(hand1, hand2)
    });

    return hand_bid_pairs.iter().enumerate().map(|(index, &(_, bid))| {
        bid * (index as i32 + 1)
    }).sum();
}

fn part2(input: &str) -> i32 {
    let mut hand_bid_pairs = get_hand_bid_pairs(input);

    hand_bid_pairs.sort_by(|&(ref hand1, _), &(ref hand2, _)| {
        let hand1_value = get_hand_value_with_jokers(hand1);
        let hand2_value = get_hand_value_with_jokers(hand2);

        if hand1_value != hand2_value {
            return hand1_value.cmp(&hand2_value);
        }

        for (card1, card2) in hand1.chars().zip(hand2.chars()) {
            let strength1 = get_card_strength_with_jokers(card1);
            let strength2 = get_card_strength_with_jokers(card2);

            match strength1.cmp(&strength2) {
                Ordering::Equal => continue,
                other => return other,
            }
        }

        Ordering::Equal
    });

    return hand_bid_pairs.iter().enumerate().map(|(index, &(_, bid))| {
        bid * (index as i32 + 1)
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&contents), 6440);
    }

    #[test]
    fn test_input_part1() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part1(&contents.trim()), 250957639);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part2(&contents), 5905);
    }

    #[test]
    fn test_input_part2() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part2(&contents.trim()), 251515496);
    }
}
