use crate::movegen;
use crate::board::Board;
use crate::board::Color;
use crate::movegen::is_square_attacked;
use crate::movegen::Move;
use crate::board::UndoInfo;


pub fn filter(board: &mut Board, moves: Vec<Move>) -> Vec<Move> {
    //every move needs to keep the king safe (check from the king's position in every direction if it can "capture" an enemy piece with the moves of that enemy piece)
    //castles also need to check if there aren't pieces between king and rook (if rook can move to it's place, the move should be in the moves vector)
    //probably a good idea to get the king position somewhere, maybe as attribute of Board, and retur it in movegen::generate_king_moves()
    //maybe this file is not needed, filter() can be a function of movegen, called before every moves.push(move)

    let mut valid_moves = Vec::with_capacity(moves.len());
    for movement in moves {
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
                valid_moves.push(movement); // you castle if you have castling rights (checked in movegen) if the king is not attacked, if the two squares between king and rook are not attacked and empty
            }
                
            if movement.is_castle_queenside() && 
            !is_square_attacked(board, old_king_position) && 
            !is_square_attacked(board, old_king_position - 1) &&
            board.squares[(old_king_position - 1) as usize].is_none() &&
            !is_square_attacked(board, old_king_position - 2) &&
            board.squares[(old_king_position - 2) as usize].is_none() &&
            !is_square_attacked(board, old_king_position - 3) && 
            board.squares[(old_king_position - 3) as usize].is_none() {
                valid_moves.push(movement);
            }
            continue;
        
        }
        let undo_info = board.make_move(movement);

        if movement.get_from() == old_king_position {
            if !is_square_attacked(board, movement.get_to()) {
                valid_moves.push(movement);
            }
        } else {
            if !is_square_attacked(board, old_king_position) {
                valid_moves.push(movement);
            }
        }

        board.unmake_move(movement, undo_info);
    }

    valid_moves
}
