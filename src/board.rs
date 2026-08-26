use std::fmt;
use std::fmt::Formatter;
use crate::piece::{Color, Kind, Piece};
use crate::piece::Color::{Black, White};
use crate::piece::Kind::{Rook, Knight, Bishop, Queen, King, Pawn};
use crate::piece_move::PieceMove;
use crate::square::{square_to_coordinate, coordinate_to_square};

#[derive(Copy, Clone, Debug)]
pub struct Board {
    board_state: [[Option<Piece>; 8]; 8],
    turn: Color,
    en_passant: Option<(usize, usize)>
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let symbol = match self.board_state[rank][file] {
                    Some(piece) => piece.symbol(),
                    None => '.',
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  a b c d e f g h")
    }
}

// Back rank predefined
const BACK_RANK: [Kind; 8] = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];

impl Board {
    pub fn new() -> Board {
        let mut board_state = [[None; 8]; 8];
        for file in 0..8 {
            let kind = BACK_RANK[file];
            board_state[7][file] = Some(Piece { kind, color: Black });
            board_state[6][file] = Some(Piece { kind: Pawn, color: Black });
            board_state[1][file] = Some(Piece { kind: Pawn, color: White });
            board_state[0][file] = Some(Piece { kind, color: White });
        }

        Board { board_state, turn: White, en_passant: None }
    }

    pub fn _add_piece(&mut self, coordinate_x: usize, coordinate_y: usize, piece: Piece) -> &mut Self {
        self.board_state[coordinate_y][coordinate_x] = Some(piece);
        self
    }

    // "8/8/8/8/8/8/8/N7 w - - 0 1"
    pub fn from_fen(fen: &str) -> Board {
        let mut board_state = [[None; 8]; 8];
        let mut fen_component = fen.split(" ");
        let placement = fen_component.next().unwrap(); // get the first part of string

        for (i, rank_str) in placement.split("/").enumerate() {
            let rank = 7 - i;
            let mut file: usize = 0;
            for c in rank_str.chars() {
                if c.is_digit(10) {
                    file += c.to_digit(10).unwrap() as usize;
                } else {
                    board_state[rank][file] = Some(Kind::char_to_piece(c));
                    file += 1;
                }
            }
        }

        let turn_str = fen_component.next().unwrap(); // white or black
        let turn = match turn_str {
            "w" => White,
            "b" => Black,
            _ => panic!("Unknown turn state.")
        };

        let _castling = fen_component.next().unwrap();
        let en_passant_str = fen_component.next().unwrap();

        // Translate en_passant to board coordinate
        let en_passant = match en_passant_str {
            "-" => None,
            _ => {
                square_to_coordinate(en_passant_str)
            }
        };

        Board { board_state, turn, en_passant }
    }

    fn piece_moves(&self, piece: Piece, starting_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let deltas = piece.kind.move_delta();
        let slider = piece.kind.is_slider();
        let (start_x, start_y) = starting_pos;
        let mut possible_moves: Vec<(usize, usize)> = Vec::new();

        for (dx, dy) in deltas {
            let mut new_x = start_x as i32 + dx;
            let mut new_y = start_y as i32 + dy;
            while (0..8).contains(&new_x) && (0..8).contains(&new_y) {
                // If blocked by something
                if let Some(target_piece) = self.board_state[new_y as usize][new_x as usize] {
                    if target_piece.color == piece.color {
                        break
                    } else {
                        possible_moves.push((new_x as usize, new_y as usize));
                        break
                    }
                }
                possible_moves.push((new_x as usize, new_y as usize));
                // if not slider, stop checking
                if slider == false {
                    break
                }
                new_x = new_x + dx;
                new_y = new_y + dy;
            }
        }
        possible_moves
    }

    fn pawn_moves(&self, piece: Piece, starting_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut possible_moves: Vec<(usize, usize)> = Vec::new();
        let (start_x, start_y) = starting_pos;
        let (dy, start_rank) = match piece.color {
            Black => {
                (-1i32, 6)
            }
            White => {
                (1, 1)
            }
        };

        let new_y = (start_y as i32 + dy) as usize;

        // Diagonal captures
        for dx in [-1i32, 1] {
            // Not in range of the board
            if !(0..8).contains(&(dx + start_x as i32)) || !(0..8).contains(&(dy + start_y as i32)) {
                println!("Not in range!");
                break
            }

            let new_x = (start_x as i32 + dx) as usize;

            if let Some(p) = self.board_state[new_y][new_x] {
                if p.color != piece.color {
                    possible_moves.push((new_x, new_y));
                }
            } else if self.en_passant == Some((new_x, new_y)) {
                possible_moves.push((new_x, new_y));
            }
        }

        // Front
        // if not blocked
        let None = self.board_state[new_y][start_x] else {
            return possible_moves;
        };

        possible_moves.push((start_x, new_y));

        let new_2y = (start_y as i32 + 2 * dy) as usize;

        // Two steps ahead
        if let Some(_p) = self.board_state[new_2y][start_x] {
            return possible_moves;
        } else {
            if start_y == start_rank {
                possible_moves.push((start_x, new_2y));
            }
        };

        possible_moves
    }

    fn pseudo_legal_moves(&self, from: (usize, usize)) -> Vec<PieceMove> {
        let (start_x, start_y) = from;

        let piece = self.board_state[start_y][start_x].unwrap();

        let possible_moves = match piece.kind {
            Pawn => self.pawn_moves(piece, from),
            _ => self.piece_moves(piece, from),
        };

        for m in &possible_moves {
            println!("{}", coordinate_to_square(m));
        }

        possible_moves.into_iter().map(|m| {
            PieceMove { from, to: m, piece }
        }).collect()
    }

    // filters out all the legal moves
    fn legal_moves(&mut self, from: (usize, usize)) -> Vec<PieceMove> {
        let pseudo = self.pseudo_legal_moves(from);
        pseudo.into_iter().filter(|m| {
            let mut cloned_board = self.clone();
            cloned_board.apply_move(*m);
            !cloned_board.is_in_check(self.turn)
        }).collect()
    }

    pub fn make_move(&mut self, starting_pos: (usize, usize), target_pos: (usize, usize)) {
        let (start_x, start_y) = starting_pos;

        let Some(piece) = self.board_state[start_y][start_x] else {
            println!("Invalid starting position!");
            return
        };

        if piece.color != self.turn {
            println!("Invalid move!");
            return
        }

        let target_move = PieceMove { from: starting_pos, to: target_pos, piece };
        let legal = self.legal_moves(starting_pos);

        if legal.contains(&target_move) {
            self.apply_move(target_move)
        } else {
            println!("Invalid move!");
        }
    }

    fn apply_move(&mut self, piece_move: PieceMove) {
        let (start_x, start_y) = piece_move.from;
        let (target_x, target_y) = piece_move.to;
        if self.board_state[target_y][target_x] != None {
            // Capture logic, maybe push them into an array in the future?
            let _captured_piece = self.board_state[target_y][target_x].take().unwrap();
        }
        if piece_move.piece.kind == Pawn && Some((target_x, target_y)) == self.en_passant {
            self.board_state[start_y][target_x] = None;
        }
        if piece_move.piece.kind == Pawn && (target_y as i32 - start_y as i32).abs() == 2 {
            self.en_passant = Some((start_x, (start_y + target_y) / 2));
        } else {
            self.en_passant = None; // clear en_passant
        }

        self.board_state[start_y][start_x] = None; // Take the piece
        self.board_state[target_y][target_x] = Some(piece_move.piece); // And place it
    }

    fn find_king(&self, color: Color) -> Option<(usize, usize)> {
        for rank in 0..8 {
            for file in 0..8 {
                let Some(piece) = self.board_state[rank][file] else {
                    break
                };
                if piece.kind != King && piece.color == color {
                    break
                } else {
                    return Some((file, rank))
                }
            }
        }
        None
    }

    // Check if coordinate is being attacked.
    pub fn is_attacked(&self, coordinate: (usize, usize), color: Color) -> bool {
        // scan every cell for opposite pieces
        for rank in 0..8 {
            for file in 0..8 {
                let Some(piece) = self.board_state[rank][file] else {
                    break
                };
                if piece.color != color {
                    let possible_moves = match piece.kind {
                        Pawn => self.pawn_moves(piece, (file, rank)),
                        _ => self.piece_moves(piece, (file, rank)),
                    };
                    if possible_moves.contains(&coordinate) {
                        return true
                    }
                }
            }
        }
        false
    }
    
    fn is_in_check(&self, color: Color) -> bool {
        match self.find_king(color) {
            None => false,
            Some(k) => self.is_attacked(k, !color),
        }
    }

    pub fn print_board(&self) {
        println!("Board: \n{}", self);
    }
}