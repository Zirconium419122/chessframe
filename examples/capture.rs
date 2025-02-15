use std::{io, str::FromStr};

use chessframe::{bitboard::EMPTY, board::Board, uci::*};

struct CaptureMaker {
    board: Option<Board>,
    quitting: bool,
}

impl CaptureMaker {
    pub fn new() -> CaptureMaker {
        CaptureMaker {
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
}

impl Uci for CaptureMaker {
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

    fn handle_command(&mut self) {
        if let Some(command) = self.read_command() {
            match command {
                UciCommand::Uci => {
                    self.send_command(UciCommand::Id {
                        name: "Capture Maker".to_string(),
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
                        let mut moves = Vec::new();

                        for mv in board.generate_moves_vec(board.occupancy(!board.side_to_move)) {
                            if let Ok(_) = board.make_move_new(&mv) {
                                moves.push(mv);
                            }
                        }

                        if moves.is_empty() {
                            for mv in board.generate_moves_vec(!EMPTY) {
                                if let Ok(_) = board.make_move_new(&mv) {
                                    moves.push(mv);
                                }
                            }
                        }

                        dbg!(&moves);
                        let mv = moves[0].clone();

                        self.send_command(UciCommand::Info(Info {
                            pv: Some(mv.to_string()),
                            ..Default::default()
                        }));
                        self.send_command(UciCommand::BestMove {
                            best_move: mv.to_string(),
                            ponder: None,
                        });
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
    let mut engine = CaptureMaker::new();
    engine.run();
}
