use std::io::Write;
use std::{fs::File, sync::Mutex};

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::SQUARES,
};

static RANKS: Mutex<[BitBoard; 8]> = Mutex::new([EMPTY; 8]);

pub fn generate_ranks() {
    for i in 0..8 {
        let mut ranks = RANKS.lock().unwrap();

        ranks[i] = SQUARES
            .iter()
            .filter(|x| x.rank().to_index() == i)
            .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square))
    }
}

pub fn write_ranks(f: &mut File) {
    generate_ranks();

    writeln!(
        f,
        "pub const RANKS: [BitBoard; 8] = {:?};",
        RANKS.lock().unwrap(),
    )
    .unwrap();
}
