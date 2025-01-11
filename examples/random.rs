use std::io;

use chess_frame::{
    bitboard::EMPTY,
    board::Board,
    uci::{Uci, UciCommand},
};
use rand_chacha::rand_core::{RngCore, SeedableRng};

struct RandomMoveMaker {
    board: Option<Board>,
    rng: rand_chacha::ChaCha8Rng,
    quitting: bool,
}

impl RandomMoveMaker {
    fn new() -> Self {
        Self {
            board: None,
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(123),
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

impl Uci for RandomMoveMaker {
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
                        name: "Random Move Maker".to_string(),
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
                    if let Some(ref mut board) = self.board {
                        let mut moves = Vec::new();

                        for mv in board.generate_moves_vec(!EMPTY) {
                            if let Ok(_) = board.make_move_new(&mv) {
                                moves.push(mv);
                            }
                        }

                        let best_move = moves[self.rng.next_u32() as usize % moves.len()].clone();

                        self.send_command(UciCommand::Info(format!("pv {}", best_move)));
                        self.send_command(UciCommand::BestMove {
                            best_move: best_move.to_string(),
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
    let mut engine = RandomMoveMaker::new();
    engine.run();
}
