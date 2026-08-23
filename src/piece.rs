use std::fmt;
use std::fmt::Formatter;
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{Bishop, King, Knight, Pawn, Queen, Rook};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Pawn,
    King,
    Knight,
    Rook,
    Bishop,
    Queen
}

impl Kind {
    pub fn is_slider(self) -> bool {
        match self {
            Pawn | King | Knight => false,
            Rook | Bishop | Queen => true,
        }
    }

    pub fn symbol(self) -> char {
        match self {
            Pawn => 'P',
            King => 'K',
            Knight => 'N',
            Rook => 'R',
            Bishop => 'B',
            Queen => 'Q',
        }
    }

    pub fn char_to_piece(c: char) -> Piece {
        match c {
            'p' => Piece { kind: Pawn, color: Black },
            'r' => Piece { kind: Rook, color: Black },
            'n' => Piece { kind: Knight, color: Black },
            'b' => Piece { kind: Bishop, color: Black },
            'q' => Piece { kind: Queen, color: Black },
            'k' => Piece { kind: King, color: Black },
            'P' => Piece { kind: Pawn, color: White },
            'R' => Piece { kind: Rook, color: White },
            'N' => Piece { kind: Knight, color: White },
            'B' => Piece { kind: Bishop, color: White },
            'Q' => Piece { kind: Queen, color: White },
            'K' => Piece { kind: King, color: White },
            _ => panic!("Unknown piece!")
        }
    }

    pub fn move_delta(self) -> &'static[(i32, i32)] {
        match self {
            Pawn => &[(0, 1)],
            King | Queen => {
                &[
                    (-1, -1), (0, -1), (1, -1),
                    (-1,  0),          (1,  0),
                    (-1,  1), (0,  1), (1,  1),
                ]
            }
            Knight => {
                &[
                              (-1, -2), (1, -2),
                    (-2, -1),                   (2, -1),
                    (-2,  1),                   (2, 1),
                              (-1,  2), (1,  2)
                ]
            }
            Rook => {
                &[
                              (0, -1),
                    (-1,  0),          (1,  0),
                              (0,  1),
                ]
            }
            Bishop => {
                &[
                    (-1, -1),          (1, -1),
                    (-1,  1),          (1,  1),
                ]
            }
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Pawn => 'P',
            King => 'K',
            Knight => 'N',
            Rook => 'R',
            Bishop => 'B',
            Queen => 'Q',
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    White
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color
}

impl Piece {
    pub fn new(kind: Kind, color: Color) -> Piece {
        Piece { kind, color }
    }
}
