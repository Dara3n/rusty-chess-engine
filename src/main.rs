pub mod board;
pub mod movegen;
pub mod movefilters;

fn main() {
    movefilters::filter(movegen::generate_moves(&board::init())); //maybe movegen::filter(...) ?
}
