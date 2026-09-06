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
    fn piece_moves(&self, piece: Piece, starting_pos: (usize, usize), out: &mut Vec<PieceMove>) {

        let deltas = piece.kind.move_delta();
        let slider = piece.kind.is_slider();
        let (start_x, start_y) = starting_pos;

        let mut piece_move = PieceMove {
            to: (0, 0),
            from: (start_x, start_y),
            piece,
            promotion: None,
        };

        for (dx, dy) in deltas {
            let mut new_x = start_x as i32 + dx;
            let mut new_y = start_y as i32 + dy;
            while (0..8).contains(&new_x) && (0..8).contains(&new_y) {
                // If blocked by something
                if let Some(target_piece) = self.board_state[new_y as usize][new_x as usize] {
                    if target_piece.color == piece.color {
                        break
                    } else {
                        piece_move.to = (new_x as usize, new_y as usize);
                        out.push(piece_move);
                        break
                    }
                }
                piece_move.to = (new_x as usize, new_y as usize);
                out.push(piece_move);
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
            self.castle_moves(piece, out);
        }
    }

    // Get all possible moves for pawns on board
    fn pawn_moves(&self, piece: Piece, starting_pos: (usize, usize), out: &mut Vec<PieceMove>) {
        let (start_x, start_y) = starting_pos;
        let piece_move = PieceMove {
            to: (0, 0),
            from: (start_x, start_y),
            piece,
            promotion: None,
        };
        let promo_rank = match piece.color {
            Black => 0,
            White => 7
        };
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
                    Board::push_pawn(piece_move, (new_x, new_y), promo_rank, out);
                }
            } else if self.en_passant == Some((new_x, new_y)) {
                Board::push_pawn(piece_move, (new_x, new_y), promo_rank, out);
            }
        }

        // Front
        // if not blocked
        let None = self.board_state[new_y][start_x] else {
            return;
        };

        Board::push_pawn(piece_move, (start_x, new_y), promo_rank, out);

        if start_y == start_rank {
            let new_2y = (start_y as i32 + 2 * dy) as usize;
            if self.board_state[new_2y][start_x].is_none() {
                Board::push_pawn(piece_move, (start_x, new_2y), promo_rank, out);
            }
        }

    }

    fn castle_moves(&self, piece: Piece, out: &mut Vec<PieceMove>) {
        let rank = match piece.color { White => 0, Black => 7 };

        for (side, king_from, king_to, _rook_from, empty, safe) in Self::CASTLES {
            if side.color() != piece.color { continue } // color doesn't match
            if !self.castling[side] { continue } // castling rights revoked
            if empty.iter().any(|&f| self.board_state[rank][f].is_some()) { continue } // If any cell in between is empty
            if self.is_attacked((4, rank), !piece.color) { continue } // currently in check
            if safe.iter().any(|&f| self.is_attacked((f, rank), !piece.color)) { continue } // If any cell is being attacked
            let piece_move = PieceMove {
                to: (king_to, rank),
                from: (king_from, rank),
                piece,
                promotion: None,
            };
            out.push(piece_move);
        }
    }

    fn push_pawn(mut m: PieceMove, to: (usize, usize), promo_rank: usize, out: &mut Vec<PieceMove>) {
        m.to = to;
        if to.1 == promo_rank {
            for k in [Queen, Knight, Rook, Bishop] {
                m.promotion = Some(k);
                out.push(m);
            }
        } else {
            out.push(m);
        }
    }

    fn pseudo_legal_moves(&self, from: (usize, usize), out: &mut Vec<PieceMove>) {
        let (start_x, start_y) = from;

        let Some(piece) = self.board_state[start_y][start_x] else {
            return;
        };

        match piece.kind {
            Pawn => self.pawn_moves(piece, from, out),
            _ => self.piece_moves(piece, from, out),
        }
    }

    // filters out all the legal moves
    pub(crate) fn legal_moves(&mut self, from: (usize, usize)) -> Vec<PieceMove> {
        let mut out = Vec::with_capacity(64);
        self.pseudo_legal_moves(from, &mut out);

        out.retain(|&m| {
            let undo = self.make(m);
            let legal = !self.is_in_check(m.piece.color);
            self.unmake(m, undo);
            legal
        });

        out
    }

    // legal moves without creating a new vector
    pub(crate) fn all_legal_moves(&mut self, out: &mut Vec<PieceMove>) {
        for rank in 0..8 {
            for file in 0..8 {
                let Some(p) = self.board_state[rank][file] else { continue };
                if p.color != self.turn { continue }
                self.pseudo_legal_moves((file, rank), out);
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