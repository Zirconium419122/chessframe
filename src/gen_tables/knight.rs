use std::fs::File;
use std::io::Write;

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::SQUARES,
};

static mut KNIGHT_MOVES: [BitBoard; 64] = [EMPTY; 64];

pub fn generate_knight_moves() {
    for src in SQUARES.iter() {
        unsafe {
            KNIGHT_MOVES[src.to_index()] = SQUARES
                .iter()
                .filter(|dest| {
                    let src_rank = src.get_rank().to_index() as i8;
                    let src_file = src.get_file().to_index() as i8;
                    let dest_rank = dest.get_rank().to_index() as i8;
                    let dest_file = dest.get_file().to_index() as i8;

                    if (src_rank - dest_rank).abs() == 2 && (src_file - dest_file).abs() == 1 {
                        return true;
                    }

                    if (src_rank - dest_rank).abs() == 1 && (src_file - dest_file).abs() == 2 {
                        return true;
                    }

                    false
                })
                .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square))
        }
    }
}

pub fn write_knight_moves(f: &mut File) {
    generate_knight_moves();

    unsafe {
        writeln!(
            f,
            "pub const KNIGHT_MOVES: [BitBoard; 64] = {:?};",
            KNIGHT_MOVES
        )
        .unwrap();
    }
}
