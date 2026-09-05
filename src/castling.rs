use std::ops::{Index, IndexMut};
use crate::castling::CastlingSide::{BlackKing, BlackQueen, WhiteKing, WhiteQueen};
use crate::piece::Color;
use crate::piece::Color::{Black, White};

#[derive(Clone, Copy)]
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

impl CastlingSide {
    pub(crate) fn color(&self) -> Color {
        match self {
            WhiteKing | WhiteQueen => White,
            BlackKing | BlackQueen => Black,
        }
    }
}

// Turns FEN string to castling rights
pub fn str_to_castling(s: &str) -> CastlingRights {
    let mut castling = CastlingRights ([false; 4]);
    if s == "-" { return castling }
    for c in s.chars() {
        match c {
            'K' => castling[WhiteKing] = true,
            'Q' => castling[WhiteQueen] = true,
            'k' => castling[BlackKing] = true,
            'q' => castling[BlackQueen] = true,
            _ => panic!("Invalid castling string"),
        }
    }
    castling
}

// See if a position matches the four corners.
pub fn check_castling_pos(pos: &(usize, usize)) -> Option<CastlingSide> {
    match pos {
        (0, 0) => Some(WhiteQueen),
        (7, 0) => Some(WhiteKing),
        (0, 7) => Some(BlackQueen),
        (7, 7) => Some(BlackKing),
        _ => None
    }
}