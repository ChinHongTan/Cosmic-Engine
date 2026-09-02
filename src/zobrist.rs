use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

pub(crate) struct Zobrist {
    pub random_table: [[[u64; 64]; 6]; 2],
    pub black_to_move: u64,
    pub castling: [u64; 4],
    pub en_passant_file: [u64; 8]
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(0x1234_5678_9ABC_DEF0);

        let mut random_table = [[[0u64; 64]; 6]; 2];

        for color in 0..2 {
            for kind in 0..6 {
                for square in 0..64 {
                    random_table[color][kind][square] = rng.random();
                }
            }
        }

        Zobrist {
            random_table,
            black_to_move: rng.random(),
            castling: std::array::from_fn(|_| rng.random()),
            en_passant_file: std::array::from_fn(|_| rng.random()),
        }
    }
}