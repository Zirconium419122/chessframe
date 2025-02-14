use std::fs::File;
use std::io::Write;

use rand_chacha::rand_core::{RngCore, SeedableRng};

use crate::{color::COLORS, piece::PIECES, square::SQUARES};

static mut ZOBRIST_SIDE_TO_MOVE: u64 = 0;
static mut ZOBRIST_PIECES: [[[u64; 64]; 6]; 2] = [[[0; 64]; 6]; 2];
static mut ZOBRIST_CASTLE: [[u64; 4]; 2] = [[0; 4]; 2];
static mut ZOBRIST_ENPASSANT: [[u64; 8]; 2] = [[0; 8]; 2];

pub fn generate_zobrist_side_to_move() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123456789);

    unsafe {
        ZOBRIST_SIDE_TO_MOVE = rng.next_u64();
    }
}

pub fn generate_zobrist_pieces() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(234567890);

    for (color, _) in COLORS.iter().enumerate() {
        for (piece, _) in PIECES.iter().enumerate() {
            for (square, _) in SQUARES.iter().enumerate() {
                unsafe {
                    ZOBRIST_PIECES[color][piece][square] = rng.next_u64();
                }
            }
        }
    }
}

pub fn generate_zobrist_castle() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(345678901);

    for (color, _) in COLORS.iter().enumerate() {
        for i in 0..4 {
            unsafe {
                ZOBRIST_CASTLE[color][i] = rng.next_u64();
            }
        }
    }
}

pub fn generate_zobrist_enpassant() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(456789012);

    for (color, _) in COLORS.iter().enumerate() {
        for i in 0..8 {
            unsafe {
                ZOBRIST_ENPASSANT[color][i] = rng.next_u64();
            }
        }
    }
}

pub fn write_zobrist_side_to_move(f: &mut File) {
    generate_zobrist_side_to_move();

    unsafe {
        writeln!(
            f,
            "pub const ZOBRIST_SIDE_TO_MOVE: u64 = {};",
            ZOBRIST_SIDE_TO_MOVE
        )
        .unwrap();
    }
}

pub fn write_zobrist_pieces(f: &mut File) {
    generate_zobrist_pieces();

    unsafe {
        writeln!(
            f,
            "pub const ZOBRIST_PIECES: [[[u64; 64]; 6]; 2] = {:?};",
            ZOBRIST_PIECES
        )
        .unwrap();
    }
}

pub fn write_zobrist_castle(f: &mut File) {
    generate_zobrist_castle();

    unsafe {
        writeln!(
            f,
            "pub const ZOBRIST_CASTLE: [[u64; 4]; 2] = {:?};",
            ZOBRIST_CASTLE,
        )
        .unwrap();
    }
}

pub fn write_zobrist_enpassant(f: &mut File) {
    generate_zobrist_enpassant();

    unsafe {
        writeln!(
            f,
            "pub const ZOBRIST_ENPASSANT: [[u64; 8]; 2] = {:?};",
            ZOBRIST_ENPASSANT,
        )
        .unwrap();
    }
}

pub fn write_zobrist(f: &mut File) {
    write_zobrist_side_to_move(f);
    write_zobrist_pieces(f);
    write_zobrist_castle(f);
    write_zobrist_enpassant(f);
}
