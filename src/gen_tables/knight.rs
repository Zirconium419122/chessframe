use std::io::Write;
use std::{fs::File, sync::Mutex};

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::SQUARES,
};

static KNIGHT_MOVES: Mutex<[BitBoard; 64]> = Mutex::new([EMPTY; 64]);

pub fn generate_knight_moves() {
    for src in SQUARES.iter() {
        let mut knight_moves = KNIGHT_MOVES.lock().unwrap();

        knight_moves[src.to_index()] = SQUARES
            .iter()
            .filter(|dest| {
                let src_rank = src.rank().to_index() as i8;
                let src_file = src.file().to_index() as i8;
                let dest_rank = dest.rank().to_index() as i8;
                let dest_file = dest.file().to_index() as i8;

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

pub fn write_knight_moves(f: &mut File) {
    generate_knight_moves();

    writeln!(
        f,
        "pub const KNIGHT_MOVES: [BitBoard; 64] = {:?};",
        KNIGHT_MOVES.lock().unwrap(),
    )
    .unwrap();
}
