use std::{
    collections::{hash_map::Entry, HashMap},
    time::Instant,
};

fn parse_rule(input: &str) -> Option<(u32, u32)> {
    input
        .split_once('|')
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
}

fn parse_print_run(input: &str) -> Vec<u32> {
    input.split(',').map(|a| a.parse().unwrap()).collect()
}

fn solve1(input: &str) -> u64 {
    let mut lines = input.lines();

    let rules: Vec<(u32, u32)> = lines.by_ref().map_while(parse_rule).collect();

    let mut rulemap: HashMap<u32, Vec<u32>> = HashMap::new();

    rules
        .iter()
        .for_each(|(a, b)| rulemap.entry(*a).or_default().push(*b));

    let updates: Vec<Vec<u32>> = lines.map(parse_print_run).collect();

    let mut sum = 0;

    for update in updates {
        if update
            .iter()
            .enumerate()
            .all(|(i, val)| match rulemap.entry(*val) {
                Entry::Occupied(rules) => update[0..i].iter().all(|x| !rules.get().contains(x)),
                _ => true,
            })
        {
            sum += update[update.len() / 2] as u64;
        }
    }

    sum
}

fn sort_update(mut update: Vec<u32>, rulemap: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    update.sort_by(|a, b| {
        if let Some(rules) = rulemap.get(a) {
            if rules.contains(b) {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Less
    });
    update
}

fn solve2(input: &str) -> u64 {
    let mut lines = input.lines();

    let rules: Vec<(u32, u32)> = lines.by_ref().map_while(parse_rule).collect();

    let mut rulemap: HashMap<u32, Vec<u32>> = HashMap::new();

    rules
        .iter()
        .for_each(|(a, b)| rulemap.entry(*a).or_default().push(*b));

    let updates: Vec<Vec<u32>> = lines.map(parse_print_run).collect();

    let mut sum = 0;

    for update in updates {
        if !update
            .iter()
            .enumerate()
            .all(|(i, val)| match rulemap.entry(*val) {
                Entry::Occupied(rules) => update[0..i].iter().all(|x| !rules.get().contains(x)),
                _ => true,
            })
        {
            let update = sort_update(update, &rulemap);
            sum += update[update.len() / 2] as u64;
        }
    }

    sum
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
        assert_eq!(solve1(EXAMPLE), 143);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 123);
    }
}
