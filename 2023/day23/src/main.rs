use std::fs;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn x_offset(&self, offset: i32) -> Self {
        Self {
            x: self.x + offset,
            y: self.y,
        }
    }
    fn y_offset(&self, offset: i32) -> Self {
        Self {
            x: self.x,
            y: self.y + offset,
        }
    }
}

#[derive(Debug)]
struct Section {
    start: Point,
    end: Point,
    len: u32,
}

fn get_point(input: &[Vec<char>], point: Point) -> Option<char> {
    if point.x >= 0
        && point.x < input[0].len() as i32
        && point.y >= 0
        && point.y < input.len() as i32
    {
        Some(input[point.y as usize][point.x as usize])
    } else {
        None
    }
}

fn is_crossing(input: &[Vec<char>], point: Point) -> bool {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter(|(x, y)| {
            matches!(
                get_point(input, point.x_offset(*x).y_offset(*y)),
                Some('>') | Some('v')
            )
        })
        .count()
        >= 3
}

fn get_next_points(input: &[Vec<char>], current: Point) -> Vec<Point> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|(x, y)| {
            let point = current.x_offset(*x).y_offset(*y);
            if let Some(ch) = get_point(input, point) {
                if ch.is_ascii_uppercase() {
                    return Some(point);
                }
                if ".>v".contains(ch) {
                    return Some(point);
                }
            }
            None
        })
        .collect()
}

fn get_downhill_points(input: &[Vec<char>], current: Point) -> Vec<Point> {
    [(1, 0, '>'), (0, 1, 'v')]
        .iter()
        .filter_map(|(x, y, downhill)| {
            let point = current.x_offset(*x).y_offset(*y);
            if get_point(input, point) == Some(*downhill) {
                Some(point)
            } else {
                None
            }
        })
        .collect()
}

fn travel_sections(input: &mut [Vec<char>], start: Point, end: Point, output: &mut Vec<Section>) {
    let mut steps = 0;
    let mut current = start;

    loop {
        let mut next_points = get_next_points(input, current);

        if next_points.is_empty() {
            if current == end {
                output.push(Section {
                    start,
                    end,
                    len: steps,
                });
            } else if get_point(input, current) == Some('X') {
                output.push(Section {
                    start,
                    end: current,
                    len: steps + 1,
                });
            } else {
                println!("Found the end at {:?}", current);
            }
            break;
        }

        if next_points.len() == 2 {
            next_points = next_points
                .iter()
                .filter(|point| !get_point(input, **point).unwrap().is_ascii_uppercase())
                .copied()
                .collect();
        }

        if next_points.len() == 1 {
            input[current.y as usize][current.x as usize] = ' ';
            steps += 1;

            current = next_points[0];
        } else {
            output.push(Section {
                start,
                end: current,
                len: steps + 1,
            });

            for next in get_downhill_points(input, current) {
                println!("Next: {:?}", next);
                travel_sections(input, next, end, output);
            }
            break;
        }
    }
}

fn calculate_path(start: Point, sections: &[Section]) -> u64 {
    let mut max = 0;

    for section in sections.iter().filter(|x| x.start == start) {
        println!("Try section: {:?}", section);
        let pathlen = calculate_path(section.end.x_offset(1), sections) + section.len as u64;
        if pathlen > max {
            max = pathlen;
        }
        let pathlen = calculate_path(section.end.y_offset(1), sections) + section.len as u64;
        if pathlen > max {
            max = pathlen;
        }
    }

    max
}

fn parse_input(mut input: Vec<Vec<char>>) -> u64 {
    let start = Point { x: 1, y: 0 };
    let end = Point {
        x: input[0].len() as i32 - 2,
        y: input.len() as i32 - 1,
    };

    println!("End: {:?}", end);

    input[0][1] = 'A';

    let y = input.len();
    let x = input[0].len();
    input[y - 1][x - 2] = 'Z';

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if is_crossing(
                &input,
                Point {
                    x: x as i32,
                    y: y as i32,
                },
            ) {
                input[y][x] = 'X';
            }
        }
    }

    let mut output = Vec::new();
    travel_sections(&mut input, start, end, &mut output);

    for line in input {
        println!("Line: {}", line.iter().collect::<String>());
    }
    println!("Output {:#?}", output);

    calculate_path(start, &output)
}

fn solve1(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines: Vec<Vec<char>> = input.lines().map(|v| v.chars().collect()).collect();

    parse_input(lines)
}

fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    0
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

    const EXAMPLE_FILENAME: &str = "./src/example.txt";

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE_FILENAME), 94);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 154);
    }
}
