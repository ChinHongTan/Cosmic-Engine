use crate::board::Board;

mod board;
mod piece;
mod square;
mod castling;
mod piece_move;
mod zobrist;
mod movegen;
mod attacks;
mod fen;
mod perft;

#[cfg(test)] mod tests;
pub mod engine;

fn main() {
    let mut board = Board::new(); // Init board
    println!("Check board perft");
    let start = std::time::Instant::now();
    let nodes = board.perft(8);
    println!("{} nodes in {:?} ({:.0} nps)",
             nodes, start.elapsed(), nodes as f64 / start.elapsed().as_secs_f64());

    println!("Check white pawn 1 step");
    board.print_board();
    board.make_move((1, 1), (1, 2), None); // Check white pawn 1 step
    board.print_board();
    println!("Check black pawn 2 step");
    board.make_move((1, 6), (1, 4), None); // Check black pawn 2 step
    board.print_board();
    println!("Expect invalid black move");
    board.make_move((1, 4), (1, 3), None); // Expect invalid black move

    println!("Check white pawn 2 step");
    board.make_move((2, 1), (2, 3), None); // Check white pawn 2 step
    board.print_board();
    board.make_move((2, 6), (2, 4), None);
    board.print_board();

    println!("Check white knight move");
    board.make_move((1, 0), (2, 2), None); // Check white knight move
    board.print_board();
    println!("Check black pawn capture");
    board.make_move((1, 4), (2, 3), None); // Check black pawn capture
    board.print_board();
    board.make_move((3, 1), (3, 3), None);
    board.print_board();

    println!("Check black en passant");
    board.make_move((2, 3), (3, 2), None); // Check black en passant
    board.print_board();
    board.make_move((6, 0), (5, 2), None);
    board.print_board();
    println!("Check capture white pawn");
    board.make_move((3, 2), (4, 1), None); // Check capture white pawn
    board.print_board();

    println!("Check FEN parser.");
    let mut board_fen = Board::from_fen("8/8/8/8/8/8/8/N7 w - - 0 1");
    board_fen.print_board();

    println!("Check white castling.");
    board_fen = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
    board_fen.print_board();
    board_fen.make_move((4, 0), (6, 0), None);
    board_fen.print_board();
    println!("Expect invalid castling since white rook is in the way");
    board_fen.make_move((4, 7), (6, 7), None);
    board_fen.print_board();
    println!("Check black queen side castling");
    board_fen.make_move((4, 7), (2, 7), None);
    board_fen.print_board();
}
