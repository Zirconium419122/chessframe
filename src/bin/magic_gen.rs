use std::{
    fs::File,
    io::{self, Write},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};

use chessframe::{
    bitboard::{self, BitBoard},
    square::{self, SQUARES},
};

#[path = "../gen_tables/magic.rs"]
mod magic;

use magic::{Magic, MagicPiece, MagicSearch, flatten_data, write_magic_tables};

fn update_tables(
    rng: &mut ChaCha8Rng,
    searches: &mut [MagicSearch; 64],
    tables: &mut [Option<(Magic, Vec<BitBoard>)>; 64],
) {
    for (search, table) in searches.iter_mut().zip(tables) {
        if let Some(candidate) = search.find(rng, 1_000) {
            if table
                .as_ref()
                .is_none_or(|best| candidate.1.len() < best.1.len())
            {
                *table = Some(candidate);
            }
        }
    }
}

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(123456789);
    let mut bishop_searches = SQUARES.map(|square| MagicSearch::new(MagicPiece::Bishop, square));
    let mut rook_searches = SQUARES.map(|square| MagicSearch::new(MagicPiece::Rook, square));
    let mut bishop_tables = [const { None }; 64];
    let mut rook_tables = [const { None }; 64];

    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = stop_flag.clone();

    thread::spawn(move || {
        let mut line = String::new();
        while let Ok(1..) = io::stdin().read_line(&mut line) {
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
        update_tables(&mut rng, &mut bishop_searches, &mut bishop_tables);
        update_tables(&mut rng, &mut rook_searches, &mut rook_tables);

        let bishop_moves_kb = bishop_tables
            .iter()
            .flatten()
            .map(|(_, moves)| moves.len() * 8 / 1024)
            .sum::<usize>();
        let rook_moves_kb = rook_tables
            .iter()
            .flatten()
            .map(|(_, moves)| moves.len() * 8 / 1024)
            .sum::<usize>();
        let found_bishop_squares = bishop_tables.iter().flatten().count();
        let found_rook_squares = rook_tables.iter().flatten().count();

        print!(
            "\rIteration: {}, Bishop moves size: {} KB, Rook moves size: {} KB | Progress: Bishops {}/64, Rooks {}/64",
            iterations, bishop_moves_kb, rook_moves_kb, found_bishop_squares, found_rook_squares
        );
        io::stdout().flush().unwrap();

        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
    }

    if bishop_tables.iter().any(Option::is_none) {
        eprintln!("\nFailed to generate bishop moves and magics!");
        return;
    }
    if rook_tables.iter().any(Option::is_none) {
        eprintln!("\nFailed to generate rook moves and magics!");
        return;
    }

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR should be set");
    let mut file = File::create(format!("{}/magic_tables.rs", out_dir)).unwrap();

    println!("\nWriting magic tables to {}/magic_tables.rs...", out_dir);

    write_magic_tables(
        &mut file,
        flatten_data(bishop_tables.map(Option::unwrap)),
        flatten_data(rook_tables.map(Option::unwrap)),
    );

    println!("Done!");
}
