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

    pub fn distance_xy(&self, other: Self) -> Distance {
        Distance {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Distance {
    pub x: i32,
    pub y: i32,
}

impl Distance {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self.step(rhs)
    }
}

impl Add<Distance> for Position {
    type Output = Self;

    fn add(self, rhs: Distance) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Distance;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Distance> for Position {
    type Output = Self;

    fn sub(self, rhs: Distance) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign<Distance> for Position {
    fn add_assign(&mut self, rhs: Distance) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Distance> for Position {
    fn sub_assign(&mut self, rhs: Distance) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn with_capacity(width: usize, height: usize) -> Self {
        Self {
            grid: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn new(width: usize, height: usize, start: char) -> Self {
        Self {
            grid: vec![start; width * height],
            width,
            height,
        }
    }

    pub fn from_text(input: &str) -> Self {
        let mut grid = Vec::with_capacity(input.len()); // Not exact but good estimation

        let width = input.lines().next().unwrap().len();
        let mut height = 0;

        for line in input.lines() {
            let mut row: Vec<char> = line.chars().collect();
            if width != row.len() {
                panic!("Row widths should match");
            }
            grid.append(&mut row);
            height += 1;
        }

        Grid {
            grid,
            width,
            height,
        }
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    pub fn get(&self, pos: Position) -> Option<char> {
        if !self.in_bounds(pos) {
            return None;
        }
        self.grid
            .get(pos.y as usize * self.width + pos.x as usize)
            .copied()
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut char> {
        if !self.in_bounds(pos) {
            return None;
        }
        self.grid
            .get_mut(pos.y as usize * self.width + pos.x as usize)
    }

    pub fn get_mut_row(&mut self, row: i32) -> &mut [char] {
        &mut self.grid[(row as usize * self.width)..((row as usize + 1) * self.width)]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter(&self) -> impl Iterator<Item = (Position, char)> + '_ {
        self.grid.iter().enumerate().map(|(i, ch)| {
            (
                Position {
                    x: (i % self.width) as i32,
                    y: (i / self.width) as i32,
                },
                *ch,
            )
        })
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = (usize, &[char])> + '_ {
        self.grid.chunks(self.width).enumerate()
    }

    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = (usize, &mut [char])> + '_ {
        self.grid.chunks_mut(self.width).enumerate()
    }

    pub fn find_one(&self, item: char) -> Option<Position> {
        self.grid.iter().enumerate().find_map(|(i, ch)| {
            if *ch == item {
                Some(Position {
                    x: (i % self.width) as i32,
                    y: (i / self.width) as i32,
                })
            } else {
                None
            }
        })
    }

    pub fn count_filtered<F>(&self, filter: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        self.grid.iter().filter(|ch| filter(**ch)).count()
    }

    pub fn print(&self) {
        self.grid
            .chunks(self.width)
            .for_each(|line| println!("{}", line.iter().collect::<String>()))
    }

    pub fn replace_all(&mut self, from: char, to: char) {
        self.grid
            .iter_mut()
            .filter(|ch| **ch == from)
            .for_each(|ch| *ch = to);
    }

    pub fn transpose(&self) -> Self {
        let mut grid = Vec::with_capacity(self.grid.len());
        for x in 0..self.width {
            for y in 0..self.height {
                let pos = y * self.width + x;
                grid.push(*self.grid.get(pos).unwrap());
            }
        }
        Self {
            grid,
            width: self.height,
            height: self.width,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose() {
        let grid1 = r"
11115
22226
33337
";

        let grid2 = r"
123
123
123
123
567
";
        let grid1 = Grid::from_text(grid1.trim());
        let grid2 = Grid::from_text(grid2.trim());
        assert_eq!(grid1.transpose(), grid2);
    }
}
