use std::fs::File;
use std::io::Write;

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::SQUARES,
};

use super::helpers::{between, diagonal, orthagonal};

static mut BETWEEN: [[BitBoard; 64]; 64] = [[EMPTY; 64]; 64];

pub fn generate_between() {
    for src in SQUARES.iter() {
        for dest in SQUARES.iter() {
            unsafe {
                BETWEEN[src.to_index()][dest.to_index()] = SQUARES
                    .iter()
                    .filter(|test| {
                        if diagonal(*src, *dest) && src != dest {
                            diagonal(*src, **test)
                                && diagonal(*dest, **test)
                                && between(*src, *dest, **test)
                        } else if orthagonal(*src, *dest) && src != dest {
                            orthagonal(*src, **test)
                                && orthagonal(*dest, **test)
                                && between(*src, *dest, **test)
                        } else {
                            false
                        }
                    })
                    .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square))
            }
        }
    }
}

pub fn write_between(f: &mut File) {
    generate_between();

    unsafe {
        writeln!(
            f,
            "pub const BETWEEN: [[BitBoard; 64]; 64] = {:?};",
            BETWEEN,
        )
        .unwrap();
    }
}
