use crate::movegen;
use crate::board::Board;
use crate::board::Color;
use crate::movegen::is_square_attacked;
use crate::movegen::Move;
use crate::board::UndoInfo;
use crate::movegen::MoveType;


pub fn filter(board: &mut Board, moves: Vec<Move>) -> Vec<Move> {
    //every move needs to keep the king safe (check from the king's position in every direction if it can "capture" an enemy piece with the moves of that enemy piece)
    //castles also need to check if there aren't pieces between king and rook (if rook can move to it's place, the move should be in the moves vector)
    //probably a good idea to get the king position somewhere, maybe as attribute of Board, and retur it in movegen::generate_king_moves()
    //maybe this file is not needed, filter() can be a function of movegen, called before every moves.push(move)

    let mut valid_moves = Vec::new();
    for movement in moves {
        let king_position = match board.side_to_move {
            Color::White => board.white_king,
            Color::Black => board.black_king,
        };

        if movement.is_castle() {
            match movement.get_move_type() {
                MoveType::CastleKingside => if !is_square_attacked(board, king_position) && 
                !is_square_attacked(board, king_position + 1) &&
                board.squares[(king_position + 1) as usize].is_none() &&
                !is_square_attacked(board, king_position + 2) &&
                board.squares[(king_position + 2) as usize].is_none() &&
                !is_square_attacked(board, king_position + 3) && 
                board.squares[(king_position + 3) as usize].is_none() {
                    valid_moves.push(movement);
                },
                    


                MoveType::CastleQueenside => ,

                _ => unreachable!()
            }
            continue;
        }
        let undo_info = board.make_move(movement);


        if !is_square_attacked(board, king_position) {
            valid_moves.push(movement);
        }

        board.unmake_move(movement, undo_info);
    
    }

    valid_moves
}

pub fn filter_castle(m: Move, moves: &mut Vec<Move>) {
    match m.get_move_type() {
        
    }

}