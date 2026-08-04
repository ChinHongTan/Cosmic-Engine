use crate::board::Board;
use crate::piece::{Pawn, King, PieceEnum};

mod board;
mod piece;

fn main() {
    let mut board = Board::new();
    let piece = Pawn {};
    let king = King {};
    board.add_piece(1, 1, PieceEnum::Pawn(piece));
    board.add_piece(3, 3, PieceEnum::King(king));
    board.print_board();
    board.make_move((1, 1), (1, 2));
    board.print_board();
    board.make_move((3, 3), (3, 2));
    board.print_board();
}
