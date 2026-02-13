use std::io::Write;
use std::{fs::File, sync::Mutex};

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::SQUARES,
};

use super::helpers::{diagonal, orthagonal};

static TANGENT: Mutex<[[BitBoard; 64]; 64]> = Mutex::new([[EMPTY; 64]; 64]);

pub fn generate_tangent() {
    for src in SQUARES.iter() {
        for dest in SQUARES.iter() {
            let mut tangent = TANGENT.lock().unwrap();

            tangent[src.to_index()][dest.to_index()] = SQUARES
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
                .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square));

            if src == dest {
                tangent[src.to_index()][dest.to_index()] = EMPTY;
            }
        }
    }
}

pub fn write_tangent(f: &mut File) {
    generate_tangent();

    writeln!(
        f,
        "pub static TANGENT: [[BitBoard; 64]; 64] = {:?};",
        TANGENT.lock().unwrap(),
    )
    .unwrap();
}
