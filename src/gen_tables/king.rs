use std::fs::File;
use std::io::Write;

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::{Square, SQUARES},
};

static mut KING_MOVES: [BitBoard; 64] = [EMPTY; 64];
static mut CASTLE_MOVES: BitBoard = EMPTY;

pub fn generate_king_moves() {
    for src in SQUARES.iter() {
        unsafe {
            KING_MOVES[src.to_index()] = SQUARES
                .iter()
                .filter(|dest| {
                    let src_rank = src.rank().to_index() as i8;
                    let src_file = src.file().to_index() as i8;
                    let dest_rank = dest.rank().to_index() as i8;
                    let dest_file = dest.file().to_index() as i8;

                    if (src_rank - dest_rank).abs() <= 1 && (src_file - dest_file).abs() <= 1 {
                        *src != **dest
                    } else {
                        false
                    }
                })
                .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square))
        }
    }
}

pub fn generate_castle_moves() {
    unsafe {
        CASTLE_MOVES = BitBoard::from_square(Square::C1)
            | BitBoard::from_square(Square::C8)
            | BitBoard::from_square(Square::E1)
            | BitBoard::from_square(Square::E8)
            | BitBoard::from_square(Square::G1)
            | BitBoard::from_square(Square::G8)
    }
}

pub fn write_king_moves(f: &mut File) {
    generate_king_moves();
    generate_castle_moves();

    unsafe {
        writeln!(
            f,
            "pub const KING_MOVES: [BitBoard; 64] = {:?};",
            KING_MOVES
        )
        .unwrap();

        writeln!(f, "pub const CASTLE_MOVES: BitBoard = {:?};", CASTLE_MOVES).unwrap();
    }
}
