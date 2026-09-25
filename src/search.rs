use std::time::{Duration, Instant};
use crate::board::Board;
use crate::piece::Kind::Pawn;
use crate::piece_move::PieceMove;
use crate::search::NodeType::{Exact, LowerBound, UpperBound};

#[derive(Copy, Clone, Debug)]
pub enum NodeType {
    LowerBound,
    Exact,
    UpperBound
}

#[derive(Copy, Clone, Debug)]
pub struct TTEntry {
    score: i32,
    depth: u32,
    hash: u64,
    node_type: NodeType,
    best_move: Option<PieceMove>,
}

pub struct Search {
    tt: Vec<Option<TTEntry>>,
    nodes: u64,
    deadline: Option<Instant>,
    stop: bool,
}

// mate score
const MATE_THRESHOLD: i32 = Board::MATE - 1000;

// Mate scores are -MATE + ply, but a TT entry is keyed by
// position only and can be read back at a different ply
fn to_tt_score(score: i32, ply: u32) -> i32 {
    if score >= MATE_THRESHOLD { score + ply as i32 }
    else if score <= -MATE_THRESHOLD { score - ply as i32 }
    else { score }
}

fn from_tt_score(score: i32, ply: u32) -> i32 {
    if score >= MATE_THRESHOLD { score - ply as i32 }
    else if score <= -MATE_THRESHOLD { score + ply as i32 }
    else { score }
}

impl Search {
    pub fn new() -> Search {
        Search { tt: vec![None; 1 << 20], nodes: 0, deadline: None, stop: false }
    }

    fn check_time(&mut self) {
        if self.nodes & 2047 == 0 {
            if let Some(d) = self.deadline {
                if Instant::now() >= d { self.stop = true; }
            }
        }
    }

    pub fn tt_probe(&self, hash: u64, ply: u32) -> Option<TTEntry> {
        let index = hash as usize & (self.tt.len() - 1);
        let Some(entry) = &self.tt[index] else { return None };
        if entry.hash != hash { return None }
        let mut entry = *entry;
        entry.score = from_tt_score(entry.score, ply);
        Some(entry)
    }

    pub fn tt_store(&mut self, hash: u64, score: i32, depth: u32, ply: u32, node_type: NodeType, best_move: Option<PieceMove>) {
        let index = hash as usize & (self.tt.len() - 1);
        let score = to_tt_score(score, ply);
        let entry = TTEntry { score, depth, hash, node_type, best_move };
        self.tt[index] = Some(entry);
    }

    pub fn negamax(&mut self, board: &mut Board, mut depth: u32, ply: u32, mut alpha: i32, beta: i32) -> i32 {
        if self.stop { return 0; }
        self.nodes += 1;
        self.check_time();
        if board.is_repeated() { return 0; }
        let original_alpha = alpha;
        let mut best_score= -Board::INF;
        let mut tt_move = None;
        let mut best_move = None;
        let hash = board.hash;

        let in_check = board.is_in_check(board.turn);

        // a mate delivered on the 100th half-move still counts as a mate
        if board.halfmove >= 100 && (!in_check || board.has_legal_moves()) { return 0; }

        if in_check {
            depth += 1;
        }

        if depth == 0 {
            return self.quiescence(board, ply, alpha, beta);
        }

        if let Some(e) = self.tt_probe(hash, ply) {
            if e.depth >= depth {
                match e.node_type {
                    LowerBound => {
                        if e.score >= beta { return e.score }
                    }
                    Exact => {
                        return e.score;
                    }
                    UpperBound => {
                        if e.score <= alpha { return e.score }
                    }
                }
            }
            tt_move = e.best_move;
        }

        let mut moves = Vec::with_capacity(64);
        board.all_legal_moves(&mut moves);

        if moves.len() == 0 {
            return if in_check { -Board::MATE + ply as i32 } else { 0 }
        }

        moves.sort_by_key(|m| {
            if Some(*m) == tt_move { -1_000_000 }
            else { -board.move_score(&m) }
        });

        for m in moves {
            let undo = board.make(m);
            board.history.push(board.hash);
            let score = -self.negamax(board, depth - 1, ply + 1, -beta, -alpha);
            board.history.pop();
            board.unmake(m, undo);
            if score >= beta {
                best_score = score;          // cutoff — opponent won't allow this line
                best_move = Some(m);
                break;
            }
            if score > best_score {
                best_score = score;          // new best
                best_move = Some(m);
                if score > alpha { alpha = score; }
            }
        }
        let node_type;
        if best_score >= beta {
            node_type = LowerBound;
        } else if best_score > original_alpha {
            node_type = Exact;
        } else {
            node_type = UpperBound;
        }
        if self.stop { return 0; }
        self.tt_store(hash, best_score, depth, ply, node_type, best_move);
        best_score
    }

    pub fn quiescence(&mut self, board: &mut Board, ply: u32, mut alpha: i32, beta: i32) -> i32 {
        if self.stop { return 0; }
        self.nodes += 1;
        self.check_time();
        let in_check = board.is_in_check(board.turn);
        let mut moves = Vec::with_capacity(64);
        board.all_legal_moves(&mut moves);

        if moves.len() == 0 {
            return if in_check { -Board::MATE + ply as i32 } else { 0 }
        }
        // a quiet check evasion can reach the 100th half-move here
        if board.halfmove >= 100 { return 0; }

        if !in_check {
            let stand_pat = board.evaluate();
            if stand_pat >= beta { return beta; }
            if stand_pat > alpha { alpha = stand_pat }
            moves.retain(|m| {
                board.get_piece(m.to).is_some()
                    || (m.piece.kind == Pawn && Some(m.to) == board.en_passant)
                    || m.promotion.is_some()
            });
        }

        moves.sort_by_key(|m| -board.move_score(m));

        for m in moves {
            let undo = board.make(m);
            let score = -self.quiescence(board, ply + 1, -beta, -alpha);
            board.unmake(m, undo);

            if score >= beta { return beta; }
            if score > alpha { alpha = score; }
        }

        alpha
    }

    pub fn search(&mut self, board: &mut Board, max_depth: u32) -> Option<PieceMove> {
        self.stop = false;
        let mut best = None;
        for depth in 1..=max_depth {
            best = self.best_move(board, depth, best).0;
            // println!("depth {depth}: {:?}", best)
        }
        best
    }

    pub fn search_timed(&mut self, board: &mut Board, max_depth: u32, soft: Duration, hard: Duration) -> Option<PieceMove> {
        self.stop = false;
        self.deadline = None;
        self.nodes = 0;
        let start = Instant::now();

        // depth 1 always completes, so we're guaranteed a legal move
        let (mut best, score) = self.best_move(board, 1, None);
        self.print_info(board, 1, score, best, start);

        self.deadline = Some(start + hard);
        let mut stability = 0;
        let mut prev_score = score;
        for depth in 2..=max_depth {
            let iter_start = Instant::now();
            let (m, score) = self.best_move(board, depth, best);
            if self.stop { break; }
            let mut drop: u32 = 0;
            if depth >= 3 {
                if best == m { stability += 1; } else { stability = 0; }
                if prev_score > score && score.abs() < MATE_THRESHOLD && prev_score.abs() < MATE_THRESHOLD {
                    drop = (prev_score - score) as u32;
                }
            }
            best = m;
            self.print_info(board, depth, score, best, start);
            prev_score = score;
            let mult = 10u32.saturating_sub(stability.min(3));
            let drop_mult = 10 + (drop.min(200) / 20);   // 10 → 20, i.e. 1.0× → 2.0×
            let effective_soft = (soft * mult * drop_mult / 100).min(hard);
            let last_iter = iter_start.elapsed();
            if start.elapsed() + last_iter * 2 >= effective_soft { break }
        }

        self.deadline = None;
        best
    }

    fn print_info(&self, board: &mut Board, depth: u32, score: i32, best: Option<PieceMove>, start: Instant) {
        let score_str = if score >= MATE_THRESHOLD {
            format!("mate {}", (Board::MATE - score + 1) / 2)
        } else if score <= -MATE_THRESHOLD {
            format!("mate -{}", (Board::MATE + score) / 2)
        } else {
            format!("cp {}", score)
        };

        let pv: Vec<String> = self.pv_line(board, best, depth).iter().map(|m| m.to_uci()).collect();
        let ms = start.elapsed().as_millis() as u64;
        let nps = self.nodes * 1000 / ms.max(1);

        println!("info depth {} score {} nodes {} nps {} time {} pv {}",
                 depth, score_str, self.nodes, nps, ms, pv.join(" "));
    }

    // The TT keeps the best move of every searched node
    fn pv_line(&self, board: &mut Board, best: Option<PieceMove>, depth: u32) -> Vec<PieceMove> {
        let mut pv = Vec::new();
        let mut undos = Vec::new();
        // positions since the last pawn move or capture, including the real game
        let mut seen = board.history[board.irreversible..].to_vec();
        let mut next = best;

        while let Some(m) = next {
            // a hash collision can hand back a move from another position
            let mut legal = Vec::with_capacity(64);
            board.all_legal_moves(&mut legal);
            if !legal.contains(&m) { break; }

            undos.push(board.make(m));
            pv.push(m);

            // the game ends on a threefold, so the PV must too
            let repeats = seen.iter().filter(|&&h| h == board.hash).count();
            seen.push(board.hash);
            if pv.len() >= depth as usize || repeats >= 2 || board.halfmove >= 100 { break; }
            next = self.tt_probe(board.hash, 0).and_then(|e| e.best_move);
        }

        for (m, undo) in pv.iter().zip(undos).rev() {
            board.unmake(*m, undo);
        }
        pv
    }

    pub fn best_move(&mut self, board: &mut Board, depth: u32, prev: Option<PieceMove>) -> (Option<PieceMove>, i32) {
        let mut moves = Vec::with_capacity(64);
        board.all_legal_moves(&mut moves);

        moves.sort_by_key(|m| {
            if Some(*m) == prev { -1_000_000 }        // previous best goes first
            else { -board.move_score(m) }
        });

        let mut best_score = -Board::INF;
        let mut best_move = None;

        for m in moves {
            let undo = board.make(m);
            board.history.push(board.hash);
            let score = -self.negamax(board, depth - 1, 1, -Board::INF, -best_score);
            board.history.pop();
            board.unmake(m, undo);

            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
        }

        (best_move, best_score)
    }
}