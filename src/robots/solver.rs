use std::collections::{HashSet};

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
    let mut visited_positions: HashSet<Position> = HashSet::new();
    let mut current_positions: Vec<(Vec<Move>, Position)> = vec![(Vec::new(), position)];
    let mut solved_positions: Vec<(Vec<Move>, Position)> = Vec::new();
    for i in 0..MAX_MOVES {
        println!("Move number {i}. Visited: {}, Current: {}, Solved: {}", visited_positions.len(), current_positions.len(), solved_positions.len());
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
                }
                else if !visited_positions.contains(&new_position){
                    new_positions.push((new_moves, new_position));
                }
            };
            visited_positions.insert(pos);
        };
        current_positions = new_positions;
    };
    solved_positions
}