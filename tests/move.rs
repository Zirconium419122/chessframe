use chess_frame::{piece::Piece, r#move::Move, square::Square};

#[test]
fn test_pretty_print() {
    {
        let mv = Move::new(Square::E2, Square::E4);
        assert_eq!(mv.to_string(), "e2e4");
    }

    {
        let mv = Move::new(Square::B1, Square::C3);
        assert_eq!(mv.to_string(), "b1c3");
    }

    {
        let mv = Move::new(Square::E1, Square::G1);
        assert_eq!(mv.to_string(), "e1g1");
    }

    {
        let mv = Move::new(Square::F1, Square::C4);
        assert_eq!(mv.to_string(), "f1c4");
    }

    {
        let mv = Move::new_promotion(Square::G7, Square::G8, Piece::Queen);
        assert_eq!(mv.to_string(), "g7g8q");
    }
}
