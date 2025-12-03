use std::{collections::HashSet, time::Instant};

use utils::is_prime;

fn digits(mut val: u64) -> u64 {
    let mut digits = 0;
    while val > 0 {
        digits += 1;
        val /= 10;
    }
    digits
}

fn solve1(input: &str) -> u64 {
    let parts = input.split(',').map(|part| part.split_once('-').unwrap());

    let mut invalid = Vec::new();

    for (begin, end) in parts {
        let begin_val: u64 = begin.parse().unwrap();
        let end_val: u64 = end.parse().unwrap();
        for val in begin_val..=end_val {
            let nr_digits = digits(val);
            if !nr_digits.is_multiple_of(2) {
                continue;
            }

            let div = [10, 100, 1_000, 10_000, 100_000, 1_000_000][(nr_digits as usize / 2) - 1];
            if val / div == val % div {
                invalid.push(val);
            }
        }
    }

    invalid.iter().sum()
}

fn solve2(input: &str) -> u64 {
    let parts = input.split(',').map(|part| part.split_once('-').unwrap());

    let mut invalid = HashSet::new();

    for (begin, end) in parts {
        let begin_val: u64 = begin.parse().unwrap();
        let end_val: u64 = end.parse().unwrap();
        for val in begin_val..=end_val {
            let nr_digits = digits(val);
            let nr_divisors = if is_prime(nr_digits) {
                1
            } else {
                nr_digits / 2
            };

            for div in [10, 100, 1_000, 10_000, 100_000, 1_000_000]
                .iter()
                .take(nr_divisors as usize)
            {
                let cmp = val % div;
                let mut tmp = val;
                while tmp % div == cmp {
                    tmp /= div;
                }
                if tmp == 0 {
                    invalid.insert(val);
                }
            }
        }
    }

    invalid.iter().sum()
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
        assert_eq!(solve1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 4174379265);
    }
}
