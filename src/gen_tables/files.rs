use std::fs::File;
use std::io::Write;

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::SQUARES,
};

static mut FILES: [BitBoard; 8] = [EMPTY; 8];
static mut ADJACENT_FILES: [BitBoard; 8] = [EMPTY; 8];

pub fn generate_files() {
    for i in 0..8 {
        unsafe {
            FILES[i] = SQUARES
                .iter()
                .filter(|x| x.get_file().to_index() == i)
                .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square))
        }
    }
}

pub fn generate_adjacent_files() {
    for i in 0..8 {
        unsafe {
            ADJACENT_FILES[i] = SQUARES
                .iter()
                .filter(|x| {
                    x.get_file().to_index() == i.wrapping_add(1)
                        || x.get_file().to_index() == i.wrapping_sub(1)
                })
                .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square))
        }
    }
}

pub fn write_files(f: &mut File) {
    generate_files();
    generate_adjacent_files();

    unsafe {
        writeln!(f, "pub const FILES: [BitBoard; 8] = {:?};", FILES,).unwrap();

        writeln!(
            f,
            "pub const ADJACENT_FILES: [BitBoard; 8] = {:?};",
            ADJACENT_FILES,
        )
        .unwrap();
    }
}
