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

fn get_seeds_and_maps_list(input: &str) -> (Vec<i64>, Vec<Vec<&str>>) {
    let seeds: Vec<i64> = input
        .lines()
        .nth(0)
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

    return (seeds, maps_list);
}

fn get_intersection_range(range1: &(i64, i64), range2: &(i64, i64)) -> Option<(i64, i64)> {
    let intersection_start = range1.0.max(range2.0);
    let intersection_end = (range1.0 + range1.1).min(range2.0 + range2.1);
    let intersection_length = intersection_end - intersection_start;

    return if intersection_length > 0 {
        Some((intersection_start, intersection_length))
    } else {
        None
    }
}

fn get_complementary_ranges(range: (i64, i64), mapped_ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut complementary_ranges: Vec<(i64, i64)> = vec![];

    let mut previous_range_end = range.0;
    mapped_ranges.iter().for_each(|(mapped_range_start, mapped_range_length)| {
        if previous_range_end < *mapped_range_start {
            complementary_ranges.push((previous_range_end, *mapped_range_start - previous_range_end));
        }
        previous_range_end = mapped_range_start + mapped_range_length;
    });

    if previous_range_end < range.0 + range.1 {
        complementary_ranges.push((previous_range_end, range.0 + range.1 - previous_range_end));
    }

    return complementary_ranges;
}

fn part1(input: &str) -> i64 {
    let (seeds, maps_list) = get_seeds_and_maps_list(input);

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
    let (seeds, maps_list) = get_seeds_and_maps_list(input);

    let mut seed_ranges: Vec<(i64, i64)> = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();
    let mut mapped_seed_ranges: Vec<((i64, i64), i64)> = vec![];
    let mut unmapped_seed_ranges: Vec<(i64, i64)> = vec![];

    maps_list.iter().for_each(|maps| {
        seed_ranges.iter().for_each(|seed_range| {
            for line in maps {
                let numbers: Vec<i64> = line.split(' ').map(|s| s.parse().unwrap()).collect();
                let (destination_range_start, source_range_start, range_length) = (numbers[0], numbers[1], numbers[2]);

                if let Some(intersection_range) = get_intersection_range(seed_range, &(source_range_start, range_length)) {
                    mapped_seed_ranges.push((intersection_range, destination_range_start - source_range_start));
                }
            }

            mapped_seed_ranges.sort_by_key(|((intersection_start, _), _)| *intersection_start);
            unmapped_seed_ranges = get_complementary_ranges(*seed_range, &mapped_seed_ranges.iter().map(|(mapped_seed_range, _)| *mapped_seed_range).collect::<Vec<(i64, i64)>>());
        });

        seed_ranges = mapped_seed_ranges
            .iter()
            .map(|((range_start, range_length), offset)| (range_start + offset, range_length.clone()))
            .collect();
        seed_ranges.append(&mut unmapped_seed_ranges);
        mapped_seed_ranges = vec![];
        unmapped_seed_ranges = vec![];
    });
    return seed_ranges.iter().map(|(range_start, _)| range_start).min().unwrap().clone();
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
        assert_eq!(part2(&contents), 46);
    }

    #[test]
    fn test_get_complementary_ranges() {
        let range = (0, 10);
        let mapped_ranges = vec![(0, 5), (7, 3)];
        let complementary_ranges = get_complementary_ranges(range, &mapped_ranges);
        assert_eq!(complementary_ranges, vec![(5, 2)]);
    }

    #[test]
    fn test_get_intersection_range() {
        let range1 = (0, 10);
        let range2 = (5, 10);
        let intersection = get_intersection_range(&range1, &range2);
        assert_eq!(intersection, Some((5, 5)));
    }
}
