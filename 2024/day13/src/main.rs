use std::time::Instant;

fn get_x_y(input: &str) -> (i64, i64) {
    let (input, y) = input.split_once('Y').unwrap();
    let (_, x) = input.split_once('X').unwrap();
    let (x, _) = x.split_once(',').unwrap();

    (x[1..].parse().unwrap(), y[1..].parse().unwrap())
}

fn solve_game(input: &str, offset: u64) -> u64 {
    let mut lines = input.lines();

    let (a_x, a_y) = get_x_y(lines.next().unwrap());
    let (b_x, b_y) = get_x_y(lines.next().unwrap());
    let (mut loc_x, mut loc_y) = get_x_y(lines.next().unwrap());

    loc_x += offset as i64;
    loc_y += offset as i64;

    let a = ((b_x * loc_y) - (b_y * loc_x)) / (b_x * a_y - b_y * a_x);
    let b = ((a_x * loc_y) - (a_y * loc_x)) / (a_x * b_y - a_y * b_x);

    if a_x * a + b_x * b != loc_x || a_y * a + b_y * b != loc_y {
        return 0;
    }

    a as u64 * 3 + b as u64
}

fn solve1(input: &str) -> u64 {
    let games = input.split("\n\n");

    games.map(|x| solve_game(x, 0)).sum()
}

fn solve2(input: &str) -> u64 {
    let games = input.split("\n\n");

    games.map(|x| solve_game(x, 10000000000000)).sum()
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
        assert_eq!(solve1(EXAMPLE), 480);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 0);
    }
}
