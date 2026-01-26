use std::io::Write;
use std::{fs::File, sync::Mutex};

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::SQUARES,
};

use super::helpers::{between, diagonal, orthagonal};

static BETWEEN: Mutex<[[BitBoard; 64]; 64]> = Mutex::new([[EMPTY; 64]; 64]);

pub fn generate_between() {
    for src in SQUARES.iter() {
        for dest in SQUARES.iter() {
            let mut between_mutex = BETWEEN.lock().unwrap();

            between_mutex[src.to_index()][dest.to_index()] = SQUARES
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

pub fn write_between(f: &mut File) {
    generate_between();

    writeln!(
        f,
        "pub static BETWEEN: [[BitBoard; 64]; 64] = {:?};",
        BETWEEN.lock().unwrap(),
    )
    .unwrap();
}
