use crate::board::Board;

mod board;
mod piece;
mod square;
pub mod piece_move;

fn main() {
    let mut board = Board::new(); // Init board
    board.print_board();
    board.make_move((1, 1), (1, 2)); // Check white pawn 1 step
    board.print_board();
    board.make_move((1, 6), (1, 4)); // Check black pawn 2 step
    board.print_board();
    board.make_move((1, 4), (1, 3)); // Expect invalid black move

    board.make_move((2, 1), (2, 3)); // Check white pawn 2 step
    board.print_board();
    board.make_move((2, 6), (2, 4));
    board.print_board();


    board.make_move((1, 0), (2, 2)); // Check white knight move
    board.print_board();
    board.make_move((1, 4), (2, 3)); // Check black pawn capture
    board.print_board();
    board.make_move((3, 1), (3, 3));
    board.print_board();

    board.make_move((2, 3), (3, 2)); // Check black en passant
    board.print_board();
    board.make_move((6, 0), (5, 2));
    board.print_board();
    board.make_move((3, 2), (4, 1)); // Check capture white pawn
    board.print_board();

    let board_fen = Board::from_fen("8/8/8/8/8/8/8/N7 w - - 0 1");
    board_fen.print_board()
}
