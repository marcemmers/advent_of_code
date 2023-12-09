use std::fs;
use std::time::Instant;

fn calculate_differences(input: &Vec<i64>) -> Vec<i64> {
    input
        .as_slice()
        .windows(2)
        .map(|val| val[1] - val[0])
        .collect()
}

fn generate_next(input: &Vec<i64>) -> i64 {
    let differences = calculate_differences(&input);
    let retval = if differences.iter().all(|x| *x == 0) {
        *input.last().unwrap()
    } else {
        input.last().unwrap() + generate_next(&differences)
    };
    // println!("Next: {retval}");
    return retval;
}

fn generate_previous(input: &Vec<i64>) -> i64 {
    let differences = calculate_differences(&input);
    let retval = if differences.iter().all(|x| *x == 0) {
        *input.first().unwrap()
    } else {
        input.first().unwrap() - generate_previous(&differences)
    };
    // println!("Next: {retval}");
    return retval;
}

fn common(filename: &str) -> Vec<Vec<i64>> {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    return lines
        .map(|line| {
            line.split(" ")
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
}

fn solve1(filename: &str) -> i64 {
    let input = common(filename);

    return input.iter().map(|val| generate_next(&val)).sum();
}

fn solve2(filename: &str) -> i64 {
    let input = common(filename);

    return input.iter().map(|val| generate_previous(&val)).sum();
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
        let result = solve1(EXAMPLE_FILENAME);
        assert_eq!(result, 114);
    }

    #[test]
    fn test2() {
        let result = solve2(EXAMPLE_FILENAME);
        assert_eq!(result, 2);
    }
}
