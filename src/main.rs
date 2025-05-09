use movefilter::filter;
use movegen::generate_moves;

mod board;
mod movegen;
mod movefilter;

fn main() {
    let mut board = board::init(); //maybe movegen::filter(...) ?
    let pseudo_legal_moves = generate_moves(&board);
    let legal_moves = filter(&mut board, pseudo_legal_moves);
}
