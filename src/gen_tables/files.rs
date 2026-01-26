use std::io::Write;
use std::{fs::File, sync::Mutex};

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::SQUARES,
};

static FILES: Mutex<[BitBoard; 8]> = Mutex::new([EMPTY; 8]);
static ADJACENT_FILES: Mutex<[BitBoard; 8]> = Mutex::new([EMPTY; 8]);

pub fn generate_files() {
    for i in 0..8 {
        let mut files = FILES.lock().unwrap();

        files[i] = SQUARES
            .iter()
            .filter(|x| x.file().to_index() == i)
            .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square))
    }
}

pub fn generate_adjacent_files() {
    for i in 0..8 {
        let mut adjacent_files = ADJACENT_FILES.lock().unwrap();

        adjacent_files[i] = SQUARES
            .iter()
            .filter(|x| {
                x.file().to_index() == i.wrapping_add(1)
                    || x.file().to_index() == i.wrapping_sub(1)
            })
            .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square))
    }
}

pub fn write_files(f: &mut File) {
    generate_files();
    generate_adjacent_files();

    writeln!(
        f,
        "pub const FILES: [BitBoard; 8] = {:?};",
        FILES.lock().unwrap(),
    )
    .unwrap();

    writeln!(
        f,
        "pub const ADJACENT_FILES: [BitBoard; 8] = {:?};",
        ADJACENT_FILES.lock().unwrap(),
    )
    .unwrap();
}
