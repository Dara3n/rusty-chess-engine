use rand::Rng;

use crate::board::Color;
use crate::eval;
use crate::{board::Board, movegen::{Move, generate_moves}};


pub fn _get_random_element(vector: &Vec<Move>) -> Option<Move> {
    if vector.is_empty() {
        return None;
    }
    
    let mut rng = rand::rng();
    let index = rng.random_range(0..vector.len());
    
    Some(vector[index].clone())
}


pub fn _minmax_depth_1(board: &Board, moves: &Vec<Move>) -> Option<Move> {
    if moves.is_empty() {
        return None;
    }

    let mut best_score = if board.side_to_move == Color::White {
        i32::MIN
    } else {
        i32::MAX
    };

    let mut best_move = moves[0];

    for &move_candidate in moves {
        let mut board_copy = *board;
        board_copy.make_move(move_candidate);

        let score = eval::eval(&board_copy);
        
        let is_better = if board.side_to_move == Color::White {
            score > best_score
        } else {
            score < best_score
        };

        if is_better {
            println!("New best score ({}) for {}", score, board.side_to_move.to_string());
            best_score = score;
            best_move = move_candidate;
        }
    }
    return Some(best_move);

}

pub fn minimax_best_move(board: &Board, depth: u8) -> Option<Move> {
    let moves = generate_moves(board);
    if moves.is_empty() {
        return None;
    }
    
    let maximizing = board.side_to_move == Color::White;
    let mut best_move = moves[0];
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
    
    for move_candidate in moves {
        let mut board_copy = *board;
        board_copy.make_move(move_candidate);
        let score = minimax(&board_copy, depth - 1, !maximizing, board.side_to_move.opposite());
        
        let is_better = if maximizing {
            score > best_score
        } else {
            score < best_score
        };
        
        if is_better {
            best_score = score;
            best_move = move_candidate;
        }
    }
    
    println!("{}", best_score);
    Some(best_move)
}

fn minimax(board: &Board, depth: u8, maximizing_player: bool, active_side: Color) -> i32 {
    if depth == 0 {
        return eval::eval(&board);
    }

    let mut board_copy = *board;
    board_copy.side_to_move = active_side;
    let moves = generate_moves(&board_copy);

    if moves.is_empty() {
        if board_copy.is_check() {
            return if active_side == Color::White { i32::MIN + depth as i32 } else { i32::MAX - depth as i32 };
        } else {
            //stalemate
            return 0;
        }
    }

    let mut best_score = if maximizing_player {
        i32::MIN
    } else {
        i32::MAX
    };

    for move_candidate in moves {
        let mut board_for_move = board_copy;
        
        board_for_move.make_move(move_candidate);

        let opposite_side = active_side.opposite(); 
            
        let eval_score = minimax(&board_for_move, depth - 1, !maximizing_player, opposite_side);

        best_score = if maximizing_player {
            best_score.max(eval_score)
        } else {
            best_score.min(eval_score)
        };
    }

    best_score

}