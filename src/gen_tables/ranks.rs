use std::io::Write;
use std::{fs::File, sync::Mutex};

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::SQUARES,
};

pub static RANKS: Mutex<[BitBoard; 8]> = Mutex::new([EMPTY; 8]);

pub static FORWARD_RANKS: Mutex<[[BitBoard; 8]; 2]> = Mutex::new([[EMPTY; 8]; 2]);

pub static BACKWARD_RANKS: Mutex<[[BitBoard; 8]; 2]> = Mutex::new([[EMPTY; 8]; 2]);

pub fn generate_ranks() {
    for i in 0..8 {
        let mut ranks = RANKS.lock().unwrap();

        ranks[i] = SQUARES
            .iter()
            .filter(|x| x.rank().to_index() == i)
            .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square));
    }
}

pub fn generate_forward_ranks() {
    for i in 0..8 {
        let mut forward_ranks = FORWARD_RANKS.lock().unwrap();

        forward_ranks[0][i] = SQUARES
            .iter()
            .filter(|x| x.rank().to_index() > i)
            .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square));

        forward_ranks[1][i] = SQUARES
            .iter()
            .filter(|x| x.rank().to_index() < i)
            .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square));
    }
}

pub fn generate_backward_ranks() {
    for i in 0..8 {
        let mut backward_ranks = BACKWARD_RANKS.lock().unwrap();

        backward_ranks[0][i] = SQUARES
            .iter()
            .filter(|x| x.rank().to_index() < i)
            .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square));

        backward_ranks[1][i] = SQUARES
            .iter()
            .filter(|x| x.rank().to_index() > i)
            .fold(EMPTY, |acc, square| acc | BitBoard::from_square(*square));
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

pub fn write_forward_ranks(f: &mut File) {
    generate_forward_ranks();

    writeln!(
        f,
        "pub const FORWARD_RANKS: [[BitBoard; 8]; 2] = {:?};",
        FORWARD_RANKS.lock().unwrap(),
    )
    .unwrap();
}

pub fn write_backward_ranks(f: &mut File) {
    generate_backward_ranks();

    writeln!(
        f,
        "pub const BACKWARD_RANKS: [[BitBoard; 8]; 2] = {:?};",
        BACKWARD_RANKS.lock().unwrap(),
    )
    .unwrap();
}
