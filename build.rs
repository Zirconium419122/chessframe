use std::fs::File;
use std::io::Write;
use std::thread;

const STACK_SIZE: usize = 8 * 1024 * 1024;

fn main() {
    let rook_blocker_mask = generate_rook_blocker_mask();

    let mut file = File::create("src/tables.rs").unwrap();
    writeln!(
        file,
        "pub const ROOK_BLOCKER_MASK: [u64; 64] = {:?};",
        rook_blocker_mask
    )
    .unwrap();

    let thread = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || {
            let rook_moves = generate_rook_moves_table();
            writeln!(
                file,
                "pub const ROOK_MOVES_TABLE: [[u64; 4096]; 64] = {:?};",
                rook_moves,
            )
            .unwrap();
        })
        .unwrap();

    thread.join().unwrap();
}

fn generate_rook_blocker_mask() -> [u64; 64] {
    let mut table = [0_u64; 64];

    for square in 0..64 {
        table[square] = generate_rook_mask(square);
    }

    table
}

fn generate_rook_mask(square: usize) -> u64 {
    let mut mask: u64 = 0;

    let (file, rank) = (square % 8, square / 8);

    let vertical_mask: u64 = 0x0001010101010100;
    let horizontal_mask: u64 = 0x000000000000007E;

    mask |= vertical_mask << file;
    mask |= horizontal_mask << rank * 8;

    mask & !(1 << square)
}

fn generate_rook_moves_table() -> [[u64; 4096]; 64] {
    let mut moves = [[0_u64; 4096]; 64];

    let rook_blocker_mask = generate_rook_blocker_mask();

    for square in 0..64 {
        for (i, subset) in generate_mask_subsets(rook_blocker_mask[square])
            .into_iter()
            .enumerate()
        {
            moves[square][i] = generate_rook_moves(1 << square, subset);
        }
    }

    moves
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

fn generate_mask_subsets(mask: u64) -> Vec<u64> {
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

fn shift_south(bitboard: u64) -> u64 {
    bitboard >> 8
}

fn shift_west(bitboard: u64) -> u64 {
    (bitboard & !0x0101010101010101) >> 1 // Mask out the H file
}

fn shift_east(bitboard: u64) -> u64 {
    (bitboard & !0x8080808080808080) << 1 // Mask out the A file
}
