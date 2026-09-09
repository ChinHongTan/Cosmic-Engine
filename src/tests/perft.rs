use crate::board::Board;
use crate::castling::CastlingSide::BlackKing;

const PERFT_CASES: &[(&str, &[u64])] = &[
    // start position
    ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
     &[20, 400, 8_902, 197_281, 4_865_609]),
    // kiwipete — castling, ep, checks, pins
    ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
     &[48, 2_039, 97_862, 4_085_603]),
    // position 3 — en passant horizontal pin
    ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
     &[14, 191, 2_812, 43_238, 674_624]),
    // position 4 — promotions at depth 1
    ("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
     &[6, 264, 9_467, 422_333]),
    // position 5
    ("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
     &[44, 1_486, 62_379, 2_103_487]),
    // position 6
    ("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
     &[46, 2_079, 89_890, 3_894_594]),
];

#[test]
fn perft_suite() {
    for (fen, expected) in PERFT_CASES {
        let mut b = Board::from_fen(fen);
        for (i, &n) in expected.iter().enumerate() {
            let depth = i as u32 + 1;
            assert_eq!(b.perft(depth), n, "{fen} at depth {depth}");
        }
    }
}

#[test]
fn capturing_rook_on_home_square_revokes_castling() {
    // white knight f7, black rook h8 with kingside rights
    let mut b = Board::from_fen("r3k2r/p1ppqNb1/bn2p1p1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    b.make_move((5, 6), (7, 7), None);   // Nxh8
    assert!(!b.castling[BlackKing]);
}

#[test]
fn alpha_beta_matches_plain_negamax() {
    for fen in ["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"] {
        let mut b = Board::from_fen(fen);
        assert_eq!(
            b.negamax(4, 0, -Board::INF, Board::INF),
            b.negamax_plain(4, 0),
            "{fen}"
        );
    }
}