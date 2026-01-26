use std::io::Write;
use std::{fs::File, sync::Mutex};

use rand_chacha::rand_core::{RngCore, SeedableRng};

use crate::{color::COLORS, piece::PIECES, square::SQUARES};

static ZOBRIST_SIDE_TO_MOVE: Mutex<u64> = Mutex::new(0);
static ZOBRIST_PIECES: Mutex<[[[u64; 64]; 6]; 2]> = Mutex::new([[[0; 64]; 6]; 2]);
static ZOBRIST_CASTLE: Mutex<[[u64; 4]; 2]> = Mutex::new([[0; 4]; 2]);
static ZOBRIST_ENPASSANT: Mutex<[[u64; 8]; 2]> = Mutex::new([[0; 8]; 2]);

pub fn generate_zobrist_side_to_move() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123456789);

    let mut zobrist_side_to_move = ZOBRIST_SIDE_TO_MOVE.lock().unwrap();
    *zobrist_side_to_move = rng.next_u64();
}

pub fn generate_zobrist_pieces() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(234567890);

    for (color, _) in COLORS.iter().enumerate() {
        for (piece, _) in PIECES.iter().enumerate() {
            for (square, _) in SQUARES.iter().enumerate() {
                let mut zobrist_pieces = ZOBRIST_PIECES.lock().unwrap();
                zobrist_pieces[color][piece][square] = rng.next_u64();
            }
        }
    }
}

pub fn generate_zobrist_castle() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(345678901);

    for (color, _) in COLORS.iter().enumerate() {
        for i in 0..4 {
            let mut zobrist_castle = ZOBRIST_CASTLE.lock().unwrap();
            zobrist_castle[color][i] = rng.next_u64();
        }
    }
}

pub fn generate_zobrist_enpassant() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(456789012);

    for (color, _) in COLORS.iter().enumerate() {
        for i in 0..8 {
            let mut zobrist_enpassant = ZOBRIST_ENPASSANT.lock().unwrap();
            zobrist_enpassant[color][i] = rng.next_u64();
        }
    }
}

pub fn write_zobrist_side_to_move(f: &mut File) {
    generate_zobrist_side_to_move();

    writeln!(
        f,
        "pub const ZOBRIST_SIDE_TO_MOVE: u64 = {};",
        ZOBRIST_SIDE_TO_MOVE.lock().unwrap(),
    )
    .unwrap();
}

pub fn write_zobrist_pieces(f: &mut File) {
    generate_zobrist_pieces();

    writeln!(
        f,
        "pub const ZOBRIST_PIECES: [[[u64; 64]; 6]; 2] = {:?};",
        ZOBRIST_PIECES.lock().unwrap(),
    )
    .unwrap();
}

pub fn write_zobrist_castle(f: &mut File) {
    generate_zobrist_castle();

    writeln!(
        f,
        "pub const ZOBRIST_CASTLE: [[u64; 4]; 2] = {:?};",
        ZOBRIST_CASTLE.lock().unwrap(),
    )
    .unwrap();
}

pub fn write_zobrist_enpassant(f: &mut File) {
    generate_zobrist_enpassant();

    writeln!(
        f,
        "pub const ZOBRIST_ENPASSANT: [[u64; 8]; 2] = {:?};",
        ZOBRIST_ENPASSANT.lock().unwrap(),
    )
    .unwrap();
}

pub fn write_zobrist(f: &mut File) {
    write_zobrist_side_to_move(f);
    write_zobrist_pieces(f);
    write_zobrist_castle(f);
    write_zobrist_enpassant(f);
}
