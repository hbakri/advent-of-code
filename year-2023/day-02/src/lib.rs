use std::fs;

fn part1(input: &str) -> u32 {
    const MAX_BALLS_PER_COLOR: [(&str, u32); 3] = [
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ];
    return input.lines().map(|line| {
        let parts: Vec<&str> = line.split(':').collect();
        let game_parts: Vec<&str> = parts[0].split_whitespace().collect();
        let game_id: u32 = game_parts[1].parse().unwrap();
        let ball_sets: Vec<&str> = parts[1].split(';').collect();
        for ball_set in ball_sets {
            let balls: Vec<&str> = ball_set.split(',').collect();
            for ball in balls {
                let ball_parts: Vec<&str> = ball.trim().split(' ').collect();
                let color = ball_parts[1];
                let count = ball_parts[0].parse::<u32>().unwrap();
                let max_balls = MAX_BALLS_PER_COLOR.iter().find(|(c, _)| c == &color).unwrap().1;
                if count > max_balls {
                    return 0;
                }
            }
        }
        return game_id;
    }).sum();
}

fn part2(input: &str) -> u32 {
    return input.lines().map(|line| {
        let parts: Vec<&str> = line.split(':').collect();
        let ball_sets: Vec<&str> = parts[1].split(';').collect();
        let mut saved_balls: [(&str, u32); 3] = [
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ];
        for ball_set in ball_sets {
            let balls: Vec<&str> = ball_set.split(',').collect();
            for ball in balls.iter() {
                let ball_parts: Vec<&str> = ball.trim().split(' ').collect();
                let color = ball_parts[1];
                let count = ball_parts[0].parse::<u32>().unwrap();
                if saved_balls.iter().find(|(c, _)| c == &color).unwrap().1 < count {
                    saved_balls.iter_mut().find(|(c, _)| c == &color).unwrap().1 = count;
                }
            }
        }
        return saved_balls.iter().map(|(_, count)| count).product::<u32>();
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&contents), 8);
    }

    #[test]
    fn test_input_part1() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part1(&contents.trim()), 2720);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part2(&contents), 2286);
    }

    #[test]
    fn test_input_part2() {
        let contents = fs::read_to_string("input.txt").unwrap();
        assert_eq!(part2(&contents.trim()), 71535);
    }
}