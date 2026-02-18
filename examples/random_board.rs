// cargo run --example random_board
use rust_ricochet_robots::robots::{generator::{generate_random_grid, generate_random_position, random_robot}, solver::solve};

fn main() {
    let grid = generate_random_grid(15, 15, 100);
    println!("{}", grid);

    let position = generate_random_position(16, 16, 20);

    println!("{}", position);

    let target = random_robot(16, 16);

    println!("{:?}", solve(position, target))
}