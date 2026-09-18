use crate::castling::CastlingSide::{BlackKing, BlackQueen, WhiteKing, WhiteQueen};
use crate::castling::{CastlingRights, check_castling_pos};
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::piece::{Color, Kind, Piece};
use crate::piece_move::PieceMove;
use crate::zobrist::ZOBRIST;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct Board {
    pub(crate) board_state: [[Option<Piece>; 8]; 8],
    pub(crate) turn: Color,
    pub(crate) en_passant: Option<(usize, usize)>,
    pub(crate) castling: CastlingRights,
    pub(crate) halfmove: u32,
    pub(crate) fullmove: u32,
    pub(crate) hash: u64,
    pub(crate) history: Vec<u64>,
    pub(crate) irreversible: usize,
    pub(crate) king_pos: [(usize, usize); 2],
}

pub struct Unmake {
    captured: Option<Piece>,
    castling: CastlingRights,
    en_passant: Option<(usize, usize)>,
    halfmove: u32,
    king_pos: [(usize, usize); 2]
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let symbol = match self.board_state[rank][file] {
                    Some(piece) => piece.symbol(),
                    None => '.',
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  a b c d e f g h")
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            board_state: [[None; 8]; 8],
            turn: White,
            en_passant: None,
            castling: CastlingRights([false; 4]),
            halfmove: 0,
            fullmove: 1,
            hash: 0,
            history: vec![],
            irreversible: 0,
            king_pos: [(4, 7), (4, 0)],
        }
    }
}

// Back rank predefined
const BACK_RANK: [Kind; 8] = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];

pub enum GameState {
    Ongoing,
    Checkmate { winner: Color },   // side to move has lost
    Stalemate,   // draw
    DrawByFiftyMove,
    DrawByInsufficientMaterial,
    DrawByRepetition,
}

pub enum MoveResult {
    Ok(GameState),
    NoPieceThere,
    NotYourTurn,
    Illegal,
}

impl Board {
    pub(crate) fn finalize(mut self) -> Board {
        self.hash = self.hash();
        self.history.push(self.hash);
        for c in [Black, White] {
            if let Some(pos) = self.find_king(c) {
                self.king_pos[c as usize] = pos;
            }
        }
        self
    }

    pub fn new() -> Board {
        let mut board_state = [[None; 8]; 8];
        for file in 0..8 {
            let kind = BACK_RANK[file];
            board_state[7][file] = Some(Piece { kind, color: Black });
            board_state[6][file] = Some(Piece { kind: Pawn, color: Black });
            board_state[1][file] = Some(Piece { kind: Pawn, color: White });
            board_state[0][file] = Some(Piece { kind, color: White });
        }
        let castling = CastlingRights ([true; 4]);

        Board { board_state, castling, ..Default::default() }
            .finalize()
    }

    pub fn hash(&self) -> u64 {
        let mut h = 0u64;

        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.board_state[rank][file] {
                    h ^= ZOBRIST.random_table[piece.color as usize][piece.kind as usize][rank * 8 + file];
                }
            }
        }

        if self.turn == Black {
            h ^= ZOBRIST.black_to_move;
        }

        for i in 0..4 {
            if self.castling.0[i] {
                h ^= ZOBRIST.castling[i];
            }
        }

        if let Some((file, _)) = self.en_passant {
            h ^= ZOBRIST.en_passant_file[file];
        }

        h
    }

    pub fn get_piece(&self, pos: (usize, usize)) -> Option<Piece> {
        let (file, rank) = pos;
        self.board_state[rank][file]
    }

    pub fn game_state(&mut self) -> GameState {
        if self.halfmove >= 100 {
            return GameState::DrawByFiftyMove;
        }
        if self.insufficient_material() {
            return GameState::DrawByInsufficientMaterial;
        }
        if self.repetition() {
            return GameState::DrawByRepetition
        }
        if self.has_legal_moves() {
            return GameState::Ongoing;
        }
        
        if self.is_in_check(self.turn) {
            GameState::Checkmate { winner: !self.turn }
        } else {
            GameState::Stalemate
        }
    }

    pub fn make_move(&mut self, starting_pos: (usize, usize), target_pos: (usize, usize), promote_to: Option<Kind>) -> MoveResult {
        let (start_x, start_y) = starting_pos;

        let Some(piece) = self.board_state[start_y][start_x] else {
            return MoveResult::NoPieceThere;
        };

        if piece.color != self.turn {
            return MoveResult::NotYourTurn;
        }

        let target_move = PieceMove { from: starting_pos, to: target_pos, piece, promotion: promote_to };
        let mut legal = Vec::with_capacity(64);
        self.legal_moves(starting_pos, &mut legal);

        if legal.contains(&target_move) {
            let undo = self.make(target_move);
            self.hash = self.hash();
            self.history.push(self.hash);
            if piece.kind == Pawn || undo.captured.is_some() {
                self.irreversible = self.history.len() - 1;   // index just pushed
            }
            MoveResult::Ok(self.game_state())
        } else {
            MoveResult::Illegal
        }
    }

    pub(crate) fn make(&mut self, piece_move: PieceMove) -> Unmake {
        let undo = Unmake {
            captured: self.board_state[piece_move.to.1][piece_move.to.0],
            castling: self.castling.clone(),
            en_passant: self.en_passant,
            halfmove: self.halfmove,
            king_pos: self.king_pos,
        };

        let (start_x, start_y) = piece_move.from;
        let (target_x, target_y) = piece_move.to;
        let castling_pos = check_castling_pos(&piece_move.from);
        if self.board_state[target_y][target_x] != None {
            // Capture logic, maybe push them into an array in the future?
            let _captured_piece = self.board_state[target_y][target_x].take().unwrap();
            // If a move involves corner, revoke castling rights
            if let Some(c) = check_castling_pos(&piece_move.from) { self.castling[c] = false; }
            if let Some(c) = check_castling_pos(&piece_move.to) { self.castling[c] = false; }
        }

        if piece_move.piece.kind == Pawn && Some((target_x, target_y)) == self.en_passant {
            self.board_state[start_y][target_x] = None; // En passant capture
        }
        if piece_move.piece.kind == Pawn && (target_y as i32 - start_y as i32).abs() == 2 {
            self.en_passant = Some((start_x, (start_y + target_y) / 2));
        } else {
            self.en_passant = None; // clear en_passant
        }

        // Castling logic
        // If king is moved
        if piece_move.piece.kind == King {
            // Update king position
            self.king_pos[piece_move.piece.color as usize] = piece_move.to;
            match piece_move.piece.color {
                Black => {
                    self.castling[BlackKing] = false;
                    self.castling[BlackQueen] = false;
                }
                White => {
                    self.castling[WhiteKing] = false;
                    self.castling[WhiteQueen] = false;
                }
            }

            // King castling
            if (target_x as i32 - start_x as i32).abs() == 2 {
                let (rook_from_x, rook_to_x) = if target_x == 6 { (7, 5) } else { (0, 3) };
                // Move rook into castling position
                self.board_state[start_y][rook_to_x] = self.board_state[start_y][rook_from_x].take();
            }
        }

        // If rook is moved
        if piece_move.piece.kind == Rook && castling_pos.is_some() {
            self.castling[check_castling_pos(&piece_move.from).unwrap()] = false;
        }

        self.board_state[start_y][start_x] = None; // Take the piece

        let was_capture = undo.captured.is_some();
        if piece_move.piece.kind == Pawn || was_capture {
            self.halfmove = 0;
        } else {
            self.halfmove += 1;
        }
        if piece_move.piece.color == Black {
            self.fullmove += 1;
        }

        let after_promote = match piece_move.promotion { // Account for promotion
            None => piece_move.piece,
            Some(kind) => Piece { kind, color: piece_move.piece.color }
        };

        self.board_state[target_y][target_x] = Some(after_promote); // And place it
        self.turn = !self.turn;

        undo
    }

    pub(crate) fn unmake(&mut self, piece_move: PieceMove, undo: Unmake) {
        // move the piece back and replace captured piece
        self.board_state[piece_move.from.1][piece_move.from.0] = Some(piece_move.piece);
        self.board_state[piece_move.to.1][piece_move.to.0] = undo.captured;

        if piece_move.piece.kind == King && (piece_move.to.0 as i32 - piece_move.from.0 as i32).abs() == 2 {
            let (rook_from_x, rook_to_x) = if piece_move.to.0 == 6 { (7, 5) } else { (0, 3) };
            // Move rook into castling position
            self.board_state[piece_move.from.1][rook_from_x] = self.board_state[piece_move.from.1][rook_to_x].take();
        }

        if piece_move.piece.kind == Pawn && Some((piece_move.to.0, piece_move.to.1)) == undo.en_passant {
            self.board_state[piece_move.from.1][piece_move.to.0] = Some(Piece { kind: Pawn, color: !piece_move.piece.color });
        }

        self.castling = undo.castling;
        self.en_passant = undo.en_passant;
        self.halfmove = undo.halfmove;
        self.turn = !self.turn;
        self.king_pos = undo.king_pos;
    }

    // A square is dark if (file + rank) is even, light if odd.
    fn square_color(pos: (usize, usize)) -> bool {
        (pos.0 + pos.1) % 2 == 0
    }

    fn insufficient_material(&self) -> bool {
        let mut minors = Vec::new();

        for rank in 0..8 {
            for file in 0..8 {
                let Some(piece) = self.board_state[rank][file] else { continue };
                match piece.kind {
                    King => {}
                    Bishop | Knight => minors.push((piece.kind, (file, rank))),
                    _ => return false
                }
            }
        }

        match minors.len() {
            0 | 1 => true, // K vs K, K&B vs K, K&N vs K
            2 => {  // 2 bishops of the same color
                let [(k1, p1), (k2, p2)] = minors[..] else { return false };
                k1 == Bishop && k2 == Bishop && Self::square_color(p1) == Self::square_color(p2)
            }
            _ => false
        }
    }

    fn repetition(&self) -> bool {
        let current = *self.history.last().unwrap();
        self.history[self.irreversible..].iter().filter(|&&h| h == current).count() >= 3
    }

    pub fn print_board(&self) {
        println!("Board: \n{}", self);
    }
}