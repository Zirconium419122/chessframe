use chessframe::{
    bitboard::{BitBoard, EMPTY},
    board::*,
    castling_rights::CastlingRights,
    chess_move::ChessMove,
    color::Color,
    error::Error,
    piece::Piece,
    square::Square,
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
#[rustfmt::skip]
fn test_from_fen_starting_position() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    // Test if the pieces are placed correctly
    // White pawns
    for file in 0..8 {
        // Rank 2 (index 8-15)
        assert!(board.pieces_color(Piece::Pawn, Color::White).is_set(Square::new(8 + file)));
    }
    // Black pawns
    for file in 0..8 {
        // Rank 7 (index 48-55)
        assert!(board.pieces_color(Piece::Pawn, Color::Black).is_set(Square::new(48 + file)));
    }

    // Test White pieces
    assert!(board.pieces_color(Piece::Rook, Color::White).is_set(Square::A1)); // a1
    assert!(board.pieces_color(Piece::Knight, Color::White).is_set(Square::B1)); // b1
    assert!(board.pieces_color(Piece::Bishop, Color::White).is_set(Square::C1)); // c1
    assert!(board.pieces_color(Piece::Queen, Color::White).is_set(Square::D1)); // d1
    assert!(board.pieces_color(Piece::King, Color::White).is_set(Square::E1)); // e1
    assert!(board.pieces_color(Piece::Bishop, Color::White).is_set(Square::F1)); // f1
    assert!(board.pieces_color(Piece::Knight, Color::White).is_set(Square::G1)); // g1
    assert!(board.pieces_color(Piece::Rook, Color::White).is_set(Square::H1)); // h1

    // Test Black pieces
    assert!(board.pieces_color(Piece::Rook, Color::Black).is_set(Square::A8)); // a8
    assert!(board.pieces_color(Piece::Knight, Color::Black).is_set(Square::B8)); // b8
    assert!(board.pieces_color(Piece::Bishop, Color::Black).is_set(Square::C8)); // c8
    assert!(board.pieces_color(Piece::Queen, Color::Black).is_set(Square::D8)); // d8
    assert!(board.pieces_color(Piece::King, Color::Black).is_set(Square::E8)); // e8
    assert!(board.pieces_color(Piece::Bishop, Color::Black).is_set(Square::F8)); // f8
    assert!(board.pieces_color(Piece::Knight, Color::Black).is_set(Square::G8)); // g8
    assert!(board.pieces_color(Piece::Rook, Color::Black).is_set(Square::H8)); // h8

    // Test side to move
    assert_eq!(board.side_to_move, Color::White);

    // Test castling rights
    assert!(board.castling_rights.can_castle(Color::White, true)); // White kingside
    assert!(board.castling_rights.can_castle(Color::White, false)); // White queenside
    assert!(board.castling_rights.can_castle(Color::Black, true)); // Black kingside
    assert!(board.castling_rights.can_castle(Color::Black, false)); // Black queenside

    // Test en passant square
    // Since en passant target square is "-", it should be none or unset
    assert!(board.en_passant_square.is_none());
}

#[test]
fn test_to_fen() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    assert_eq!(board.to_fen(), fen);

    // Test that FEN preserves the half-move clock and the absence of castling rights
    {
        let fen = "k7/8/8/8/8/8/7R/7K b - - 42 99";
        let expected_fen = "k7/8/8/8/8/8/7R/7K b - - 42 1";
        let moved_fen = "1k6/8/8/8/8/8/7R/7K w - - 43 1";
        let mut board = Board::from_fen(fen);
        let mv = ChessMove::new(Square::A8, Square::B8);

        assert_eq!(board.to_fen(), expected_fen);
        assert_eq!(board.make_move_new(mv).unwrap().to_fen(), moved_fen);

        let unmake_data = board.unmake_data();
        let metadata = board.make_move_metadata(mv).unwrap();
        assert_eq!(board.to_fen(), moved_fen);

        board.unmake_move(mv, metadata, unmake_data).unwrap();
        assert_eq!(board.to_fen(), expected_fen);
    }
}

#[test]
fn test_validate_move() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    // Test that you can't push a pawn three squares
    {
        let mut board = board;

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::Pawn, Color::White),
            BitBoard(0x000000000000FF00)
        );
        assert_eq!(board.en_passant_square, None);

        assert_eq!(
            board.validate_move(ChessMove::new(Square::E2, Square::E5)),
            Err(Error::InvalidMove)
        );

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::Pawn, Color::White),
            BitBoard(0x000000000000FF00)
        );
        assert_eq!(board.en_passant_square, None);
    }

    // Test that you can't capture your own pieces
    {
        let mut board = board;

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::King, Color::White),
            BitBoard(0x000000000000010)
        );

        assert_eq!(
            board.validate_move(ChessMove::new(Square::E1, Square::D1)),
            Err(Error::InvalidMove)
        );

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::King, Color::White),
            BitBoard(0x000000000000010)
        );
    }

    // Test that castling is not possible from the start position
    {
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::King, Color::White),
            BitBoard(0x000000000000010)
        );

        assert_eq!(
            board.validate_move(ChessMove::new(Square::E1, Square::G1)),
            Err(Error::InvalidMove)
        );

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::King, Color::White),
            BitBoard(0x000000000000010)
        );
    }

    // Test that you can't move a pinned piece.
    {
        let fen = "r1bqk2r/ppp2ppp/2np1n2/4p3/1bB1P3/2NP1N2/PPP2PPP/R1BQK2R w KQkq - 0 6";
        let mut board = Board::from_fen(fen);

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::Knight, Color::White),
            BitBoard(0x000000000240000)
        );

        assert_eq!(
            board.validate_move(ChessMove::new(Square::C3, Square::D5)),
            Err(Error::CannotMovePinned)
        );

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::Knight, Color::White),
            BitBoard(0x000000000240000)
        );
    }

    // Test that only a pawn reaching its last rank can promote
    {
        let fen = "7k/P7/8/8/8/8/7P/6K1 w - - 0 1";
        let mut board = Board::from_fen(fen);

        assert_eq!(
            board.validate_move(ChessMove::new(Square::A7, Square::A8)),
            Err(Error::InvalidMove)
        );

        let early_promotion = ChessMove::new_promotion(Square::H2, Square::H4, Piece::Queen);
        assert_eq!(
            board.validate_move(early_promotion),
            Err(Error::InvalidMove)
        );

        for piece in [Piece::Pawn, Piece::King] {
            let invalid_promotion = ChessMove::new_promotion(Square::A7, Square::A8, piece);
            assert_eq!(
                board.validate_move(invalid_promotion),
                Err(Error::InvalidMove)
            );
        }

        let promotion = ChessMove::new_promotion(Square::A7, Square::A8, Piece::Knight);
        assert_eq!(board.validate_move(promotion), Ok(Piece::Pawn));
    }
}

#[test]
fn test_make_move() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    // Test that you can push a pawn
    {
        let mut board = board;

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::Pawn, Color::White),
            BitBoard(0x000000000000FF00)
        );
        assert_eq!(board.en_passant_square, None);

        assert_eq!(
            board.make_move(ChessMove::new(Square::E2, Square::E4)),
            Ok(())
        );

        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(
            board.pieces_color(Piece::Pawn, Color::White),
            BitBoard(0x00000001000EF00)
        );
        assert_eq!(board.en_passant_square, None);
    }

    // Test that rejecting a pinned move leaves the board unchanged
    {
        let fen = "k3r3/8/8/8/8/8/4R3/4K3 w - - 0 1";
        let board = Board::from_fen(fen);
        let mut moved_board = board;
        let mv = ChessMove::new(Square::E2, Square::F2);

        assert!(moved_board.generate_moves_vec(!EMPTY).contains(&mv));
        assert_eq!(board.make_move_new(mv), Err(Error::CannotMovePinned));
        assert_eq!(moved_board.make_move(mv), Err(Error::CannotMovePinned));
        assert_eq!(moved_board, board);
    }

    // Test that an en passant capture cannot expose the king to check
    {
        let fen = "k7/8/8/r3pPK1/8/8/8/8 w - e6 0 2";
        let board = Board::from_fen(fen);
        let mut moved_board = board;
        let mv = ChessMove::new(Square::F5, Square::E6);

        assert!(moved_board.generate_moves_vec(!EMPTY).contains(&mv));
        assert_eq!(board.make_move_new(mv), Err(Error::CannotMovePinned));
        assert_eq!(moved_board.make_move(mv), Err(Error::CannotMovePinned));
        assert_eq!(moved_board, board);
        assert_eq!(
            moved_board.make_move_metadata(mv),
            Err(Error::CannotMovePinned)
        );
        assert_eq!(moved_board, board);
    }

    // Test that moving from an empty square leaves the board unchanged
    {
        let mut moved_board = board;
        let mv = ChessMove::new(Square::A3, Square::A4);

        assert_eq!(moved_board.make_move(mv), Err(Error::NoPieceOnSquare));
        assert_eq!(moved_board, board);
        assert_eq!(
            moved_board.make_move_metadata(mv),
            Err(Error::NoPieceOnSquare)
        );
        assert_eq!(moved_board, board);
    }
}

#[test]
fn test_pinned_bitboard() {
    let fen = "r1bqk2r/pppp1ppp/2n2n2/4p3/1bB1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 1 5";
    let mut board = Board::from_fen(fen);

    assert_eq!(board.pinned, EMPTY);

    board
        .make_move(ChessMove::new(Square::B1, Square::C3))
        .unwrap();

    board
        .make_move(ChessMove::new(Square::E8, Square::G8))
        .unwrap();

    assert_eq!(board.pinned, BitBoard(0x40000));
}

#[test]
fn test_hash() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    assert_eq!(board.hash, 0x89FB87680071BBF1);
    assert_eq!(board.hash(), 0x1D0D28B8BD0816CA);

    board
        .make_move(ChessMove::new(Square::E2, Square::E4))
        .unwrap();

    assert_eq!(board.hash, 0xD392461D4BD9B816);
    assert_eq!(board.hash(), 0xE4866C8BF44CB8F2);
}

#[test]
fn test_can_castle() {
    let fen = "r1bqk2r/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N2/PP3PPP/RNBQK2R w KQkq - 1 6";
    let mut board = Board::from_fen(fen);

    {
        let board = board;

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::King, Color::White),
            BitBoard(0x000000000000010)
        );

        assert_eq!(board.can_castle(true), Ok(()));

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(
            board.pieces_color(Piece::King, Color::White),
            BitBoard(0x000000000000010)
        );
    }

    assert_eq!(
        board.make_move(ChessMove::new(Square::E1, Square::G1)),
        Ok(())
    );
    assert_eq!(
        board.pieces_color(Piece::King, Color::White),
        BitBoard(0x000000000000040)
    );
    assert_eq!(
        board.pieces_color(Piece::Rook, Color::White),
        BitBoard(0x000000000000021)
    );

    {
        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(
            board.pieces_color(Piece::King, Color::Black),
            BitBoard(0x1000000000000000)
        );

        assert_eq!(board.can_castle(true), Ok(()));

        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(
            board.pieces_color(Piece::King, Color::Black),
            BitBoard(0x1000000000000000)
        );
    }
}

#[test]
fn test_revoke_castling_rights() {
    let mut rights = CastlingRights::from_fen("KQkq");

    rights.revoke(Color::White, true);
    assert_eq!(rights.to_fen(), "Qkq");

    rights.revoke_all(Color::Black);
    assert_eq!(rights.to_fen(), "Q");
}
