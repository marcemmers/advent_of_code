use std::time::Instant;

fn count_xmas(input: &[Vec<char>]) -> u64 {
    input
        .iter()
        .map(|line| {
            line.windows(4)
                .filter(|chs| *chs == ['X', 'M', 'A', 'S'] || *chs == ['S', 'A', 'M', 'X'])
                .count() as u64
        })
        .sum()
}

fn solve1(input: &str) -> u64 {
    let lines = input.lines();

    let mut puzzle: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut sum = count_xmas(&puzzle);

    let transposed: Vec<Vec<char>> = (0..puzzle[0].len())
        .map(|col| (0..puzzle.len()).map(|row| puzzle[row][col]).collect())
        .collect();

    sum += count_xmas(&transposed);

    let mut diagonal: Vec<Vec<char>> = vec![Vec::new(); puzzle.len() + puzzle[0].len() - 1];

    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            diagonal[x + y].push(puzzle[y][x]);
        }
    }
    sum += count_xmas(&diagonal);

    let mut diagonal: Vec<Vec<char>> = vec![Vec::new(); puzzle.len() + puzzle[0].len() - 1];

    puzzle.reverse();

    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            diagonal[x + y].push(puzzle[y][x]);
        }
    }
    sum += count_xmas(&diagonal);

    sum
}

fn solve2(input: &str) -> u64 {
    let lines = input.lines();

    let puzzle: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    for y in 1..(puzzle.len() - 1) {
        for x in 1..(puzzle[0].len() - 1) {
            if puzzle[y][x] == 'A'
                && matches!(
                    (
                        puzzle[y - 1][x - 1],
                        puzzle[y + 1][x + 1],
                        puzzle[y - 1][x + 1],
                        puzzle[y + 1][x - 1],
                    ),
                    ('M', 'S', 'M', 'S')
                        | ('M', 'S', 'S', 'M')
                        | ('S', 'M', 'M', 'S')
                        | ('S', 'M', 'S', 'M')
                )
            {
                sum += 1;
            }
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
        assert_eq!(solve1(EXAMPLE), 18);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 9);
    }
}
