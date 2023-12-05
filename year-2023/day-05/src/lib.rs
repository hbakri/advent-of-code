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

fn get_section_map<'a>(input: &'a str, section_title: &str) -> Vec<&'a str> {
    let section = input
        .split("\n\n")
        .find(|section| section.starts_with(section_title))
        .unwrap();

    return section
        .lines()
        .skip(1)
        .collect::<Vec<&str>>();
}

fn part1(input: &str) -> i64 {
    let seeds_parts = input.lines().next().unwrap().split(':').collect::<Vec<&str>>();
    let seeds_str = seeds_parts[1].trim().split(' ').collect::<Vec<&str>>();
    let mut seeds = seeds_str.iter().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    return seeds.iter().map(|seed| {
        let mut seed = seed.clone();

        SECTION_TITLES.iter().for_each(|section_title| {
            let section_map = get_section_map(input, section_title);
            for line in section_map {
                let numbers = line.split(' ').collect::<Vec<&str>>().iter().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                let destination_range_start = numbers[0];
                let source_range_start = numbers[1];
                let range_length = numbers[2];

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
