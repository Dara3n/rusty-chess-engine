use chess_engine_rust::{board::{Board, Color}, eval, movegen::{generate_moves, Move}, search::minimax_best_move};
#[test]
fn test_initial_position() {
    let board = Board::default(); 

    assert!(board.en_passant_square.is_none());
    assert_eq!(board.castling_rights, 0b1111);
    assert_eq!(board.fullmove_number, 0);
    assert_eq!(board.halfmove_clock, 0);

}

#[test]
fn test_fen() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen).unwrap();

    assert_eq!(board.side_to_move, Color::White);
    assert_eq!(board.castling_rights, 0b1111);
    assert_eq!(board.to_fen(), fen);
    
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 0 1";
    board = Board::from_fen(fen).unwrap();

    assert_eq!(board.side_to_move, Color::Black);
    assert_eq!(board.castling_rights, 0);
    assert_eq!(board.to_fen(), fen);

}

#[test]
fn test_movegen() {
    let mut board = Board::default();

    let legal_moves = generate_moves(&mut board);

    assert_eq!(legal_moves.len(), 20);

    let fen = "rnbqkbnr/ppp1pppp/8/3pP3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq d6 0 1";
    board = Board::from_fen(fen).unwrap();

    assert_eq!(board.en_passant_square, Some(Board::string_to_square("d6").unwrap()));
}

#[test]
fn test_check() {
    let mut fen = "4k3/8/8/8/8/8/4R3/4K3 b - - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    assert!(board.is_check());

    fen = "4K3/8/8/8/1B6/8/8/4k3 b - - 0 1";
    board = Board::from_fen(fen).unwrap();
    assert!(board.is_check());

    fen = "4K3/8/8/8/8/5N2/8/4k3 b - - 0 1";
    board = Board::from_fen(fen).unwrap();
    assert!(board.is_check());

    fen = "4K3/3p4/8/8/8/8/8/4k3 w - - 0 1";
    board = Board::from_fen(fen).unwrap();
    assert!(board.is_check());

    fen = "8/8/8/8/8/4k3/4p3/4K3 w - - 0 1";
    board = Board::from_fen(fen).unwrap();
    assert!(!board.is_check());
}

#[test]
fn test_stalemate() {
    let mut fen = "7k/5Q2/8/8/8/8/8/7K b - - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    let moves = generate_moves(&mut board);
    assert_eq!(moves.len(), 0);
    assert!(!board.is_check());
}
