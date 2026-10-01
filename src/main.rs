use std::cmp::min;
use std::io;
use std::io::{BufRead, Write};
use std::time::{Duration};
use crate::board::{Board, GameState, MoveResult};
use crate::board::GameState::Ongoing;
use crate::piece::{Color};
use crate::piece::Kind::{Bishop, Knight, Queen, Rook};
use crate::piece_move::PieceMove;
use crate::search::Search;
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
pub mod search;

fn get_user_input(board: &mut Board) -> PieceMove {
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

        match get_move(board, s) {
            Some(m) => { return m }
            None => { continue }
        }
    }
}

fn get_move(board: &mut Board, s: &str) -> Option<PieceMove> {
    let Some(from) = square_to_coordinate(&s[0..2]) else { return None };
    let Some(to) = square_to_coordinate(&s[2..4]) else { return None };

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
        return Some(*m);
    }
    eprintln!("Illegal move.");
    None
}

fn play_console() {
    let mut board = Board::new(); // Init board
    let mut search = Search::new();
    loop {
        board.print_board();

        let result = match board.turn {
            Color::Black => {
                let start = std::time::Instant::now();
                let best = search.search(&mut board, 10).unwrap();
                println!("Engine: {:?} ({:?})", best, start.elapsed());
                board.make_move(best.from, best.to, best.promotion)
            }
            Color::White => {
                let m = get_user_input(&mut board);
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
}


fn play_uci() {
    let stdin = io::stdin();
    let mut board = Board::new();
    let mut search = Search::new();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();

        match tokens.first().copied() {
            Some("uci") => {
                println!("id name Cosmic Engine");
                println!("id author Chinono");
                println!("uciok");
            }
            Some("isready") => {
                println!("readyok");
            }
            Some("ucinewgame") => {
                board = Board::new();
                search = Search::new();
            }
            Some("go") => {
                let mut wtime = 0;
                let mut btime = 0;
                let mut winc  = 0;
                let mut binc  = 0;
                let mut movetime: Option<u64> = None;

                let mut i = 1;
                while i + 1 < tokens.len() {
                    let v = tokens[i + 1].parse::<u64>().unwrap_or(0);
                    match tokens[i] {
                        "wtime" => wtime = v,
                        "btime" => btime = v,
                        "winc"  => winc  = v,
                        "binc"  => binc  = v,
                        "movetime" => movetime = Some(v),
                        _ => {}
                    }
                    i += 2;
                }

                let (my_time, my_inc) = match board.turn {
                    Color::White => (wtime, winc),
                    Color::Black => (btime, binc),
                };

                const OVERHEAD: u64 = 200;

                let (soft, hard) = if let Some(mt) = movetime {
                    let t = mt.saturating_sub(OVERHEAD).max(1);
                    (t, t)
                } else {
                    let base = (my_time / 20 + my_inc / 2).min(my_time.saturating_sub(OVERHEAD)).max(1);
                    let soft = base * 3 / 4;
                    let hard = min(base * 3 / 2, my_time * 3 / 4).saturating_sub(OVERHEAD).max(soft);
                    (soft, hard)
                };

                // let before = Instant::now();
                let best = search.search_timed(&mut board, 64, Duration::from_millis(soft), Duration::from_millis(hard)).unwrap();
                // let elapsed = before.elapsed();
                // eprintln!("budget {}ms, used {}ms", budget, elapsed.as_millis());
                println!("bestmove {}", best.to_uci());
            }
            Some("position") => {
                let moves_idx = tokens.iter().position(|&t| t == "moves");

                let move_tokens: &[&str] = match moves_idx {
                    Some(idx) => &tokens[idx + 1..],
                    None => &[], // No moves were provided
                };

                let setup_end = moves_idx.unwrap_or(tokens.len());
                let setup_tokens = &tokens[1..setup_end];

                match setup_tokens.first().copied() {
                    Some("startpos") => {
                        board = Board::new();
                    }
                    Some("fen") => {
                        let fen_str = setup_tokens[1..].join(" ");
                        board = Board::from_fen(fen_str.as_str());
                    }
                    _ => {}
                }

                for uci_move in move_tokens {
                    // println!("Applying move: {}", uci_move);
                    let m = get_move(&mut board, uci_move).unwrap();
                    board.make_move(m.from, m.to, m.promotion);
                }
            }
            Some("quit") => { break }
            _ => {}
        }
    }

    io::stdout().flush().unwrap();
}

fn main() {
    if std::env::args().any(|a| a == "--console") {
        play_console();
    } else {
        play_uci();
    }
}