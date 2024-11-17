use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
enum Record {
    Empty,
    Unknown,
    Spring,
}

fn parse_record(line: &str) -> Vec<Record> {
    line.trim()
        // .trim_matches('.')
        .chars()
        .fold(Vec::new(), |mut vec: Vec<Record>, ch| {
            match ch {
                '?' => vec.push(Record::Unknown),
                '#' => vec.push(Record::Spring),
                _ => {
                    if let Some(record) = vec.last() {
                        if record != &Record::Empty {
                            vec.push(Record::Empty);
                        }
                    } else {
                        vec.push(Record::Empty);
                    }
                }
            }
            vec
        })
}

fn parse_sizes(sizes: &str) -> Vec<usize> {
    sizes
        .split(',')
        .map(|val| {
            val.parse::<usize>()
                .expect("Should be able to parse into usize")
        })
        .collect()
}

fn parse_line(line: &str) -> (Vec<Record>, Vec<usize>) {
    let (record, sizes) = line
        .split_once(' ')
        .expect("Line should have a space in it");
    (parse_record(record), parse_sizes(sizes))
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    records: Vec<Record>,
    sizes: Vec<usize>,
    active: Option<usize>,
}

fn calculate_arrangements(
    records: &[Record],
    sizes: &[usize],
    active: Option<usize>,
    memo: &mut HashMap<State, u64>,
) -> u64 {
    let state = State {
        records: records.to_vec(),
        sizes: sizes.to_vec(),
        active,
    };
    if let Occupied(items) = memo.entry(state.clone()) {
        return *items.get();
    }

    if (active.is_none() || active == Some(0)) && sizes.is_empty() {
        if records.iter().any(|record| record == &Record::Spring) {
            return 0;
        }
        return 1;
    }
    if records.is_empty() {
        return 0;
    }

    let (record, records) = records.split_first().unwrap();

    let value = if let Some(remainder) = active {
        if remainder == 0 {
            match record {
                Record::Spring => 0,
                _ => calculate_arrangements(records, sizes, None, memo),
            }
        } else {
            match record {
                Record::Empty => 0,
                _ => calculate_arrangements(records, sizes, Some(remainder - 1), memo),
            }
        }
    } else {
        match record {
            Record::Empty => calculate_arrangements(records, sizes, None, memo),
            Record::Spring => {
                calculate_arrangements(records, &sizes[1..], Some(sizes[0] - 1), memo)
            }
            Record::Unknown => {
                calculate_arrangements(records, sizes, None, memo)
                    + calculate_arrangements(records, &sizes[1..], Some(sizes[0] - 1), memo)
            }
        }
    };
    memo.insert(state, value);

    value
}

fn unfold(records: &[Record], sizes: &[usize]) -> (Vec<Record>, Vec<usize>) {
    let records = [records; 5];

    (
        records.join(&Record::Unknown),
        std::iter::repeat_n(sizes, 5).flatten().copied().collect(),
    )
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let mut memo = HashMap::new();
    lines
        .map(parse_line)
        .map(|(records, sizes)| calculate_arrangements(&records, &sizes, None, &mut memo))
        .sum()
}

fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    let mut memo = HashMap::new();
    lines
        .map(parse_line)
        .map(|(records, sizes)| {
            let (records, sizes) = unfold(&records, &sizes);

            calculate_arrangements(&records, &sizes, None, &mut memo)
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
    fn test_parse_record_multiple_empty() {
        assert_eq!(
            parse_record("?..?"),
            vec![Record::Unknown, Record::Empty, Record::Unknown]
        );
    }

    #[test]
    fn test_calculate_arrangements() {
        assert_eq!(
            calculate_arrangements(&[Record::Unknown; 3], &[1], None, &mut HashMap::new()),
            3
        );
    }

    #[test]
    fn test_calculate_arrangements2() {
        assert_eq!(
            calculate_arrangements(&[Record::Unknown; 3], &[1, 1], None, &mut HashMap::new()),
            1
        );
    }

    #[test]
    fn test_first_string() {
        let (records, sizes) = parse_line("?????????#?#.#?.?.# 4,3,1,1,1");
        assert_eq!(
            calculate_arrangements(&records, &sizes, None, &mut HashMap::new()),
            8
        );
    }

    #[test]
    fn test_first_string_2nd() {
        let (records, sizes) = parse_line("?###???????? 3,2,1");
        let (records, sizes) = unfold(&records, &sizes);
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
