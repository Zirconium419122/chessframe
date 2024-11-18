use chess_frame::{chess_move::ChessMove, piece::Piece, square::Square};

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
