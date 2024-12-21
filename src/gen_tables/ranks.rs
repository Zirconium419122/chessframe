use std::fs::File;
use std::io::Write;

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::SQUARES,
};

static mut RANKS: [BitBoard; 8] = [EMPTY; 8];

pub fn generate_ranks() {
    for i in 0..8 {
        unsafe {
            RANKS[i] = SQUARES
                .iter()
                .filter(|x| x.get_rank().to_index() == i)
                .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square))
        }
    }
}

pub fn write_ranks(f: &mut File) {
    generate_ranks();

    unsafe {
        writeln!(f, "pub const RANKS: [BitBoard; 8] = {:?};", RANKS,).unwrap();
    }
}
