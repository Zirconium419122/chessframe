use std::fs::File;
use std::io::Write;

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::{Square, SQUARES},
};

#[allow(dead_code)]
enum MagicPiece {
    Bishop,
    Rook,
}

#[derive(Debug, Clone, Copy, Default)]
struct Magic {
    pub mask: BitBoard,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}

fn rook_directions() -> [fn(Square) -> Option<Square>; 4] {
    fn north(square: Square) -> Option<Square> {
        square.up()
    }
    fn east(square: Square) -> Option<Square> {
        square.right()
    }
    fn south(square: Square) -> Option<Square> {
        square.down()
    }
    fn west(square: Square) -> Option<Square> {
        square.left()
    }
    [north, east, south, west]
}

fn bishop_directions() -> [fn(Square) -> Option<Square>; 4] {
    fn north_east(square: Square) -> Option<Square> {
        square.up().and_then(|square| square.right())
    }
    fn south_east(square: Square) -> Option<Square> {
        square.down().and_then(|square| square.right())
    }
    fn south_west(square: Square) -> Option<Square> {
        square.down().and_then(|square| square.left())
    }
    fn north_west(square: Square) -> Option<Square> {
        square.up().and_then(|square| square.left())
    }
    [north_east, south_east, south_west, north_west]
}

fn flatten_data(data: ([Magic; 64], [Vec<BitBoard>; 64])) -> ([Magic; 64], Vec<BitBoard>) {
    let (magic_array, moves_array) = data;

    let mut offset = 0;

    let updated_magic = magic_array
        .iter()
        .zip(moves_array.iter())
        .map(|(magic, moves)| {
            let mut new_magic = *magic;
            new_magic.offset = offset;
            offset += moves.len() as u32;
            new_magic
        })
        .collect::<Vec<Magic>>()
        .try_into()
        .unwrap();

    let flattened_moves = moves_array
        .iter()
        .flat_map(|moves| moves.clone())
        .collect::<Vec<BitBoard>>();

    (updated_magic, flattened_moves)
}

#[rustfmt::skip]
fn find_magic(piece: MagicPiece, square: Square) -> Result<(Magic, Vec<BitBoard>), &'static str> {
    let mask = match piece {
        MagicPiece::Bishop => generate_bishop_moves(square, BitBoard(0)) & BitBoard(0x007e7e7e7e7e7e00),
        MagicPiece::Rook => generate_rook_mask(square),
    };

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123456789);

    // Try magic numbers until we find one that works
    for _ in 0..1_000_000 {
        let magic_number = generate_magic_candidate(&mut rng);
        let magic = Magic {
            mask,
            magic: magic_number,
            shift: 64 - mask.0.count_ones() as u8,
            offset: 0,
        };

        if let Ok(table) = try_make_table(&piece, square, magic) {
            return Ok((magic, table));
        }
    }

    Err("Failed to find magic!")
}

fn try_make_table(piece: &MagicPiece, square: Square, magic: Magic) -> Result<Vec<BitBoard>, &str> {
    let mut table: Vec<BitBoard> =
        vec![BitBoard::default(); 1 << magic.mask.0.count_ones() as usize];

    for blockers in subsets(magic.mask) {
        let moves = match piece {
            MagicPiece::Bishop => generate_bishop_moves(square, blockers),
            MagicPiece::Rook => generate_rook_moves(square, blockers),
        };
        let table_entry = &mut table[magic_index(magic, blockers)];

        if *table_entry == EMPTY {
            *table_entry = moves;
        } else if *table_entry != moves {
            return Err("Hash collision!");
        }
    }

    Ok(table)
}

fn generate_magic_candidate(rng: &mut ChaCha8Rng) -> u64 {
    rng.next_u64() & rng.next_u64() & rng.next_u64()
}

fn magic_index(magic: Magic, blockers: BitBoard) -> usize {
    let blockers = blockers & magic.mask;
    let hash = blockers.0.wrapping_mul(magic.magic);
    let index = (hash >> magic.shift) as usize;
    magic.offset as usize + index
}

fn generate_bishop_magics_and_moves() -> ([Magic; 64], Vec<BitBoard>) {
    let mut magics = [Magic::default(); 64];
    let mut moves = [const { Vec::new() }; 64];

    for square in SQUARES {
        let magic_moves = find_magic(MagicPiece::Bishop, square).unwrap();
        magics[square.to_index()] = magic_moves.0;
        moves[square.to_index()] = magic_moves.1;
    }

    flatten_data((magics, moves))
}

fn generate_bishop_moves(square: Square, blockers: BitBoard) -> BitBoard {
    let mut moves = BitBoard(0);

    for mv in bishop_directions() {
        let mut next = mv(square);
        while let Some(current) = next {
            moves |= BitBoard::from_square(current);
            next = mv(current);

            if BitBoard::from_square(current) & blockers != EMPTY {
                break;
            }
        }
    }

    moves
}

fn generate_rook_mask(square: Square) -> BitBoard {
    let mut mask = BitBoard(0);

    let (file, rank) = (square.to_index() % 8, square.to_index() / 8);

    let vertical_mask = BitBoard(0x0001010101010100);
    let horizontal_mask = BitBoard(0x000000000000007E);

    mask |= vertical_mask << file;
    mask |= horizontal_mask << (rank * 8);

    mask.clear_bit(square);

    mask
}

fn generate_rook_moves_and_magics() -> ([Magic; 64], Vec<BitBoard>) {
    let mut magics = [Magic::default(); 64];
    let mut moves = [const { Vec::new() }; 64];

    for square in SQUARES {
        let magic_moves = find_magic(MagicPiece::Rook, square).unwrap();
        magics[square.to_index()] = magic_moves.0;
        moves[square.to_index()] = magic_moves.1;
    }

    flatten_data((magics, moves))
}

fn generate_rook_moves(square: Square, blockers: BitBoard) -> BitBoard {
    let mut moves = BitBoard(0);

    for mv in rook_directions() {
        let mut next = mv(square);
        while let Some(current) = next {
            moves |= BitBoard::from_square(current);
            next = mv(current);

            if BitBoard::from_square(current) & blockers != EMPTY {
                break;
            }
        }
    }

    moves
}

fn subsets(mask: BitBoard) -> Vec<BitBoard> {
    let mut subsets = Vec::new();

    let mut subset: BitBoard = BitBoard(0);
    loop {
        subsets.push(subset);

        subset = BitBoard(subset.0.wrapping_sub(mask.0)) & mask;
        if subset == EMPTY {
            break;
        }
    }

    subsets
}

pub fn write_bishop_moves(f: &mut File) {
    let bishop_magics_and_moves = generate_bishop_magics_and_moves();
    writeln!(
        f,
        "pub const BISHOP_MAGICS: [Magic; 64] = {:?};",
        bishop_magics_and_moves.0,
    )
    .unwrap();

    writeln!(
        f,
        "pub static BISHOP_MOVES_TABLE: &[BitBoard] = &{:?};",
        bishop_magics_and_moves.1,
    )
    .unwrap();
}

pub fn write_rook_moves(f: &mut File) {
    let rook_magics_and_moves = generate_rook_moves_and_magics();
    writeln!(
        f,
        "pub const ROOK_MAGICS: [Magic; 64] = {:?};",
        rook_magics_and_moves.0,
    )
    .unwrap();

    writeln!(
        f,
        "pub static ROOK_MOVES_TABLE: &[BitBoard] = &{:?};",
        rook_magics_and_moves.1,
    )
    .unwrap();
}
