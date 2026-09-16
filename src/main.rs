use std::io;
use std::io::Write;
use crate::board::{Board, GameState, MoveResult};
use crate::board::GameState::Ongoing;
use crate::piece::{Color};
use crate::piece::Kind::{Bishop, Knight, Queen, Rook};
use crate::piece_move::PieceMove;
use crate::square::square_to_coordinate;

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

fn get_user_move(board: &mut Board) -> PieceMove {
    loop {
        print!("Your move: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let s = input.trim();

        if s.len() < 4 {
            println!("Format: e2e4 (or e7e8q to promote)");
            continue
        }

        let Some(from) = square_to_coordinate(&s[0..2]) else { continue };
        let Some(to) = square_to_coordinate(&s[2..4]) else { continue };

        let promotion = match s.as_bytes().get(4) {
            Some(b'q') => Some(Queen),
            Some(b'b') => Some(Bishop),
            Some(b'r') => Some(Rook),
            Some(b'n') => Some(Knight),
            _ => None
        };

        let mut moves = Vec::new();
        board.all_legal_moves(&mut moves);
        if let Some(m) = moves.iter().find(|m| m.to == to && m.from == from && m.promotion == promotion) {
            return *m
        }
        println!("Illegal move.");
    }
}

fn main() {
    const INF: i32 = 1_000_000;
    let mut board = Board::new(); // Init board
    loop {
        board.print_board();

        let result = match board.turn {
            Color::Black => {
                let start = std::time::Instant::now();
                let best = board.search(5).unwrap();
                println!("Engine: {:?} ({:?})", best, start.elapsed());
                println!("Total nodes: {:?}", board.nodes);
                let mut biggest_repeat = 0;
                for (_key, value) in &board.hit_pairs {
                    biggest_repeat = biggest_repeat.max(*value);
                }
                println!("Distinct positions: {:?}", board.hit_pairs.len());
                println!("Biggest repeat count: {:?}", biggest_repeat);
                board.make_move(best.from, best.to, best.promotion)
            }
            Color::White => {
                let m = get_user_move(&mut board);
                board.make_move(m.from, m.to, m.promotion)
            }
        };

        if let MoveResult::Ok(state) = result {
            match state {
                Ongoing => {}
                GameState::Checkmate { winner } => {
                    println!("Checkmate! {winner} wins.");
                    break
                }
                GameState::Stalemate => {
                    println!("Stalemate!");
                    break
                }
                GameState::DrawByFiftyMove => {
                    println!("Draw by 50 moves!");
                    break
                }
                GameState::DrawByInsufficientMaterial => {
                    println!("Draw by insufficient material!");
                    break
                }
                GameState::DrawByRepetition => {
                    println!("Draw by repetition!");
                    break
                }
            }
        }
    }




    // println!("Check board perft");
    // let start = std::time::Instant::now();
    // let nodes = board.perft(7);
    // println!("{} nodes in {:?} ({:.0} nps)",
    //          nodes, start.elapsed(), nodes as f64 / start.elapsed().as_secs_f64());
    // println!("{:?}", board.best_move(8));
    // board.nodes = 0;
    // let start = std::time::Instant::now();
    // board.negamax(8, 1, -INF, INF);
    // let nodes = board.nodes;
    // println!("{} nodes in {:?} ({:.0} nps)",
    //          nodes, start.elapsed(), nodes as f64 / start.elapsed().as_secs_f64());
}
