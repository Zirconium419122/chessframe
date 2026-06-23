use std::io::Write;
use std::{arch::x86_64::_pext_u64, fs::File};

use crate::{
    bitboard::{BitBoard, EMPTY},
    gen_tables::magic::{
        generate_bishop_moves, generate_rook_mask, generate_rook_moves, subsets, MagicPiece,
    },
    square::{Square, SQUARES},
};

#[allow(unused)]
#[derive(Debug)]
struct PextEntry {
    mask: BitBoard,
    offset: u32,
}

fn pext_index(mask: BitBoard, blockers: BitBoard) -> usize {
    unsafe { _pext_u64(blockers.0, mask.0) as usize }
}

fn generate_table(piece: MagicPiece, square: Square) -> (BitBoard, Vec<BitBoard>) {
    let mask = match piece {
        MagicPiece::Bishop => generate_bishop_moves(square, EMPTY) & BitBoard(0x007e7e7e7e7e7e00),
        MagicPiece::Rook => generate_rook_mask(square),
    };

    let mut table = vec![EMPTY; 1 << mask.count_ones()];

    for blockers in subsets(mask) {
        let moves = match piece {
            MagicPiece::Bishop => generate_bishop_moves(square, blockers),
            MagicPiece::Rook => generate_rook_moves(square, blockers),
        };

        let index = pext_index(mask, blockers);
        table[index] = moves;
    }

    (mask, table)
}

fn generate_tables(piece: MagicPiece) -> Vec<(BitBoard, Vec<BitBoard>)> {
    SQUARES
        .iter()
        .map(|square| generate_table(piece, *square))
        .collect()
}

fn flatten_tables(tables: Vec<(BitBoard, Vec<BitBoard>)>) -> ([PextEntry; 64], Vec<BitBoard>) {
    let (masks, moves): (Vec<BitBoard>, Vec<Vec<BitBoard>>) = tables.iter().cloned().unzip();

    let mut offset = 0;

    let pext_entries: [PextEntry; 64] = masks
        .iter()
        .zip(moves.iter())
        .map(|(mask, moves)| {
            let entry = PextEntry {
                mask: *mask,
                offset,
            };
            offset += moves.len() as u32;
            entry
        })
        .collect::<Vec<PextEntry>>()
        .try_into()
        .unwrap();

    let moves = moves
        .iter()
        .flat_map(|moves| moves.iter().copied())
        .collect();

    (pext_entries, moves)
}

fn generate_pext_tables(piece: MagicPiece) -> ([PextEntry; 64], Vec<BitBoard>) {
    let tables = generate_tables(piece);
    flatten_tables(tables)
}

pub fn write_bishop_pext(f: &mut File) {
    let bishop_tables = generate_pext_tables(MagicPiece::Bishop);

    writeln!(
        f,
        "pub const BISHOP_PEXT_ENTRIES: [PextEntry; 64] = {:?};",
        bishop_tables.0,
    )
    .unwrap();

    writeln!(
        f,
        "pub static BISHOP_MOVES_TABLE: &[BitBoard] = &{:?};",
        bishop_tables.1,
    )
    .unwrap();
}

pub fn write_rook_pext(f: &mut File) {
    let rook_tables = generate_pext_tables(MagicPiece::Rook);

    writeln!(
        f,
        "pub const ROOK_PEXT_ENTRIES: [PextEntry; 64] = {:?};",
        rook_tables.0,
    )
    .unwrap();

    writeln!(
        f,
        "pub static ROOK_MOVES_TABLE: &[BitBoard] = &{:?};",
        rook_tables.1,
    )
    .unwrap();
}
