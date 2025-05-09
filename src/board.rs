pub struct Board{
    pub squares: [Option<Piece>; 64],
    pub side_to_move: Color,
    pub castling_rights: u8, // no hay un tipo de datos de 4 bits
    pub en_passant_square: Option<u8>, // u8, = 2^8 = 255 posibilidades, necesitamos solo 64
    pub halfmove_clock: u32, // para la regla de los 50 movimientos
    pub fullmove_number: u32, //para notación en general
    pub white_king: u8,
    pub black_king: u8
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

pub fn init(){
    let board = Board::new();
    board.print_board();
    board;
}