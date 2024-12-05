use std::time::Instant;

fn solve1(input: &str) -> u64 {
    let lines = input.lines();

    let mut chars: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut sum = chars
        .iter()
        .map(|line| {
            line.windows(4)
                .filter(|chs| *chs == ['X', 'M', 'A', 'S'] || *chs == ['S', 'A', 'M', 'X'])
                .count() as u64
        })
        .sum();

    for y in 0..(chars.len() - 3) {
        for x in 0..chars[0].len() {
            if (chars[y][x] == 'X'
                && chars[y + 1][x] == 'M'
                && chars[y + 2][x] == 'A'
                && chars[y + 3][x] == 'S')
                || (chars[y][x] == 'S'
                    && chars[y + 1][x] == 'A'
                    && chars[y + 2][x] == 'M'
                    && chars[y + 3][x] == 'X')
            {
                sum += 1;
            }
        }
    }

    let mut diagonal: Vec<Vec<char>> = vec![Vec::new(); chars.len() + chars[0].len() - 1];

    for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            diagonal[x + y].push(chars[y][x]);
        }
    }

    sum += diagonal
        .iter()
        .map(|line| {
            line.windows(4)
                .filter(|chs| *chs == ['X', 'M', 'A', 'S'] || *chs == ['S', 'A', 'M', 'X'])
                .count() as u64
        })
        .sum::<u64>();

    let mut diagonal: Vec<Vec<char>> = vec![Vec::new(); chars.len() + chars[0].len() - 1];

    chars.reverse();

    for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            diagonal[x + y].push(chars[y][x]);
        }
    }

    sum += diagonal
        .iter()
        .map(|line| {
            line.windows(4)
                .filter(|chs| *chs == ['X', 'M', 'A', 'S'] || *chs == ['S', 'A', 'M', 'X'])
                .count() as u64
        })
        .sum::<u64>();

    sum
}

fn solve2(input: &str) -> u64 {
    let lines = input.lines();

    let mut chars: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    for y in 1..(chars.len() - 1) {
        for x in 1..(chars[0].len() - 1) {
            if chars[y][x] == 'A'
                && matches!(
                    (
                        chars[y - 1][x - 1],
                        chars[y + 1][x + 1],
                        chars[y - 1][x + 1],
                        chars[y + 1][x - 1],
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
