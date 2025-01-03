use chess_frame::board::Board;

type TranspositionTable = std::collections::HashMap<u64, (usize, usize)>;

fn perft(board: &Board, transposition_table: &mut TranspositionTable, depth: usize, divide: bool) -> usize {
    if depth == 0 {
        return 1;
    }

    if let Some((perft_result, transposition_depth)) = transposition_table.get(&board.get_hash()) {
        if *transposition_depth == depth {
            return *perft_result;
        }
    }

    let mut count = 0;

    for mv in board.generate_moves_vec() {
        if let Ok(ref board) = board.make_move_new(&mv) {
            let perft_results = perft(board, transposition_table, depth - 1, false);
            count += perft_results;

            if divide {
                println!("{}: {}", mv, perft_results);
            }
        }
    }

    transposition_table.insert(board.get_hash(), (count, depth));

    count
}

#[test]
fn test_perft_depth_1() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    let transposition_table = &mut TranspositionTable::new();

    assert_eq!(perft(&board, transposition_table, 1, false), 20);
}

#[test]
fn test_perft_depth_2() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    let transposition_table = &mut TranspositionTable::new();

    assert_eq!(perft(&board, transposition_table, 2, false), 400);
}

#[test]
fn test_perft_depth_3() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    let transposition_table = &mut TranspositionTable::new();

    assert_eq!(perft(&board, transposition_table, 3, true), 8902);
}

#[test]
fn test_perft_depth_4() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    let transposition_table = &mut TranspositionTable::new();

    assert_eq!(perft(&board, transposition_table, 4, false), 197281);
}

#[test]
fn test_perft_depth_5() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    let transposition_table = &mut TranspositionTable::new();

    assert_eq!(perft(&board, transposition_table, 5, false), 4865609);
}

// These test should only be run in release mode as during debug profile it's much slower
#[test]
#[cfg(not(debug_assertions))]
fn test_perft_depth_6() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    let transposition_table = &mut TranspositionTable::new();

    assert_eq!(perft(&board, transposition_table, 6, false), 119060324);
}

#[test]
#[cfg(not(debug_assertions))]
fn test_perft_depth_7() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    let transposition_table = &mut TranspositionTable::new();

    assert_eq!(perft(&board, transposition_table, 7, false), 3195901860);
}
