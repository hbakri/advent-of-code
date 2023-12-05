use std::fs;

const SECTION_TITLES: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

fn part1(input: &str) -> i64 {
    let seeds: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let maps_list: Vec<Vec<&str>> = SECTION_TITLES.iter().map(|section_title| {
        return input
            .split("\n\n")
            .find(|section| section.starts_with(section_title))
            .unwrap()
            .lines()
            .skip(1)
            .collect::<Vec<&str>>();
    }).collect();

    return seeds.iter().map(|seed| {
        let mut seed = seed.clone();

        maps_list.iter().for_each(|maps| {
            for line in maps {
                let numbers: Vec<i64> = line.split(' ').map(|s| s.parse().unwrap()).collect();
                let (destination_range_start, source_range_start, range_length) = (numbers[0], numbers[1], numbers[2]);

                if seed >= source_range_start && seed < source_range_start + range_length {
                    seed = destination_range_start + (seed - source_range_start);
                    break;
                }
            }
        });
        return seed;
    })
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .unwrap()
        .clone();
}

fn part2(input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&contents), 35);
    }

    #[test]
    fn test_input_part1() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part1(&contents.trim()), 251346198);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&contents), 46);
    }
}
