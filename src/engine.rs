use crate::board::Board;
use crate::piece::{Color};
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{King, Pawn};
use crate::piece_move::PieceMove;

impl Board {
    pub(crate) const MATE: i32 = 100_000;
    pub(crate) const INF: i32 = 1_000_000;


    const PASSED_MG: [i32; 8] = [0, 5, 10, 15, 25, 40, 60, 0];
    const PASSED_EG: [i32; 8] = [0, 10, 20, 35, 60, 100, 150, 0];

    pub fn evaluate(&self) -> i32 {
        let (mut mg, mut eg, mut phase) = (0, 0, 0);
        for rank in 0..8 {
            for file in 0..8 {
                let Some(p) = self.board_state[rank][file] else { continue };
                let index = match p.color {
                    Black => rank * 8 + file,
                    White => (7 - rank) * 8 + file,
                };

                let (mut m, mut e) = p.kind.score(index);

                if p.kind == Pawn && self.is_passed(rank, file, p.color) {
                    let rel = match p.color { White => rank, Black => 7 - rank };
                    m += Self::PASSED_MG[rel];
                    e += Self::PASSED_EG[rel]
                }

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

    fn is_passed(&self, rank: usize, file: usize, color: Color) -> bool {
        let ahead = match color {
            White => rank + 1..8,
            Black => 0..rank
        };
        for r in ahead {
            for f in file.saturating_sub(1)..=(file + 1).min(7) {
                if let Some(q) = self.board_state[r][f] {
                    if q.kind == Pawn && q.color != color { return false; }
                }
            }
        }
        true
    }
}