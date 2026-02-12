use crate::robots::grid::{Direction, Wall};

use super::grid::Grid;

pub fn generate_random_grid(width: usize, height: usize, wall_count: usize) -> Grid {
    let mut walls = vec![];
    for _ in 0..wall_count {
        let x = rand::random_range(1..width - 1);
        let y = rand::random_range(1..height - 1);
        let wall = Wall::new(x, y, rand::random_bool(1.0/2.0));
        if !walls.contains(&wall){
            walls.push(wall);
        }
    }
    Grid::new(width, height, walls)
}