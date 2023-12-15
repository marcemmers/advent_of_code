use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
enum Action {
    Remove,
    Add(u8),
}

#[derive(Debug)]
struct Operation<'a> {
    box_id: u8,
    label: &'a str,
    action: Action,
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    strength: u8,
}

fn hash_string(string: &str) -> u8 {
    string
        .chars()
        .fold(0, |acc, ch| acc.wrapping_add(ch as u8).wrapping_mul(17))
}

fn generate_op(input: &str) -> Operation {
    if input.contains('-') {
        let label = input.strip_suffix('-').unwrap();
        return Operation {
            box_id: hash_string(&label),
            label,
            action: Action::Remove,
        };
    }
    let (label, strength) = input.split_once('=').unwrap();
    return Operation {
        box_id: hash_string(&label),
        label,
        action: Action::Add(strength.parse::<u8>().unwrap()),
    };
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let items = input.split(',');

    return items.map(|item| hash_string(item) as u64).sum();
}

fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let items = input.split(',');

    let map =
        items
            .map(|item| generate_op(item))
            .fold(HashMap::<u8, Vec<Lens>>::new(), |mut acc, op| {
                if !acc.contains_key(&op.box_id) {
                    acc.insert(op.box_id, Vec::new());
                }
                acc.entry(op.box_id).and_modify(|entry| {
                    if let Action::Add(strength) = op.action {
                        if let Some(pos) = entry.iter().position(|x| x.label == op.label) {
                            // println!("Change box {} label {}: {}->{}", op.box_id, op.label, strength, entry[pos].strength);
                            entry[pos].strength = strength;
                        } else {
                            // println!("Add box {} label {}: {}", op.box_id, op.label, strength);
                            entry.push(Lens {
                                label: op.label,
                                strength,
                            });
                        }
                    } else {
                        if let Some(pos) = entry.iter().position(|x| x.label == op.label) {
                            // println!("Remove box {} label {}: {}", op.box_id, op.label, entry[pos].strength);
                            entry.remove(pos);
                        }
                    }
                });
                acc
            });

    // println!("Map: {:?}", map);

    return (0..=255u8).fold(0, |acc, i| {
        if let Some(lenses) = map.get(&i) {
            return acc
                + lenses.iter().enumerate().fold(0, |acc, (idx, lens)| {
                    acc + ((i as u64 + 1) * (idx as u64 + 1) * lens.strength as u64)
                });
        }
        acc
    });
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
        assert_eq!(solve1(EXAMPLE_FILENAME), 1320);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 145);
    }
}
