use chessframe::{
    bitboard::{BitBoard, EMPTY},
    board::*,
};

#[test]
fn test_generate_moves_vec() {
    let fen = "8/p7/1k2Pp2/1P3P2/1K6/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);

    let moves = board.generate_moves_vec(!EMPTY);
    assert_eq!(moves.len(), 8)
}

#[test]
fn test_pawn_move_generation() {
    // Test pawn move generation from a starting position
    {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);

        let pawn_quiets = board.generate_pawn_moves() & !board.occupancy(!board.side_to_move);
        assert_eq!(pawn_quiets, BitBoard(0x00000000FFFF0000));

        let pawn_captures = board.generate_pawn_moves() & board.occupancy(!board.side_to_move);
        assert_eq!(pawn_captures, BitBoard(0));

        let en_passant = board.generate_pawn_moves()
            & BitBoard::from_square(board.en_passant_square().unwrap_or_default());
        assert_eq!(en_passant, BitBoard(0));

        let pawn_moves = board.generate_pawn_moves();
        assert_eq!(pawn_moves, BitBoard(0x00000000FFFF0000));
    }

    // Test pawn move generation from a opening position
    {
        let fen = "rnbqkbnr/1ppppppp/8/P7/8/8/P1PPPPPP/RNBQKBNR b KQkq - 0 2";
        let board = Board::from_fen(fen);

        let pawn_pushes = board.generate_pawn_moves() & !board.occupancy(!board.side_to_move);
        assert_eq!(pawn_pushes, BitBoard(0x0000FEFE00000000));

        let pawn_captures = board.generate_pawn_moves() & board.occupancy(!board.side_to_move);
        assert_eq!(pawn_captures, BitBoard(0));

        let en_passant = board.generate_pawn_moves()
            & BitBoard::from_square(board.en_passant_square().unwrap_or_default());
        assert_eq!(en_passant, BitBoard(0));
    }

    // Test pawn move generation from a endgame position
    {
        let fen = "8/p7/1k2Pp2/1P3P2/1K6/8/8/8 w - - 0 1";
        let board = Board::from_fen(fen);

        let pawn_quiets = board.generate_pawn_moves() & !board.occupancy(!board.side_to_move);
        assert_eq!(pawn_quiets, BitBoard(0x10000000000000));

        let pawn_captures = board.generate_pawn_moves() & board.occupancy(!board.side_to_move);
        assert_eq!(pawn_captures, BitBoard(0));

        let en_passant = board.generate_pawn_moves()
            & BitBoard::from_square(board.en_passant_square().unwrap_or_default());
        assert_eq!(en_passant, BitBoard(0));

        let pawn_moves = board.generate_pawn_moves();
        assert_eq!(pawn_moves, BitBoard(0x10000000000000));
    }
}

#[test]
fn test_knight_move_generation() {
    {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);

        let knight_moves = board.generate_knight_moves();
        assert_eq!(knight_moves, BitBoard(0x0000000000A50000));
    }

    // Test move generation from a endgame position
    {
        let fen = "8/pn6/1k2Pp2/1P3P2/1K6/8/8/8 b - - 0 1";
        let board = Board::from_fen(fen);

        let kinght_moves = board.generate_knight_moves();
        assert_eq!(kinght_moves, BitBoard(0x0800080500000000));
    }
}

#[test]
fn test_bishop_move_generation() {
    // Test that there are no legal moves from the start position
    {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);

        let bishop_moves = board.generate_bishop_moves();
        assert_eq!(bishop_moves, BitBoard(0));
    }

    // Test that there are legal moves from a opening position
    {
        let fen = "rnbqkbnr/p1pp1ppp/8/1p2p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 3";
        let board = Board::from_fen(fen);

        let bishop_moves = board.generate_bishop_moves();
        assert_eq!(bishop_moves, BitBoard(0x0000000204081000));
    }

    // Test move generation from a endgame position
    {
        let fen = "8/p3r3/1k2Pp2/1P3P2/1K4B1/8/8/8 w H - 0 1";
        let board = Board::from_fen(fen);

        let bishop_moves = board.generate_bishop_moves();
        assert_eq!(bishop_moves, BitBoard(0x0000008000a01008));
    }
}

#[test]
fn test_rook_move_generation() {
    // Test that there are no legal moves from the start position
    {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);

        let rook_moves = board.generate_rook_moves();
        assert_eq!(rook_moves, BitBoard(0));
    }

    // Test that there are legal moves from a opening position
    {
        let fen = "rnbqkbnr/p1pp1ppp/8/1p2p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 3";
        let board = Board::from_fen(fen);

        let rook_moves = board.generate_rook_moves();
        assert_eq!(rook_moves, BitBoard(0x0000000000000040));
    }

    // Test move generation from a endgame position
    {
        let fen = "8/p3r3/1k2Pp2/1P3P2/1K6/8/8/8 b - - 0 1";
        let board = Board::from_fen(fen);

        let rook_moves = board.generate_rook_moves();
        assert_eq!(rook_moves, BitBoard(0x10ee100000000000));
    }
}

#[test]
fn test_queen_move_generation() {
    // Test that there are no legal moves from the start position
    {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);

        let queen_moves = board.generate_queen_moves();
        assert_eq!(queen_moves, BitBoard(0));
    }

    // Test that there are legal moves from a opening position
    {
        let fen = "rnbqkbnr/p1pp1ppp/8/1p2p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 3";
        let board = Board::from_fen(fen);

        let queen_moves = board.generate_queen_moves();
        assert_eq!(queen_moves, BitBoard(0x0000000000001000));
    }

    // Test move generation from a endgame position
    {
        let fen = "8/p1q5/1k2Pp2/1P3P2/1K6/8/R7/8 b H - 0 1";
        let board = Board::from_fen(fen);

        let queen_moves = board.generate_queen_moves();
        assert_eq!(queen_moves, BitBoard(0x0efa0c1424448404));
    }
}

#[test]
fn test_king_move_generation() {
    // Test that there are no legal moves from the start position
    {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);

        let king_moves = board.generate_king_moves();
        assert_eq!(king_moves, BitBoard(0));
    }

    // Test that there are legal moves from a opening position
    {
        let fen = "r1bqk2r/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N2/PP3PPP/RNBQK2R w KQkq - 1 6";
        let board = Board::from_fen(fen);

        let king_moves = board.generate_king_moves();
        assert_eq!(king_moves, BitBoard(0x0000000000001820));
    }

    // Test move generation from a endgame position
    {
        let fen = "8/p7/1k2Pp2/1P3P2/1K6/8/8/8 w - - 0 1";
        let board = Board::from_fen(fen);

        let king_moves = board.generate_king_moves();
        assert_eq!(king_moves, BitBoard(0x0000000505070000));
    }
}

#[test]
fn test_castling_move_generation() {
    // Test that there are castling moves from the start position
    {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);

        let castling_moves = board.generate_castling_moves();
        assert_eq!(castling_moves, BitBoard(0));
    }

    // Test that there are castling moves from a opening position
    {
        let fen = "r1bqk2r/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N2/PP3PPP/RNBQK2R w KQkq - 1 6";
        let board = Board::from_fen(fen);

        let castling_moves = board.generate_castling_moves();
        assert_eq!(castling_moves, BitBoard(0x40));
    }
}
