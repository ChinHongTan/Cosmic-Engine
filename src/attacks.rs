use crate::board::Board;
use crate::piece::{Color, Piece};
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{King, Pawn, Rook, Queen, Bishop, Knight};

impl Board {
    // Sends ray outwards to detect if any piece is attacking this square
    pub fn is_attacked(&self, coordinate: (usize, usize), by: Color) -> bool {
        let (x, y) = coordinate;

        // Rook and Bishop
        for piece in [Rook, Bishop] {
            for (dx, dy) in piece.move_delta() {
                if self.first_piece(x, y, *dx, *dy).is_some_and(|p| p.color == by && [piece, Queen].contains(&p.kind)) {
                    return true;
                }
            }
        }

        // Knight and King
        for piece in [Knight, King] {
            for (dx, dy) in piece.move_delta() {
                if self.piece_at_offset(x, y, *dx, *dy).is_some_and(|p| p.color == by && p.kind == piece) {
                    return true;
                }
            }
        }

        // Pawn
        let dy = match by {
            Black => 1,
            White => -1,
        };
        for dx in [-1, 1] {
            if self.piece_at_offset(x, y, dx, dy).is_some_and(|p| p.color == by && p.kind == Pawn) {
                return true;
            }
        }

        false
    }

    fn piece_at_offset(&self, x: usize, y: usize, dx: i32, dy: i32) -> Option<Piece> {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        if (0..8).contains(&new_x) && (0..8).contains(&new_y) {
            let Some(piece) = self.board_state[new_y as usize][new_x as usize] else {
                return None;
            };

            return Some(piece);
        }
        None
    }

    fn first_piece(&self, x: usize, y: usize, dx: i32, dy: i32) -> Option<Piece> {
        let mut new_x = x as i32 + dx;
        let mut new_y = y as i32 + dy;
        while (0..8).contains(&new_x) && (0..8).contains(&new_y) {
            // If blocked by something
            if let Some(target_piece) = self.board_state[new_y as usize][new_x as usize] {
                return Some(target_piece)
            }

            new_x = new_x + dx;
            new_y = new_y + dy;
        }
        None
    }

    pub(crate) fn is_in_check(&self, color: Color) -> bool {
        self.is_attacked(self.king_pos[color as usize], !color)
    }

    pub(crate) fn find_king(&self, color: Color) -> Option<(usize, usize)> {
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
}