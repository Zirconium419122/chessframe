use std::fs::File;
use std::io::Write;

fn main() {
    let rook_blocker_mask = generate_rook_blocker_mask();

    let mut file = File::create("src/tables.rs").unwrap();
    writeln!(
        file,
        "pub const ROOK_ATTACK_TABLE: [u64; 64] = {:?};",
        rook_blocker_mask
    )
    .unwrap();
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
