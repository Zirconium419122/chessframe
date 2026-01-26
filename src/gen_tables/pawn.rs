use std::io::Write;
use std::{fs::File, sync::Mutex};

use crate::{
    bitboard::{BitBoard, EMPTY},
    color::COLORS,
    square::SQUARES,
};

static PAWN_MOVES: Mutex<[[BitBoard; 64]; 2]> = Mutex::new([[EMPTY; 64]; 2]);

static PAWN_ATTACKS: Mutex<[[BitBoard; 64]; 2]> = Mutex::new([[EMPTY; 64]; 2]);

pub fn generate_pawn_moves() {
    for color in COLORS.iter() {
        for src in SQUARES.iter() {
            let mut pawn_moves = PAWN_MOVES.lock().unwrap();

            if src.rank() == color.to_second_rank() {
                pawn_moves[color.to_index()][src.to_index()] =
                    BitBoard::from_square(src.wrapping_forward(*color))
                        | BitBoard::from_square(
                            src.wrapping_forward(*color).wrapping_forward(*color),
                        )
            } else if let Some(dest) = src.forward(*color) {
                pawn_moves[color.to_index()][src.to_index()] = BitBoard::from_square(dest)
            }
        }
    }
}

pub fn generate_pawn_attacks() {
    for color in COLORS.iter() {
        for src in SQUARES.iter() {
            let mut pawn_attacks = PAWN_ATTACKS.lock().unwrap();

            if let Some(i_need_a_good_name_for_this) = src.forward(*color) {
                if let Some(dest) = i_need_a_good_name_for_this.left() {
                    pawn_attacks[color.to_index()][src.to_index()] |=
                        BitBoard::from_square(dest)
                }
                if let Some(dest) = i_need_a_good_name_for_this.right() {
                    pawn_attacks[color.to_index()][src.to_index()] |=
                        BitBoard::from_square(dest)
                }
            }
        }
    }
}

pub fn write_pawn_moves(f: &mut File) {
    generate_pawn_moves();

    writeln!(
        f,
        "pub const PAWN_MOVES: [[BitBoard; 64]; 2] = {:?};",
        PAWN_MOVES.lock().unwrap(),
    )
    .unwrap();
}

pub fn write_pawn_attacks(f: &mut File) {
    generate_pawn_attacks();

    writeln!(
        f,
        "pub const PAWN_ATTACKS: [[BitBoard; 64]; 2] = {:?};",
        PAWN_ATTACKS.lock().unwrap(),
    )
    .unwrap();
}
