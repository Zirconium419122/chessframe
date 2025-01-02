use std::fs::File;

use crate::gen_tables::*;

mod bitboard;
mod color;
mod error;
mod file;
mod gen_tables;
mod piece;
mod rank;
mod square;

fn main() {
    println!("cargo::rerun-if-changed=src/build.rs");

    let mut file = File::create("src/tables.rs").unwrap();

    write_files(&mut file);

    write_ranks(&mut file);

    write_between(&mut file);

    write_tangent(&mut file);

    write_pawn_moves(&mut file);

    write_pawn_attacks(&mut file);

    write_knight_moves(&mut file);

    write_bishop_moves(&mut file);

    write_rook_moves(&mut file);

    write_king_moves(&mut file);

    write_zobrist(&mut file);
}
