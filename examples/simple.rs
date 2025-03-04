use std::{io, str::FromStr};

use chessframe::{
    bitboard::EMPTY,
    board::Board,
    chess_move::ChessMove,
    color::Color,
    piece::{Piece, PIECES},
    uci::*,
};

const PIECE_VALUES: [isize; 6] = [100, 300, 325, 500, 900, 0];

/// MVV_LVA[victim][attacker]
const MVV_LVA: [[i8; 6]; 6] = [
    [15, 14, 13, 12, 11, 10], // victim Pawn, attacker P, N, B, R, Q, K
    [25, 24, 23, 22, 21, 20], // victim Knight, attacker P, N, B, R, Q, K
    [35, 34, 33, 32, 31, 30], // victim Bishop, attacker P, N, B, R, Q, K
    [45, 44, 43, 42, 41, 40], // victim Rook, attacker P, N, B, R, Q, K
    [55, 54, 53, 52, 51, 50], // victim Queen, attacker P, N, B, R, Q, K
    [0, 0, 0, 0, 0, 0],       // victim King, attacker P, N, B, R, Q, K
];

fn get_mvv_lva(victim: Piece, attacker: Piece) -> i8 {
    unsafe {
        *MVV_LVA
            .get_unchecked(victim.to_index())
            .get_unchecked(attacker.to_index())
    }
}

struct SimpleMoveMaker {
    board: Option<Board>,
    quitting: bool,
}

impl SimpleMoveMaker {
    const SEARCH_DEPTH: usize = 7;
    const MATE_SCORE: isize = 1_000_000_000;

    pub fn new() -> SimpleMoveMaker {
        SimpleMoveMaker {
            board: None,
            quitting: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.handle_command();

            if self.quitting {
                break;
            }
        }
    }

    pub fn search_base(board: &Board, depth: usize) -> (isize, Option<ChessMove>) {
        let mut max = isize::MIN;
        let mut best_move = None;

        let alpha = isize::MIN;
        let beta = isize::MAX;

        let mut moves = board.generate_moves_vec(!EMPTY);
        Self::sort_moves(board, &mut moves);
        for mv in moves {
            if let Ok(board) = board.make_move_new(&mv) {
                let score = -Self::search(&board, alpha, beta, depth - 1);

                if score > max {
                    max = score;
                    best_move = Some(mv);
                }
            }
        }

        (max, best_move)
    }

    #[rustfmt::skip]
    fn search(board: &Board, mut alpha: isize, beta: isize, depth: usize) -> isize {
        if depth == 0 {
            return Self::quiescence_search(board, alpha, beta);
        }

        let mut legal_moves = false;
        let mut max = isize::MIN;

        let mut moves = board.generate_moves_vec(!EMPTY);
        Self::sort_moves(board, &mut moves);
        for mv in moves {
            if let Ok(board) = board.make_move_new(&mv) {
                legal_moves = true;
                let score = -Self::search(&board, -beta, -alpha, depth - 1);

                if score > max {
                    max = score;
                    if score > alpha {
                        alpha = score;
                    }
                }
                if score >= beta {
                    return max;
                }
            }
        }

        if !legal_moves {
            if board.in_check() {
                return -SimpleMoveMaker::MATE_SCORE + SimpleMoveMaker::SEARCH_DEPTH as isize - depth as isize;
            } else {
                return 0;
            }
        }

        max
    }

    fn quiescence_search(board: &Board, mut alpha: isize, beta: isize) -> isize {
        let evaluation = Self::evaluate(board);
        if evaluation >= beta {
            return beta;
        }
        if evaluation > alpha {
            alpha = evaluation;
        }

        let mut moves = board.generate_moves_vec(board.occupancy(!board.side_to_move));
        Self::sort_moves(board, &mut moves);

        for mv in moves {
            if let Ok(board) = board.make_move_new(&mv) {
                let score = -Self::quiescence_search(&board, -beta, -alpha);

                if score >= beta {
                    return beta;
                }
                if score > alpha {
                    alpha = score;
                }
            }
        }

        alpha
    }

    fn evaluate(board: &Board) -> isize {
        let mut score = 0;

        for piece in PIECES.iter() {
            score += board.pieces_color(*piece, Color::White).count_ones() as isize
                * PIECE_VALUES[piece.to_index()];
            score -= board.pieces_color(*piece, Color::Black).count_ones() as isize
                * PIECE_VALUES[piece.to_index()];
        }

        if board.in_check() {
            score -= 50;
        }

        let perspective = if board.side_to_move == Color::White {
            1
        } else {
            -1
        };
        score * perspective
    }

    fn sort_moves(board: &Board, moves: &mut Vec<ChessMove>) {
        moves.sort_by_key(|mv| -Self::score_move(board, *mv));
    }

    fn score_move(board: &Board, mv: ChessMove) -> isize {
        let moved = unsafe { board.get_piece(mv.from).unwrap_unchecked() };

        if let Some(capture) = board.get_piece(mv.to) {
            return get_mvv_lva(capture, moved) as isize;
        }

        0
    }
}

impl Uci for SimpleMoveMaker {
    fn send_command(&mut self, command: UciCommand) {
        match command {
            UciCommand::Id { name, author } => {
                println!("id name {}", name);
                println!("id author {}", author);
            }
            UciCommand::UciOk => {
                println!("uciok");
            }
            UciCommand::ReadyOk => {
                println!("readyok");
            }
            UciCommand::BestMove { best_move, ponder } => {
                if let Some(ponder) = ponder {
                    println!("bestmove {} ponder {}", best_move, ponder);
                } else {
                    println!("bestmove {}", best_move);
                }
            }
            UciCommand::Info(info) => {
                println!("{}", info);
            }
            _ => {}
        }
    }

    fn read_command(&mut self) -> Option<UciCommand> {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        UciCommand::from_str(line.trim()).ok()
    }

    #[rustfmt::skip]
    fn handle_command(&mut self) {
        if let Some(command) = self.read_command() {
            match command {
                UciCommand::Uci => {
                    self.send_command(UciCommand::Id {
                        name: "Simple Move Maker".to_string(),
                        author: "Zirconium419122".to_string(),
                    });
                    self.send_command(UciCommand::UciOk);
                }
                UciCommand::Debug(debug) => {
                    if debug {
                        self.send_command(UciCommand::Info(Info {
                            string: Some("Debug mode not supported!".to_string()),
                            ..Default::default()
                        }));
                    }
                }
                UciCommand::IsReady => {
                    self.send_command(UciCommand::ReadyOk);
                }
                UciCommand::UciNewGame => self.board = None,
                UciCommand::Position { fen, moves } => {
                    if fen == "startpos" {
                        self.board = Some(Board::default());
                    } else {
                        self.board = Some(Board::from_fen(&fen));
                    };

                    if let Some(moves) = moves {
                        let board = self.board.as_mut().unwrap();

                        for mv in moves {
                            let mv = board.infer_move(&mv).unwrap();

                            let _ = board.make_move(&mv);
                        }
                    }
                }
                UciCommand::Go { .. } => {
                    if let Some(ref board) = self.board {
                        let (score, best_move) = Self::search_base(&board, SimpleMoveMaker::SEARCH_DEPTH);

                        if let Some(best_move) = best_move {
                            if score.abs() >= SimpleMoveMaker::MATE_SCORE - 100 {
                                let correction = if score > 0 { 1 } else { -1 };
                                let moves_to_mate = SimpleMoveMaker::MATE_SCORE - score.abs();
                                let mate_in_moves = (moves_to_mate / 2) + 1;

                                let mut score = Score::default();
                                score.mate = Some(correction * mate_in_moves);

                                self.send_command(UciCommand::Info(Info {
                                    pv: Some(best_move.to_string()),
                                    score: Some(score),
                                    ..Default::default()
                                }));
                            } else {
                                let cp = score;

                                let mut score = Score::default();
                                score.cp = Some(cp);

                                self.send_command(UciCommand::Info(Info {
                                    pv: Some(best_move.to_string()),
                                    score: Some(score),
                                    ..Default::default()
                                }));
                            }
                            self.send_command(UciCommand::BestMove {
                                best_move: best_move.to_string(),
                                ponder: None,
                            });
                        }
                    }
                }
                UciCommand::Stop => {}
                UciCommand::Quit => self.quitting = true,
                _ => {}
            }
        }
    }
}

fn main() {
    let mut engine = SimpleMoveMaker::new();
    engine.run();
}
