use std::fs::File;
use std::io::Write;

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};

#[allow(dead_code)]
enum Piece {
    Bishop,
    Rook,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
struct Magic {
    pub mask: u64,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let mut file = File::create("src/tables.rs").unwrap();

    let bishop_magics_and_moves = flatten_data(generate_bishop_magics_and_moves());
    writeln!(
        file,
        "pub const BISHOP_MAGICS: [Magic; 64] = {:?};",
        bishop_magics_and_moves.0,
    )
    .unwrap();

    writeln!(
        file,
        "pub static BISHOP_MOVES_TABLE: &[u64] = &{:?};",
        bishop_magics_and_moves.1,
    )
    .unwrap();

    let rook_magics_and_moves = flatten_data(generate_rook_moves_and_magics());
    writeln!(
        file,
        "pub const ROOK_MAGICS: [Magic; 64] = {:?};",
        rook_magics_and_moves.0,
    )
    .unwrap();

    writeln!(
        file,
        "pub static ROOK_MOVES_TABLE: &[u64] = &{:?};",
        rook_magics_and_moves.1,
    )
    .unwrap();

    let king_moves = generate_king_table();
    writeln!(file, "pub static KING_MOVES: [u64; 64] = {:?};", king_moves).unwrap();
}

fn flatten_data(data: ([Magic; 64], [Vec<u64>; 64])) -> ([Magic; 64], Vec<u64>) {
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
        .collect::<Vec<u64>>();

    (updated_magic, flattened_moves)
}

fn find_magic(piece: Piece, square: usize) -> Result<(Magic, Vec<u64>), &'static str> {
    let mask = match piece {
        Piece::Bishop => generate_bishop_moves(1 << square, 0) & 0x007e7e7e7e7e7e00,
        Piece::Rook => generate_rook_mask(square),
    };

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123456789);

    // Try magic numbers until we find one that works
    for _ in 0..1000000 {
        let magic_number = generate_magic_candidate(&mut rng);
        let magic = Magic {
            mask,
            magic: magic_number,
            shift: 64 - mask.count_ones() as u8,
            offset: 0,
        };

        if let Ok(table) = try_make_table(&piece, square, magic) {
            return Ok((magic, table));
        }
    }

    Err("Failed to find magic!")
}

fn try_make_table(piece: &Piece, square: usize, magic: Magic) -> Result<Vec<u64>, &str> {
    let mut table = vec![0; 1 << magic.mask.count_ones() as usize];

    for blockers in subsets(magic.mask) {
        let moves = match piece {
            Piece::Bishop => generate_bishop_moves(1 << square, blockers),
            Piece::Rook => generate_rook_moves(1 << square, blockers),
        };
        let table_entry = &mut table[magic_index(magic, blockers)];

        if *table_entry == 0 {
            *table_entry = moves;
        } else if *table_entry != moves {
            return Err("Hash collision!");
        }
    }

    Ok(table)
}

fn magic_index(magic: Magic, blockers: u64) -> usize {
    let blockers = blockers & magic.mask;
    let hash = blockers.wrapping_mul(magic.magic);
    (hash >> magic.shift) as usize
}

fn generate_magic_candidate(rng: &mut ChaCha8Rng) -> u64 {
    rng.next_u64() & rng.next_u64() & rng.next_u64()
}

fn generate_bishop_magics_and_moves() -> ([Magic; 64], [Vec<u64>; 64]) {
    let mut magics = [Magic::default(); 64];
    let mut moves = [const { Vec::new() }; 64];

    for square in 0..64 {
        let magic_moves = find_magic(Piece::Bishop, square).unwrap();
        magics[square] = magic_moves.0;
        moves[square] = magic_moves.1;
    }

    (magics, moves)
}

fn generate_bishop_moves(square: u64, blockers: u64) -> u64 {
    let mut moves = 0;

    // Northwest (upleft)
    let mut current = shift_north_west(square);
    while current != 0 {
        moves |= current;
        if current & blockers != 0 {
            break;
        }
        current = shift_north_west(current);
    }

    // Northeast (upright)
    let mut current = shift_north_east(square);
    while current != 0 {
        moves |= current;
        if current & blockers != 0 {
            break;
        }
        current = shift_north_east(current);
    }

    // Southwest (downleft)
    let mut current = shift_south_west(square);
    while current != 0 {
        moves |= current;
        if current & blockers != 0 {
            break;
        }
        current = shift_south_west(current);
    }

    // Southeast (downright)
    let mut current = shift_south_east(square);
    while current != 0 {
        moves |= current;
        if current & blockers != 0 {
            break;
        }
        current = shift_south_east(current);
    }

    moves
}

fn generate_rook_mask(square: usize) -> u64 {
    let mut mask: u64 = 0;

    let (file, rank) = (square % 8, square / 8);

    let vertical_mask: u64 = 0x0001010101010100;
    let horizontal_mask: u64 = 0x000000000000007E;

    mask |= vertical_mask << file;
    mask |= horizontal_mask << (rank * 8);

    mask & !(1 << square)
}

fn generate_rook_moves_and_magics() -> ([Magic; 64], [Vec<u64>; 64]) {
    let mut magics = [Magic::default(); 64];
    let mut moves = [const { Vec::new() }; 64];

    for square in 0..64 {
        let magic_moves = find_magic(Piece::Rook, square).unwrap();
        magics[square] = magic_moves.0;
        moves[square] = magic_moves.1;
    }

    (magics, moves)
}

fn generate_rook_moves(square: u64, blockers: u64) -> u64 {
    let mut moves = 0;

    // North (up)
    let mut current = shift_north(square);
    while current != 0 {
        moves |= current;
        if current & blockers != 0 {
            break;
        }
        current = shift_north(current);
    }

    // South (down)
    let mut current = shift_south(square);
    while current != 0 {
        moves |= current;
        if current & blockers != 0 {
            break;
        }
        current = shift_south(current);
    }

    // West (left)
    let mut current = shift_west(square);
    while current != 0 {
        moves |= current;
        if current & blockers != 0 {
            break;
        }
        current = shift_west(current);
    }

    // East (right)
    let mut current = shift_east(square);
    while current != 0 {
        moves |= current;
        if current & blockers != 0 {
            break;
        }
        current = shift_east(current);
    }

    moves
}

fn generate_king_table() -> [u64; 64] {
    let mut table = [0; 64];

    for (i, square) in table.iter_mut().enumerate() {
        *square = generate_king_moves(1 << i);
    }

    table
}

fn generate_king_moves(square: u64) -> u64 {
    let mut moves = 0;

    // North (up)
    let mut current = shift_north(square);
    while current != 0 {
        moves |= current;
        if current != 0 {
            break;
        }
        current = shift_north(current);
    }

    // Northweast (upleft)
    let mut current = shift_north_west(square);
    while current != 0 {
        moves |= current;
        if current != 0 {
            break;
        }
        current = shift_north_west(current);
    }

    // Northeast (upright)
    let mut current = shift_north_east(square);
    while current != 0 {
        moves |= current;
        if current != 0 {
            break;
        }
        current = shift_north_east(current);
    }

    // South (down)
    let mut current = shift_south(square);
    while current != 0 {
        moves |= current;
        if current != 0 {
            break;
        }
        current = shift_south(current);
    }

    // Southwest (downleft)
    let mut current = shift_south_west(square);
    while current != 0 {
        moves |= current;
        if current != 0 {
            break;
        }
        current = shift_south_west(current);
    }

    // Southeast (downright)
    let mut current = shift_south_east(square);
    while current != 0 {
        moves |= current;
        if current != 0 {
            break;
        }
        current = shift_south_east(current);
    }

    // West (left)
    let mut current = shift_west(square);
    while current != 0 {
        moves |= current;
        if current != 0 {
            break;
        }
        current = shift_west(current);
    }

    // East (right)
    let mut current = shift_east(square);
    while current != 0 {
        moves |= current;
        if current != 0 {
            break;
        }
        current = shift_east(current);
    }

    moves
}

fn subsets(mask: u64) -> Vec<u64> {
    let mut subsets = Vec::new();

    let mut subset: u64 = 0;
    loop {
        subsets.push(subset);

        subset = subset.wrapping_sub(mask) & mask;
        if subset == 0 {
            break;
        }
    }

    subsets
}

fn shift_north(bitboard: u64) -> u64 {
    bitboard << 8
}

fn shift_north_west(bitboard: u64) -> u64 {
    (bitboard & !0x0101010101010101) << 7
}

fn shift_north_east(bitboard: u64) -> u64 {
    (bitboard & !0x8080808080808080) << 9
}

fn shift_south(bitboard: u64) -> u64 {
    bitboard >> 8
}

fn shift_south_west(bitboard: u64) -> u64 {
    (bitboard & !0x0101010101010101) >> 9
}

fn shift_south_east(bitboard: u64) -> u64 {
    (bitboard & !0x8080808080808080) >> 7
}

fn shift_west(bitboard: u64) -> u64 {
    (bitboard & !0x0101010101010101) >> 1 // Mask out the H file
}

fn shift_east(bitboard: u64) -> u64 {
    (bitboard & !0x8080808080808080) << 1 // Mask out the A file
}
