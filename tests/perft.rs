use chessframe::{bitboard::EMPTY, board::Board, transpositiontable::TranspositionTable};

struct Perft(TranspositionTable<usize>);

impl Perft {
    fn perft(&mut self, board: &Board, depth: usize, divide: bool) -> usize {
        if let Some(entry) = self.0.get(board.hash())
            && entry.depth == depth as u8
        {
            return entry.value;
        }

        let mut count = 0;

        for mv in board.generate_moves_vec(!EMPTY) {
            if let Ok(board) = board.make_move_new(mv) {
                let perft_results = if depth == 1 {
                    1
                } else {
                    self.perft(&board, depth - 1, false)
                };
                count += perft_results;

                if divide {
                    println!("{}: {}", mv, perft_results);
                }
            }
        }

        self.0.store(board.hash(), count, depth as u8);

        count
    }

    fn perft_unmake(&mut self, board: &mut Board, depth: usize, divide: bool) -> usize {
        if let Some(entry) = self.0.get(board.hash())
            && entry.depth == depth as u8
        {
            return entry.value;
        }

        let mut count = 0;

        let en_passant_square = board.en_passant_square();
        let castling_rights = board.castling_rights;

        for mv in board.generate_moves_vec(!EMPTY) {
            if let Ok(metadata) = board.make_move_metadata(mv) {
                let perft_results = if depth == 1 {
                    1
                } else {
                    self.perft_unmake(board, depth - 1, false)
                };
                count += perft_results;

                board.unmake_move(mv, metadata, en_passant_square, castling_rights).unwrap();

                if divide {
                    println!("{}: {}", mv, perft_results);
                }
            }
        }

        self.0.store(board.hash(), count, depth as u8);

        count
    }
}

trait PerftImpl {
    fn run(board: &Board, depth: usize, divide: bool) -> usize;
}

struct MakeNew;
struct Unmake;

impl PerftImpl for MakeNew {
    fn run(board: &Board, depth: usize, divide: bool) -> usize {
        let mut perft = Perft(TranspositionTable::with_size_mb(256));
        let board = *board;

        perft.perft(&board, depth, divide)
    }
}

impl PerftImpl for Unmake {
    fn run(board: &Board, depth: usize, divide: bool) -> usize {
        let mut perft = Perft(TranspositionTable::with_size_mb(256));
        let mut board = *board;

        perft.perft_unmake(&mut board, depth, divide)
    }
}

fn perft_test<T: PerftImpl>(fen: &str, depth: usize, expected: usize) {
    let board = Board::from_fen(fen);
    assert_eq!(T::run(&board, depth, false), expected);
}

macro_rules! generate_perft_tests {
    ($suffix:ident, $impl:ty) => {
        paste::paste! {
            #[test]
            fn [<test_perft_depth_1_ $suffix>]() {
                perft_test::<$impl>(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                    1,
                    20,
                );
            }

            #[test]
            fn [<test_perft_depth_2_ $suffix>]() {
                perft_test::<$impl>(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                    2,
                    400,
                );
            }

            #[test]
            fn [<test_perft_depth_3_ $suffix>]() {
                perft_test::<$impl>(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                    3,
                    8902,
                );
            }

            #[test]
            fn [<test_perft_depth_4_ $suffix>]() {
                perft_test::<$impl>(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                    4,
                    197281,
                );
            }

            #[test]
            fn [<test_perft_depth_5_ $suffix>]() {
                perft_test::<$impl>(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                    5,
                    4865609,
                );
            }

            #[test]
            fn [<test_perft_depth_6_ $suffix>]() {
                perft_test::<$impl>(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                    6,
                    119060324,
                );
            }

            #[test]
            fn [<test_perft_depth_7_ $suffix>]() {
                perft_test::<$impl>(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                    7,
                    3195901860,
                );
            }

            #[test]
            fn [<test_perft_endgame_ $suffix>]() {
                perft_test::<$impl>(
                    "8/p4ppp/P7/1nk4P/4KPP1/4P3/8/8 b - - 2 41",
                    8,
                    151117231,
                );
            }
        }
    };
}

generate_perft_tests!(make_new, MakeNew);
generate_perft_tests!(unmake, Unmake);
