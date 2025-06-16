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
        let movement = search::minimax_best_move(&board, 4);
        let mut movestring = String::new();
        if movement.is_none() {
            //check or stalemate, there are no moves left
            if board.is_check() {
                board.print_board();
                println!("{} loses by checkmate!", board.side_to_move.to_string());
                break;
            } else {
                board.print_board();
                println!("Stalemate!");
                break;
            }
        }

        if board.halfmove_clock >= 50 {
            board.print_board();
            println!("Draw by 50 move rule!");
            break;
        }

        movestring = movement.unwrap().to_string(&board);
        board.make_move(movement.unwrap());
            
        if board.side_to_move == Color::White {
            board.fullmove_number += 1
        }


        board.print_board();
        println!("Move made: {}", movestring);

        board.side_to_move = board.side_to_move.opposite();

        sleep(time::Duration::from_millis(100));
    }

}
