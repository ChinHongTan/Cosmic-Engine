use crate::board::Board;

mod board;
mod piece;

fn main() {
    let mut board = Board::new();
    board.print_board();
    board.make_move((1, 1), (1, 2));
    board.print_board();

    let board_fen = Board::from_fen("8/8/8/8/8/8/8/N7 w - - 0 1");
    board_fen.print_board()
}
