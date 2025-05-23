use chessframe::{bitboard::EMPTY, board::Board, transpositiontable::TranspositionTable};

struct Perft(TranspositionTable<usize>);

impl Perft {
    pub fn run(board: &Board, depth: usize, divide: bool) -> usize {
        let mut perft = Perft(TranspositionTable::with_size_mb(256));

        perft.perft(&board, depth, divide)
    }

    fn perft(&mut self, board: &Board, depth: usize, divide: bool) -> usize {
        if let Some(entry) = self.0.get(board.hash()) {
            if entry.depth == depth as u8 {
                return entry.value;
            }
        }

        let mut count = 0;

        for mv in board.generate_moves_vec(!EMPTY) {
            if let Ok(ref board) = board.make_move_new(&mv) {
                let perft_results = if depth == 1 {
                    1
                } else {
                    self.perft(board, depth - 1, false)
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
}

#[test]
fn test_perft_depth_1() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    assert_eq!(Perft::run(&board, 1, false), 20);
}

#[test]
fn test_perft_depth_2() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    assert_eq!(Perft::run(&board, 2, false), 400);
}

#[test]
fn test_perft_depth_3() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    assert_eq!(Perft::run(&board, 3, false), 8902);
}

#[test]
fn test_perft_depth_4() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    assert_eq!(Perft::run(&board, 4, false), 197281);
}

#[test]
fn test_perft_depth_5() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    assert_eq!(Perft::run(&board, 5, false), 4865609);
}

// These test should only be run in release mode as during debug profile it's much slower
#[test]
#[cfg(not(debug_assertions))]
fn test_perft_depth_6() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    assert_eq!(Perft::run(&board, 6, false), 119060324);
}

#[test]
#[cfg(not(debug_assertions))]
fn test_perft_depth_7() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    assert_eq!(Perft::run(&board, 7, false), 3195901860);
}
