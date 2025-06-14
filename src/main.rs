use core::time;
use std::process::exit;
use std::thread::sleep;

use board::Color;
use board::Board;
use movegen::generate_moves;

mod board;
mod movegen;
mod search;


fn main() {
    let mut board = Board::default();
    let mut legal_moves;

    loop {

        board.print_board();
        println!("");

        legal_moves = generate_moves(&mut board);
        print!("moves for {} = ", if board.side_to_move == Color::White {"White"} else {"Black"});
        println!("{}", legal_moves.len());
        println!("rule of 50 clock: {}", board.halfmove_clock);
        sleep(time::Duration::from_millis(1000));
        if let Some(movement) = search::get_random_element(&legal_moves) {
            board.make_move(movement);
            //probably future will be board.make_move(search::get_optimal_move(depth, legal_moves))
        } else {
            //check or stalemate, there are no moves left
            if board.is_check() {
                println!("{} loses by checkmate!", board.side_to_move.to_string());
                break;
            } else {
                println!("Stalemate!");
                break;
            }
        }

        
        if board.side_to_move == Color::White {
            board.fullmove_number += 1
        }

        if board.halfmove_clock >= 50 {
            println!("Draw by 50 move rule!");
            break;
        }

        board.side_to_move = board.side_to_move.opposite();

    }

}
