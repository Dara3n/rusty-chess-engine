use std::thread::sleep;
use std::time;

use board::Color;
use board::Board;

mod board;
mod movegen;
mod search;
mod eval;

fn main() {
    let mut board = Board::default();

    loop {
        sleep(time::Duration::from_millis(100));
        let movement = search::minimax_best_move(&board, 3);
        let mut movestring = String::new();
        if movement.is_some() {
            movestring = movement.unwrap().to_string(&board);
            board.make_move(movement.unwrap());
            //probably future will be board.make_move(search::get_optimal_move(depth, legal_moves))
        } else {
            //check or stalemate, there are no moves left
            if board.is_check() {
                board.print_board();
                println!("Move made: {}", movestring);
                println!("{} loses by checkmate!", board.side_to_move.to_string());
                break;
            } else {
                board.print_board();
                println!("Move made: {}", movestring);
                println!("Stalemate!");
                break;
            }
        }
        if board.side_to_move == Color::White {
            board.fullmove_number += 1
        }

        if board.halfmove_clock >= 50 {
            board.print_board();
            println!("Move made: {}", movestring);
            println!("Draw by 50 move rule!");
            break;
        }

        board.print_board();
        println!("Move made: {}", movestring);

        board.side_to_move = board.side_to_move.opposite();
    }

}
