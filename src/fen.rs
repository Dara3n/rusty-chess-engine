use crate::board::Board;
use crate::board::Color;
use crate::board::Piece;
use crate::board::BLACK_KINGSIDE_CASTLING_RIGHTS;
use crate::board::BLACK_QUEENSIDE_CASTLING_RIGHTS;
use crate::board::WHITE_KINGSIDE_CASTLING_RIGHTS;
use crate::board::WHITE_QUEENSIDE_CASTLING_RIGHTS;


/* Fen is a notation method for boardstates. it is divided by spaces first in 6 parts, 
1- piece position (divided by /, a number indicates number of squares empty, a letter a piece)
2- active color
3- castling rights
4- en passant target
5- halfmove clock
6- fullmove number
*/
impl Board {
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err("FEN must have 6 parts".to_string());
        }
        let mut board = Board::new();

        board.parse_pieces(parts[0])?;

        board.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid color".to_string()),
        };

        board.castling_rights = Self::parse_castling_rights(parts[2])?;

        board.en_passant_square = Self::parse_en_passant(parts[3])?;

        board.halfmove_clock = parts[4].parse().map_err(|_| "Invalid halfmove clock".to_string())?;
        board.fullmove_number = parts[5].parse().map_err(|_| "Invalid fullmove number".to_string())?;


        Ok(board)
    }
    
    pub fn parse_pieces(&mut self, pieces: &str) -> Result<(), String> {
        let ranks: Vec<&str> = pieces.split('/').collect();
        if ranks.len() != 8 {
            return Err("Not 8 ranks".to_string());
        }
        for (rank_index, pieces) in ranks.iter().enumerate() {
            let mut file_index = 0;
            for ch in pieces.chars() {
                if file_index >= 8 {
                    return Err("too many squares in a rank".to_string());
                }
                if ch.is_ascii_digit() {
                    let empty = ch.to_digit(10).unwrap() as usize;
                    if empty == 0 || empty > 8 {
                        return Err("invalid number of empty squares".to_string());
                    }
                    file_index += empty;
                } else {
                    let piece = Self::char_to_piece(ch)?;
                    let square = rank_index * 8 + file_index;
                    self.squares[square] = Some(piece);
                    
                    match piece {
                        Piece::King(Color::White) => self.white_king = square as u16,
                        Piece::King(Color::Black) => self.black_king= square as u16,
                        _ => {}
                    }
                    file_index += 1;
                }
            }
            if file_index != 8 {
                return Err("Rank doesnt have 8 squares exactly".to_string());
            }
        }
        Ok(())
    }
 
    fn parse_castling_rights(rights: &str) -> Result<u8, String> {
        if rights == "-" {
            return Ok(0);
        }
        let mut castling_rights = 0u8;

        for ch in rights.chars() {
            match ch {
                'K' => castling_rights |= WHITE_KINGSIDE_CASTLING_RIGHTS,
                'Q' => castling_rights |= WHITE_QUEENSIDE_CASTLING_RIGHTS,
                'k' => castling_rights |= BLACK_KINGSIDE_CASTLING_RIGHTS,
                'q' => castling_rights |= BLACK_QUEENSIDE_CASTLING_RIGHTS,
                _ => return Err(format!("invalid castling right {}", ch)),
            }
        }
        Ok(castling_rights)
    }

    fn parse_en_passant(coordinates: &str) -> Result<Option<u8>, String> {
        if coordinates == "-" {
            return Ok(None);
        }

        let square = Self::string_to_square(coordinates)?;
        return Ok(Some(square))
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank in 0..8 {
            let mut empty_spaces = 0;

            for file in 0..8 {
                let square = rank * 8 + file;
                if let Some(piece) = &self.squares[square] {
                    if empty_spaces > 0 {
                        fen.push_str(&empty_spaces.to_string());
                    }
                    fen.push(Board::piece_to_char(piece));    
                } else {
                    empty_spaces += 1;
                }
            }
            if empty_spaces > 0 {
                fen.push_str(&empty_spaces.to_string());
            }

            if rank < 7 {
                fen.push('/');
            }
        }
        


        fen.push(' ' );
        
        let side = match self.side_to_move {
            Color::Black => 'b',
            Color::White => 'w',
        };
        fen.push(side);

        fen.push(' ' );

        let mut castling = String::new();
        if self.castling_rights & WHITE_KINGSIDE_CASTLING_RIGHTS != 0 { castling.push('K'); }
        if self.castling_rights & WHITE_QUEENSIDE_CASTLING_RIGHTS != 0 { castling.push('Q'); }
        if self.castling_rights & BLACK_KINGSIDE_CASTLING_RIGHTS != 0 { castling.push('k'); }
        if self.castling_rights & BLACK_QUEENSIDE_CASTLING_RIGHTS != 0 { castling.push('q'); }

        if castling.is_empty() {
            fen.push('-');
        } else {
            fen.push_str(&castling);
        }

        fen.push(' ' );

        if let Some(square) = self.en_passant_square {
            let file = (square % 8) as u8 + b'a';
            let rank = (square / 8) as u8 + b'1';
            fen.push(file as char);
            fen.push(rank as char);
        } else {
            fen.push('-');
        }
        
        fen.push(' ');

        fen.push_str(&self.halfmove_clock.to_string());
        
        fen.push(' ');

        fen.push_str(&self.fullmove_number.to_string());
        
        fen
    }

    pub fn string_to_square(coordinates: &str) -> Result<u8, String> {
        if coordinates.len() != 2 {
            return Err(format!("Invalid coordinate {}", coordinates));
        }

        let chars: Vec<char> = coordinates.chars().collect();
        let file = chars[0];
        let rank = chars[1];

        if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
            return Err("Invalid square".to_string());
        }

        let file_index = (file as u8 - b'a') as u8;
        let rank_index = (rank as u8 - b'1') as u8;

        let square = rank_index * 8 + file_index;
        Ok(square)

    }

}