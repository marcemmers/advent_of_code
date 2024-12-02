use std::collections::HashMap;
use std::time::Instant;

fn parse_line(line: &str) -> (u64, u64) {
    let line: Vec<u64> = line
        .split_whitespace()
        .map(|val| val.parse::<u64>().expect("Should be able to parse"))
        .collect();
    (line[0], line[1])
}

fn solve1(input: &str) -> u64 {
    let lines = input.lines();
    let (mut list_a, mut list_b): (Vec<u64>, Vec<u64>) = lines.map(parse_line).collect();

    list_a.sort();
    list_b.sort();

    list_a
        .iter()
        .zip(list_b.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn solve2(input: &str) -> u64 {
    let lines = input.lines();
    let (list_a, list_b): (Vec<u64>, Vec<u64>) = lines.map(parse_line).collect();

    let mut map: HashMap<u64, u64> = HashMap::new();
    list_b.iter().for_each(|b| *map.entry(*b).or_insert(0) += 1);
    list_a.iter().map(|a| map.get(a).unwrap_or(&0) * a).sum()
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
    fn test1() {
        assert_eq!(solve1(EXAMPLE), 11);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 31);
    }
}
