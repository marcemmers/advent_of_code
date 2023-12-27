use std::collections::HashSet;
use std::time::Instant;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhatten_distance(&self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn step(&self, heading: Heading) -> Point {
        match heading {
            Heading::North => Point {
                x: self.x,
                y: self.y + 1,
            },
            Heading::South => Point {
                x: self.x,
                y: self.y - 1,
            },
            Heading::East => Point {
                x: self.x + 1,
                y: self.y,
            },
            Heading::West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn is_on_map(&self, map: &Vec<Vec<char>>) -> bool {
        self.x >= 0 && self.y >= 0 && self.y < map.len() as i32 && self.x < map[0].len() as i32
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new() -> Self {
        Grid { grid: Vec::new() }
    }

    fn from_string(filename: &str) -> Grid {
        let input = fs::read_to_string(filename).expect("Should have been read");
        let lines = input.lines();
        let mut grid: Grid = Grid::new();
        for line in lines {
            grid.insert(line.chars().collect());
        }
        return grid;
    }

    fn get_value(&self, point: Point) -> Option<char> {
        self.grid
            .get(point.y as usize)?
            .get(point.x as usize)
            .copied()
    }

    fn insert(&mut self, new: Vec<char>) {
        self.grid.insert(0, new);
    }
}


fn solve1(filename: &str, steps: i32) -> u64 {
    println!("Solving for file: {filename}");

    let mut grid = Grid::from_string(filename);


    let mut start = None;
    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[0].len() {
            let point = Point{x: x as i32, y: y as i32};
            if grid.get_value(point) == Some('S') {
                start = Some(point);
            }
        }
    }

    let start = start.expect("Should have found start");

    let mut set: HashSet<Point> = HashSet::new();

    set.insert(start);

    for _ in 0..steps {
        let mut new_set: HashSet<Point> = HashSet::new();

        let mut insert_if_option = |point: Point| {
            if point.is_on_map(&grid.grid) && grid.grid[point.y as usize][point.x as usize] == '.' {
                new_set.insert(point);
                grid.grid[point.y as usize][point.x as usize] = 'O';
            }
        };

        for item in set.iter() {
            insert_if_option(item.step(Heading::North));
            insert_if_option(item.step(Heading::South));
            insert_if_option(item.step(Heading::East));
            insert_if_option(item.step(Heading::West));
        }

        set = new_set;
    }


    let is_even = steps % 2 == 0;

    let mut sum = 0;

    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[0].len() {
            let point = Point{x: x as i32, y: y as i32};
            let distance = start.manhatten_distance(point);
            let value = grid.get_value(point);
            if distance <= steps && (value == Some('S') || value == Some('O')) {
                if (is_even && distance % 2 == 0) || (!is_even && distance % 2 == 1) {
                    sum += 1;
                }
            }
        }
    }

    for line in grid.grid.iter().rev() {
        println!("{}", line.iter().collect::<String>());
    }

    return sum;
}


fn solve2(filename: &str) -> u64 {
    println!("Solving for file: {filename}");
    let input = fs::read_to_string(filename).expect("Should have been read");

    let lines = input.lines();

    return 0;
}


const PUZZLE_FILENAME: &'static str = "./src/puzzle.txt";

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve1(PUZZLE_FILENAME, 64));
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
        assert_eq!(solve1(EXAMPLE_FILENAME, 6), 16);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE_FILENAME), 0);
    }
}