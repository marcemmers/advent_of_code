use std::fs;
use std::time::Instant;

struct SpringMap {
    spring: u128,
    potential: u128
}

impl SpringMap {
    fn new() -> SpringMap {
        SpringMap{ spring: 0, potential: 0 }
    }

    fn parse(input: &str) -> SpringMap {
        input.chars().fold(SpringMap::new(), |mut map, ch| {
            map.shift_left(1);
            match ch {
                '#' => map.spring = map.spring.saturating_add(1),
                '?' => map.potential = map.potential.saturating_add(1),
                _ => ()
            }
            return map;
        })
    }

    fn shift_left(&mut self, shift: u32) {
        self.spring <<= shift;
        self.potential <<= shift;
    }

    fn next_spring(&self) -> SpringMap {
        let bit = 1u128 << (u128::BITS - self.potential.leading_zeros() - 1);
        SpringMap{
            spring: self.spring | bit,
            potential: self.potential & !bit,
        }
    }

    fn next_empty(&self) -> SpringMap {
        let bit = 1u128 << (u128::BITS - self.potential.leading_zeros() - 1);
        SpringMap{
            spring: self.spring,
            potential: self.potential & !bit,
        }
    }
}

fn equal_lengths(map: SpringMap, numbers: &[u32]) -> bool {
    let mut springs = map.spring;

    let result = numbers.iter().rev().all(|nr| {
        springs = springs.wrapping_shr(springs.trailing_zeros());
        if *nr != springs.trailing_ones() {
            return false;
        }
        springs = springs.wrapping_shr(*nr);
        return true;
    });

    if springs.count_ones() != 0 {
        return false;
    }

    return result;
}

fn try_arrangements(map: SpringMap, numbers: &[u32], questions_left: usize) -> u64 {
    if questions_left == 0 {
        return if equal_lengths(map, numbers) { 1 } else { 0 };
    }

    return try_arrangements(
        map.next_empty(),
        &numbers,
        questions_left - 1,
    ) + try_arrangements(
        map.next_spring(),
        &numbers,
        questions_left - 1,
    );
}

fn calculate_arrangements(line: &str) -> u64 {
    let (map, numbers) = line.split_once(' ').unwrap();

    let numbers: Vec<u32> = numbers
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let map = SpringMap::parse(map);

    // println!("Springs    {:>#30b}", map.spring);
    // println!("Potential: {:>#30b}", map.potential);

    let potentials = map.potential.count_ones() as usize;

    return try_arrangements(
        map,
        numbers.as_slice(),
        potentials
    );
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let result = lines.map(|line| calculate_arrangements(line)).sum();

    return result;
}

fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    return 0;
}

const PUZZLE_FILENAME: &'static str = "./src/puzzle.txt";

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve1(PUZZLE_FILENAME));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve2(PUZZLE_FILENAME));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILENAME: &'static str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE_FILENAME), 21);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 0);
    }
}
