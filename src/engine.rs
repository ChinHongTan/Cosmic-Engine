use crate::board::Board;
use crate::piece::{Kind};
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{Bishop, King, Knight, Pawn, Queen, Rook};

impl Board {
    pub(crate) fn evaluate(&self) -> i32 {
        const PIECE_VALUE: [(Kind, i32); 6] = [
            (Pawn, 100),
            (King, 0),
            (Knight, 320),
            (Rook, 500),
            (Bishop, 330),
            (Queen, 900)
        ];
        let all_white_piece = self.all_squares_with_own_pieces(White);
        let mut score = 0;
        for p in all_white_piece {
            for (kind, value) in PIECE_VALUE {
                if self.get_piece(p).unwrap().kind == kind {
                    score += value;
                }
            }
        }
        let all_black_piece = self.all_squares_with_own_pieces(Black);
        for p in all_black_piece {
            for (kind, value) in PIECE_VALUE {
                if self.get_piece(p).unwrap().kind == kind {
                    score -= value;
                }
            }
        }
        score
    }
}