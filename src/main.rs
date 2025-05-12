use core::time;
use std::thread::sleep;

use movefilter::filter;
use movegen::generate_moves;
use rand::Rng;
mod board;
mod movegen;
mod movefilter;

fn main() {
    let mut rng = rand::rng();
    let mut board = board::init(); //maybe movegen::filter(...) ?
    let pseudo_legal_moves = generate_moves(&board);
    let mut legal_moves = filter(&mut board, pseudo_legal_moves);

    sleep(time::Duration::from_millis(1000));
    board.make_move(Option::expect(legal_moves.pop(), "aaa legal moves esta vacio"));
    board.print_board();
}
