use crate::board::Board;
use crate::piece::{Pawn, King, PieceEnum, Knight};

mod board;
mod piece;

fn main() {
    let mut board = Board::new();
    let piece = Pawn {};
    let king = King {};
    let knight = Knight {};
    board.add_piece(1, 1, PieceEnum::Pawn(piece));
    board.add_piece(3, 3, PieceEnum::King(king));
    board.add_piece(5, 5, PieceEnum::Knight(knight));
    board.print_board();
    board.make_move((1, 1), (1, 2));
    board.print_board();
    board.make_move((3, 3), (3, 2));
    board.print_board();
    board.make_move((5, 5), (7, 6));
    board.print_board();
}
