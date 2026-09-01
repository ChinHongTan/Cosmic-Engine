use crate::piece::{Kind, Piece};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PieceMove {
    pub to: (usize, usize),
    pub from: (usize, usize),
    pub piece: Piece,
    pub promotion: Option<Kind>
}