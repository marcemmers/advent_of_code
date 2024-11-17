use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn parse_sizes(sizes: &str) -> Vec<usize> {
    sizes
        .split(',')
        .map(|val| {
            val.parse::<usize>()
                .expect("Should be able to parse into usize")
        })
        .collect()
}

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let (record, sizes) = line
        .split_once(' ')
        .expect("Line should have a space in it");
    (record, parse_sizes(sizes))
}

type State = (String, Vec<usize>, Option<usize>);

fn calculate_arrangements(
    records: &str,
    sizes: &[usize],
    active: Option<usize>,
    memo: &mut HashMap<State, u64>,
) -> u64 {
    let state: State = (records.to_owned(), sizes.to_vec(), active);
    if let Occupied(items) = memo.entry(state.clone()) {
        return *items.get();
    }

    if (active.is_none() || active == Some(0)) && sizes.is_empty() {
        if records.chars().any(|ch| ch == '#') {
            return 0;
        }
        return 1;
    }
    if records.is_empty() {
        return 0;
    }

    let record = records
        .chars()
        .next()
        .expect("Should be at least 1 character here");
    let records = &records[1..];

    let value = if let Some(remainder) = active {
        if remainder == 0 {
            match record {
                '#' => 0,
                _ => calculate_arrangements(records, sizes, None, memo),
            }
        } else {
            match record {
                '.' => 0,
                _ => calculate_arrangements(records, sizes, Some(remainder - 1), memo),
            }
        }
    } else {
        match record {
            '.' => calculate_arrangements(records, sizes, None, memo),
            '#' => calculate_arrangements(records, &sizes[1..], Some(sizes[0] - 1), memo),
            '?' => {
                calculate_arrangements(records, sizes, None, memo)
                    + calculate_arrangements(records, &sizes[1..], Some(sizes[0] - 1), memo)
            }
            _ => 0,
        }
    };
    memo.insert(state, value);

    value
}

fn unfold(records: &str, sizes: &[usize]) -> (String, Vec<usize>) {
    (
        [records; 5].join("?"),
        std::iter::repeat_n(sizes, 5).flatten().copied().collect(),
    )
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    lines
        .map(parse_line)
        .map(|(records, sizes)| calculate_arrangements(records, &sizes, None, &mut HashMap::new()))
        .sum()
}

fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    lines
        .map(parse_line)
        .map(|(records, sizes)| {
            let (records, sizes) = unfold(records, &sizes);
            calculate_arrangements(&records, &sizes, None, &mut HashMap::new())
        })
        .sum()
}

const PUZZLE_FILENAME: &str = "./src/puzzle.txt";

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

    #[test]
    fn test_calculate_arrangements() {
        assert_eq!(
            calculate_arrangements("???", &[1], None, &mut HashMap::new()),
            3
        );
    }

    #[test]
    fn test_calculate_arrangements2() {
        assert_eq!(
            calculate_arrangements("???", &[1, 1], None, &mut HashMap::new()),
            1
        );
    }

    #[test]
    fn test_first_string() {
        let (records, sizes) = parse_line("?????????#?#.#?.?.# 4,3,1,1,1");
        assert_eq!(
            calculate_arrangements(records, &sizes, None, &mut HashMap::new()),
            8
        );
    }

    #[test]
    fn test_first_string_2nd() {
        let (records, sizes) = parse_line("?###???????? 3,2,1");
        let (records, sizes) = unfold(records, &sizes);
        assert_eq!(
            calculate_arrangements(&records, &sizes, None, &mut HashMap::new()),
            506250
        );
    }

    const EXAMPLE_FILENAME: &str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE_FILENAME), 21);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 525152);
    }
}
