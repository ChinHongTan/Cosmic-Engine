use std::fmt;
use std::fmt::Formatter;
use crate::piece::{Color, Kind, Piece};
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{Rook, Knight, Bishop, Queen, King, Pawn};
use crate::piece_move::PieceMove;
use crate::castling::{check_castling_pos, CastlingRights, CastlingSide};
use crate::square::{square_to_coordinate, coordinate_to_square};
use crate::castling::{str_to_castling};
use crate::castling::CastlingSide::{BlackKing, BlackQueen, WhiteKing, WhiteQueen};

#[derive(Clone, Debug)]
pub struct Board {
    board_state: [[Option<Piece>; 8]; 8],
    turn: Color,
    en_passant: Option<(usize, usize)>,
    castling: CastlingRights
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

// Back rank predefined
const BACK_RANK: [Kind; 8] = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];

impl Board {
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

        Board { board_state, turn: White, en_passant: None, castling }
    }

    // "8/8/8/8/8/8/8/N7 w - - 0 1"
    pub fn from_fen(fen: &str) -> Board {
        let mut board_state = [[None; 8]; 8];
        let mut fen_component = fen.split(" ");
        let placement = fen_component.next().unwrap(); // get the first part of string

        for (i, rank_str) in placement.split("/").enumerate() {
            let rank = 7 - i;
            let mut file: usize = 0;
            for c in rank_str.chars() {
                if c.is_digit(10) {
                    file += c.to_digit(10).unwrap() as usize;
                } else {
                    board_state[rank][file] = Some(Kind::char_to_piece(c));
                    file += 1;
                }
            }
        }

        let turn_str = fen_component.next().unwrap(); // white or black
        let turn = match turn_str {
            "w" => White,
            "b" => Black,
            _ => panic!("Unknown turn state.")
        };

        let castling_str = fen_component.next().unwrap();
        let castling = match castling_str {
            "-" => CastlingRights([false; 4]),
            _ => {
                str_to_castling(castling_str)
            }
        };

        let en_passant_str = fen_component.next().unwrap();

        // Translate en_passant to board coordinate
        let en_passant = match en_passant_str {
            "-" => None,
            _ => {
                square_to_coordinate(en_passant_str)
            }
        };

        Board { board_state, turn, en_passant, castling }
    }

    // Get all possible moves for a piece (excluding pawn moves)
    fn piece_moves(&self, piece: Piece, starting_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let deltas = piece.kind.move_delta();
        let slider = piece.kind.is_slider();
        let (start_x, start_y) = starting_pos;
        let mut possible_moves: Vec<(usize, usize)> = Vec::new();

        for (dx, dy) in deltas {
            let mut new_x = start_x as i32 + dx;
            let mut new_y = start_y as i32 + dy;
            while (0..8).contains(&new_x) && (0..8).contains(&new_y) {
                // If blocked by something
                if let Some(target_piece) = self.board_state[new_y as usize][new_x as usize] {
                    if target_piece.color == piece.color {
                        break
                    } else {
                        possible_moves.push((new_x as usize, new_y as usize));
                        break
                    }
                }
                possible_moves.push((new_x as usize, new_y as usize));
                // if not slider, stop checking
                if slider == false {
                    break
                }
                new_x = new_x + dx;
                new_y = new_y + dy;
            }
        }
        // Add case for king castling
        if piece.kind == King {
            possible_moves.append(&mut self.castle_moves(piece.color));
        }
        possible_moves
    }

    // Get all possible moves for pawns on board
    fn pawn_moves(&self, piece: Piece, starting_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut possible_moves: Vec<(usize, usize)> = Vec::new();
        let (start_x, start_y) = starting_pos;
        let (dy, start_rank) = match piece.color {
            Black => {
                (-1i32, 6)
            }
            White => {
                (1, 1)
            }
        };

        let new_y = (start_y as i32 + dy) as usize;

        // Diagonal captures
        for dx in [-1i32, 1] {
            // Not in range of the board
            if !(0..8).contains(&(dx + start_x as i32)) || !(0..8).contains(&(dy + start_y as i32)) {
                // println!("Not in range!");
                continue
            }

            let new_x = (start_x as i32 + dx) as usize;

            if let Some(p) = self.board_state[new_y][new_x] {
                if p.color != piece.color {
                    possible_moves.push((new_x, new_y));
                }
            } else if self.en_passant == Some((new_x, new_y)) {
                possible_moves.push((new_x, new_y));
            }
        }

        // Front
        // if not blocked
        let None = self.board_state[new_y][start_x] else {
            return possible_moves;
        };

        possible_moves.push((start_x, new_y));

        let new_2y = (start_y as i32 + 2 * dy) as usize;

        // Two steps ahead
        if let Some(_p) = self.board_state[new_2y][start_x] {
            return possible_moves;
        } else {
            if start_y == start_rank {
                possible_moves.push((start_x, new_2y));
            }
        };

        possible_moves
    }

    // Indicate squares attacked by pawn
    fn pawn_attacks(from: (usize, usize), color: Color) -> Vec<(usize, usize)> {
        let (x, y) = from;
        let mut pawn_attacks = Vec::new();
        let dy = match color {
            Black => -1,
            White => 1,
        };
        let new_y = y as i32 + dy;
        if !(0..8).contains(&new_y) {
            return vec![];
        };

        for dx in [-1i32, 1] {
            if !(0..8).contains(&(x as i32 + dx)) {
                continue
            }
            pawn_attacks.push(((x as i32 + dx) as usize, new_y as usize));
        }
        pawn_attacks
    }

    fn king_attacks(from: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = from;
        let mut king_attacks = Vec::new();
        for dx in [-1i32, 0, 1] {
            for dy in [-1i32, 0, 1] {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                if !(0..8).contains(&new_x) || !(0..8).contains(&new_y) {
                    continue
                }

                king_attacks.push((new_x as usize, new_y as usize));
            }
        }
        king_attacks
    }

    fn pseudo_legal_moves(&self, from: (usize, usize)) -> Vec<PieceMove> {
        let (start_x, start_y) = from;

        let piece = self.board_state[start_y][start_x].unwrap();

        let possible_moves = match piece.kind {
            Pawn => self.pawn_moves(piece, from),
            _ => self.piece_moves(piece, from),
        };

        let promo_rank = match piece.color {
            White => 7,
            Black => 0,
        };

        let mut out = Vec::new();

        for m in possible_moves {
            if piece.kind == Pawn && m.1 == promo_rank {
                for k in [Queen, Rook, Bishop, Knight] {
                    out.push(PieceMove {from, to: m, piece, promotion: Some(k) });
                }
            } else {
                out.push(PieceMove { from, to: m, piece, promotion: None });
            }
        }
        out
    }

    // filters out all the legal moves
    fn legal_moves(&mut self, from: (usize, usize)) -> Vec<PieceMove> {
        let pseudo = self.pseudo_legal_moves(from);
        pseudo.into_iter().filter(|m| {
            let mut cloned_board = self.clone();
            cloned_board.apply_move(*m);
            !cloned_board.is_in_check(self.turn)
        }).collect()
    }

    pub fn make_move(&mut self, starting_pos: (usize, usize), target_pos: (usize, usize), promote_to: Option<Kind>) {
        let (start_x, start_y) = starting_pos;

        let Some(piece) = self.board_state[start_y][start_x] else {
            println!("Invalid starting position!");
            return
        };

        if piece.color != self.turn {
            println!("Invalid move - Not your turn!");
            return
        }

        let target_move = PieceMove { from: starting_pos, to: target_pos, piece, promotion: promote_to };
        let legal = self.legal_moves(starting_pos);

        if legal.contains(&target_move) {
            self.apply_move(target_move);
            self.turn = !self.turn;
        } else {
            println!("Invalid move!");
        }
    }

    fn apply_move(&mut self, piece_move: PieceMove) {
        let (start_x, start_y) = piece_move.from;
        let (target_x, target_y) = piece_move.to;
        let castling_pos = check_castling_pos(&piece_move.from);
        if self.board_state[target_y][target_x] != None {
            // Capture logic, maybe push them into an array in the future?
            let captured_piece = self.board_state[target_y][target_x].take().unwrap();
            // If a rook is captured, revoke castling rights
            if captured_piece.kind == Rook && castling_pos.is_some() {
                self.castling[castling_pos.clone().unwrap()] = false;
            }
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

        let after_promote = match piece_move.promotion { // Account for promotion
            None => piece_move.piece,
            Some(kind) => Piece { kind, color: piece_move.piece.color }
        };

        self.board_state[target_y][target_x] = Some(after_promote); // And place it
    }

    fn find_king(&self, color: Color) -> Option<(usize, usize)> {
        for rank in 0..8 {
            for file in 0..8 {
                let Some(piece) = self.board_state[rank][file] else {
                    continue
                };
                if piece.kind == King && piece.color == color {
                    return Some((file, rank))
                } else {
                    continue
                }
            }
        }
        None
    }

    // Check if coordinate is being attacked.
    pub fn is_attacked(&self, coordinate: (usize, usize), by: Color) -> bool {
        // scan every cell for opposite pieces
        for rank in 0..8 {
            for file in 0..8 {
                let Some(piece) = self.board_state[rank][file] else {
                    continue
                };
                if piece.color == by {
                    let possible_moves = match piece.kind {
                        Pawn => Board::pawn_attacks((file, rank), piece.color),
                        King => Board::king_attacks((file, rank)),
                        _ => self.piece_moves(piece, (file, rank)),
                    };
                    if possible_moves.contains(&coordinate) {
                        return true
                    }
                }
            }
        }
        false
    }

    fn is_in_check(&self, color: Color) -> bool {
        match self.find_king(color) {
            None => false,
            Some(k) => self.is_attacked(k, !color),
        }
    }

    // (side, king_from_x, king_to_x, rook_from_x, must_be_empty, must_be_safe)
    const CASTLES: [(CastlingSide, usize, usize, usize, &[usize], &[usize]); 4] = [
        (WhiteKing,  4, 6, 7, &[5, 6],    &[5, 6]),
        (WhiteQueen, 4, 2, 0, &[1, 2, 3], &[2, 3]),
        (BlackKing,  4, 6, 7, &[5, 6],    &[5, 6]),
        (BlackQueen, 4, 2, 0, &[1, 2, 3], &[2, 3]),
    ];

    fn castle_moves(&self, color: Color) -> Vec<(usize, usize)> {
        let rank = match color { White => 0, Black => 7 };
        let mut out = Vec::new();

        for (side, _kx, king_to, _rook_from, empty, safe) in Self::CASTLES {
            if side.color() != color { continue } // color doesn't match
            if !self.castling[side] { continue } // castling rights revoked
            if empty.iter().any(|&f| self.board_state[rank][f].is_some()) { continue } // If any cell in between is empty
            if self.is_attacked((4, rank), !color) { continue } // currently in check
            if safe.iter().any(|&f| self.is_attacked((f, rank), !color)) { continue } // If any cell is being attacked
            out.push((king_to, rank));
        }
        out
    }

    pub fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 { return 1; }
        let mut nodes = 0;
        for from in self.all_squares_with_own_pieces(self.turn) {
            for m in self.legal_moves(from) {
                let mut next = self.clone();
                next.apply_move(m);
                next.turn = !next.turn;
                nodes += next.perft(depth - 1)
            }
        }
        nodes
    }

    pub fn perft_divide(&mut self, depth: u32) {
        let mut total = 0;
        for from in self.all_squares_with_own_pieces(self.turn) {
            for m in self.legal_moves(from) {
                let mut next = self.clone();
                next.apply_move(m);
                next.turn = !next.turn;
                let n = next.perft(depth - 1);
                println!("{}{}: {}", coordinate_to_square(&m.from), coordinate_to_square(&m.to), n);
                total += n;
            }
        }
        println!("total: {}", total);
    }

    fn all_squares_with_own_pieces(&self, color: Color) -> Vec<(usize, usize)> {
        let mut output = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                let Some(piece) = self.board_state[rank][file] else {
                    continue
                };
                if piece.color == color {
                    output.push((file, rank));
                }
            }
        }
        output
    }

    pub fn print_board(&self) {
        println!("Board: \n{}", self);
    }
}