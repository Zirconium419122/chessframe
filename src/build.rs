use std::fs::File;

use crate::gen_tables::*;

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

    write_knight_moves(&mut file);

    write_bishop_moves(&mut file);

    write_rook_moves(&mut file);

    write_king_moves(&mut file);
}
