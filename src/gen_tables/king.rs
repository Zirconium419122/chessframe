use std::io::Write;
use std::{fs::File, sync::Mutex};

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::{Square, SQUARES},
};

static KING_MOVES: Mutex<[BitBoard; 64]> = Mutex::new([EMPTY; 64]);
static CASTLE_MOVES: Mutex<BitBoard> = Mutex::new(EMPTY);

pub fn generate_king_moves() {
    for src in SQUARES.iter() {
        let mut king_moves = KING_MOVES.lock().unwrap();

        king_moves[src.to_index()] = SQUARES
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

pub fn generate_castle_moves() {
    let mut castle_moves = CASTLE_MOVES.lock().unwrap();

    *castle_moves = BitBoard::from_square(Square::C1)
        | BitBoard::from_square(Square::C8)
        | BitBoard::from_square(Square::E1)
        | BitBoard::from_square(Square::E8)
        | BitBoard::from_square(Square::G1)
        | BitBoard::from_square(Square::G8)
}

pub fn write_king_moves(f: &mut File) {
    generate_king_moves();
    generate_castle_moves();

    writeln!(
        f,
        "pub const KING_MOVES: [BitBoard; 64] = {:?};",
        KING_MOVES.lock().unwrap(),
    )
    .unwrap();

    writeln!(
        f,
        "pub const CASTLE_MOVES: BitBoard = {:?};",
        CASTLE_MOVES.lock().unwrap(),
    )
    .unwrap();
}
