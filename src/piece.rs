use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy)]
pub enum PieceEnum {
    Pawn(Pawn),
    King(King),
}

impl fmt::Display for PieceEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)?;
        Ok(())
    }
}

pub trait Piece {
    const MOVE_DELTA: &[(i32, i32)];
}

#[derive(Clone, Copy)]
pub struct Pawn {
}

impl Piece for Pawn {
    const MOVE_DELTA: &[(i32, i32)] = &[(0, 1)];
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
}