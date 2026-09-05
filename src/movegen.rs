use crate::board::Board;
use crate::castling::CastlingSide;
use crate::castling::CastlingSide::{BlackKing, BlackQueen, WhiteKing, WhiteQueen};
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::piece::{Color, Piece};
use crate::piece_move::PieceMove;

impl Board {
    // (side, king_from_x, king_to_x, rook_from_x, must_be_empty, must_be_safe)
    const CASTLES: [(CastlingSide, usize, usize, usize, &[usize], &[usize]); 4] = [
        (WhiteKing,  4, 6, 7, &[5, 6],    &[5, 6]),
        (WhiteQueen, 4, 2, 0, &[1, 2, 3], &[2, 3]),
        (BlackKing,  4, 6, 7, &[5, 6],    &[5, 6]),
        (BlackQueen, 4, 2, 0, &[1, 2, 3], &[2, 3]),
    ];

    // Get all possible moves for a piece (excluding pawn moves)
    pub(crate) fn piece_moves(&self, piece: Piece, starting_pos: (usize, usize)) -> Vec<(usize, usize)> {
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
    pub(crate) fn legal_moves(&mut self, from: (usize, usize)) -> Vec<PieceMove> {
        let pseudo = self.pseudo_legal_moves(from);
        let mut out = Vec::with_capacity(pseudo.len()); // preallocate

        for m in pseudo {
            let undo = self.make(m);
            let leaves_king_in_check = self.is_in_check(m.piece.color);
            self.unmake(m, undo);

            if !leaves_king_in_check {
                out.push(m);
            }
        }

        out
    }

    // legal moves without creating a new vector
    pub(crate) fn all_legal_moves(&mut self, from: (usize, usize), out: &mut Vec<PieceMove>) {
        for rank in 0..8 {
            for file in 0..8 {
                let Some(p) = self.board_state[rank][file] else { continue };
                if p.color != self.turn { continue }
                let pseudo = self.pseudo_legal_moves(from);
                out.extend(pseudo);
            }
        }

        out.retain(|&m| {
            let undo = self.make(m);
            let legal = !self.is_in_check(m.piece.color);
            self.unmake(m, undo);
            legal
        })
    }

    pub(crate) fn has_legal_moves(&mut self) -> bool {
        for from in self.all_squares_with_own_pieces(self.turn) {
            for _m in self.legal_moves(from) {
                return true
            }
        }
        false
    }

    pub(crate) fn all_squares_with_own_pieces(&self, color: Color) -> Vec<(usize, usize)> {
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
}