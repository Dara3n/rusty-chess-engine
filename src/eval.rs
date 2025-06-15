use crate::board::{Board, Color, Piece};

//pawn table, every table looks reversed, and it is. So... yeah 
const PAWN_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

const KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50
];

const BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,  
    -10,  0,  0,  0,  0,  0,  0,-10, 
    -10,  0,  5, 10, 10,  5,  0,-10, 
    -10,  5,  5, 10, 10,  5,  5,-10, 
    -10,  0, 10, 10, 10, 10,  0,-10,  
    -10, 10, 10, 10, 10, 10, 10,-10,  
    -10,  5,  0,  0,  0,  0,  5,-10,  
    -20,-10,-10,-10,-10,-10,-10,-20   

];

const ROOK_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,  
    5, 10, 10, 10, 10, 10, 10,  5,  
    -5,  0,  0,  0,  0,  0,  0, -5, 
    -5,  0,  0,  0,  0,  0,  0, -5, 
    -5,  0,  0,  0,  0,  0,  0, -5, 
    -5,  0,  0,  0,  0,  0,  0, -5, 
    -5,  0,  0,  0,  0,  0,  0, -5, 
    0,  0,  0,  5,  5,  0,  0,  0   
];

const QUEEN_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5, 
    0,  0,  5,  5,  5,  5,  0, -5,  
    -10,  5,  5,  5,  5,  5,  0,-10, 
    -10,  0,  5,  0,  0,  0,  0,-10, 
    -20,-10,-10, -5, -5,-10,-10,-20   
];

const KING_TABLE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30, 
    -30,-40,-40,-50,-50,-40,-40,-30, 
    -30,-40,-40,-50,-50,-40,-40,-30, 
    -30,-40,-40,-50,-50,-40,-40,-30, 
    -20,-30,-30,-40,-40,-30,-30,-20, 
    -10,-20,-20,-20,-20,-20,-20,-10, 
    20, 20,  0,  0,  0,  0, 20, 20,  
    20, 30, 10,  0,  0, 10, 30, 20   
];

const PIECE_SQUARE_TABLE:[[i32; 64]; 6] = [
    PAWN_TABLE,
    KNIGHT_TABLE,
    BISHOP_TABLE,
    ROOK_TABLE,
    QUEEN_TABLE,
    KING_TABLE,
];

pub fn eval(board: &Board) -> i32 {
    let white_material = count_material(board, Color::White);
    let black_material = count_material(board, Color::Black);

    white_material - black_material
}

pub fn count_material(board: &Board, color: Color) -> i32 {
    let mut material = 0;
    for (index, piece) in board.squares.iter().enumerate() {
        if let Some(piece) = piece {
            if piece.color() == color {
                material += piece_value(&piece);

                let piece_type_index = match piece {
                    Piece::Pawn(_) => 0, 
                    Piece::Knight(_) => 1, 
                    Piece::Bishop(_) => 2, 
                    Piece::Rook(_) => 3, 
                    Piece::Queen(_) => 4, 
                    Piece::King(_) => 5, 
                };

                let table_index = if piece.color() == Color::White {
                    63 - index
                } else {
                    index
                };

                material += PIECE_SQUARE_TABLE[piece_type_index][table_index];
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