use std::collections::{HashMap, VecDeque};

use grid::{Direction, Grid, Position};

pub struct AStar {
    grid: Grid,
    start: Position,
    goal: Position,
}

impl AStar {
    pub fn new(grid: Grid, start: Position, goal: Position) -> Self {
        Self { grid, start, goal }
    }

    pub fn calculate_path(
        &mut self,
        is_allowed: impl Fn(&Grid, Position, Position) -> bool,
    ) -> Vec<Position> {
        let grid_positions = self.grid.height() * self.grid.width();

        let mut open_set = VecDeque::with_capacity(grid_positions);
        let mut came_from = HashMap::with_capacity(grid_positions);
        let mut g_score = HashMap::with_capacity(grid_positions);
        let mut f_score = HashMap::with_capacity(grid_positions);

        let start_f_score = self.h(self.start);
        open_set.push_front((self.start, start_f_score));
        g_score.insert(self.start, 0);
        f_score.insert(self.start, start_f_score);

        while let Some((current, _)) = open_set.pop_front() {
            if current == self.goal {
                return reconstruct_path(&came_from, current);
            }

            for dir in Direction::all_directions() {
                let neighbor = current + dir;
                if !is_allowed(&self.grid, current, neighbor) {
                    continue;
                }
                let tentative = g_score.get(&current).unwrap() + 1;
                if tentative < *g_score.entry(neighbor).or_insert(i32::MAX) {
                    came_from.entry(neighbor).insert_entry(current);
                    g_score.entry(neighbor).insert_entry(tentative);
                    let f_score_value = tentative + self.h(neighbor);
                    f_score.entry(neighbor).insert_entry(f_score_value);
                    if !open_set.iter().any(|(pos, _)| *pos == neighbor) {
                        if let Some(pos) = open_set
                            .iter()
                            .position(|(_, score)| f_score_value < *score)
                        {
                            open_set.insert(pos, (neighbor, f_score_value));
                        } else {
                            open_set.push_back((neighbor, f_score_value));
                        }
                    }
                }
            }
        }
        Vec::new()
    }

    fn h(&self, pos: Position) -> i32 {
        let distance = self.goal.distance_xy(pos);
        distance.x + distance.y
    }
}

fn reconstruct_path(
    came_from: &HashMap<Position, Position>,
    mut current: Position,
) -> Vec<Position> {
    let mut path = Vec::new();
    path.push(current);
    while let Some(new) = came_from.get(&current) {
        current = *new;
        path.push(current);
    }
    path.reverse();
    path
}
