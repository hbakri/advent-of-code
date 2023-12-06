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

fn get_intersection_range(seed_range: &(i64, i64), map_range: &(i64, i64)) -> (Option<(i64, i64)>, Vec<(i64, i64)>) {
    let intersection_start = seed_range.0.max(map_range.0);
    let intersection_end = (seed_range.0 + seed_range.1).min(map_range.0 + map_range.1);
    let intersection_length = intersection_end - intersection_start;

    if intersection_length > 0 {
        let intersection_range = (intersection_start, intersection_length);
        let mut complementary_ranges: Vec<(i64, i64)> = vec![];

        if seed_range.0 < intersection_start {
            complementary_ranges.push((seed_range.0, intersection_start - seed_range.0));
        }
        if seed_range.0 + seed_range.1 > intersection_end {
            complementary_ranges.push((intersection_end, seed_range.0 + seed_range.1 - intersection_end));
        }
        return (Some(intersection_range), complementary_ranges);
    } else {
        return (None, vec![]);
    }
}

fn part1(input: &str) -> i64 {
    let (seeds, maps_list) = get_seeds_and_maps_list(input);

    return seeds.iter().map(|seed| {
        let mut seed = seed.clone();

        maps_list.iter().for_each(|maps| {
            for line in maps {
                let numbers: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
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
        maps.iter().for_each(|line| {
            let numbers: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            let (destination_range_start, source_range_start, range_length) = (numbers[0], numbers[1], numbers[2]);

            seed_ranges.iter().for_each(|seed_range| {
                if let (Some(intersection_range), mut complementary_ranges) = get_intersection_range(seed_range, &(source_range_start, range_length)) {
                    mapped_seed_ranges.push((intersection_range, destination_range_start - source_range_start));
                    unmapped_seed_ranges.append(&mut complementary_ranges);
                } else {
                    unmapped_seed_ranges.push(seed_range.clone());
                }
            });

            seed_ranges.clear();
            seed_ranges.extend(unmapped_seed_ranges.iter().cloned());
            unmapped_seed_ranges.clear();
        });

        seed_ranges.extend(
            mapped_seed_ranges
                .iter()
                .map(|((range_start, range_length), offset)| (range_start + offset, *range_length))
        );
        mapped_seed_ranges.clear();
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
    fn test_input_part2() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part2(&contents.trim()), 72263011);
    }
}
