use crate::board::Board;
use crate::piece::Color::{Black, White};
use crate::piece_move::PieceMove;
use crate::square::coordinate_to_square;

impl Board {
    const MATE: i32 = 100_000;
    const INF: i32 = 1_000_000;
    pub fn evaluate(&self) -> i32 {
        let mut material = 0;
        for rank in 0..8 {
            for file in 0..8 {
                let Some(p) = self.board_state[rank][file] else { continue };
                match p.color {
                    Black => material -= p.kind.value(),
                    White => material += p.kind.value(),
                }
            }
        }
        // negamax
        match self.turn {
            Black => -material,
            White => material
        }
    }

    pub fn negamax(&mut self, depth: u32, ply: u32) -> (i32, u64) {
        if depth == 0 {
            return (self.evaluate(), 1);
        }

        let mut nodes = 0;
        let mut moves = Vec::with_capacity(64);
        self.all_legal_moves(&mut moves);
        if moves.len() == 0 {
            return if self.is_in_check(self.turn) { (-Self::MATE + ply as i32, 1) } else { (0, 1) }
        }

        let mut best = -Self::INF;

        for m in moves {
            let undo = self.make(m);
            let (score, n) = self.negamax(depth - 1, ply + 1);
            nodes += n;
            self.unmake(m, undo);
            best = best.max(-score);
        }
        (best, nodes)
    }

    pub fn best_move(&mut self, depth: u32) -> Option<PieceMove> {
        let mut moves = Vec::with_capacity(64);
        self.all_legal_moves(&mut moves);

        let mut best_score = -Self::INF;
        let mut best_move = None;

        for m in moves {
            let undo = self.make(m);
            let score = -self.negamax(depth - 1, 1).0;
            self.unmake(m, undo);

            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
        }

        best_move
    }

    pub fn search_divide(&mut self, depth: u32, ply: u32) {
        let mut moves = Vec::with_capacity(64);
        self.all_legal_moves(&mut moves);
        for m in moves {
            let undo = self.make(m);
            let score = -self.negamax(depth - 1, ply + 1).0;
            self.unmake(m, undo);
            println!("{}{}: {}", coordinate_to_square(&m.from), coordinate_to_square(&m.to), score);
        }
    }
}