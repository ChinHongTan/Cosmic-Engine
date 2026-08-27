use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub enum CastlingSide {
    WhiteKing = 0,
    WhiteQueen = 1,
    BlackKing = 2,
    BlackQueen = 3
}

#[derive(Clone, Debug)]
pub(crate) struct CastlingRights (pub [bool; 4]);

impl Index<CastlingSide> for CastlingRights {
    type Output = bool;

    fn index(&self, index: CastlingSide) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<CastlingSide> for CastlingRights {
    fn index_mut(&mut self, index: CastlingSide) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

// Turns FEN string to castling rights
pub fn str_to_castling(s: &str) -> CastlingRights {
    let mut castling = CastlingRights ([false; 4]);
    for c in s.chars() {
        match c {
            'K' => castling[CastlingSide::WhiteKing] = true,
            'Q' => castling[CastlingSide::WhiteQueen] = true,
            'k' => castling[CastlingSide::BlackKing] = true,
            'q' => castling[CastlingSide::BlackQueen] = true,
            _ => panic!("Invalid castling string"),
        }
    }
    castling
}

// See if a position matches the four corners. If yes, revoke castling rights
pub fn check_castling_pos(pos: &(usize, usize)) -> Option<CastlingSide> {
    match pos {
        (0, 0) => Some(CastlingSide::WhiteQueen),
        (7, 0) => Some(CastlingSide::WhiteKing),
        (0, 7) => Some(CastlingSide::BlackQueen),
        (7, 7) => Some(CastlingSide::BlackKing),
        _ => None
    }
}