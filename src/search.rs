use crate::board::Board;
use crate::piece_move::PieceMove;

#[derive(Copy, Clone, Debug)]
pub enum NodeType {
    LowerBound,
    Exact,
    UpperBound
}

#[derive(Copy, Clone, Debug)]
pub struct TTEntry {
    depth: u32,
    hash: u64,
    node_type: NodeType
}

pub struct Search {
    tt: Vec<Option<TTEntry>>,
    nodes: u32
}

impl Search {
    pub fn new() -> Search {
        Search { tt: vec![None; 1 << 20], nodes: 0 }
    }

    pub fn tt_probe(&self, hash: u64) -> Option<TTEntry> {
        let index = hash as usize & (self.tt.len() - 1);
        let Some(entry) = &self.tt[index] else { return None };
        if entry.hash != hash { return None }
        Some(*entry)
    }

    pub fn tt_store() {
        todo!()
    }

    pub fn negamax(&mut self, board: &mut Board, depth: u32, ply: u32, mut alpha: i32, beta: i32) -> i32 {
        self.nodes += 1;
        let hash = board.hash();

        if let Some(_e) = self.tt_probe(hash) {}

        if depth == 0 {
            return self.quiescence(board, alpha, beta);
        }

        let mut moves = Vec::with_capacity(64);
        board.all_legal_moves(&mut moves);

        if moves.len() == 0 {
            return if board.is_in_check(board.turn) { -Board::MATE + ply as i32 } else { 0 }
        }

        moves.sort_by_key(|m| -board.move_score(&m));

        for m in moves {
            let undo = board.make(m);
            let score = -self.negamax(board, depth - 1, ply + 1, -beta, -alpha);
            board.unmake(m, undo);
            if score >= beta {
                return beta;          // cutoff — opponent won't allow this line
            }
            if score > alpha {
                alpha = score;        // new best
            }
        }
        alpha
    }

    pub fn quiescence(&mut self, board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
        self.nodes += 1;

        let stand_pat = board.evaluate();
        if stand_pat >= beta { return beta; }
        if stand_pat > alpha { alpha = stand_pat }

        let mut captures = Vec::with_capacity(64);
        board.capture_moves(&mut captures);
        captures.sort_by_key(|m| -board.move_score(m));

        for m in captures {
            let undo = board.make(m);
            let score = -self.quiescence(board, -beta, -alpha);
            board.unmake(m, undo);

            if score >= beta { return beta; }
            if score > alpha { alpha = score; }
        }

        alpha
    }

    pub fn search(&mut self, board: &mut Board, max_depth: u32) -> Option<PieceMove> {
        let mut best = None;
        for depth in 1..=max_depth {
            best = self.best_move(board, depth, best);
            println!("depth {depth}: {:?}", best)
        }
        best
    }

    pub fn best_move(&mut self, board: &mut Board, depth: u32, prev: Option<PieceMove>) -> Option<PieceMove> {
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
            let score = -self.negamax(board, depth - 1, 1, -Board::INF, Board::INF);
            board.unmake(m, undo);

            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
        }

        best_move
    }
}