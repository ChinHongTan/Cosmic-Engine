use crate::board::Board;
use crate::piece::Color;
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{King, Pawn};

impl Board {
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

    pub(crate) fn is_in_check(&self, color: Color) -> bool {
        match self.find_king(color) {
            None => false,
            Some(k) => self.is_attacked(k, !color),
        }
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
}