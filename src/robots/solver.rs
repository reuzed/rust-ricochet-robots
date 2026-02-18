use std::collections::{HashSet, VecDeque};

use rustc_hash::{FxHashMap, FxHashSet};

use super::grid::{Move, Position, Robot};

fn is_solved(position: &Position, target: &Robot) -> bool {
    for robot in &position.robots {
        if robot == target {
            return true;
        }
    }
    false
}

const MAX_MOVES: u32 = 20;
// Do BFS, try all moves until finding a winning position
pub fn solve(position: Position, target: Robot) -> Vec<(Vec<Move>, Position)> {
    // If robot is in winning position done
    let mut visited_positions: HashSet<Position> = HashSet::with_capacity(100_000);
    let mut current_positions: Vec<(Vec<Move>, Position)> = Vec::with_capacity(100_000);
    current_positions.push((Vec::new(), position));
    let mut solved_positions: Vec<(Vec<Move>, Position)> = Vec::new();
    let start_time = std::time::Instant::now();
    for i in 0..MAX_MOVES {
        println!(
            "Move number {i}. Visited: {}, Current: {}, Solved: {}, Time: {:?}, Log current: {}",
            visited_positions.len(),
            current_positions.len(),
            solved_positions.len(),
            start_time.elapsed(),
            f32::log(current_positions.len() as f32, 20.0)
        );
        let mut new_positions: Vec<(Vec<Move>, Position)> = Vec::new();
        for m_p in current_positions {
            let moves = m_p.0;
            let pos = m_p.1;
            for robot_move in pos.moves() {
                let new_position = pos.make_move(&robot_move);
                let mut new_moves = moves.clone();
                new_moves.push(robot_move);
                if is_solved(&new_position, &target) {
                    solved_positions.push((new_moves, new_position));
                } else if !visited_positions.contains(&new_position) {
                    new_positions.push((new_moves, new_position));
                }
            }
            visited_positions.insert(pos);
        }
        current_positions = new_positions;
    }
    solved_positions
}

pub struct BfsData {
    positions: Vec<Position>,
    positions_hash: FxHashSet<Position>,
    queue: VecDeque<usize>,
    prev: FxHashMap<usize, usize>,
    solved_positions: Vec<usize>
}

impl BfsData {
    fn solutions_count(&self) -> usize {
        self.solved_positions.len()
    }
}

pub fn solve_2(position: Position, target: Robot) -> BfsData { 
    let mut data = BfsData {
        positions: vec![position.clone()],
        positions_hash: {
            let mut p = FxHashSet::default();
            p.insert(position.clone());
            p
        },
        queue: VecDeque::from([0]),
        prev: FxHashMap::default(),
        solved_positions: Vec::new(),
    };

    let mut new_move_count_index = 0;

    let start_time = std::time::Instant::now();
    while let Some(i) = data.queue.pop_front() {
        if i == new_move_count_index {
            new_move_count_index = data.positions.len();
            println!(
            "Move number {i}. Visited: {}, Current: {}, Solved: {}, Time: {:?}",
            data.positions.len(),
            data.queue.len(),
            data.solutions_count(),
            start_time.elapsed(),
            );
        }
        
        let pos = &data.positions[i].clone();
        if is_solved(&pos, &target) {
            println!("Solved !");
            data.solved_positions.push(i);
        }
        for robot_move in pos.moves() {
            let new_position = pos.make_move(&robot_move);
            if data.positions_hash.contains(&new_position) {
                continue
            } 
            let j = data.positions.len(); // Index of new position in data.positions
            data.queue.push_back(j); // Queue new position for processing
            data.positions.push(new_position.clone()); // Add the position
            data.positions_hash.insert(new_position);
            data.prev.insert(j, i); // Link to where the position came from
        }
    }
    data
}
