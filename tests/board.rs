use chess_frame::{
    bitboard::BitBoard, board::*, chess_move::ChessMove, color::Color, piece::Piece, square::Square,
};

#[test]
fn test_square_to_bitboard() {
    macro_rules! generate_assertions {
        ($($x:literal),+) => {
            $(
                assert_eq!(BitBoard::from_square(Square::new(&x)), BitBoard(1 << $x));
            )+
        };
        ($start:literal..$end:literal) => {
            {
                for i in $start..$end {
                    assert_eq!(BitBoard::from_square(Square::new(i)), BitBoard(1 << i));
                }
            }
        }
    }

    generate_assertions!(0..64);
}

#[test]
fn test_from_fen_starting_position() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    // Test if the pieces are placed correctly
    // White pawns
    for file in 0..8 {
        // Rank 2 (index 8-15)
        assert!(board.pieces[Piece::Pawn.piece_index(&Color::White)].is_set(Square::new(8 + file)));
    }
    // Black pawns
    for file in 0..8 {
        // Rank 7 (index 48-55)
        assert!(board.pieces[Piece::Pawn.piece_index(&Color::Black)].is_set(Square::new(48 + file)));
    }

    // Test White pieces
    assert!(board.pieces[Piece::Rook.piece_index(&Color::White)].is_set(Square::A1)); // a1
    assert!(board.pieces[Piece::Knight.piece_index(&Color::White)].is_set(Square::B1)); // b1
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::White)].is_set(Square::C1)); // c1
    assert!(board.pieces[Piece::Queen.piece_index(&Color::White)].is_set(Square::D1)); // d1
    assert!(board.pieces[Piece::King.piece_index(&Color::White)].is_set(Square::E1)); // e1
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::White)].is_set(Square::F1)); // f1
    assert!(board.pieces[Piece::Knight.piece_index(&Color::White)].is_set(Square::G1)); // g1
    assert!(board.pieces[Piece::Rook.piece_index(&Color::White)].is_set(Square::H1)); // h1

    // Test Black pieces
    assert!(board.pieces[Piece::Rook.piece_index(&Color::Black)].is_set(Square::A8)); // a8
    assert!(board.pieces[Piece::Knight.piece_index(&Color::Black)].is_set(Square::B8)); // b8
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::Black)].is_set(Square::C8)); // c8
    assert!(board.pieces[Piece::Queen.piece_index(&Color::Black)].is_set(Square::D8)); // d8
    assert!(board.pieces[Piece::King.piece_index(&Color::Black)].is_set(Square::E8)); // e8
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::Black)].is_set(Square::F8)); // f8
    assert!(board.pieces[Piece::Knight.piece_index(&Color::Black)].is_set(Square::G8)); // g8
    assert!(board.pieces[Piece::Rook.piece_index(&Color::Black)].is_set(Square::H8)); // h8

    // Test side to move
    assert_eq!(board.side_to_move, Color::White);

    // Test castling rights
    assert!(board.castling_rights.can_castle(&Color::White, true)); // White kingside
    assert!(board.castling_rights.can_castle(&Color::White, false)); // White queenside
    assert!(board.castling_rights.can_castle(&Color::Black, true)); // Black kingside
    assert!(board.castling_rights.can_castle(&Color::Black, false)); // Black queenside

    // Test en passant square
    // Since en passant target square is "-", it should be none or unset
    assert!(board.en_passant_square.is_none());

    // Test halfmove clock
    assert_eq!(board.half_move_clock, 0);

    // Test fullmove number
    assert_eq!(board.full_move_clock, 1);
}

#[test]
fn test_infer_move() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    let mv = "e2e4";
    assert_eq!(
        board.infer_move(mv).unwrap(),
        ChessMove::new(Square::E2, Square::E4)
    );

    let mv = "b1c3";
    assert_eq!(
        board.infer_move(mv).unwrap(),
        ChessMove::new(Square::B1, Square::C3)
    );
}

#[test]
fn test_validate_move() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    // Test that you can't push a pawn three squares
    {
        let mut board = board.clone();

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[0], BitBoard(0x000000000000FF00));
        assert_eq!(board.en_passant_square, None);

        assert_eq!(
            board.validate_move(&ChessMove::new(Square::E2, Square::E5)),
            Err("Invalid move!")
        );

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[0], BitBoard(0x000000000000FF00));
        assert_eq!(board.en_passant_square, None);
    }

    // Test that you can't capture your own pieces
    {
        let mut board = board.clone();

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[5], BitBoard(0x000000000000010));

        assert_eq!(
            board.validate_move(&ChessMove::new_capture(Square::E1, Square::D1)),
            Err("Invalid move!")
        );

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[5], BitBoard(0x000000000000010));
    }

    // Test that castling is not possible from the start position
    {
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[5], BitBoard(0x000000000000010));

        assert_eq!(
            board.validate_move(&ChessMove::new(Square::E1, Square::G1)),
            Err("Invalid move!")
        );

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[5], BitBoard(0x000000000000010));
    }
}

#[test]
fn test_make_move() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    // Test that you can push a pawn
    {
        let mut board = board.clone();

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[0], BitBoard(0x000000000000FF00));
        assert_eq!(board.en_passant_square, None);

        assert_eq!(
            board.make_move(&ChessMove::new(Square::E2, Square::E4)),
            Ok(())
        );

        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(board.pieces[0], BitBoard(0x00000001000EF00));
        assert_eq!(board.en_passant_square, Some(BitBoard(0x100000)));
    }
}

#[test]
fn test_can_castle() {
    let fen = "r1bqk2r/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N2/PP3PPP/RNBQK2R w KQkq - 1 6";
    let mut board = Board::from_fen(fen);

    {
        let mut board = board.clone();

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[5], BitBoard(0x000000000000010));

        assert_eq!(board.can_castle(true), Ok(()));

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[5], BitBoard(0x000000000000010));
    }

    assert_eq!(
        board.make_move(&ChessMove::new_castle(Square::E1, Square::G1)),
        Ok(())
    );
    assert_eq!(board.pieces[5], BitBoard(0x000000000000040));
    assert_eq!(board.pieces[3], BitBoard(0x000000000000021));

    {
        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(board.pieces[11], BitBoard(0x1000000000000000));

        assert_eq!(board.can_castle(true), Ok(()));

        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(board.pieces[11], BitBoard(0x1000000000000000));
    }
}
