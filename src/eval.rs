use crate::board::{Board, Color, Piece};

pub fn eval(board: &Board) -> i32 {
    let white_material = count_material(board, Color::White);
    let black_material = count_material(board, Color::Black);

    white_material - black_material
}

pub fn count_material(board: &Board, color: Color) -> i32 {
    let mut material = 0;
    for piece in board.squares {
        if let Some(piece) = piece {
            if piece.color() == color {
                material += piece_value(&piece);
            }
        }
    }
    material

}

pub fn piece_value(piece: &Piece) -> i32 {
    match piece {
        Piece::Pawn(_) => 100,
        Piece::Knight(_) => 300,
        Piece::Bishop(_) => 300,
        Piece::Rook(_) => 500,
        Piece::Queen(_) => 900,
        Piece::King(_) => 0,
    }
}