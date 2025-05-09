use crate::movegen

pub fn filter(board: &Board, moves: &Vec<Move>) -> Vec<Move> {
    //every move needs to keep the king safe (check from the king's position in every direction if it can "capture" an enemy piece with the moves of that enemy piece)
    //castles also need to check if there aren't pieces between king and rook (if rook can move to it's place, the move should be in the moves vector)
    //probably a good idea to get the king position somewhere, maybe as attribute of Board, and retur it in movegen::generate_king_moves()
    //maybe this file is not needed, filter() can be a function of movegen, called before every moves.push(move)

    let mut next_board = board::Clone;
    let mut valid_moves = Vec::new();
    let king_position = match board.side_to_move {
        Color::White => board.white_king,
        Color::Black => board.black_king,
    }
    for movement in moves.iter() {
        let move_type = movement.get_move_type().
        let moving_piece = Some(board.squares[movement.get_from]);
        if move_type != MoveType::castle_kingside && move_type != castle_queenside {
            board.squares[movement.get_from] = None;
            board.squares[movement.get_to] = moving_piece;
            is_attacked = movegen::is_square_attacked(board, king_position)
            if !is_attacked {
                valid_moves.push(movement)
            }
        }
    } else if move_type == MoveType::castle_kingside {
        if board.side_to_move == Color::White && (board.castling_rights & WHITE_KINGSIDE_CASTLING_RIGHTS != 0) {

        }

        if board.side_to_move == Color::Black && (board.castling_rights & BLACK_KINGSIDE_CASTLING_RIGHTS != 0) {

        }
        
        if board.side_to_move == Color::White && (board.castling_rights & WHITE_QUEENSIDE_CASTLING_RIGHTS != 0) {

        }
        
        if board.side_to_move == Color::Black && (board.castling_rights & BLACK_QUEENSIDE_CASTLING_RIGHTS != 0) {

        }
    }

}