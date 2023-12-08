use std::collections::{HashMap};
use std::fs;

fn parse_input(input: &str) -> (&str, HashMap<&str, (String, String)>) {
    let instructions = input.lines().nth(0).unwrap();
    let nodes_string = input.split("\n\n").nth(1).unwrap();
    let mut nodes_map = HashMap::new();
    for line in nodes_string.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        let node = parts[0].trim();
        let rest = parts[1].trim().trim_start_matches('(').trim_end_matches(')');
        let nodes: Vec<&str> = rest.split(',').collect();
        let left = nodes[0].trim().to_string();
        let right = nodes[1].trim().to_string();
        nodes_map.insert(node, (left, right));
    }
    return (instructions, nodes_map)
}

fn part1(input: &str) -> i32 {
    let (instructions, nodes) = parse_input(input);

    let mut number_of_iterations = 0;
    let mut current_instruction_index = 0;
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        let current_instruction = instructions.chars().nth(current_instruction_index).unwrap();

        if current_instruction == 'L' {
            current_node = nodes.get(current_node).unwrap().0.as_str();
        } else if current_instruction == 'R' {
            current_node = nodes.get(current_node).unwrap().1.as_str();
        }

        current_instruction_index = (current_instruction_index + 1) % instructions.len();
        number_of_iterations += 1;
    }
    return number_of_iterations;
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn part2(input: &str) -> i64 {
    let (instructions, nodes) = parse_input(input);

    return nodes
        .iter()
        .filter(|(&node, _)| node.ends_with('A'))
        .map(|(&node, _)| {
            let mut number_of_iterations = 0;
            let mut current_instruction_index = 0;
            let mut current_node = node;

            while !current_node.ends_with('Z') {
                let current_instruction = instructions.chars().nth(current_instruction_index).unwrap();

                if current_instruction == 'L' {
                    current_node = nodes.get(current_node).unwrap().0.as_str();
                } else if current_instruction == 'R' {
                    current_node = nodes.get(current_node).unwrap().1.as_str();
                }

                current_instruction_index = (current_instruction_index + 1) % instructions.len();
                number_of_iterations += 1;
            }
            return number_of_iterations;
        })
        .reduce(|a, b| lcm(a, b))
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let contents = fs::read_to_string("test-part1-1.txt").unwrap();
        assert_eq!(part1(&contents), 2);
    }

    #[test]
    fn test2_part1() {
        let contents = fs::read_to_string("test-part1-2.txt").unwrap();
        assert_eq!(part1(&contents), 6);
    }

    #[test]
    fn input_part1() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part1(&contents), 13771);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("test-part2.txt").unwrap();
        assert_eq!(part2(&contents), 6);
    }

    #[test]
    fn input_part2() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part2(&contents), 13129439557681);
    }
}
