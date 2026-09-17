use crate::board::Board;

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
    board: Board,
    tt: Vec<Option<TTEntry>>
}

impl Search {
    pub fn new(board: Board) -> Search {
        Search {
            board,
            tt: Vec::with_capacity(1_048_576)
        }
    }
    
    pub fn tt_probe(&self, hash: u64) -> Option<TTEntry> {
        let index = hash as usize % self.tt.len();
        let Some(entry) = &self.tt[index] else { return None };
        if entry.hash != hash { return None }
        Some(*entry)
    }

    pub fn tt_store() {
        todo!()
    }
}