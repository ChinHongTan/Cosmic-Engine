use crate::board::Board;
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{King, Pawn};
use crate::piece_move::PieceMove;

impl Board {
    pub(crate) const MATE: i32 = 100_000;
    pub(crate) const INF: i32 = 1_000_000;
    pub fn evaluate(&self) -> i32 {
        let mut score = 0;
        for rank in 0..8 {
            for file in 0..8 {
                let Some(p) = self.board_state[rank][file] else { continue };
                let index = match p.color {
                    Black => rank * 8 + file,
                    White => (7 - rank) * 8 + file,
                };


                let endgame = self.endgame();
                let value = p.kind.value() + p.kind.table(endgame)[index];
                match p.color {
                    Black => score -= value,
                    White => score += value,
                }
            }
        }
        // negamax
        match self.turn {
            Black => -score,
            White => score
        }
    }

    fn endgame(&self) -> bool {
        let mut non_pawn = 0;
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(p) = self.board_state[rank][file] {
                    if p.kind != Pawn && p.kind != King {
                        non_pawn += p.kind.value();
                    }
                }
            }
        }
        non_pawn < 1300
    }

    pub fn move_score(&self, m: &PieceMove) -> i32 {
        match self.get_piece(m.to) {
            // Most valuable victim, least valuable attacker
            Some(victim) => victim.kind.value() * 10 - m.piece.kind.value(),
            None => 0
        }
    }

    pub fn capture_moves(&mut self, out: &mut Vec<PieceMove>) {
        self.all_legal_moves(out);
        out.retain(|m| {
            self.get_piece(m.to).is_some()
                || (m.piece.kind == Pawn && Some(m.to) == self.en_passant)
                || m.promotion.is_some()
        });
    }
}