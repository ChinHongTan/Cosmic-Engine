use std::fmt;
use std::fmt::Formatter;
use crate::piece::{Color, Kind, Piece};
use crate::piece::Kind::{Rook, Knight, Bishop, Queen, King, Pawn};

pub struct Board {
    board_state: [[Option<Piece>; 8]; 8],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.board_state {
            for cell in row {
                let symbol = match cell {
                    Some(piece) => piece.kind.symbol(),
                    None => '.',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Back rank predefined
const BACK_RANK: [Kind; 8] = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];

impl Board {
    pub fn new() -> Board {
        let mut board_state = [[None; 8]; 8];
        for file in 0..8 {
            let kind = BACK_RANK[file];
            board_state[7][file] = Some(Piece { kind, color: Color::Black });
            board_state[6][file] = Some(Piece { kind: Pawn, color: Color::Black });
            board_state[1][file] = Some(Piece { kind: Pawn, color: Color::White });
            board_state[0][file] = Some(Piece { kind, color: Color::White });
        }

        Board { board_state }
    }

    pub fn add_piece(&mut self, coordinate_x: usize, coordinate_y: usize, piece: Piece) -> &mut Self {
        self.board_state[coordinate_y][coordinate_x] = Some(piece);
        self
    }

    pub fn make_move(&mut self, starting_pos: (usize, usize), target_pos: (usize, usize)) {
        let (start_x, start_y) = starting_pos;
        let (target_x, target_y) = target_pos;

        if let Some(piece) = self.board_state[start_y][start_x].take() {
            let mut possible_moves: Vec<(usize, usize)> = vec![];

            let deltas = piece.kind.move_delta();
            let slider = piece.kind.is_slider();

            for (dx, dy) in deltas {
                let mut new_x = start_x as i32 + dx;
                let mut new_y = start_y as i32 + dy;
                while (0..8).contains(&new_x) && (0..8).contains(&new_y) {
                    // if not slider
                    if slider == false {
                        break
                    }
                    // If blocked by something
                    if let Some(target_piece) = self.board_state[new_y as usize][new_x as usize] {
                        if (target_piece.color == piece.color) {
                            break
                        } else {
                            possible_moves.push((new_x as usize, new_y as usize));
                            break
                        }
                    }
                    possible_moves.push((new_x as usize, new_y as usize));
                    new_x = new_x + dx;
                    new_y = new_y + dy;
                }
            }

            println!("{:?}", possible_moves);

            if possible_moves.contains(&target_pos) {
                self.board_state[target_y][target_x] = Some(piece);
            } else {
                println!("Invalid move!");
                self.board_state[start_y][start_x] = Some(piece);  // Do not move
            }

        }
    }

    pub fn print_board(&self) {
        println!("Board: \n{}", self);
    }
}
