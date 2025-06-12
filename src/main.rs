use core::time;
use std::thread::sleep;

use board::Color;
use movefilter::filter;
use movegen::generate_moves;
use rand::Rng;
mod board;
mod movegen;
mod movefilter;
mod search;


fn main() {
    let mut rng = rand::rng();
    let mut board = board::init(); 
    board.setup_initial_position();
    //board.setup_test_position();
    let mut pseudo_legal_moves = generate_moves(&board);
    let mut legal_moves = filter(&mut board, pseudo_legal_moves);

    loop {

        board.print_board();
        println!("");

        pseudo_legal_moves = generate_moves(&board);
        legal_moves = filter(&mut board, pseudo_legal_moves);
        print!("moves for {} = ", if board.side_to_move == Color::White {"White"} else {"Black"});
        println!("{}", legal_moves.len());
        sleep(time::Duration::from_millis(1000));
        if let Some(movement) = search::get_random_element(&legal_moves) {
            board.make_move(movement);
        } else {
            //check or stalemate, there are no moves left
            match board.side_to_move {
                Color::Black => if movegen::is_square_attacked(&board, board.black_king) {
                    println!("White wins!");
                } else {
                    println!("Stalemate!");
                },
                Color::White => if movegen::is_square_attacked(&board, board.white_king) {
                    println!("Black wins!");
                } else {
                    println!("Stalemate!");
                }
            }
        }
        
        if board.side_to_move == Color::White {
            board.fullmove_number += 1
        }

        board.side_to_move = board.side_to_move.opposite();

    }

}
