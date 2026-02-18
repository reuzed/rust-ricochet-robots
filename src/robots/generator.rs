use crate::robots::{grid::{Robot, RobotName, Wall}};

use super::grid::{Grid, Position};

const ROBOT_NAMES: [RobotName; 5] = [RobotName::Blue, RobotName::Green, RobotName:: Red, RobotName::Yellow, RobotName::Silver];

fn random_coord(width: usize,height: usize) -> (usize, usize) {
    let x = rand::random_range(0..width - 1);
    let y = rand::random_range(0..height - 1);
    (x,y)
}

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

pub fn generate_random_position(width: usize, height: usize, wall_count: usize) -> Position {
    let robots: Vec<Robot> = ROBOT_NAMES.map(|name| 
        {
            let (x,y) = random_coord(width, height);
            Robot::new(name, x, y)
    }).into_iter().collect();
    Position {
        grid: generate_random_grid(width, height, wall_count),
        robots,
    }
}

pub fn random_robot(width: usize, height: usize) -> Robot {
    use rand::seq::IndexedRandom;
    let (x,y) = random_coord(width, height);
    let name = ROBOT_NAMES.choose(&mut rand::rng()).unwrap().clone();
    Robot::new(name, x, y)
}
