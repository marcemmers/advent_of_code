#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
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
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
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
}

#[derive(Default, Clone)]
pub struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn from_text(input: &str) -> Self {
        Grid {
            grid: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    pub fn get(&self, pos: Position) -> Option<char> {
        self.grid.get(pos.y as usize)?.get(pos.x as usize).copied()
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut char> {
        self.grid.get_mut(pos.y as usize)?.get_mut(pos.x as usize)
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
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
}
