use std::fs::File;
use std::io::Write;

use crate::{bitboard::{BitBoard, EMPTY}, square::SQUARES};

use super::helpers::{diagonal, orthagonal};

static mut TANGENT: [[BitBoard; 64]; 64] = [[EMPTY; 64]; 64];

pub fn generate_tangent() {
    for src in SQUARES.iter() {
        for dest in SQUARES.iter() {
            unsafe {
                TANGENT[src.to_index()][dest.to_index()] = SQUARES
                    .iter()
                    .filter(|test| {
                        if diagonal(*src, *dest) {
                            diagonal(*src, **test) && diagonal(*dest, **test)
                        } else if orthagonal(*src, *dest) {
                            orthagonal(*src, **test) && orthagonal(*dest, **test)
                        } else {
                            false
                        }
                    })
                    .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square))
            }
        }
    }
}

pub fn write_tangent(f: &mut File) {
    generate_tangent();

    unsafe {
        writeln!(
            f,
            "pub const TANGENT: [[BitBoard; 64]; 64] = {:?};",
            TANGENT,
        )
        .unwrap();
    }
}
