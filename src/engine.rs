use crate::board::Board;
use crate::piece::{Color};
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{King, Pawn};
use crate::piece_move::PieceMove;

impl Board {
    pub(crate) const MATE: i32 = 100_000;
    pub(crate) const INF: i32 = 1_000_000;

    pub fn evaluate(&self) -> i32 {
        let (mut mg, mut eg, mut phase) = (0, 0, 0);
        for rank in 0..8 {
            for file in 0..8 {
                let Some(p) = self.board_state[rank][file] else { continue };
                let index = match p.color {
                    Black => rank * 8 + file,
                    White => (7 - rank) * 8 + file,
                };

                let (m, e) = p.kind.score(index);

                match p.color {
                    White => { mg += m; eg += e },
                    Black => { mg -= m; eg -= e },
                }
                phase += p.kind.phase();
            }
        }
        let mg_phase = phase.min(24);
        let score = (mg * mg_phase + eg * (24 - mg_phase)) / 24;
        // negamax
        match self.turn {
            Black => -score,
            White => score
        }
    }

    pub fn move_score(&self, m: &PieceMove) -> i32 {
        match self.get_piece(m.to) {
            // Most valuable victim, least valuable attacker
            Some(victim) => victim.kind.value() * 10 - m.piece.kind.value(),
            None => 0
        }
    }

    pub(crate) fn has_non_pawn(&self, color: Color) -> bool {
        self.board_state.iter().flatten().flatten()
            .any(|p| p.color == color && p.kind != King && p.kind != Pawn)
    }
}