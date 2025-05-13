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
    let mut board = board::init(); //maybe movegen::filter(...) ?
    let mut pseudo_legal_moves = generate_moves(&board);
    let mut legal_moves = filter(&mut board, pseudo_legal_moves);

    loop {

        pseudo_legal_moves = generate_moves(&board);
        legal_moves = filter(&mut board, pseudo_legal_moves);
        sleep(time::Duration::from_millis(1000));
        board.make_move(Option::expect(search::get_random_element(&legal_moves),"aaa legal moves esta vacio"));

        
        if board.side_to_move == Color::White {
            board.fullmove_number += 1
        }

        board.print_board();

        board.side_to_move = board.side_to_move.opposite();
    }

}
