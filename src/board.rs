use crate::movegen::{Move, MoveType};

pub struct Board{
    pub squares: [Option<Piece>; 64],
    pub side_to_move: Color,
    pub castling_rights: u8, // no hay un tipo de datos de 4 bits
    pub en_passant_square: Option<u8>, // u8, = 2^8 = 255 posibilidades, necesitamos solo 64
    pub halfmove_clock: u32, // para la regla de los 50 movimientos
    pub fullmove_number: u32, //para notación en general
    pub white_king: u16,
    pub black_king: u16
}

pub const WHITE_KINGSIDE_CASTLING_RIGHTS: u8 = 0b1000;
pub const WHITE_QUEENSIDE_CASTLING_RIGHTS: u8 = 0b0100;
pub const BLACK_KINGSIDE_CASTLING_RIGHTS: u8 = 0b0010;
pub const BLACK_QUEENSIDE_CASTLING_RIGHTS: u8 = 0b0001;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color{
    Black,
    White
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece{
    Pawn(Color),
    Knight(Color), 
    Bishop(Color), 
    Rook(Color), 
    Queen(Color), 
    King(Color)
}


#[derive(Clone, Copy)]
pub struct UndoInfo {
    pub captured_piece: Option<Piece>,
    pub en_passant_square: Option<u8>,
    pub castling_rights: u8,
    pub halfmove_clock: u32,
    pub special_info: SpecialInfo,
}

#[derive(Clone, Copy)]
pub enum SpecialInfo{
    None, 

    Castle {
        rook_from: u16,
        rook_to: u16,
    },

    EnPassant {
        en_passant_square: u16,
    },
    
    Promotion

}


impl Board{
    pub fn new() -> Self{
        let squares:[Option<Piece>; 64] = [None; 64]; 
        let castling_rights = 0b1111;
        let en_passant_square = None;
        let halfmove_clock = 0;
        let fullmove_number = 1;
        let mut board = Board{
            squares, 
            side_to_move: Color::White,
            castling_rights,
            en_passant_square,
            halfmove_clock,
            fullmove_number,
            black_king: 60, // initial squares of black and white kings
            white_king: 4,
        };
        board.setup_initial_position();
        board
        
    }

    fn setup_initial_position(&mut self){
        for i in 0..8 {
            self.squares[8 + i] = Some(Piece::Pawn(Color::White));
            self.squares[48 + i] = Some(Piece::Pawn(Color::Black));

        }

        self.squares[0] = Some(Piece::Rook(Color::White));
        self.squares[7] = Some(Piece::Rook(Color::White));
        self.squares[56] = Some(Piece::Rook(Color::Black));
        self.squares[63] = Some(Piece::Rook(Color::Black));

        self.squares[1] = Some(Piece::Knight(Color::White));
        self.squares[6] = Some(Piece::Knight(Color::White));
        self.squares[57] = Some(Piece::Knight(Color::Black));
        self.squares[62] = Some(Piece::Knight(Color::Black));
        
        self.squares[2] = Some(Piece::Bishop(Color::White));
        self.squares[5] = Some(Piece::Bishop(Color::White));
        self.squares[58] = Some(Piece::Bishop(Color::Black));
        self.squares[61] = Some(Piece::Bishop(Color::Black));

        self.squares[3] = Some(Piece::Queen(Color::White));
        self.squares[59] = Some(Piece::Queen(Color::Black));

        self.squares[4] = Some(Piece::King(Color::White));
        self.squares[60] = Some(Piece::King(Color::Black));

    }

    pub fn print_board(&self){
        println!("  +-----------------+");
        for rank in (0..8).rev(){
            print!("{} | ", rank + 1); 
            for file in (0..8).rev(){
                let index = rank * 8 + file;
                let square = &self.squares[index];

                match square {
                    Some(piece) => print!("{} ", piece_to_char(piece)),
                    None=> print!(". "),
                }
            }
            println!("|");
        }
        println!("  +-----------------+");
        println!("    a b c d e f g h");
        
        println!("Side to move: {}", if self.side_to_move == Color::White {"White"} else {"Black"});

        print!("Castling rights: ");
        if self.castling_rights & WHITE_KINGSIDE_CASTLING_RIGHTS != 0 { print!("K"); }
        if self.castling_rights & WHITE_QUEENSIDE_CASTLING_RIGHTS != 0 { print!("Q"); }
        if self.castling_rights & BLACK_KINGSIDE_CASTLING_RIGHTS != 0 { print!("k"); }
        if self.castling_rights & BLACK_QUEENSIDE_CASTLING_RIGHTS != 0 { print!("q"); }
        if self.castling_rights == 0 { print!("-"); }
        println!();
        print!("Turn: {}", self.fullmove_number);
    }

    pub fn make_move(&mut self, m: Move) -> UndoInfo {
        let mut undo_info = UndoInfo {
            captured_piece: None,
            en_passant_square: self.en_passant_square,
            castling_rights: self.castling_rights,
            halfmove_clock: self.halfmove_clock,
            special_info: SpecialInfo::None

        };

        let from = m.get_from() as usize;
        let to = m.get_to() as usize;

        if self.squares[from] == Some(Piece::Pawn(self.side_to_move)) {
            self.halfmove_clock = 0;
        }
        
        if m.is_capture() {
            if m.get_move_type() == MoveType::EnPassant {
                let captured_pawn_square = match self.side_to_move {
                    Color::White => to - 8,
                    Color::Black => to + 8,
                };
                undo_info.captured_piece = self.squares[captured_pawn_square];
                undo_info.special_info = SpecialInfo::EnPassant { en_passant_square: captured_pawn_square as u16 };
                self.squares[captured_pawn_square] = None;
                self.squares[to] = self.squares[from];
                self.squares[from] = None;
            } else {
                undo_info.captured_piece = self.squares[to];
                self.squares[to] = self.squares[from];
                self.squares[from] = None;

            }
            self.halfmove_clock = 0;

        } else if m.is_castle() { // this does NOT check for checks or pieces in the way !!! (movefilter does)
            let (rook_from, rook_to) = match m.get_move_type() {
                MoveType::CastleKingside => match self.side_to_move {
                    Color::White => (7, 5),
                    Color::Black => (63, 61),
                },
                MoveType::CastleQueenside => match self.side_to_move {
                    Color::White => (0, 3),
                    Color::Black => (57, 59),
                }, 
                _ => unreachable!()
            };

            undo_info.special_info = SpecialInfo::Castle { rook_from: rook_from, rook_to: rook_to };

            self.squares[from] = None;
            self.squares[to] = Some(Piece::King(self.side_to_move));
            self.squares[rook_from as usize] = None;
            self.squares[rook_to as usize] = Some(Piece::Rook(self.side_to_move));

        } else {
            self.squares[to] = self.squares[from];
            self.squares[from] = None;
            
        }

        if m.is_promotion() {
            let promotion_piece = match m.promotion_piece() {
                Some(0) => Piece::Queen(self.side_to_move),
                Some(1) => Piece::Rook(self.side_to_move),
                Some(2) => Piece::Bishop(self.side_to_move),
                Some(3) => Piece::Knight(self.side_to_move),
                _ => unreachable!()

            
            };
            undo_info.special_info = SpecialInfo::Promotion;

            self.squares[from] = None;
            self.squares[to] = Some(promotion_piece);
            self.halfmove_clock = 0;

        } else if self.squares[from] == Some(Piece::Pawn(self.side_to_move)) {
            if (from as i32 - to as i32).abs() == 16 {
                let ep_square = match self.side_to_move {
                    Color::White => from + 8,
                    Color::Black => from - 8,
                };
                self.en_passant_square = Some(ep_square as u8);
            }
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // update castling rights, update fullmove clock and change side_to_move elsewhere seems better        

        undo_info
    }

    pub fn unmake_move(&mut self, m: Move, undo: UndoInfo) {

    }
}

fn piece_to_char(piece: &Piece) -> char {
    match piece {
        Piece::Pawn(Color::White) => 'P',
        Piece::Knight(Color::White) => 'N',
        Piece::Bishop(Color::White) => 'B',
        Piece::Rook(Color::White) => 'R',
        Piece::Queen(Color::White) => 'Q',
        Piece::King(Color::White) => 'K',
        
        Piece::Pawn(Color::Black) => 'p',
        Piece::Knight(Color::Black) => 'n',
        Piece::Bishop(Color::Black) => 'b',
        Piece::Rook(Color::Black) => 'r',
        Piece::Queen(Color::Black) => 'q',
        Piece::King(Color::Black) => 'k',
    }
}

pub fn init() -> Board {
    let board = Board::new();
    board.print_board();
    board
}