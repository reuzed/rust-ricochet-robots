// cargo run --example random_board
use rust_ricochet_robots::robots::generator::generate_random_grid;

fn main() {
    let grid = generate_random_grid(15, 15, 100);
    println!("{}", grid);
}