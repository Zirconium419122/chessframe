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

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR should be set");
    let mut file = File::create(format!("{}/tables.rs", out_dir)).unwrap();

    write_files(&mut file);

    write_ranks(&mut file);

    write_between(&mut file);

    write_tangent(&mut file);

    write_pawn_moves(&mut file);

    write_pawn_attacks(&mut file);

    write_knight_moves(&mut file);

    write_king_moves(&mut file);

    write_bishop_rays(&mut file);

    write_rook_rays(&mut file);

    write_zobrist(&mut file);

    if OpenOptions::new()
        .read(true)
        .open(format!("{}/magic_tables.rs", out_dir))
        .is_err()
    {
        let mut file = File::create(format!("{}/magic_tables.rs", out_dir)).unwrap();

        write_bishop_moves(&mut file);

        write_rook_moves(&mut file);
    } else if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .open(format!("{}/magic_tables.rs", out_dir))
        && file.metadata().expect("file metadata not found").len() == 0
    {
        let reader = BufReader::new(
            OpenOptions::new()
                .read(true)
                .open(format!("{}/magic_tables.rs", out_dir))
                .unwrap(),
        );
        if reader.lines().count() != 4 {
            write_bishop_moves(&mut file);

            write_rook_moves(&mut file);
        }
    }
}
