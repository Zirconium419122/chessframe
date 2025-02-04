use std::{
    fs::File,
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};

use chessframe::{
    bitboard::{BitBoard, EMPTY},
    square::{Square, SQUARES},
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
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

fn generate_magics_and_moves(
    rng: &mut ChaCha8Rng,
    piece: MagicPiece,
    search_squares: &Option<Vec<Square>>,
) -> Result<[Option<(Magic, Vec<BitBoard>)>; 64], &'static str> {
    let mut magics_and_moves = [const { None }; 64];

    if let Some(search_squares) = search_squares {
        for square in search_squares {
            magics_and_moves[square.to_index()] =
                if let Ok(magic_moves) = find_magic(rng, piece, *square) {
                    Some((magic_moves.0, magic_moves.1))
                } else {
                    None
                }
        }
    } else {
        for square in SQUARES {
            magics_and_moves[square.to_index()] =
                if let Ok(magic_moves) = find_magic(rng, piece, square) {
                    Some((magic_moves.0, magic_moves.1))
                } else {
                    None
                }
        }
    }

    Ok(magics_and_moves)
}

#[rustfmt::skip]
fn find_magic(rng: &mut ChaCha8Rng, piece: MagicPiece, square: Square) -> Result<(Magic, Vec<BitBoard>), &'static str> {
    let mask = match piece {
        MagicPiece::Bishop => generate_bishop_moves(square, BitBoard(0)) & BitBoard(0x007e7e7e7e7e7e00),
        MagicPiece::Rook => generate_rook_mask(square),
    };

    // Try magic numbers until we find one that works
    for _ in 0..1_000 {
        let magic_number = generate_magic_candidate(rng);
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

fn extract_moves_and_magics(
    moves_and_magics: &[Option<(Magic, Vec<BitBoard>)>; 64],
) -> ([Magic; 64], [Vec<BitBoard>; 64]) {
    let magics = moves_and_magics
        .iter()
        .map(|x| x.as_ref().unwrap().0)
        .collect::<Vec<Magic>>()
        .try_into()
        .unwrap();
    let moves = moves_and_magics
        .iter()
        .map(|x| x.as_ref().unwrap().1.clone())
        .collect::<Vec<Vec<BitBoard>>>()
        .try_into()
        .unwrap();
    (magics, moves)
}

fn write_bishop_moves(f: &mut File, bishop_magics_and_moves: ([Magic; 64], Vec<BitBoard>)) {
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

fn write_rook_moves(f: &mut File, rook_magics_and_moves: ([Magic; 64], Vec<BitBoard>)) {
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

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123456789);

    let mut missing_bishop_squares = Some(Vec::from(SQUARES));
    let mut bishop_moves_and_magics: [Option<(Magic, Vec<BitBoard>)>; 64] = [const { None }; 64];

    let mut missing_rook_squares = Some(Vec::from(SQUARES));
    let mut rook_moves_and_magics: [Option<(Magic, Vec<BitBoard>)>; 64] = [const { None }; 64];

    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = stop_flag.clone();

    thread::spawn(move || {
        let mut line = String::new();
        while io::stdin().read_line(&mut line).is_ok() {
            if line.trim_end() == "stop" {
                stop_flag_clone.store(true, Ordering::Relaxed);
                break;
            }
            line.clear();
        }
    });

    let mut iterations = 0;

    loop {
        iterations += 1;

        if let Some(missing_squares) = &missing_bishop_squares {
            if missing_squares.is_empty() {
                missing_bishop_squares = None;
            }
        }

        if let Ok(latest_bishop_magics_and_moves) =
            generate_magics_and_moves(&mut rng, MagicPiece::Bishop, &missing_bishop_squares)
        {
            for (i, latest_bishop_magics_and_moves) in latest_bishop_magics_and_moves
                .iter()
                .enumerate()
                .filter(|(_, x)| x.is_some())
            {
                let latest_bishop_magics_and_moves =
                    latest_bishop_magics_and_moves.as_ref().unwrap();
                if let Some(bishop_moves_and_magics) = &mut bishop_moves_and_magics[i] {
                    if latest_bishop_magics_and_moves.1.len() < bishop_moves_and_magics.1.len() {
                        *bishop_moves_and_magics = latest_bishop_magics_and_moves.clone();
                    }
                } else {
                    bishop_moves_and_magics[i] = Some(latest_bishop_magics_and_moves.clone());
                    // println!("Found bishop moves and magics for square {}!", i);
                }
            }
        }

        if let Some(missing_squares) = &missing_rook_squares {
            if missing_squares.is_empty() {
                missing_rook_squares = None;
            }
        }

        if let Ok(latest_rook_magics_and_moves) =
            generate_magics_and_moves(&mut rng, MagicPiece::Rook, &missing_rook_squares)
        {
            for (i, latest_rook_magics_and_moves) in latest_rook_magics_and_moves
                .iter()
                .enumerate()
                .filter(|(_, x)| x.is_some())
            {
                let latest_rook_magics_and_moves = latest_rook_magics_and_moves.as_ref().unwrap();
                if let Some(rook_moves_and_magics) = &mut rook_moves_and_magics[i] {
                    if latest_rook_magics_and_moves.1.len() < rook_moves_and_magics.1.len() {
                        *rook_moves_and_magics = latest_rook_magics_and_moves.clone();
                    }
                } else {
                    rook_moves_and_magics[i] = Some(latest_rook_magics_and_moves.clone());
                    // println!("Found rook moves and magics for square {}!", i);
                }
            }
        }

        let bishop_moves_kb = bishop_moves_and_magics
            .iter()
            .filter_map(|x| x.as_ref())
            .fold(0, |acc, (_, moves)| acc + moves.len() * 8 / 1024);
        let rook_moves_kb = rook_moves_and_magics
            .iter()
            .filter_map(|x| x.as_ref())
            .fold(0, |acc, (_, moves)| acc + moves.len() * 8 / 1024);
        let found_bishop_squares = bishop_moves_and_magics
            .iter()
            .filter(|x| x.is_some())
            .count();
        let found_rook_squares = rook_moves_and_magics.iter().filter(|x| x.is_some()).count();

        print!(
            "\rIteration: {}, Bishop moves size: {} KB, Rook moves size: {} KB | Progress: Bishops {}/64, Rooks {}/64",
            iterations, bishop_moves_kb, rook_moves_kb, found_bishop_squares, found_rook_squares
        );
        io::stdout().flush().unwrap();

        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
    }

    if bishop_moves_and_magics
        .iter()
        .filter(|x| x.is_none())
        .count()
        > 0
    {
        eprintln!("\nFailed to generate bishop moves and magics!");
        return;
    } else if rook_moves_and_magics.iter().filter(|x| x.is_none()).count() > 0 {
        eprintln!("\nFailed to generate rook moves and magics!");
        return;
    }

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR should be set");
    let mut file = File::create(format!("{}/magic_tables.rs", out_dir)).unwrap();

    println!("\nWriting magic tables to {}/magic_tables.rs...", out_dir);

    write_bishop_moves(
        &mut file,
        flatten_data(extract_moves_and_magics(&bishop_moves_and_magics)),
    );
    write_rook_moves(
        &mut file,
        flatten_data(extract_moves_and_magics(&rook_moves_and_magics)),
    );

    println!("Done!");
}
