use crate::piece::Piece;

#[derive(Copy, Clone, PartialEq)]
pub struct PieceMove {
    pub to: (usize, usize),
    pub from: (usize, usize),
    pub piece: Piece,
}