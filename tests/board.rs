use chess_frame::{
    bitboard::BitBoard,
    board::*,
    color::Color,
    piece::Piece,
    r#move::{Move, Square},
};

#[test]
fn test_square_to_bitboard() {
    macro_rules! generate_assertions {
        ($($x:literal),+) => {
            $(
                assert_eq!(BitBoard::from(Square::try_from($x).unwrap()), BitBoard(1 << $x));
            )+
        };
        ($start:literal..$end:literal) => {
            {
                for i in $start..$end {
                    assert_eq!(BitBoard::from(Square::try_from(i).unwrap()), BitBoard(1 << i));
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
        assert!(board.pieces[Piece::Pawn.piece_index(&Color::White)].is_set(8 as usize + file));
    }
    // Black pawns
    for file in 0..8 {
        // Rank 7 (index 48-55)
        assert!(board.pieces[Piece::Pawn.piece_index(&Color::Black)].is_set(48 as usize + file));
    }

    // Test White pieces
    assert!(board.pieces[Piece::Rook.piece_index(&Color::White)].is_set(0 as usize)); // a1
    assert!(board.pieces[Piece::Knight.piece_index(&Color::White)].is_set(1 as usize)); // b1
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::White)].is_set(2 as usize)); // c1
    assert!(board.pieces[Piece::Queen.piece_index(&Color::White)].is_set(3 as usize)); // d1
    assert!(board.pieces[Piece::King.piece_index(&Color::White)].is_set(4 as usize)); // e1
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::White)].is_set(5 as usize)); // f1
    assert!(board.pieces[Piece::Knight.piece_index(&Color::White)].is_set(6 as usize)); // g1
    assert!(board.pieces[Piece::Rook.piece_index(&Color::White)].is_set(7 as usize)); // h1

    // Test Black pieces
    assert!(board.pieces[Piece::Rook.piece_index(&Color::Black)].is_set(56 as usize)); // a8
    assert!(board.pieces[Piece::Knight.piece_index(&Color::Black)].is_set(57 as usize)); // b8
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::Black)].is_set(58 as usize)); // c8
    assert!(board.pieces[Piece::Queen.piece_index(&Color::Black)].is_set(59 as usize)); // d8
    assert!(board.pieces[Piece::King.piece_index(&Color::Black)].is_set(60 as usize)); // e8
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::Black)].is_set(61 as usize)); // f8
    assert!(board.pieces[Piece::Knight.piece_index(&Color::Black)].is_set(62 as usize)); // g8
    assert!(board.pieces[Piece::Rook.piece_index(&Color::Black)].is_set(63 as usize)); // h8

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

    // Test halfmove clock
    assert_eq!(board.half_move_clock, 0);

    // Test fullmove number
    assert_eq!(board.full_move_clock, 1);
}

#[test]
fn test_make_move() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    // Test that you can push a pawn
    {
        let mut board = board.clone();

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[0], BitBoard(0x000000000000FF00));
        assert_eq!(board.en_passant_square, None);

        assert_eq!(board.make_move(Move::new(Square::E2, Square::E4)), Ok(()));

        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(board.pieces[0], BitBoard(0x00000001000EF00));
        assert_eq!(board.en_passant_square, Some(BitBoard(0x100000)));
    }

    // Test that you can't capture your own pieces
    {
        let mut board = board.clone();

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[5], BitBoard(0x000000000000010));

        assert_eq!(
            board.make_move(Move::new_capture(Square::E1, Square::D1)),
            Err("Can't move piece to square: 3!".to_string())
        );

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[5], BitBoard(0x000000000000010));
    }

    // Test that castling is not possible from the start position
    {
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[5], BitBoard(0x000000000000010));

        assert_eq!(
            board.make_move(Move::new(Square::E1, Square::G1)),
            Err("Can't move piece to square: 6!".to_string())
        );

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[5], BitBoard(0x000000000000010));
    }
}

#[test]
fn test_unmake_move() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    // Test initialy no move to unmake
    {
        assert_eq!(board.board_history.len(), 0);
        assert_eq!(board.unmake_move(), Err("No move to unmake!"));
    }

    // Test that you can make a move
    {
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[0], BitBoard(0x000000000000FF00));
        assert_eq!(board.en_passant_square, None);

        assert_eq!(board.make_move(Move::new(Square::E2, Square::E4)), Ok(()));

        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(board.pieces[0], BitBoard(0x00000001000EF00));
        assert_eq!(board.en_passant_square, Some(BitBoard(0x100000)));
    }

    // Test that you can unmake a move
    {
        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(board.pieces[0], BitBoard(0x00000001000EF00));

        assert_eq!(board.unmake_move(), Ok(()));

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[0], BitBoard(0x000000000000FF00));
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
        board.make_move(Move::new_castle(Square::E1, Square::G1)),
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
