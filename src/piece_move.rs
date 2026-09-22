use crate::piece::{Kind, Piece};
use crate::square::coordinate_to_square;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PieceMove {
    pub to: (usize, usize),
    pub from: (usize, usize),
    pub piece: Piece,
    pub promotion: Option<Kind>
}
impl PieceMove {
    pub fn to_uci(&self) -> String {
        let promo = match self.promotion {
            Some(Kind::Queen) => "q", Some(Kind::Rook) => "r",
            Some(Kind::Bishop) => "b", Some(Kind::Knight) => "n",
            _ => "",
        };
        format!("{}{}{}", coordinate_to_square(&self.from), coordinate_to_square(&self.to), promo)
    }
}
