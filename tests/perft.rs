use chess_frame::board::Board;

fn perft(board: &mut Board, depth: usize, divide: bool) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;

    for mv in board.generate_moves_vec() {
        if let Ok(_) = board.make_move(mv.clone()) {
            let perft = perft(board, depth - 1, false);
            count += perft;

            if divide {
                println!("{}: {}", mv, perft);
            }

            let _ = board.unmake_move();
        }
    }

    count
}

#[test]
fn test_perft_depth_1() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    assert_eq!(perft(&mut board, 1, false), 20);
}

#[test]
fn test_perft_depth_2() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    assert_eq!(perft(&mut board, 2, false), 400);
}

#[test]
fn test_perft_depth_3() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    assert_eq!(perft(&mut board, 3, false), 8902);
}

#[test]
fn test_perft_depth_4() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    assert_eq!(perft(&mut board, 4, false), 197281);
}

#[test]
fn test_perft_depth_5() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    assert_eq!(perft(&mut board, 5, false), 4865609);
}
