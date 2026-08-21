use crate::board::Board;
use crate::piece::{Pawn, King, PieceEnum, Knight, Rook};

mod board;
mod piece;

fn main() {
    let mut board = Board::new();
    let piece = Pawn {};
    let king = King {};
    let knight = Knight {};
    let rook = Rook {};
    board.add_piece(1, 1, PieceEnum::Pawn(piece));
    board.add_piece(3, 3, PieceEnum::King(king));
    board.add_piece(5, 5, PieceEnum::Knight(knight));
    board.add_piece(0, 0, PieceEnum::Rook(rook));
    board.print_board();
    board.make_move((1, 1), (1, 2));
    board.print_board();
    board.make_move((3, 3), (3, 2));
    board.print_board();
    board.make_move((5, 5), (7, 6));
    board.print_board();
    board.make_move((0, 0), (1, 0));
    board.print_board();
    board.make_move((1, 0), (3, 0));
    board.print_board();
    board.make_move((3, 0), (3, 4));
    board.print_board();
}
