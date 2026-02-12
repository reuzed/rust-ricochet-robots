// cargo run --example test_board
use rust_ricochet_robots::robots::grid::{Grid, Wall};

fn main() {
    let walls = vec![
        Wall::new(2,2,true),
        Wall::new(2,2,false),
        Wall::new(3,4,true),
    ];
    let grid = Grid::new(5, 5, walls);
    println!("{}", grid);
    println!("{:#?}", grid.move_maps);
}