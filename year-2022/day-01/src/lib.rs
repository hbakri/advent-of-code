use std::fs;

fn part1(input: &str) -> u32 {
    return input
        .split("\n\n")
        .map(|group| {
            return group
                .split("\n")
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>();
        })
        .max()
        .unwrap();
}

fn part2(input: &str) -> u32 {
    let mut results = input
        .split("\n\n")
        .map(|group| {
            return group
                .split("\n")
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>();
        })
        .collect::<Vec<u32>>();

    results.sort_by(|a, b| b.cmp(a));

    return results.iter().take(3).sum::<u32>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&contents), 24000);
        println!("Part 1: {}", part1(&contents.trim()));
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part2(&contents), 45000);
        println!("Part 2: {}", part2(&contents.trim()));
    }
}
