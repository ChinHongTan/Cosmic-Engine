use std::fmt;
use std::fmt::Formatter;
use crate::piece::{PieceEnum};

pub struct Board {
    board_state: [[Option<PieceEnum>; 8]; 8],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.board_state {
            for cell in row {
                let symbol = match cell {
                    Some(PieceEnum::Pawn(_)) => 'P',
                    Some(PieceEnum::King(_)) => 'K',
                    Some(PieceEnum::Knight(_)) => 'N',
                    Some(PieceEnum::Rook(_)) => 'R',
                    None => '.',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            board_state: [[None; 8]; 8],
        }
    }

    pub fn add_piece(&mut self, coordinate_x: usize, coordinate_y: usize, piece: PieceEnum) -> &mut Self {
        self.board_state[coordinate_y][coordinate_x] = Some(piece);
        self
    }

    pub fn make_move(&mut self, starting_pos: (usize, usize), target_pos: (usize, usize)) {
        let (start_x, start_y) = starting_pos;
        let (target_x, target_y) = target_pos;

        if let Some(piece) = self.board_state[start_y][start_x].take() {
            let mut possible_moves: Vec<(usize, usize)> = vec![];

            let (deltas, slider) = piece.properties();

            for (dx, dy) in deltas {
                let mut new_x = (start_x as i32 + dx) as usize;
                let mut new_y = (start_y as i32 + dy) as usize;
                if new_x >= 8 || new_y >= 8 {
                    println!("Not possible!");
                    continue;
                }
                possible_moves.push((new_x, new_y));

                if slider == true {
                    loop {
                        new_x = (new_x as i32 + dx) as usize;
                        new_y = (new_y as i32 + dy) as usize;

                        if new_x >= 8 || new_y >= 8 {
                            break;
                        }

                        possible_moves.push((new_x, new_y));
                    }
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
