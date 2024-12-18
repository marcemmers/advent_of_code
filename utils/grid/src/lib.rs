use core::panic;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_char(ch: char) -> Self {
        match ch {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Parsing failed"),
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn all_directions() -> [Self; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    pub fn distance_xy(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self.step(rhs)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Default, Clone)]
pub struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn with_capacity(width: usize, height: usize) -> Self {
        Self {
            grid: vec![Vec::with_capacity(width); height],
        }
    }

    pub fn new(width: usize, height: usize, start: char) -> Self {
        Self {
            grid: vec![vec![start; width]; height],
        }
    }

    pub fn from_text(input: &str) -> Self {
        Grid {
            grid: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        self.get(pos).is_some()
    }

    pub fn get(&self, pos: Position) -> Option<char> {
        self.grid.get(pos.y as usize)?.get(pos.x as usize).copied()
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut char> {
        self.grid.get_mut(pos.y as usize)?.get_mut(pos.x as usize)
    }

    pub fn get_mut_row(&mut self, row: i32) -> &mut Vec<char> {
        &mut self.grid[row as usize]
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Position, char)> + '_ {
        self.grid.iter().enumerate().flat_map(|(y, line)| {
            line.iter().enumerate().map(move |(x, ch)| {
                (
                    Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    *ch,
                )
            })
        })
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = (usize, &[char])> + '_ {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, row)| (i, row.as_slice()))
    }

    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = (usize, &mut Vec<char>)> + '_ {
        self.grid.iter_mut().enumerate()
    }

    pub fn find_one(&self, item: char) -> Option<Position> {
        self.grid.iter().enumerate().find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, ch)| if *ch == item { Some(x) } else { None })
                .map(|x| Position {
                    x: x as i32,
                    y: y as i32,
                })
        })
    }

    pub fn count_filtered<F>(&self, filter: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        self.grid
            .iter()
            .map(|line| line.iter().filter(|ch| filter(**ch)).count())
            .sum()
    }

    pub fn print(&self) {
        self.grid
            .iter()
            .for_each(|line| println!("{}", line.iter().collect::<String>()))
    }

    pub fn replace_all(&mut self, from: char, to: char) {
        self.grid.iter_mut().for_each(|line| {
            line.iter_mut()
                .filter(|ch| **ch == from)
                .for_each(|ch| *ch = to);
        });
    }

    pub fn transpose(&self) -> Self {
        Self {
            grid: (0..self.grid[0].len())
                .map(|col| {
                    (0..self.grid.len())
                        .map(|row| self.grid[row][col])
                        .collect()
                })
                .collect(),
        }
    }
}
