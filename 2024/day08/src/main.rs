use grid::{Grid, Position};
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};
use utils::permutations;

fn solve1(input: &str) -> u64 {
    let grid = Grid::from_text(input);

    let mut map: HashMap<char, Vec<Position>> = HashMap::with_capacity(26 + 26 + 10);

    grid.iter()
        .filter(|(_, ch)| *ch != '.')
        .for_each(|(pos, ch)| map.entry(ch).or_default().push(pos));

    let mut set: HashSet<Position> = HashSet::new();

    map.iter().for_each(|(_, positions)| {
        permutations(positions).for_each(|(a, b)| {
            let diff = a.distance_xy(*b);
            [*a + diff, *b - diff].iter().for_each(|pos| {
                if grid.in_bounds(*pos) {
                    set.insert(*pos);
                }
            });
        });
    });

    set.len() as u64
}

fn solve2(input: &str) -> u64 {
    let grid = Grid::from_text(input);

    let mut map: HashMap<char, Vec<Position>> = HashMap::with_capacity(26 + 26 + 10);

    grid.iter()
        .filter(|(_, ch)| *ch != '.')
        .for_each(|(pos, ch)| map.entry(ch).or_default().push(pos));

    let mut set: HashSet<Position> = HashSet::new();

    map.iter().for_each(|(_, positions)| {
        permutations(positions).for_each(|(a, b)| {
            let diff = a.distance_xy(*b);
            let mut pos = *a;
            while grid.in_bounds(pos) {
                set.insert(pos);
                pos += diff;
            }
            pos = *b;
            while grid.in_bounds(pos) {
                set.insert(pos);
                pos -= diff;
            }
        });
    });

    set.len() as u64
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
        assert_eq!(solve1(EXAMPLE), 14);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 34);
    }
}
