// Define types for grids, and their contents

use std::{collections::HashMap, fmt};

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum Direction {
    Horizontal, 
    Vertical,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirection {
    fn x(&self) -> i32 {
        match self {
            CardinalDirection::Left => -1,
            CardinalDirection::Right => 1,
            _ => 0
        }
    }
    fn y(&self) -> i32 {
        match self {
            CardinalDirection::Up => -1,
            CardinalDirection::Down => 1,
            _ => 0
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    x: usize,
    y: usize,
}
impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// A horizontal wall blocks horizontal movement so is rendered vertically 
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Wall {
    direction: Direction,
    coord: Coord,
}

impl Wall {
    pub fn new(x: usize, y: usize, vert: bool) -> Wall {
        Wall {
            direction: {if vert {Direction::Vertical} else {Direction::Horizontal}},
            coord: Coord { x, y }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
#[derive(Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    walls: Vec<Wall>,
    pub move_maps: HashMap<CardinalDirection, Vec<Vec<Coord>>>
}
impl std::hash::Hash for Grid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
        self.walls.hash(state);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum RobotName {
    Red,
    Green,
    Blue,
    Yellow,
    Silver,
}

// Type for robots and targets
#[derive(Clone, PartialEq, Eq, Hash,Debug)]
pub struct Robot {
    name: RobotName,
    pos: Coord,
}
impl Robot {
    pub fn new(
        name: RobotName, x: usize, y: usize
    ) -> Robot
    {
        Robot{name, pos:Coord { x, y }}
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Position {
    pub grid: Grid,
    pub robots: Vec<Robot>,
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Grid:\n{} Robots:\n{:?}", self.grid, self.robots)
    }
}

#[derive(Clone, Debug)]
pub struct Move {
    name: RobotName,
    start: Coord,
    end: Coord,
    direction: CardinalDirection,
}

// Build lookup maps for moving up down left and right from each square in grid
fn build_move_lookup_map(width: usize, height: usize, walls: &Vec<Wall>, direction: &CardinalDirection) -> Vec<Vec<Coord>> {
    // Initialise map with 0s
    let mut map = vec![vec![Coord{x:0,y:0};width];height];
    for y in 0..width {
        for x in 0..height {
            // Set default positions if there were no walls, move to an edge of the grid
            match direction {
                CardinalDirection::Up => {map[y][x] = Coord { x, y:0 };},
                CardinalDirection::Down => {map[y][x] = Coord { x, y:height-1 };},
                CardinalDirection::Left => {map[y][x] = Coord { x:0, y };},
                CardinalDirection::Right => {map[y][x] = Coord { x:width-1, y };},
            };
            // Check for intercepting walls
            match direction {
                CardinalDirection::Up => {
                    for wall in walls {
                        if wall.direction == Direction::Vertical && wall.coord.x == x && wall.coord.y <= y {
                            map[y][x] = Coord { x, y:map[y][x].y.max(wall.coord.y) };
                        }
                    }
                },
                CardinalDirection::Down => {
                    for wall in walls {
                        if wall.direction == Direction::Vertical && wall.coord.x == x && wall.coord.y > y {
                            map[y][x] = Coord { x, y:map[y][x].y.min(wall.coord.y-1) };
                        }
                    }
                },
                CardinalDirection::Left => {
                    for wall in walls {
                        if wall.direction == Direction::Horizontal && wall.coord.y == y && wall.coord.x <= x {
                            map[y][x] = Coord { x:map[y][x].x.max(wall.coord.x), y };
                        }
                    }
                },
                CardinalDirection::Right => {
                    for wall in walls {
                        if wall.direction == Direction::Horizontal && wall.coord.y == y && wall.coord.x > x {
                            map[y][x] = Coord { x:map[y][x].x.min(wall.coord.x-1), y };
                        }
                    }
                },
            };
        }
    }
    map
} 

impl Grid {
    pub fn new(width: usize, height: usize, walls: Vec<Wall>) -> Grid {
        let mut move_maps = HashMap::new();
        for direction in &[CardinalDirection::Up, CardinalDirection::Down, CardinalDirection::Left, CardinalDirection::Right] {
            move_maps.insert(direction.clone(), build_move_lookup_map(width, height, &walls, direction));
        }
        Grid { width, height, walls, move_maps }
    }

    pub fn move_lookup(&self, start: &Coord, direction: &CardinalDirection) -> Coord {
        self.move_maps.get(&direction).unwrap()[start.y][start.x]
    }
}

impl Position {
    pub fn move_lookup(&self, start: &Coord, direction: CardinalDirection) -> Coord {
        // Use grid move lookup map to see where robot can move unconstrained
        // Check for other robots blocking the path to these unconstrained move squares
        let mut unconstrained_end = self.grid.move_lookup(start, &direction);
        match direction {
            CardinalDirection::Down =>{for robot in &self.robots {
                if robot.pos.x == unconstrained_end.x && start.y < robot.pos.y && robot.pos.y <= unconstrained_end.y {
                    unconstrained_end.y = robot.pos.y - 1;
                }
            }},
            CardinalDirection::Up =>{for robot in &self.robots {
                if robot.pos.x == unconstrained_end.x && start.y > robot.pos.y && robot.pos.y >= unconstrained_end.y {
                    unconstrained_end.y = robot.pos.y + 1;
                }
            }},
            CardinalDirection::Right =>{for robot in &self.robots {
                if robot.pos.y == unconstrained_end.y && start.x < robot.pos.x && robot.pos.x <= unconstrained_end.x {
                    unconstrained_end.y = robot.pos.x - 1;
                }
            }},
            CardinalDirection::Left =>{for robot in &self.robots {
                if robot.pos.y == unconstrained_end.y && start.x > robot.pos.x && robot.pos.x >= unconstrained_end.x {
                    unconstrained_end.y = robot.pos.x + 1;
                }
            }},
        };
        unconstrained_end
    }

    pub fn where_is(&self, robot_name: RobotName) -> Coord {
        self.robots.iter().filter(|r| r.name == robot_name).nth(1).unwrap().pos
    }

    pub fn moves(&self) -> Vec<Move> {
        // For each robot consider each cardinal direction
        // If moving in that direction does not leave them stuck, add to moveset.
        let mut moveset: Vec<Move> = vec![];
        for robot in &self.robots {
            for direction in [CardinalDirection::Down, CardinalDirection::Left, CardinalDirection::Up, CardinalDirection::Right] {
                let robot_name = robot.name;
                let start = robot.pos;
                let end = self.move_lookup(&start, direction);
                if start == end {
                    continue;
                }
                moveset.push(Move { name: robot_name, start, end, direction });
            }
        }
        moveset
    }

    pub fn make_move(&self, robot_move: &Move) -> Position {
        let mut robots: Vec<Robot> = self.robots.clone().into_iter().filter(|r| r.name != robot_move.name).collect();
        robots.push(Robot { name: robot_move.name, pos: robot_move.end });
        Position { grid: self.grid.clone(), robots: robots }
    }
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