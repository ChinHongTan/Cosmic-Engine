use crate::board::Board;
use crate::square::square_to_coordinate;

fn play(board: &mut Board, moves: &[&str]) {
    for m in moves {
        let from = square_to_coordinate(&m[0..2]).unwrap();
        let to = square_to_coordinate(&m[2..4]).unwrap();
        board.make_move(from, to, None);
    }
}

#[test]
fn knights_back_home_is_repeated() {
    let mut board = Board::new();
    play(&mut board, &["g1f3", "g8f6", "f3g1"]);
    assert!(!board.is_repeated());
    play(&mut board, &["f6g8"]);
    assert!(board.is_repeated());
}

#[test]
fn pawn_move_breaks_repetition() {
    // the start position can never come back once a pawn has moved
    let mut board = Board::new();
    play(&mut board, &["g1f3", "g8f6", "f3g1", "f6g8", "e2e4"]);
    assert!(!board.is_repeated());
}

#[test]
fn repeat_of_a_mid_sequence_position() {
    // 3. Nf3 recreates the position after 1. Nf3, not the start position
    let mut board = Board::new();
    play(&mut board, &["g1f3", "g8f6", "f3g1", "f6g8", "g1f3"]);
    assert!(board.is_repeated());
}
