use crate::board::Board;

mod board;
mod piece;

fn main() {
    let mut board = Board::new();
    board.print_board();
    board.make_move((1, 1), (1, 2));
    board.print_board();
}
