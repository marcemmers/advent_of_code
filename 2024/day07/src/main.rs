use std::time::Instant;

fn parse(line: &str) -> (u64, Vec<u64>) {
    let (sum, values) = line.split_once(':').expect("Should contain ':'");

    (
        sum.parse().unwrap(),
        values
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect(),
    )
}

fn is_valid_internal(sum: u64, values: &[u64], goal: u64) -> bool {
    if sum == goal && values.is_empty() {
        return true;
    }
    if sum > goal || values.is_empty() {
        return false;
    }

    if is_valid_internal(sum * values[0], &values[1..], goal)
        || is_valid_internal(sum + values[0], &values[1..], goal)
    {
        return true;
    }
    false
}

fn is_valid(sum: u64, values: &[u64]) -> bool {
    is_valid_internal(values[0], &values[1..], sum)
}

fn solve1(input: &str) -> u64 {
    let lines = input.lines();

    lines
        .map(parse)
        .filter(|(sum, values)| is_valid(*sum, values))
        .map(|(sum, _)| sum)
        .sum()
}

fn concat(a: u64, b: u64) -> u64 {
    let shift = b.ilog10();
    (a * (10u64.pow(shift + 1))) + b
}

fn is_valid_internal2(sum: u64, values: &[u64], goal: u64) -> bool {
    if sum == goal && values.is_empty() {
        return true;
    }
    if sum > goal || values.is_empty() {
        return false;
    }

    if is_valid_internal2(sum * values[0], &values[1..], goal)
        || is_valid_internal2(sum + values[0], &values[1..], goal)
        || is_valid_internal2(concat(sum, values[0]), &values[1..], goal)
    {
        return true;
    }
    false
}

fn is_valid2(sum: u64, values: &[u64]) -> bool {
    is_valid_internal2(values[0], &values[1..], sum)
}

fn solve2(input: &str) -> u64 {
    let lines = input.lines();

    lines
        .map(parse)
        .filter(|(sum, values)| is_valid2(*sum, values))
        .map(|(sum, _)| sum)
        .sum()
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

    #[test]
    fn test_concat() {
        assert_eq!(concat(5, 5), 55);
        assert_eq!(concat(5, 12), 512);
        assert_eq!(concat(20, 10), 2010);
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE), 3749);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 11387);
    }
}
