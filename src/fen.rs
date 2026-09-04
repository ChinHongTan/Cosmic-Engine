use crate::board::Board;
use crate::castling::{str_to_castling, CastlingRights};
use crate::piece::Color::{Black, White};
use crate::piece::{Piece};
use crate::square::square_to_coordinate;

impl Board {
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
                    board_state[rank][file] = Some(Piece::char_to_piece(c));
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

        let castling_str = fen_component.next().unwrap();
        let castling = match castling_str {
            "-" => CastlingRights([false; 4]),
            _ => {
                str_to_castling(castling_str)
            }
        };

        let en_passant_str = fen_component.next().unwrap();

        // Translate en_passant to board coordinate
        let en_passant = match en_passant_str {
            "-" => None,
            _ => {
                square_to_coordinate(en_passant_str)
            }
        };

        let halfmove = fen_component.next().unwrap().parse::<u32>().unwrap();

        let fullmove = fen_component.next().unwrap().parse::<u32>().unwrap();

        Board { board_state, turn, en_passant, castling, halfmove, fullmove, ..Default::default() }
            .finalize()
    }
}