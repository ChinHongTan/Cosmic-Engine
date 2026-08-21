use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy)]
pub enum PieceEnum {
    Pawn(Pawn),
    King(King),
    Knight(Knight),
    Rook(Rook),
}

impl fmt::Display for PieceEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)?;
        Ok(())
    }
}

pub trait Piece {
    const MOVE_DELTA: &[(i32, i32)];
    const SLIDER: bool;
}

impl PieceEnum {
    pub fn properties(&self) -> (&[(i32, i32)], bool) {
        match self {
            PieceEnum::Pawn(_) => (Pawn::MOVE_DELTA, Pawn::SLIDER),
            PieceEnum::King(_) => (King::MOVE_DELTA, King::SLIDER),
            PieceEnum::Knight(_) => (Knight::MOVE_DELTA, Knight::SLIDER),
            PieceEnum::Rook(_) => (Rook::MOVE_DELTA, Rook::SLIDER)
        }
    }
}

#[derive(Clone, Copy)]
pub struct Pawn {
}

impl Piece for Pawn {
    const MOVE_DELTA: &[(i32, i32)] = &[(0, 1)];
    const SLIDER: bool = false;
}

#[derive(Clone, Copy)]
pub struct King {
}

impl Piece for King {
    const MOVE_DELTA: &[(i32, i32)] = &[
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];
    const SLIDER: bool = false;
}

#[derive(Clone, Copy)]
pub struct Knight {
}

impl Piece for Knight {
    const MOVE_DELTA: &[(i32, i32)] = &[
                  (-1, -2), (1, -2),
        (-2, -1),                   (2, -1),
        (-2,  1),                   (2, 1),
                  (-1,  2), (1,  2)
    ];
    const SLIDER: bool = false;
}

#[derive(Clone, Copy)]
pub struct Rook {
}

impl Piece for Rook {
    const MOVE_DELTA: &[(i32, i32)] = &[
                  (0, -1),
        (-1,  0),          (1,  0),
                  (0,  1),
    ];
    const SLIDER: bool = true;
}