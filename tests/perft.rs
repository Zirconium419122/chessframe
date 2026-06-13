use chessframe::{bitboard::EMPTY, board::Board, chess_move::ChessMove};

struct Perft([[ChessMove; 96]; 10]);

impl Perft {
    fn perft(&mut self, board: &Board, depth: usize, divide: bool) -> usize {
        let mut count = 0;

        let num_moves = board.generate_moves(!EMPTY, &mut self.0[depth]);

        for i in 0..num_moves {
            let mv = self.0[depth][i];

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

        count
    }

    fn perft_unmake(&mut self, board: &mut Board, depth: usize, divide: bool) -> usize {
        let mut count = 0;

        let unmake_data = board.unmake_data();

        let num_moves = board.generate_moves(!EMPTY, &mut self.0[depth]);

        for i in 0..num_moves {
            let mv = self.0[depth][i];

            if let Ok(metadata) = board.make_move_metadata(mv) {
                let perft_results = if depth == 1 {
                    1
                } else {
                    self.perft_unmake(board, depth - 1, false)
                };
                count += perft_results;

                board.unmake_move(mv, metadata, unmake_data).unwrap();

                if divide {
                    println!("{}: {}", mv, perft_results);
                }
            }
        }

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
        let mut perft = Perft([[ChessMove::NULL_MOVE; 96]; 10]);
        let board = *board;

        perft.perft(&board, depth, divide)
    }
}

impl PerftImpl for Unmake {
    fn run(board: &Board, depth: usize, divide: bool) -> usize {
        let mut perft = Perft([[ChessMove::NULL_MOVE; 96]; 10]);
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

            #[test]
            fn [<test_perft_kiwipete_ $suffix>]() {
                perft_test::<$impl>(
                    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
                    5,
                    193690690,
                );
            }

            #[test]
            fn [<test_perft_position_5_ $suffix>]() {
                perft_test::<$impl>(
                    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
                    5,
                    89941194,
                );
            }
        }
    };
}

generate_perft_tests!(make_new, MakeNew);
generate_perft_tests!(unmake, Unmake);
