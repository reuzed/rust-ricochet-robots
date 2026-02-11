// Define types for grids, and their contents

use std::fmt;

enum Direction {
    Horizontal, 
    Vertical,
}

struct Coord {
    x: usize,
    y: usize,
}

struct Wall {
    direction: Direction,
    coord: Coord,
}

struct Grid {
    width: usize,
    height: usize,
    walls: Vec<Wall>,
}

struct Position {
    grid: Grid,
    robots: Vec<Coord>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = {
                    let mut vw = false;
                    let mut hw = false;
                    for wall in &self.walls {
                        if wall.coord.x == x && wall.coord.y == y {
                            match wall.direction {
                                Direction::Horizontal => {hw = true},
                                Direction::Vertical => {vw = true},
                            }
                        }
                    }
                    match (hw, vw) {
                        (true, true) => '┌',
                        (true, false) => '│',
                        (false, true) => '─',
                        (false, false) => '·',
                    }
                };
                write!(f,"{}",c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}