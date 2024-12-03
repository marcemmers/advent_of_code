use std::time::Instant;

fn parse_number(input: &str) -> Option<u64> {
    if input.is_empty() || input.len() > 3 || !input.chars().all(|ch| ch.is_numeric()) {
        None
    } else {
        input.parse().ok()
    }
}

fn solve1(mut input: &str) -> u64 {
    let mut multipliers = Vec::new();

    while let Some(idx) = input.find("mul(") {
        input = &input[idx..];
        input = input.strip_prefix("mul(").expect("Prefix already found");

        if let Some((val, remainder)) = input.split_once(')') {
            if let Some((a, b)) = val.split_once(',') {
                if let (Some(a), Some(b)) = (parse_number(a), parse_number(b)) {
                    multipliers.push((a, b));
                    input = remainder; // Only use when actually matches
                }
            }
        }
    }

    multipliers.iter().map(|(a, b)| a * b).sum()
}

fn solve2(mut input: &str) -> u64 {
    let mut multipliers = Vec::new();

    let mut enabled = true;

    while !input.is_empty() {
        if let Some(val) = input.strip_prefix("don't()") {
            input = val;
            enabled = false;
        } else if let Some(val) = input.strip_prefix("do()") {
            input = val;
            enabled = true;
        } else if let Some(val) = input.strip_prefix("mul(") {
            input = val;

            if let Some((val, remainder)) = input.split_once(')') {
                if let Some((a, b)) = val.split_once(',') {
                    if let (Some(a), Some(b)) = (parse_number(a), parse_number(b)) {
                        if enabled {
                            multipliers.push((a, b));
                        }
                        input = remainder; // Only use when actually matches
                    }
                }
            }
        } else {
            input = &input[1..];
        }
    }

    multipliers.iter().map(|(a, b)| a * b).sum()
}

const PUZZLE: &str = include_str!("./puzzle.txt");

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve1(PUZZLE));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve2(PUZZLE));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");
    const EXAMPLE2: &str = include_str!("./example2.txt");

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE), 161);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE2), 48);
    }
}
