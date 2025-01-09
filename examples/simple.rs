use std::io;

use chess_frame::{
    bitboard::EMPTY, board::Board, color::Color, piece::PIECES, uci::{Uci, UciCommand}
};

const PIECE_VALUES: [isize; 6] = [100, 300, 325, 500, 900, 0];

struct SimpleMoveMaker {
    board: Option<Board>,
    quitting: bool,
}

impl SimpleMoveMaker {
    pub fn new() -> Self {
        Self {
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

    fn search(board: &Board, depth: usize) -> isize {
        if depth == 0 {
            return Self::evaluate(board);
        }

        let mut max = isize::MIN;

        for mv in board.generate_moves_vec(!EMPTY) {
            if let Ok(board) = board.make_move_new(&mv) {
                let score = -Self::search(&board, depth - 1);

                if score > max {
                    max = score;
                }
            }
        }

        max
    }

    fn evaluate(board: &Board) -> isize {
        let mut score = 0;

        for piece in PIECES.iter() {
            score += board.pieces_color(*piece, Color::White).count_ones() as isize * PIECE_VALUES[piece.to_index()];
            score -= board.pieces_color(*piece, Color::Black).count_ones() as isize * PIECE_VALUES[piece.to_index()];
        }

        score
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
                println!("info {}", info);
            }
            _ => {}
        }
    }

    fn read_command(&mut self) -> Option<UciCommand> {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        Some(UciCommand::from(line.trim().to_string()))
    }

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
                        self.send_command(UciCommand::Info(
                            "string Debug mode not supported!".to_string(),
                        ));
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
                        let mut max = isize::MIN;
                        let mut best_move = None;

                        for mv in board.generate_moves_vec(!EMPTY) {
                            if let Ok(board) = board.make_move_new(&mv) {
                                let score = Self::search(&board, 4);

                                if score > max {
                                    max = score;
                                    best_move = Some(mv);
                                }
                            }
                        }

                        if let Some(best_move) = best_move {
                            self.send_command(UciCommand::Info(format!("pv {}", best_move)));
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
