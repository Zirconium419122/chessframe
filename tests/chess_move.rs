use chessframe::{board::Board, chess_move::ChessMove, piece::Piece, square::Square};

#[test]
fn test_pretty_print() {
    {
        let mv = ChessMove::new(Square::E2, Square::E4);
        assert_eq!(mv.to_string(), "e2e4");
    }

    {
        let mv = ChessMove::new(Square::B1, Square::C3);
        assert_eq!(mv.to_string(), "b1c3");
    }

    {
        let mv = ChessMove::new(Square::E1, Square::G1);
        assert_eq!(mv.to_string(), "e1g1");
    }

    {
        let mv = ChessMove::new(Square::F1, Square::C4);
        assert_eq!(mv.to_string(), "f1c4");
    }

    {
        let mv = ChessMove::new_promotion(Square::G7, Square::G8, Piece::Queen);
        assert_eq!(mv.to_string(), "g7g8q");
    }
}

#[test]
fn test_make_null_move() {
    {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut board = Board::from_fen(fen);

        assert!(board.make_null_move().is_ok());

        let inverted_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        let inveted_board = Board::from_fen(inverted_fen);

        assert_eq!(board, inveted_board);
    }

    {
        let fen = "r1bqk2r/ppp2ppp/2np1n2/4p3/2B1P3/P1bP1N2/1PP2PPP/R1BQK2R w KQkq - 0 7";
        let mut board = Board::from_fen(fen);

        assert!(board.make_null_move().is_err());
    }
}
