// cargo run --example random_board
use rust_ricochet_robots::robots::{
    generator::{generate_random_grid, generate_random_position, random_robot},
    solver::solve,
};

fn main() {
    // let grid = generate_random_grid(15, 15, 100);
    // println!("{}", grid);
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::seed_from_u64(42);

    let position = generate_random_position(16, 16, 20, &mut rng);

    println!("{}", position);

    let target = random_robot(16, 16, &mut rng);

    println!("{:?}", solve(position, target))
}
