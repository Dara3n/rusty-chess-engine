use std::io;
use std::thread::sleep;
use std::time;

use board::Color;
use board::Board;
use movegen::Move;

mod board;
mod movegen;
mod search;
mod eval;

fn main() {
    let mut board = Board::default();

    board.print_board();
    let player = select_color();

    loop {
        if board.side_to_move == player {
            let mut movement = String::new(); 
            io::stdin().read_line(&mut movement).unwrap();
            if let Ok(user_move) = Move::string_to_move(&movement.trim(), &board) {
                board.make_move(user_move);
                board.side_to_move = board.side_to_move.opposite();
            } else {
                println!("Invalid move");
                continue;
            }
        } else {

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
            println!("Move made by engine: {}", movestring);
            board.side_to_move = board.side_to_move.opposite();
        }

            
        if board.side_to_move == Color::White {
            board.fullmove_number += 1
        }


        board.print_board();


        sleep(time::Duration::from_millis(100));
    }

}

pub fn select_color() -> Color {
    Color::White
}