use crate::board::Board;
use crate::square::coordinate_to_square;

impl Board {
    pub fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 { return 1; }
        let mut nodes = 0;
        let mut moves = Vec::with_capacity(64);
        self.all_legal_moves(&mut moves);
        for m in moves {
            let undo = self.make(m);
            nodes += self.perft(depth - 1);
            self.unmake(m, undo);
        }
        nodes
    }

    pub fn perft_divide(&mut self, depth: u32) {
        let mut total = 0;
        let mut moves = Vec::with_capacity(64);
        self.all_legal_moves(&mut moves);
        for m in moves {
            let mut next = self.clone();
            next.make(m);
            let n = next.perft(depth - 1);
            println!("{}{}: {}", coordinate_to_square(&m.from), coordinate_to_square(&m.to), n);
            total += n;
        }
        println!("total: {}", total);
    }
}