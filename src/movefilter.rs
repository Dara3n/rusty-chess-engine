use crate::movegen

pub fn filter(board: &Board, moves: Vec<Move>) {
    //every move needs to keep the king safe (check from the king's position in every direction if it can "capture" an enemy piece with the moves of that enemy piece)
    //castles also need to check if there aren't pieces between king and rook (if rook can move to it's place, the move should be in the moves vector)
    //probably a good idea to get the king position somewhere, maybe as attribute of Board, and retur it in movegen::generate_king_moves()

}