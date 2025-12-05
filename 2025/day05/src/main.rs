use std::{ops::RangeInclusive, time::Instant};

fn parse_ingredient_list(input: &str) -> Vec<RangeInclusive<u64>> {
    let mut vec: Vec<RangeInclusive<u64>> = input
        .lines()
        .map(|line| {
            let (begin, end) = line.split_once('-').unwrap();
            begin.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    vec.sort_unstable_by_key(|v| *v.start());

    vec
}

fn solve1(input: &str) -> u64 {
    let (list, ingredients) = input.split_once("\n\n").unwrap();

    let ingredient_list = parse_ingredient_list(list);
    let ingredients: Vec<u64> = ingredients
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    ingredients
        .iter()
        .filter(|ingredient| ingredient_list.iter().any(|list| list.contains(ingredient)))
        .count() as u64
}

fn solve2(input: &str) -> u64 {
    let (list, _) = input.split_once("\n\n").unwrap();

    let ingredient_list = parse_ingredient_list(list);

    let mut current_id = 0;
    let mut ingredients = 0;
    for item in ingredient_list {
        if *item.end() < current_id {
            continue;
        }
        ingredients += item.end() - item.start().max(&current_id) + 1;
        current_id = *item.end() + 1;
    }

    ingredients
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
        assert_eq!(solve1(EXAMPLE), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 14);
    }
}
