use crate::board::Board;
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{King, Pawn};
use crate::piece_move::PieceMove;
use crate::square::coordinate_to_square;

impl Board {
    const MATE: i32 = 100_000;
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

    pub fn negamax(&mut self, depth: u32, ply: u32, mut alpha: i32, beta: i32) -> i32 {
        self.nodes += 1;
        if depth == 0 {
            return self.quiescence(alpha, beta);
        }

        let mut moves = Vec::with_capacity(64);
        self.all_legal_moves(&mut moves);

        if moves.len() == 0 {
            return if self.is_in_check(self.turn) { -Self::MATE + ply as i32 } else { 0 }
        }

        moves.sort_by_key(|m| -self.move_score(&m));

        for m in moves {
            let undo = self.make(m);
            let score = -self.negamax(depth - 1, ply + 1, -beta, -alpha);
            self.unmake(m, undo);
            if score >= beta {
                return beta;          // cutoff — opponent won't allow this line
            }
            if score > alpha {
                alpha = score;        // new best
            }
        }
        alpha
    }

    pub fn negamax_plain(&mut self, depth: u32, ply: u32) -> i32 {
        self.nodes += 1;
        if depth == 0 {
            return self.evaluate();
        }

        let mut moves = Vec::with_capacity(64);
        self.all_legal_moves(&mut moves);
        if moves.len() == 0 {
            return if self.is_in_check(self.turn) { -Self::MATE + ply as i32 } else { 0 }
        }

        let mut best = -Self::INF;

        for m in moves {
            let undo = self.make(m);
            let score = -self.negamax_plain(depth - 1, ply + 1);
            self.unmake(m, undo);
            best = best.max(score);
        }
        best
    }

    pub fn search(&mut self, max_depth: u32) -> Option<PieceMove> {
        let mut best = None;
        for depth in 1..=max_depth {
            best = self.best_move(depth, best);
            println!("depth {depth}: {:?}", best)
        }
        best
    }

    pub fn best_move(&mut self, depth: u32, prev: Option<PieceMove>) -> Option<PieceMove> {
        let mut moves = Vec::with_capacity(64);
        self.all_legal_moves(&mut moves);

        moves.sort_by_key(|m| {
            if Some(*m) == prev { -1_000_000 }        // previous best goes first
            else { -self.move_score(m) }
        });

        let mut best_score = -Self::INF;
        let mut best_move = None;

        for m in moves {
            let undo = self.make(m);
            let score = -self.negamax(depth - 1, 1, -Self::INF, -best_score);
            self.unmake(m, undo);

            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
        }

        best_move
    }

    pub fn move_score(&self, m: &PieceMove) -> i32 {
        match self.get_piece(m.to) {
            // Most valuable victim, least valuable attacker
            Some(victim) => victim.kind.value() * 10 - m.piece.kind.value(),
            None => 0
        }
    }

    pub fn search_divide(&mut self, depth: u32, ply: u32) {
        let mut moves = Vec::with_capacity(64);
        self.all_legal_moves(&mut moves);
        for m in moves {
            let undo = self.make(m);
            let score = -self.negamax(depth - 1, ply + 1, -Self::INF, Self::INF);
            self.unmake(m, undo);
            println!("{}{}: {}", coordinate_to_square(&m.from), coordinate_to_square(&m.to), score);
        }
    }

    pub fn quiescence(&mut self, mut alpha: i32, beta: i32) -> i32 {
        self.nodes += 1;



        let mut captures = Vec::with_capacity(64);
        self.capture_moves(&mut captures);
        captures.sort_by_key(|m| -self.move_score(m));

        for m in captures {
            let undo = self.make(m);
            let score = -self.quiescence(-beta, -alpha);
            self.unmake(m, undo);

            if score >= beta { return beta; }
            if score > alpha { alpha = score; }
        }

        alpha
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