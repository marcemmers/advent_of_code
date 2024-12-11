use std::{
    collections::{hash_map::Entry, HashMap},
    time::Instant,
};

fn split_even_length(val: u64) -> Option<(u64, u64)> {
    let log = val.ilog10() + 1;
    if log % 2 == 0 {
        let div = 10u64.pow(log / 2);
        Some((val / div, val % div))
    } else {
        None
    }
}

fn blink(stone: u64, blinks_remaining: u64, memory: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks_remaining == 0 {
        return 1;
    }
    let blinks_remaining = blinks_remaining - 1;

    if let Entry::Occupied(stones) = memory.entry((stone, blinks_remaining)) {
        return *stones.get();
    }

    let result = if stone == 0 {
        blink(1, blinks_remaining, memory)
    } else if let Some((a, b)) = split_even_length(stone) {
        blink(a, blinks_remaining, memory) + blink(b, blinks_remaining, memory)
    } else {
        blink(stone * 2024, blinks_remaining, memory)
    };

    memory.insert((stone, blinks_remaining), result);

    result
}

fn solve1(input: &str) -> u64 {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut map = HashMap::new();

    stones.iter().map(|stone| blink(*stone, 25, &mut map)).sum()
}

fn solve2(input: &str) -> u64 {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut map = HashMap::new();

    stones.iter().map(|stone| blink(*stone, 75, &mut map)).sum()
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
    fn is_even() {
        assert_eq!(split_even_length(10), Some((1, 0)));
        assert_eq!(split_even_length(1234), Some((12, 34)));
        assert_eq!(split_even_length(123456), Some((123, 456)));
    }

    #[test]
    fn is_odd() {
        assert_eq!(split_even_length(1), None);
        assert_eq!(split_even_length(123), None);
        assert_eq!(split_even_length(12345), None);
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE), 55312);
    }
}
