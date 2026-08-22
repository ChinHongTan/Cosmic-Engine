use std::fmt;
use std::fmt::Formatter;

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
            Kind::Pawn | Kind::King | Kind::Knight => false,
            Kind::Rook | Kind::Bishop | Kind::Queen => true,
        }
    }

    pub fn symbol(self) -> char {
        match self {
            Kind::Pawn => 'P',
            Kind::King => 'K',
            Kind::Knight => 'N',
            Kind::Rook => 'R',
            Kind::Bishop => 'B',
            Kind::Queen => 'Q',
        }
    }

    pub fn move_delta(self) -> &'static[(i32, i32)] {
        match self {
            Kind::Pawn => &[(0, 1)],
            Kind::King | Kind::Queen => {
                &[
                    (-1, -1), (0, -1), (1, -1),
                    (-1,  0),          (1,  0),
                    (-1,  1), (0,  1), (1,  1),
                ]
            }
            Kind::Knight => {
                &[
                              (-1, -2), (1, -2),
                    (-2, -1),                   (2, -1),
                    (-2,  1),                   (2, 1),
                              (-1,  2), (1,  2)
                ]
            }
            Kind::Rook => {
                &[
                              (0, -1),
                    (-1,  0),          (1,  0),
                              (0,  1),
                ]
            }
            Kind::Bishop => {
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
            Kind::Pawn => 'P',
            Kind::King => 'K',
            Kind::Knight => 'N',
            Kind::Rook => 'R',
            Kind::Bishop => 'B',
            Kind::Queen => 'Q',
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
