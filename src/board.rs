struct Board_state{
    pub squares: [option<Piece>; 64],
    pub side_to_move: Color,
    castling_rights: u8, // no hay un tipo de datos de 4 bits
    en_passant_square: option<u8>, // u8 para las en_passant_square, = 2^8 = 255 posibilidades
    halfmove_clock: u32, // para la regla de los 50 movimientos
    fullmove_number: u32 //para notación en general
}

enum Color{
    Black,
    White
}

#[derive(clone, copy)]
enum Piece{
    Pawn, Knight, Bishop, Rook, Queen, King
}

impl Board_state{
    pub fn new() -> self{
        let mut squares:[None;64];
        let castling_rights = 0b1111;
        let en_passant_square = None;
        let halfmove_clock = 0;
        let fullmove_number = 1;
        Board_state{
            squares,
            side_to_move: White,
            castling_rights,
            en_passant_square,
            halfmove_clock,
            fullmove_number
        }
    }
    pub fn print(&self){
        
    }
}