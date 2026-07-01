use std::fs::{File, OpenOptions};
#[cfg(not(feature = "bmi2"))]
use std::thread;

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

    write_forward_ranks(&mut file);

    write_backward_ranks(&mut file);

    write_between(&mut file);

    write_tangent(&mut file);

    write_pawn_moves(&mut file);

    write_pawn_attacks(&mut file);

    write_passed_pawn(&mut file);

    write_knight_moves(&mut file);

    write_king_moves(&mut file);

    write_bishop_rays(&mut file);

    write_rook_rays(&mut file);

    write_zobrist(&mut file);

    let path = format!("{}/magic_tables.rs", out_dir);

    let mut file = if OpenOptions::new().read(true).open(&path).is_err() {
        File::create(&path).unwrap()
    } else {
        OpenOptions::new().write(true).open(&path).unwrap()
    };

    if file.metadata().expect("file metadata not found").len() != 4 {
        #[cfg(not(feature = "bmi2"))]
        {
            let mut bishop_file = file.try_clone().unwrap();
            let bishop_thread = thread::spawn(move || {
                write_bishop_moves(&mut bishop_file);
            });
            let rook_thread = thread::spawn(move || {
                write_rook_moves(&mut file);
            });

            let _ = bishop_thread.join();
            let _ = rook_thread.join();
        }

        #[cfg(feature = "bmi2")]
        {
            write_bishop_pext(&mut file);

            write_rook_pext(&mut file);
        }
    }
}
