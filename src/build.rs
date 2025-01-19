use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

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

    write_king_moves(&mut file);

    write_zobrist(&mut file);

    if OpenOptions::new()
        .read(true)
        .open("src/magic_tables.rs")
        .is_err()
    {
        let mut file = File::create("src/magic_tables.rs").unwrap();

        write_bishop_moves(&mut file);

        write_rook_moves(&mut file);
    } else if let Ok(mut file) = OpenOptions::new().write(true).open("src/magic_tables.rs") {
        if file.metadata().expect("file metadata not found").len() == 0 {
            let reader = BufReader::new(
                OpenOptions::new()
                    .read(true)
                    .open("src/magic_tables.rs")
                    .unwrap(),
            );
            if reader.lines().count() != 4 {
                write_bishop_moves(&mut file);

                write_rook_moves(&mut file);
            }
        }
    }
}
