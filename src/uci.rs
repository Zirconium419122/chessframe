impl From<String> for UciCommand {
    fn from(value: String) -> Self {
        let tokens: Vec<&str> = value.split_whitespace().collect();

        match tokens.first() {
            Some(&"uci") => UciCommand::Uci,
            Some(&"debug") => {
                if tokens.get(1) == Some(&"on") {
                    UciCommand::Debug(true)
                } else {
                    UciCommand::Debug(false)
                }
            }
            Some(&"isready") => UciCommand::IsReady,
            Some(&"setoption") => {
                let name_index = tokens.iter().position(|&x| x == "name").unwrap_or(1) + 1;
                let value_index = tokens
                    .iter()
                    .position(|&x| x == "value")
                    .unwrap_or(name_index + 1)
                    + 1;

                let name = tokens.get(name_index).unwrap_or(&"").to_string();
                let value = tokens.get(value_index).unwrap_or(&"").to_string();

                UciCommand::SetOption { name, value }
            }
            Some(&"register") => {
                let mut name = None;
                let mut code = None;
                let mut later = false;

                let mut i = 1;
                while i < tokens.len() {
                    match tokens[i] {
                        "name" => {
                            if let Some(val) = tokens.get(i + 1) {
                                name = Some(val.to_string());
                            }
                            i += 2;
                        }
                        "code" => {
                            if let Some(val) = tokens.get(i + 1) {
                                code = Some(val.to_string());
                            }
                            i += 2;
                        }
                        "later" => {
                            later = true;
                            i += 1;
                        }
                        _ => i += 1,
                    }
                }

                UciCommand::Register { name, code, later }
            }
            Some(&"ucinewgame") => UciCommand::UciNewGame,
            Some(&"position") => {
                if tokens.get(1) == Some(&"fen") {
                    let mut fen = tokens[2..].join(" ");
                    let mut moves = Vec::new();

                    if let Some(moves_start) = tokens.iter().position(|&x| x == "moves") {
                        fen = fen.split("moves").collect::<Vec<&str>>()[0].to_string();

                        for mv in &tokens[moves_start + 1..] {
                            moves.push(mv.to_string());
                        }
                    }

                    UciCommand::Position {
                        fen,
                        moves: Some(moves),
                    }
                } else if tokens.get(1) == Some(&"startpos") {
                    let mut moves = Vec::new();

                    if let Some(moves_start) = tokens.iter().position(|&x| x == "moves") {
                        for mv in &tokens[moves_start + 1..] {
                            moves.push(mv.to_string());
                        }
                    }

                    UciCommand::Position {
                        fen: "startpos".to_string(),
                        moves: Some(moves),
                    }
                } else {
                    UciCommand::Info("string Please input a valid command".into())
                }
            }
            Some(&"go") => {
                let mut wtime = None;
                let mut btime = None;
                let mut winc = None;
                let mut binc = None;
                let mut moves_to_go = None;
                let mut depth = None;
                let mut nodes = None;
                let mut mate = None;
                let mut move_time = None;
                let mut infinite = false;
                let mut ponder = false;

                let mut i = 1;
                while i < tokens.len() {
                    match tokens[i] {
                        "wtime" => {
                            if let Some(val) = tokens.get(i + 1) {
                                wtime = val.parse().ok();
                            }
                            i += 2;
                        }
                        "btime" => {
                            if let Some(val) = tokens.get(i + 1) {
                                btime = val.parse().ok();
                            }
                            i += 2;
                        }
                        "winc" => {
                            if let Some(val) = tokens.get(i + 1) {
                                winc = val.parse().ok();
                            }
                            i += 2;
                        }
                        "binc" => {
                            if let Some(val) = tokens.get(i + 1) {
                                binc = val.parse().ok();
                            }
                            i += 2;
                        }
                        "movestogo" => {
                            if let Some(val) = tokens.get(i + 1) {
                                moves_to_go = val.parse().ok();
                            }
                            i += 2;
                        }
                        "depth" => {
                            if let Some(val) = tokens.get(i + 1) {
                                depth = val.parse().ok();
                            }
                            i += 2;
                        }
                        "nodes" => {
                            if let Some(val) = tokens.get(i + 1) {
                                nodes = val.parse().ok();
                            }
                            i += 2;
                        }
                        "mate" => {
                            if let Some(val) = tokens.get(i + 1) {
                                mate = val.parse().ok();
                            }
                            i += 2;
                        }
                        "movetime" => {
                            if let Some(val) = tokens.get(i + 1) {
                                move_time = val.parse().ok();
                            }
                            i += 2;
                        }
                        "infinite" => {
                            infinite = true;
                            i += 1;
                        }
                        "ponder" => {
                            ponder = true;
                            i += 1;
                        }
                        _ => i += 1,
                    }
                }

                UciCommand::Go {
                    wtime,
                    btime,
                    winc,
                    binc,
                    moves_to_go,
                    depth,
                    nodes,
                    mate,
                    move_time,
                    infinite,
                    ponder,
                }
            }
            Some(&"stop") => UciCommand::Stop,
            Some(&"ponderhit") => UciCommand::PonderHit,
            Some(&"quit") => UciCommand::Quit,
            Some(&"id") => {
                let id_type = tokens.get(1).unwrap_or(&"");
                let id_value = tokens[2..].join(" ");

                match *id_type {
                    "name" => UciCommand::Id {
                        name: id_value,
                        author: String::new(),
                    },
                    "author" => UciCommand::Id {
                        name: String::new(),
                        author: id_value,
                    },
                    _ => UciCommand::Info("Invalid id command".to_string()),
                }
            }
            Some(&"uciok") => UciCommand::UciOk,
            Some(&"readyok") => UciCommand::ReadyOk,
            Some(&"bestmove") => {
                let best_move = tokens.get(1).unwrap_or(&"").to_string();
                let ponder = if tokens.get(2) == Some(&"ponder") {
                    tokens.get(3).map(|&x| x.to_string())
                } else {
                    None
                };

                UciCommand::BestMove { best_move, ponder }
            }
            Some(&"copyprotection") => {
                if let Some(val) = tokens.get(1) {
                    return match *val {
                        "ok" => UciCommand::CopyProtection {
                            ok: true,
                            checking: false,
                            error: false,
                        },
                        "checking" => UciCommand::CopyProtection {
                            ok: false,
                            checking: true,
                            error: false,
                        },
                        "error" => UciCommand::CopyProtection {
                            ok: false,
                            checking: false,
                            error: true,
                        },
                        _ => UciCommand::Info("Invalid option".to_string()),
                    };
                }

                UciCommand::Info("Invalid copyprotection command".to_string())
            }
            Some(&"registration") => {
                if let Some(val) = tokens.get(1) {
                    return match *val {
                        "ok" => UciCommand::Registration {
                            ok: true,
                            checking: false,
                            error: false,
                        },
                        "checking" => UciCommand::Registration {
                            ok: false,
                            checking: true,
                            error: false,
                        },
                        "error" => UciCommand::Registration {
                            ok: false,
                            checking: false,
                            error: true,
                        },
                        _ => UciCommand::Info("Invalid option".to_string()),
                    };
                }

                UciCommand::Info("Invalid registration command".to_string())
            }
            Some(&"info") => UciCommand::Info(tokens[1..].join(" ")),
            Some(&"option") => UciCommand::Option(tokens[1..].join(" ")),
            _ => UciCommand::Info("Unknown command!".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UciCommand {
    // Basic commands from GUI to Engine
    Uci,
    Debug(bool),
    IsReady,

    SetOption {
        name: String,
        value: String,
    },

    Register {
        name: Option<String>,
        code: Option<String>,
        later: bool,
    },

    UciNewGame,

    Position {
        fen: String,
        moves: Option<Vec<String>>,
    },

    Go {
        wtime: Option<usize>,       // White's time left in ms
        btime: Option<usize>,       // Black's time left in ms
        winc: Option<usize>,        // White's increment in ms
        binc: Option<usize>,        // Black's increment in ms
        moves_to_go: Option<usize>, // Moves to next to next time control
        depth: Option<usize>,       // Limit search depth
        nodes: Option<usize>,       // Limit search nodes
        mate: Option<usize>,        // Search for mate in n moves
        move_time: Option<usize>,   // Time per move in ms
        infinite: bool,             // Infinite time control
        ponder: bool,               // Engine should ponder
    },

    PonderHit,

    Stop,
    Quit,

    // Responses from Engine to GUI
    Id {
        name: String,
        author: String,
    },
    UciOk,
    ReadyOk,

    BestMove {
        best_move: String,
        ponder: Option<String>,
    },

    CopyProtection {
        ok: bool,
        checking: bool,
        error: bool,
    },
    Registration {
        ok: bool,
        checking: bool,
        error: bool,
    },

    Info(String),
    Option(String),
}

pub trait Uci {
    fn send_command(&mut self, command: UciCommand);
    fn read_command(&mut self) -> Option<UciCommand>;
    fn handle_command(&mut self);
}
