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

#[test]
fn test_pawn_move_generation() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    let pawn_pushes = board.generate_pawn_pushes();
    assert_eq!(pawn_pushes, BitBoard(0x00000000FFFF0000));

    let pawn_captures = board.generate_pawn_captures();
    assert_eq!(pawn_captures, BitBoard(0));

    let en_passant = board.generate_en_passant();
    assert_eq!(en_passant, BitBoard(0));

    let pawn_moves = board.generate_pawn_moves();
    assert_eq!(pawn_moves, BitBoard(0x00000000FFFF0000));
}

#[test]
fn test_knight_move_generation() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    let knight_moves = board.generate_knight_moves();
    assert_eq!(knight_moves, BitBoard(0x0000000000A50000))
}

#[test]
fn test_make_move() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    {
        let mut board = board.clone();

        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.pieces[0], BitBoard(0x000000000000FF00));

        assert_eq!(board.make_move(Move::new(Square::E2, Square::E4)), Ok(()));

        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(board.pieces[0], BitBoard(0x00000001000EF00));
    }

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
