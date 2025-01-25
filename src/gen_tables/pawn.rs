use std::fs::File;
use std::io::Write;

use crate::{
    bitboard::{BitBoard, EMPTY},
    color::COLORS,
    square::SQUARES,
};

static mut PAWN_MOVES: [[BitBoard; 64]; 2] = [[EMPTY; 64]; 2];

static mut PAWN_ATTACKS: [[BitBoard; 64]; 2] = [[EMPTY; 64]; 2];

pub fn generate_pawn_moves() {
    for color in COLORS.iter() {
        for src in SQUARES.iter() {
            unsafe {
                if src.rank() == color.to_second_rank() {
                    PAWN_MOVES[color.to_index()][src.to_index()] =
                        BitBoard::from_square(src.wrapping_forward(*color))
                            | BitBoard::from_square(
                                src.wrapping_forward(*color).wrapping_forward(*color),
                            )
                } else if let Some(dest) = src.forward(*color) {
                    PAWN_MOVES[color.to_index()][src.to_index()] = BitBoard::from_square(dest)
                }
            }
        }
    }
}

pub fn generate_pawn_attacks() {
    for color in COLORS.iter() {
        for src in SQUARES.iter() {
            unsafe {
                if let Some(i_need_a_good_name_for_this) = src.forward(*color) {
                    if let Some(dest) = i_need_a_good_name_for_this.left() {
                        PAWN_ATTACKS[color.to_index()][src.to_index()] |=
                            BitBoard::from_square(dest)
                    }
                    if let Some(dest) = i_need_a_good_name_for_this.right() {
                        PAWN_ATTACKS[color.to_index()][src.to_index()] |=
                            BitBoard::from_square(dest)
                    }
                }
            }
        }
    }
}

pub fn write_pawn_moves(f: &mut File) {
    generate_pawn_moves();

    unsafe {
        writeln!(
            f,
            "pub const PAWN_MOVES: [[BitBoard; 64]; 2] = {:?};",
            PAWN_MOVES
        )
        .unwrap();
    }
}

pub fn write_pawn_attacks(f: &mut File) {
    generate_pawn_attacks();

    unsafe {
        writeln!(
            f,
            "pub const PAWN_ATTACKS: [[BitBoard; 64]; 2] = {:?};",
            PAWN_ATTACKS
        )
        .unwrap();
    }
}
