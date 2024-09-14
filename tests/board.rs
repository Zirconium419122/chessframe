use chess_frame::{board::*, color::Color, piece::Piece, r#move::Square};

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
        assert!(board.pieces[Piece::Pawn.piece_index(&Color::White)].is_set(8 + file));
    }
    // Black pawns
    for file in 0..8 {
        // Rank 7 (index 48-55)
        assert!(board.pieces[Piece::Pawn.piece_index(&Color::Black)].is_set(48 + file));
    }

    // Test White pieces
    assert!(board.pieces[Piece::Rook.piece_index(&Color::White)].is_set(0)); // a1
    assert!(board.pieces[Piece::Knight.piece_index(&Color::White)].is_set(1)); // b1
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::White)].is_set(2)); // c1
    assert!(board.pieces[Piece::Queen.piece_index(&Color::White)].is_set(3)); // d1
    assert!(board.pieces[Piece::King.piece_index(&Color::White)].is_set(4)); // e1
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::White)].is_set(5)); // f1
    assert!(board.pieces[Piece::Knight.piece_index(&Color::White)].is_set(6)); // g1
    assert!(board.pieces[Piece::Rook.piece_index(&Color::White)].is_set(7)); // h1

    // Test Black pieces
    assert!(board.pieces[Piece::Rook.piece_index(&Color::Black)].is_set(56)); // a8
    assert!(board.pieces[Piece::Knight.piece_index(&Color::Black)].is_set(57)); // b8
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::Black)].is_set(58)); // c8
    assert!(board.pieces[Piece::Queen.piece_index(&Color::Black)].is_set(59)); // d8
    assert!(board.pieces[Piece::King.piece_index(&Color::Black)].is_set(60)); // e8
    assert!(board.pieces[Piece::Bishop.piece_index(&Color::Black)].is_set(61)); // f8
    assert!(board.pieces[Piece::Knight.piece_index(&Color::Black)].is_set(62)); // g8
    assert!(board.pieces[Piece::Rook.piece_index(&Color::Black)].is_set(63)); // h8

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
