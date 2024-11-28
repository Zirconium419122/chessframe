use std::fs::File;

use crate::gen_tables::{write_pawn_attacks, write_pawn_moves, write_bishop_moves, write_king_moves, write_rook_moves};

mod bitboard;
mod color;
mod error;
mod file;
mod gen_tables;
mod rank;
mod square;

fn main() {
    println!("cargo::rerun-if-changed=src/build.rs");

    let mut file = File::create("src/tables.rs").unwrap();

    write_pawn_moves(&mut file);

    write_pawn_attacks(&mut file);

    write_bishop_moves(&mut file);

    write_rook_moves(&mut file);

    write_king_moves(&mut file);
}
