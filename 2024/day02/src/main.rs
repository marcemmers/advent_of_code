use std::time::Instant;

fn is_safe(input: &[i32]) -> bool {
    if !input.iter().is_sorted() && !input.iter().rev().is_sorted() {
        return false;
    }

    input
        .windows(2)
        .map(|x| x[0].abs_diff(x[1]))
        .all(|x| (1..=3).contains(&x))
}

fn is_safe_with_dampener(input: &[i32]) -> bool {
    if is_safe(input) {
        return true;
    }

    (0..input.len()).any(|skip| {
        let skipped: Vec<i32> = input
            .iter()
            .enumerate()
            .filter(|(i, _v)| *i != skip)
            .map(|(_i, v)| *v)
            .collect();

        is_safe(&skipped)
    })
}

fn solve1(input: &str) -> u64 {
    let lines = input.lines();

    let data: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("Should be integer"))
                .collect()
        })
        .collect();

    data.iter().filter(|val| is_safe(val)).count() as u64
}

fn solve2(input: &str) -> u64 {
    let lines = input.lines();

    let data: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("Should be integer"))
                .collect()
        })
        .collect();

    data.iter().filter(|val| is_safe_with_dampener(val)).count() as u64
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
        assert_eq!(solve1(EXAMPLE), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 4);
    }
}
