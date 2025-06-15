use crate::board;
use crate::board::Board;
use crate::board::Color;
use crate::board::Piece;


pub const FROM_MASK:u16 = 0x3F;
pub const TO_MASK:u16 = 0x3F << 6;
pub const FLAG_MASK: u16 = 0xF << 12;    

const FLAG_NORMAL: u16 = 0;
const FLAG_CAPTURE: u16 = 1;
const FLAG_EP_CAPTURE: u16 = 2;
const FLAG_CASTLE_KING: u16 = 3;
const FLAG_CASTLE_QUEEN: u16 = 4;
const FLAG_PROMOTION_QUEEN: u16 = 5;
const FLAG_PROMOTION_ROOK: u16 = 6;
const FLAG_PROMOTION_BISHOP: u16 = 7;
const FLAG_PROMOTION_KNIGHT: u16 = 8;
const FLAG_PROMOTION_CAPTURE_QUEEN: u16 = 9;
const FLAG_PROMOTION_CAPTURE_ROOK: u16 = 10;
const FLAG_PROMOTION_CAPTURE_BISHOP: u16 = 11;
const FLAG_PROMOTION_CAPTURE_KNIGHT: u16 = 12;



const KNIGHT_DIRS: [(i32, i32); 8]= [(1, 2), (2, 1), (-2, 1), (-1, 2), (1, -2), (2, -1), (-2, -1), (-1, -2)];
const ROOK_DIRS: [(i32, i32); 4]= [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRS: [(i32, i32); 4]= [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const WHITE_PAWN_CAPTURES: [(i32, i32); 2]= [(1,1), (-1, 1)];
const BLACK_PAWN_CAPTURES: [(i32, i32); 2]= [(-1,-1), (1, -1)];


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Move {
    data: u16, // 6 bits from, 6 bits to, 4 bits special flags (castling, en-passant, check, promotion, piece to promote to)
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveType {
    Normal,
    Capture,
    EnPassant,
    CastleKingside,
    CastleQueenside,
    Promotion,
    PromotionCapture,
}

impl Move {
    pub fn new(from: u16, to: u16, flags: u16) -> Self {
        let data: u16 = from | (to << 6) | (flags << 12);
        Move { data }
    }

    pub fn normal(from: u16, to: u16) -> Self {
        Self::new(from, to, FLAG_NORMAL)
    }
    
    pub fn capture(from: u16, to: u16) -> Self {
        Self::new(from, to, FLAG_CAPTURE)
    }
    
    pub fn en_passant_capture(from: u16, to: u16) -> Self {
        Self::new(from, to, FLAG_EP_CAPTURE)
    }
    
    pub fn castle_kingside(from: u16, to: u16) -> Self {
        Self::new(from, to, FLAG_CASTLE_KING)
    }
    
    pub fn castle_queenside(from: u16, to: u16) -> Self {
        Self::new(from, to, FLAG_CASTLE_QUEEN)
    }
    
    pub fn promotion(from: u16, to: u16, piece_type: u16, is_capture: bool) -> Self {
        //Piece_type 0 = queen, 1 = rook, 2 = bishop, 3 = knight
        assert!(piece_type < 4, "promotion type must be 0..3");
        let base_flag = if is_capture { FLAG_PROMOTION_CAPTURE_QUEEN } else { FLAG_PROMOTION_QUEEN };
        Self::new(from, to, base_flag + piece_type )
    }

    pub fn get_from(&self) -> u16 {
        self.data & FROM_MASK
    }

    pub fn get_to(&self) -> u16 {
        (self.data & TO_MASK) >> 6 
    }
    
    pub fn get_flag(&self) -> u16 {
        (self.data & FLAG_MASK) >> 12
    }

    pub fn is_capture(&self) -> bool {
        let flag:u16 = self.get_flag();
        flag == FLAG_CAPTURE || flag == FLAG_EP_CAPTURE || 
        (flag >= FLAG_PROMOTION_CAPTURE_QUEEN && flag <= FLAG_PROMOTION_CAPTURE_KNIGHT)
    }

    pub fn is_en_passant(&self) -> bool {
        self.get_flag() == FLAG_EP_CAPTURE
    }

    pub fn is_promotion(&self) -> bool {
        let flag:u16 = self.get_flag();
        flag >= FLAG_PROMOTION_QUEEN && flag <= FLAG_PROMOTION_CAPTURE_KNIGHT
    }

    pub fn is_castle_kingside(&self) -> bool {
        let flag:u16 = self.get_flag();
        flag == FLAG_CASTLE_KING 
    }
    
    pub fn is_castle_queenside(&self) -> bool {
        let flag:u16 = self.get_flag();
        flag == FLAG_CASTLE_QUEEN 
    }
    
    pub fn is_castle(&self) -> bool {
        self.is_castle_kingside() || self.is_castle_queenside()
    }

    pub fn promotion_piece(&self) -> Option<u16> {
        if !self.is_promotion() {
            return None;
        }

        let flag = self.get_flag();
        if flag >= FLAG_PROMOTION_CAPTURE_QUEEN && flag <= FLAG_PROMOTION_CAPTURE_KNIGHT {
            Some(flag - FLAG_PROMOTION_CAPTURE_QUEEN)
        } else {
            Some(flag - FLAG_PROMOTION_QUEEN)
        }
    }

    pub fn get_move_type(&self) -> MoveType {
        match self.get_flag() {
            FLAG_NORMAL => MoveType::Normal,
            FLAG_CAPTURE => MoveType::Capture,
            FLAG_EP_CAPTURE => MoveType::EnPassant,
            FLAG_CASTLE_KING => MoveType::CastleKingside,
            FLAG_CASTLE_QUEEN => MoveType::CastleQueenside,
            f if (FLAG_PROMOTION_QUEEN..=FLAG_PROMOTION_KNIGHT).contains(&f) => MoveType::Promotion,
            f if (FLAG_PROMOTION_CAPTURE_QUEEN..=FLAG_PROMOTION_CAPTURE_KNIGHT).contains(&f) => MoveType::PromotionCapture,
            _ => unreachable!(),
        }
    }

    pub fn to_string(&self, board: &Board) -> String {
        let from_file:u16 = self.get_from() % 8;

        let rank:u16 = self.get_to() / 8 + 1;
        let file:u16 = self.get_to() % 8;
        let to:String = format!("{}{}", (b'a' + file as u8) as char, rank);
        
        let moving_piece = Board::piece_to_char(&board.squares[self.get_from() as usize].unwrap()).to_ascii_uppercase();

        
        let piece_to_promote = match self.promotion_piece() {
            Some(0) => Board::piece_to_char(&Piece::Queen(board.side_to_move)).to_ascii_uppercase().to_string(),
            Some(1) => Board::piece_to_char(&Piece::Rook(board.side_to_move)).to_ascii_uppercase().to_string(),
            Some(2) => Board::piece_to_char(&Piece::Bishop(board.side_to_move)).to_ascii_uppercase().to_string(),
            Some(3) => Board::piece_to_char(&Piece::Knight(board.side_to_move)).to_ascii_uppercase().to_string(),
            _ => String::new() 
        }; 
        
        let disambiguation = self.disambiguation(&board);

        let movestring = match self.get_move_type() {
            MoveType::Capture | MoveType::EnPassant => {
                if moving_piece == 'P' || moving_piece == 'p' {
                    format!("{}x{}", (b'a' + from_file as u8) as char, to)
                } else {
                    format!("{}{}x{}", moving_piece, disambiguation, to)
                }
            }, 
            MoveType::CastleKingside => "0-0".to_string(), 
            MoveType::CastleQueenside => "0-0-0".to_string(),
            MoveType::Promotion => format!("{}{}={}", moving_piece, to, piece_to_promote),
            MoveType::PromotionCapture => format!("{}x{}={}", (b'a' + from_file as u8), to, piece_to_promote),
            MoveType::Normal => {
                if moving_piece == 'P' || moving_piece == 'p' {
                    format!("{}", to)
                } else {
                    format!("{}{}{}", moving_piece, disambiguation, to)
                }

            },
        };

        movestring
    }

    pub fn disambiguation(&self, mut board: &Board) -> String {
        let from_rank:u16 = self.get_from() / 8 + 1;
        let from_file:u16 = self.get_from() % 8;
        let from:u16= self.get_from(); 
        let to_square:u16 = self.get_to(); 
        
        let mut same_file = false;
        let mut same_rank = false;
        let mut needs_disambiguation = false;
        let moving_piece = &board.squares[self.get_from() as usize].unwrap();

        let legal_moves = generate_moves(&mut board);

        for movement in legal_moves {
            if movement.get_to() == to_square && movement.get_from() != from {
                if let Some(piece) = board.squares[movement.get_from() as usize] {
                    if Board::piece_to_char(&piece) == Board::piece_to_char(&moving_piece) {
                        needs_disambiguation = true;
                        let other_file = movement.get_from() % 8;
                        let other_rank = movement.get_from() / 8;
                        
                        if other_file == from_file {
                            same_file = true;
                        }
                        if other_rank == from_rank {
                            same_rank = true;
                        }

                    }
                }
            }
        }
        if !needs_disambiguation {
            return String::new();
        }
         if !same_file {
            format!("{}", (b'a' + from_file as u8) as char)
        } else if !same_rank {
            format!("{}", from_rank + 1)
        } else {
            format!("{}{}", (b'a' + from_file as u8) as char, from_rank + 1)
        }
    }
    
    //pub fn string_to_move() 

}


pub fn generate_moves(board: &Board) -> Vec<Move> {
    let mut moves:Vec<Move> = Vec::new();

    generate_all_moves(board, &mut moves);
    filter(board, moves)
}


fn generate_all_moves(board: &Board, moves: &mut Vec<Move>) {
    for square in 0..64 {
        if let Some(piece) = board.squares[square] {
            match piece {
                Piece::Pawn(color) if color == board.side_to_move => {
                    generate_one_pawn_moves(board, square as u16, moves)
                }
                Piece::Rook(color) if color == board.side_to_move => {
                    generate_one_rook_moves(board, square as u16, moves)
                }
                Piece::Bishop(color) if color == board.side_to_move => {
                    generate_one_bishop_moves(board, square as u16, moves)
                }
                Piece::Knight(color) if color == board.side_to_move => {
                    generate_one_knight_moves(board, square as u16, moves)
                }
                Piece::Queen(color) if color == board.side_to_move => {
                    generate_queen_moves(board, square as u16, moves)
                }
                Piece::King(color) if color == board.side_to_move => {
                    generate_king_moves(board, square as u16, moves)
                }
                _ => {} 
            }
        }
    }
}


fn generate_one_pawn_moves(board: &Board, from: u16, moves: &mut Vec<Move>) {
    let (direction, start_rank, promote_rank) = match board.side_to_move {
        Color::White => (8, 1, 7), // white pawns move up 8 spaces (one vertical step)
        Color::Black => (-8, 6, 0),
    };
    let from_u8 = from as u8;
    let rank:u8 = from_u8/8;
    let to_i16: i16 = from as i16 + direction;

    if to_i16 >= 0 && to_i16 < 64 {
        let to = to_i16 as u8;
        if board.squares[to as usize].is_none() {
            if rank as i16 == promote_rank - direction/8 {
                for i in 0..4{
                    moves.push(Move::promotion(from, to as u16, i, false))
                }
            } else {
                moves.push(Move::normal(from, to as u16));
                
                if rank == start_rank {
                    let double_to = from_u8 as i16 + 2 * direction;
                    if board.squares[double_to as usize].is_none() {
                        moves.push(Move::normal(from, double_to as u16));
                    }
                }
            }
        }
    }

    for capture_direction in [-1, 1] {
        let to = (from_u8 as i16 + direction + capture_direction) as u8;
        if to > 63 || to / 8 == rank{
            continue;
        }
        if let Some(piece) = &board.squares[to as usize] {
            if is_enemy(*piece, board.side_to_move) {
                if rank as i16 == promote_rank - direction/8 {
                    for piece_type in 0..4 {
                        moves.push(Move::promotion(from, to as u16, piece_type, true));
                    }
                } else {
                    moves.push(Move::capture(from, to as u16));
                }
            }
        }
        if let Some(en_passant_square) = board.en_passant_square {
            if to == en_passant_square {
                moves.push(Move::en_passant_capture(from, to as u16));
            }
        }
    }
}


fn generate_one_rook_moves(board: &Board, from: u16, moves: &mut Vec<Move>) {
    generate_long_moves(board, from, moves, &ROOK_DIRS);
}


fn generate_one_bishop_moves(board: &Board, from: u16, moves: &mut Vec<Move>) {
    generate_long_moves(board, from, moves, &BISHOP_DIRS);
}


fn generate_queen_moves(board: &Board, from: u16, moves: &mut Vec<Move>) {
    generate_long_moves(board, from, moves, &ROOK_DIRS);
    generate_long_moves(board, from, moves, &BISHOP_DIRS);
}


fn generate_long_moves(board: &Board, from: u16, moves: &mut Vec<Move>, direction: &[(i32, i32)]) {
    let from_u8 = from as u8;
    let rank:u8 = from_u8 / 8;
    let file:u8= from_u8 % 8;
    for &(vx, vy) in direction.iter() {
        let mut current_rank = rank as i8;
        let mut current_file = file as i8;

        loop {
            current_file += vx as i8;
            current_rank += vy as i8;
            if current_file > 7 || current_file < 0 || current_rank > 7 || current_rank < 0 {
                break;
            }
            let to = (current_rank * 8 + current_file) as u8;

            match &board.squares[to as usize] {
                None => {
                    // Empty square - add normal move
                    moves.push(Move::normal(from, to as u16));
                },
                Some(piece) => {
                    if is_enemy(*piece, board.side_to_move) {
                        // capture an enemy piece
                        moves.push(Move::capture(from, to as u16));
                    }
                    
                    // We hit a piece, so stop exploring this direction
                    break;
                }
            }
        }
    }
}


fn generate_king_moves(board: &Board, from: u16, moves: &mut Vec<Move>) {
    generate_short_moves(board, from, moves, &BISHOP_DIRS);
    generate_short_moves(board, from, moves, &ROOK_DIRS);
    generate_castles(board, from, moves);
}

fn generate_one_knight_moves(board: &Board, from: u16, moves: &mut Vec<Move>) {
    generate_short_moves(board, from, moves, &KNIGHT_DIRS[0..8]);

}

fn generate_short_moves(board: &Board, from: u16, moves: &mut Vec<Move>, direction: &[(i32, i32)]) {
    let from_u8 = from as u8;
    let rank:u8 = from_u8 / 8;
    let file:u8= from_u8 % 8;
    for &(vx, vy) in direction.iter() {
        let mut current_rank = rank as i8;
        let mut current_file = file as i8;
                
        current_file += vx as i8;
        current_rank += vy as i8;
        if current_file > 7 || current_file < 0 || current_rank > 7 || current_rank < 0 {
            continue;
        }
        let to = (current_rank * 8 + current_file) as u8;

        match &board.squares[to as usize] {
            None => {
                // Empty square - add normal move
                moves.push(Move::normal(from, to as u16));
            },
            Some(piece) => {
                // Get the color of the piece
                let piece_color = match piece {
                    Piece::Pawn(c) | Piece::Knight(c) | Piece::Bishop(c) |
                    Piece::Rook(c) | Piece::Queen(c) | Piece::King(c) => *c,
                };
                    
                if piece_color != board.side_to_move {
                    // capture an enemy piece
                    moves.push(Move::capture(from, to as u16));
                }
            }
        }                
    }
}

fn generate_castles(board: &Board, from: u16, moves: &mut Vec<Move>) {
    let castling_rights = board.castling_rights;
    if board.side_to_move == Color::White {
        if castling_rights & 0b1000 != 0 {
            let to:u16 = 6;
            if board.squares[7] == Some(Piece::Rook(board.side_to_move)) {
                moves.push(Move::castle_kingside(from, to));
            }
        } if castling_rights & 0b0100 != 0 {
            let to:u16 = 2;
            if board.squares[0] == Some(Piece::Rook(board.side_to_move)) {
                moves.push(Move::castle_queenside(from, to));
            }
        }
    } else {
        if castling_rights & 0b0010 != 0 {
            let to:u16 = 62;
            if board.squares[63] == Some(Piece::Rook(board.side_to_move)) {
                moves.push(Move::castle_kingside(from, to));
            }
        } if castling_rights & 0b0001 != 0 {
            let to:u16 = 58;
            if board.squares[56] == Some(Piece::Rook(board.side_to_move)) {
                moves.push(Move::castle_queenside(from, to));
            }
        }
    }
}

fn is_enemy(piece: Piece, side_to_move: Color) -> bool {

    if matches!(piece, Piece::Bishop(c) | Piece::Knight(c) | Piece::Pawn(c) | 
    Piece::Queen(c) | Piece::Rook(c) | Piece::King(c) if c != side_to_move) {
        return true;
    } else {
        return false;
    }
}


pub fn is_square_attacked(board: &Board, square: u16) -> bool {
    let attacker_color = match board.side_to_move {
        Color::Black => Color::White,
        Color::White => Color::Black,
    };
    
    let rank = (square / 8) as i8;
    let file = (square % 8) as i8;
    
    // Check pawn attacks
    let pawn_dirs = match attacker_color {
        Color::White => WHITE_PAWN_CAPTURES,
        Color::Black => BLACK_PAWN_CAPTURES,   
    };
    
    for &(dx, dy) in &pawn_dirs {
        let new_file = file - dx as i8;
        let new_rank = rank - dy as i8;
        
        if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
            let from_square = (new_rank * 8 + new_file) as usize;
            if let Some(Piece::Pawn(color)) = board.squares[from_square] {
                if color == attacker_color {
                    return true;
                }
            }
        }
    }
    
    // Check knight attacks
    for &(dx, dy) in &KNIGHT_DIRS {
        let new_file = file + dx as i8;
        let new_rank = rank + dy as i8;
        
        if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
            let from_square = (new_rank * 8 + new_file) as usize;
            if let Some(Piece::Knight(color)) = board.squares[from_square] {
                if color == attacker_color {
                    return true;
                }
            }
        }
    }
    
    // Check king attacks
    for &(dx, dy) in BISHOP_DIRS.iter().chain(ROOK_DIRS.iter()) {
        let new_file = file + dx as i8;
        let new_rank = rank + dy as i8;
        
        if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
            let from_square = (new_rank * 8 + new_file) as usize;
            if let Some(Piece::King(color)) = board.squares[from_square] {
                if color == attacker_color {
                    return true;
                }
            }
        }
    }
    
    // Check sliding pieces 
    for &(dx, dy) in ROOK_DIRS.iter().chain(BISHOP_DIRS.iter()) {
        let is_diagonal = dx != 0 && dy != 0;
        let is_straight = dx == 0 || dy == 0;
        
        let mut new_file = file;
        let mut new_rank = rank;
        
        loop {
            new_file += dx as i8;
            new_rank += dy as i8;
            
            if new_file < 0 || new_file >= 8 || new_rank < 0 || new_rank >= 8 {
                break; 
            }
            
            let from_square = (new_rank * 8 + new_file) as usize;
            
            if let Some(piece) = &board.squares[from_square] {
                match piece {
                    Piece::Queen(color) if *color == attacker_color => return true,
                    Piece::Rook(color) if *color == attacker_color && is_straight => return true,
                    Piece::Bishop(color) if *color == attacker_color && is_diagonal => return true,
                    _ => break, 
                }
            }
        }
    }
    
    false
}


pub fn filter(board: &Board, moves: Vec<Move>) -> Vec<Move> {

    let mut valid_moves = Vec::with_capacity(moves.len());
    for movement in moves {
        if is_valid_move(board, movement) {
            valid_moves.push(movement);
        }
    }
    
    valid_moves
}


pub fn is_valid_move(board: &Board, movement: Move) -> bool {
    let mut new_board:Board = *board;

    let old_king_position = match board.side_to_move {
        Color::White => board.white_king,
        Color::Black => board.black_king,
    };

    if movement.is_castle() {
        if movement.is_castle_kingside() &&
        !is_square_attacked(board, old_king_position) && 
        !is_square_attacked(board, old_king_position + 1) &&
        board.squares[(old_king_position + 1) as usize].is_none() &&
        !is_square_attacked(board, old_king_position + 2) &&
        board.squares[(old_king_position + 2) as usize].is_none() {
            return true;
        }
            
        if movement.is_castle_queenside() && 
        !is_square_attacked(board, old_king_position) && 
        !is_square_attacked(board, old_king_position - 1) &&
        board.squares[(old_king_position - 1) as usize].is_none() &&
        !is_square_attacked(board, old_king_position - 2) &&
        board.squares[(old_king_position - 2) as usize].is_none() &&
        !is_square_attacked(board, old_king_position - 3) && 
        board.squares[(old_king_position - 3) as usize].is_none() {
            return true;
        }
    
        return false;
    }

    let undo_info = new_board.make_move(movement);
    
    // Check if the king is attacked after the move
    let king_is_safe = !new_board.is_check();
    
    // Unmake the move
    new_board.unmake_move(movement, undo_info);
    if king_is_safe {
        return true;
    }
    return false;
}