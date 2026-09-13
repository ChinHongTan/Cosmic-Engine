use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Not;
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{Bishop, King, Knight, Pawn, Queen, Rook};

const PAWN_TABLE: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0,
];

const KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const ROOK_TABLE: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0,
];

const QUEEN_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const KING_MG_TABLE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

const KING_EG_TABLE: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];

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
    pub const fn is_slider(self) -> bool {
        match self {
            Pawn | King | Knight => false,
            Rook | Bishop | Queen => true,
        }
    }

    pub const fn value(self) -> i32 {
        match self {
            Pawn => 100,
            King => 0,
            Knight => 320,
            Rook => 500,
            Bishop => 330,
            Queen => 900,
        }
    }

    pub const fn table(self, endgame: bool) -> &'static [i32; 64] { // return reference to speed up
        match self {
            Pawn => &PAWN_TABLE,
            King => {
                if endgame { &KING_EG_TABLE } else { &KING_MG_TABLE }
            },
            Knight => &KNIGHT_TABLE,
            Rook => &ROOK_TABLE,
            Bishop => &BISHOP_TABLE,
            Queen => &QUEEN_TABLE,
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

    pub const fn move_delta(self) -> &'static[(i32, i32)] {
        match self {
            Pawn => &[],
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

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Black = 0,
    White = 1
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Black => White,
            White => Black
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let color = match self {
            Black => "Black",
            White => "White",
        };
        write!(f, "{}", color)
    }
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

    pub fn symbol(self) -> char {
        match self.color {
            Black => {
                self.kind.symbol().to_ascii_lowercase()
            }
            White => {
                self.kind.symbol()
            }
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
}
