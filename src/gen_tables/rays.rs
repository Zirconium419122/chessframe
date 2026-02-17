use std::io::Write;
use std::{fs::File, sync::Mutex};

use crate::gen_tables::helpers::orthagonal;
use crate::{bitboard::{BitBoard, EMPTY}, square::SQUARES};

use super::helpers::diagonal;

static BISHOP_RAYS: Mutex<[BitBoard; 64]> = Mutex::new([EMPTY; 64]);
static ROOK_RAYS: Mutex<[BitBoard; 64]> = Mutex::new([EMPTY; 64]);

pub fn generate_bishop_rays() {
    for src in SQUARES.iter() {
        let mut bishop_rays = BISHOP_RAYS.lock().unwrap();

        bishop_rays[src.to_index()] = SQUARES
            .iter()
            .filter(|dest| {
                diagonal(*src, **dest) && src != *dest
            })
            .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square));
    }
}

pub fn generate_rook_rays() {
    for src in SQUARES.iter() {
        let mut rook_rays = ROOK_RAYS.lock().unwrap();

        rook_rays[src.to_index()] = SQUARES
            .iter()
            .filter(|dest| {
                orthagonal(*src, **dest) && src != *dest
            })
            .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square));
    }
}

pub fn write_bishop_rays(f: &mut File) {
    generate_bishop_rays();

    writeln!(
        f,
        "pub static BISHOP_RAYS: [BitBoard; 64] = {:?};",
        BISHOP_RAYS.lock().unwrap(),
    )
    .unwrap();
}

pub fn write_rook_rays(f: &mut File) {
    generate_rook_rays();

    writeln!(
        f,
        "pub static ROOK_RAYS: [BitBoard; 64] = {:?};",
        ROOK_RAYS.lock().unwrap(),
    )
    .unwrap();
}