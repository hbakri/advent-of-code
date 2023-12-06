use std::fs;

fn get_time_distance_pairs(input: &str) -> Vec<(i64, i64)> {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<i64> = lines[0].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    let distances: Vec<i64> = lines[1].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();

    return times.into_iter().zip(distances.into_iter()).collect();
}

fn get_time_distance_pair(input: &str) -> (i64, i64) {
    let lines: Vec<&str> = input.lines().collect();
    let time: i64 = lines[0].split_whitespace().skip(1).collect::<Vec<&str>>().join("").parse().unwrap();
    let distance: i64 = lines[1].split_whitespace().skip(1).collect::<Vec<&str>>().join("").parse().unwrap();

    return (time, distance);
}

fn part1(input: &str) -> i64 {
    let time_distance_pairs = get_time_distance_pairs(input);
    return time_distance_pairs.iter().map(|&(time, distance)| {
        return (0..time)
            .map(|t| (time - t) * t > distance)
            .filter(|&t| t)
            .count() as i64;
    }).product();
}

fn part2(input: &str) -> i64 {
    let (time, distance) = get_time_distance_pair(input);
    return (0..time)
        .map(|t| (time - t) * t > distance)
        .filter(|&t| t)
        .count() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&contents), 288);
    }

    #[test]
    fn test_input_part_1() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part1(&contents.trim()), 625968);
    }

    #[test]
    fn test_part_2() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part2(&contents), 71503);
    }

    #[test]
    fn test_input_part_2() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part2(&contents.trim()), 43663323);
    }
}
