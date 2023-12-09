use std::fs;

fn compute_extrapolation<F: Fn(&Vec<Vec<i32>>) -> i32>(line: &str, extrapolate: F) -> i32 {
    let mut histories: Vec<Vec<i32>> = Vec::new();
    let initial_numbers: Vec<i32> = line.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    histories.push(initial_numbers);

    while !histories.last().unwrap().iter().all(|&x| x == 0) {
        let next_history = histories.last().unwrap().windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        histories.push(next_history);
    }

    return extrapolate(&histories);
}

fn part1(input: &str) -> i32 {
    return input.lines().map(|line| {
        return compute_extrapolation(line, |histories| {
            return histories.iter().rev().fold(0, |acc, history| acc + history.last().unwrap());
        });
    }).sum();
}

fn part2(input: &str) -> i32 {
    return input.lines().map(|line| {
        return compute_extrapolation(line, |histories| {
            return histories.iter().rev().fold(0, |acc, history| history.first().unwrap() - acc)
        });
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let contents = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part1(&contents), 114);
    }

    #[test]
    fn test_input_part1() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part1(&contents.trim()), 1868368343);
    }

    #[test]
    fn test_example_part2() {
        let contents = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part2(&contents), 2);
    }

    #[test]
    fn test_input_part2() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part2(&contents.trim()), 1022);
    }
}
