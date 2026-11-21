#![allow(unused)]
use std::fs::File;
use std::io::{BufWriter, Write};

use rand_chacha::{
    ChaCha8Rng,
    rand_core::{Rng, SeedableRng},
};

use crate::{
    bitboard::{BitBoard, EMPTY},
    square::{SQUARES, Square},
};

#[derive(Debug, Clone, Copy)]
pub enum MagicPiece {
    Bishop,
    Rook,
}

impl MagicPiece {
    pub(crate) fn blocker_mask(self, square: Square) -> BitBoard {
        match self {
            Self::Bishop => self.attacks(square, EMPTY) & BitBoard(0x007e7e7e7e7e7e00),
            Self::Rook => rook_blocker_mask(square),
        }
    }

    pub(crate) fn attacks(self, square: Square, blockers: BitBoard) -> BitBoard {
        let directions = match self {
            Self::Bishop => bishop_directions(),
            Self::Rook => rook_directions(),
        };
        let mut attacks = EMPTY;

        for step in directions {
            let mut next = step(square);
            while let Some(current) = next {
                let target = BitBoard::from_square(current);
                attacks |= target;
                if target & blockers != EMPTY {
                    break;
                }
                next = step(current);
            }
        }

        attacks
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Magic {
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

pub(crate) fn flatten_data(data: [(Magic, Vec<BitBoard>); 64]) -> ([Magic; 64], Vec<BitBoard>) {
    let mut flattened_moves = Vec::with_capacity(data.iter().map(|(_, moves)| moves.len()).sum());
    let magics = data.map(|(mut magic, moves)| {
        magic.offset = flattened_moves.len() as u32;
        flattened_moves.extend(moves);
        magic
    });

    (magics, flattened_moves)
}

pub(crate) struct MagicSearch {
    mask: BitBoard,
    shift: u8,
    blocker_attacks: Vec<(BitBoard, BitBoard)>,
    scratch_attacks: Vec<BitBoard>,
    slot_attempts: Vec<u32>,
    attempt: u32,
}

impl MagicSearch {
    pub(crate) fn new(piece: MagicPiece, square: Square) -> Self {
        let mask = piece.blocker_mask(square);
        let blocker_attacks: Vec<_> = subsets(mask)
            .into_iter()
            .map(|blockers| (blockers, piece.attacks(square, blockers)))
            .collect();
        let table_len = blocker_attacks.len();

        Self {
            mask,
            shift: 64 - mask.count_ones() as u8,
            blocker_attacks,
            scratch_attacks: vec![EMPTY; table_len],
            slot_attempts: vec![0; table_len],
            attempt: 0,
        }
    }

    pub(crate) fn find(
        &mut self,
        rng: &mut ChaCha8Rng,
        attempts: u32,
    ) -> Option<(Magic, Vec<BitBoard>)> {
        self.reserve_attempts(attempts);

        for _ in 0..attempts {
            self.attempt += 1;
            let multiplier = generate_magic_candidate(rng);
            if self.candidate_fits(multiplier) {
                return Some(self.build_result(multiplier));
            }
        }

        None
    }

    fn reserve_attempts(&mut self, attempts: u32) {
        if self.attempt > u32::MAX - attempts {
            self.slot_attempts.fill(0);
            self.attempt = 0;
        }
    }

    fn candidate_fits(&mut self, multiplier: u64) -> bool {
        self.blocker_attacks.iter().all(|&(blockers, attacks)| {
            let slot = (blockers.0.wrapping_mul(multiplier) >> self.shift) as usize;
            // A slot belongs to this candidate only if its attempt number matches.
            if self.slot_attempts[slot] == self.attempt {
                self.scratch_attacks[slot] == attacks
            } else {
                self.slot_attempts[slot] = self.attempt;
                self.scratch_attacks[slot] = attacks;
                true
            }
        })
    }

    fn build_result(&self, multiplier: u64) -> (Magic, Vec<BitBoard>) {
        let magic = Magic {
            mask: self.mask,
            magic: multiplier,
            shift: self.shift,
            offset: 0,
        };
        let attacks = self
            .scratch_attacks
            .iter()
            .zip(&self.slot_attempts)
            .map(|(&attacks, &attempt)| {
                if attempt == self.attempt {
                    attacks
                } else {
                    EMPTY
                }
            })
            .collect();

        (magic, attacks)
    }
}

fn generate_magic_candidate(rng: &mut ChaCha8Rng) -> u64 {
    rng.next_u64() & rng.next_u64() & rng.next_u64()
}

fn rook_blocker_mask(square: Square) -> BitBoard {
    let mut mask = BitBoard(0);

    let (file, rank) = (square.to_index() % 8, square.to_index() / 8);

    let vertical_mask = BitBoard(0x0001010101010100);
    let horizontal_mask = BitBoard(0x000000000000007E);

    mask |= vertical_mask << file;
    mask |= horizontal_mask << (rank * 8);

    mask.clear_bit(square);

    mask
}

pub(crate) fn subsets(mask: BitBoard) -> Vec<BitBoard> {
    let mut subsets = Vec::with_capacity(1 << mask.count_ones());

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

pub fn write_magic_moves(f: &mut File) {
    fn generate(piece: MagicPiece) -> ([Magic; 64], Vec<BitBoard>) {
        let mut rng = ChaCha8Rng::seed_from_u64(123456789);
        flatten_data(SQUARES.map(|square| {
            MagicSearch::new(piece, square)
                .find(&mut rng, 1_000_000)
                .expect("Failed to find magic!")
        }))
    }

    let bishops = std::thread::spawn(|| generate(MagicPiece::Bishop));
    let rooks = generate(MagicPiece::Rook);
    write_magic_tables(f, bishops.join().unwrap(), rooks);
}

pub(crate) fn write_magic_tables(
    f: &mut File,
    (bishop_magics, bishop_moves): ([Magic; 64], Vec<BitBoard>),
    (rook_magics, rook_moves): ([Magic; 64], Vec<BitBoard>),
) {
    let mut f = BufWriter::new(f);
    writeln!(
        f,
        "pub const BISHOP_MAGICS: [Magic; 64] = {bishop_magics:?};"
    )
    .unwrap();
    writeln!(
        f,
        "pub static BISHOP_MOVES_TABLE: &[BitBoard] = &{bishop_moves:?};"
    )
    .unwrap();

    writeln!(f, "pub const ROOK_MAGICS: [Magic; 64] = {rook_magics:?};").unwrap();
    writeln!(
        f,
        "pub static ROOK_MOVES_TABLE: &[BitBoard] = &{rook_moves:?};"
    )
    .unwrap();

    f.flush().unwrap();
}
