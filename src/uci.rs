use std::{fmt, str::FromStr};

impl FromStr for UciCommand {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = value.split_whitespace().collect();

        match tokens.first() {
            Some(&"uci") => Ok(UciCommand::Uci),
            Some(&"debug") => {
                if tokens.get(1) == Some(&"on") {
                    Ok(UciCommand::Debug(true))
                } else {
                    Ok(UciCommand::Debug(false))
                }
            }
            Some(&"isready") => Ok(UciCommand::IsReady),
            Some(&"setoption") => {
                let name_index = tokens.iter().position(|&x| x == "name").unwrap_or(1) + 1;
                let value_index = tokens
                    .iter()
                    .position(|&x| x == "value")
                    .unwrap_or(name_index + 1)
                    + 1;

                let name = tokens.get(name_index).unwrap_or(&"").to_string();
                let value = tokens.get(value_index).unwrap_or(&"").to_string();

                Ok(UciCommand::SetOption { name, value })
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

                Ok(UciCommand::Register { name, code, later })
            }
            Some(&"ucinewgame") => Ok(UciCommand::UciNewGame),
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

                    Ok(UciCommand::Position {
                        fen,
                        moves: Some(moves),
                    })
                } else if tokens.get(1) == Some(&"startpos") {
                    let mut moves = Vec::new();

                    if let Some(moves_start) = tokens.iter().position(|&x| x == "moves") {
                        for mv in &tokens[moves_start + 1..] {
                            moves.push(mv.to_string());
                        }
                    }

                    Ok(UciCommand::Position {
                        fen: "startpos".to_string(),
                        moves: Some(moves),
                    })
                } else {
                    Err("Please input a valid position command")
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

                Ok(UciCommand::Go {
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
                })
            }
            Some(&"stop") => Ok(UciCommand::Stop),
            Some(&"ponderhit") => Ok(UciCommand::PonderHit),
            Some(&"quit") => Ok(UciCommand::Quit),
            Some(&"id") => {
                let id_type = tokens.get(1).unwrap_or(&"");
                let id_value = tokens[2..].join(" ");

                match *id_type {
                    "name" => Ok(UciCommand::Id {
                        name: id_value,
                        author: String::new(),
                    }),
                    "author" => Ok(UciCommand::Id {
                        name: String::new(),
                        author: id_value,
                    }),
                    _ => Err("Invalid id command"),
                }
            }
            Some(&"uciok") => Ok(UciCommand::UciOk),
            Some(&"readyok") => Ok(UciCommand::ReadyOk),
            Some(&"bestmove") => {
                let best_move = tokens.get(1).unwrap_or(&"").to_string();
                let ponder = if tokens.get(2) == Some(&"ponder") {
                    tokens.get(3).map(|&x| x.to_string())
                } else {
                    None
                };

                Ok(UciCommand::BestMove { best_move, ponder })
            }
            Some(&"copyprotection") => {
                if let Some(val) = tokens.get(1) {
                    return match *val {
                        "ok" => Ok(UciCommand::CopyProtection {
                            ok: true,
                            checking: false,
                            error: false,
                        }),
                        "checking" => Ok(UciCommand::CopyProtection {
                            ok: false,
                            checking: true,
                            error: false,
                        }),
                        "error" => Ok(UciCommand::CopyProtection {
                            ok: false,
                            checking: false,
                            error: true,
                        }),
                        _ => Err("Invalid option"),
                    };
                }

                Err("Invalid copyprotection command")
            }
            Some(&"registration") => {
                if let Some(val) = tokens.get(1) {
                    return match *val {
                        "ok" => Ok(UciCommand::Registration {
                            ok: true,
                            checking: false,
                            error: false,
                        }),
                        "checking" => Ok(UciCommand::Registration {
                            ok: false,
                            checking: true,
                            error: false,
                        }),
                        "error" => Ok(UciCommand::Registration {
                            ok: false,
                            checking: false,
                            error: true,
                        }),
                        _ => Err("Invalid option"),
                    };
                }

                Err("Invalid registration command")
            }
            Some(&"info") => {
                let mut depth = None;
                let mut seldepth = None;
                let mut time = None;
                let mut nodes = None;
                let mut pv = None;
                let mut multipv = None;
                let mut score_cp = None;
                let mut score_mate = None;
                let mut score_lowerbound = false;
                let mut score_upperbound = false;
                let mut currmove = None;
                let mut currmove_number = None;
                let mut hashfull = None;
                let mut nps = None;
                let mut tbhits = None;
                let mut sbhits = None;
                let mut cpuload = None;
                let mut string = None;
                let mut refutation = None;
                let mut currline = None;

                let mut i = 1;
                while i < tokens.len() {
                    match tokens[i] {
                        "depth" => {
                            if let Some(val) = tokens.get(i + 1) {
                                depth = val.parse().ok();
                            }
                            i += 2;
                        }
                        "seldepth" => {
                            if let Some(val) = tokens.get(i + 1) {
                                seldepth = val.parse().ok();
                            }
                            i += 2;
                        }
                        "time" => {
                            if let Some(val) = tokens.get(i + 1) {
                                time = val.parse().ok();
                            }
                            i += 2;
                        }
                        "nodes" => {
                            if let Some(val) = tokens.get(i + 1) {
                                nodes = val.parse().ok();
                            }
                            i += 2;
                        }
                        "pv" => {
                            pv = tokens.get(i + 1).map(|s| s.to_string());
                            i += 2;
                        }
                        "multipv" => {
                            if let Some(val) = tokens.get(i + 1) {
                                multipv = val.parse().ok();
                            }
                            i += 2;
                        }
                        "score" => {
                            if let Some(score_type) = tokens.get(i + 1) {
                                match *score_type {
                                    "cp" => {
                                        if let Some(val) = tokens.get(i + 2) {
                                            score_cp = val.parse().ok();
                                        }
                                        i += 3;
                                    }
                                    "mate" => {
                                        if let Some(val) = tokens.get(i + 2) {
                                            score_mate = val.parse().ok();
                                        }
                                        i += 3;
                                    }
                                    "lowerbound" => {
                                        score_lowerbound = true;
                                        i += 2;
                                    }
                                    "upperbound" => {
                                        score_upperbound = true;
                                        i += 2;
                                    }
                                    _ => i += 1,
                                }
                            } else {
                                i += 1;
                            }
                        }
                        "currmove" => {
                            currmove = tokens.get(i + 1).map(|s| s.to_string());
                            i += 2;
                        }
                        "currmovenumber" => {
                            if let Some(val) = tokens.get(i + 1) {
                                currmove_number = val.parse().ok();
                            }
                            i += 2;
                        }
                        "hashfull" => {
                            if let Some(val) = tokens.get(i + 1) {
                                hashfull = val.parse().ok();
                            }
                            i += 2;
                        }
                        "nps" => {
                            if let Some(val) = tokens.get(i + 1) {
                                nps = val.parse().ok();
                            }
                            i += 2;
                        }
                        "tbhits" => {
                            if let Some(val) = tokens.get(i + 1) {
                                tbhits = val.parse().ok();
                            }
                            i += 2;
                        }
                        "sbhits" => {
                            if let Some(val) = tokens.get(i + 1) {
                                sbhits = val.parse().ok();
                            }
                            i += 2;
                        }
                        "cpuload" => {
                            if let Some(val) = tokens.get(i + 1) {
                                cpuload = val.parse().ok();
                            }
                            i += 2;
                        }
                        "string" => {
                            string = tokens.get(i + 1).map(|s| s.to_string());
                            i += 2;
                        }
                        "refutation" => {
                            refutation = tokens.get(i + 1).map(|s| s.to_string());
                            i += 2;
                        }
                        "currline" => {
                            currline = tokens.get(i + 1).map(|s| s.to_string());
                            i += 2;
                        }
                        _ => i += 1,
                    }
                }

                Ok(UciCommand::Info(Info {
                    depth,
                    seldepth,
                    time,
                    nodes,
                    pv,
                    multipv,
                    score: Some(Score {
                        cp: score_cp,
                        mate: score_mate,
                        lowerbound: score_lowerbound,
                        upperbound: score_upperbound,
                    }),
                    currmove,
                    currmove_number,
                    hashfull,
                    nps,
                    tbhits,
                    sbhits,
                    cpuload,
                    string,
                    refutation,
                    currline,
                }))
                // Ok(UciCommand::Info {
                //     depth,
                //     seldepth,
                //     time,
                //     nodes,
                //     pv,
                //     multipv,
                //     score: Some(Score {
                //         cp: score_cp,
                //         mate: score_mate,
                //         lowerbound: score_lowerbound,
                //         upperbound: score_upperbound,
                //     }),
                //     currmove,
                //     currmove_number,
                //     hashfull,
                //     nps,
                //     tbhits,
                //     sbhits,
                //     cpuload,
                //     string,
                //     refutation,
                //     currline,
                // })
            }
            Some(&"option") => Ok(UciCommand::Option(tokens[1..].join(" "))),
            _ => Err("Not a command"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Default)]
pub struct Score {
    pub cp: Option<isize>,
    pub mate: Option<isize>,
    pub lowerbound: bool,
    pub upperbound: bool,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(cp) = self.cp {
            write!(f, "score cp {}", cp)?;
        } else if let Some(mate) = self.mate {
            write!(f, "score mate {}", mate)?;
        } else {
            return Ok(()); // No score info
        }

        if self.lowerbound {
            write!(f, " lowerbound")?;
        }
        if self.upperbound {
            write!(f, " upperbound")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct Info {
    pub depth: Option<usize>,
    pub seldepth: Option<usize>,
    pub time: Option<usize>,
    pub nodes: Option<usize>,
    pub pv: Option<String>,
    pub multipv: Option<usize>,
    pub score: Option<Score>,
    pub currmove: Option<String>,
    pub currmove_number: Option<usize>,
    pub hashfull: Option<usize>,
    pub nps: Option<usize>,
    pub tbhits: Option<usize>,
    pub sbhits: Option<usize>,
    pub cpuload: Option<usize>,
    pub string: Option<String>,
    pub refutation: Option<String>,
    pub currline: Option<String>,
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "info")?;

        if let Some(depth) = self.depth {
            write!(f, " depth {}", depth)?;
        }
        if let Some(seldepth) = self.seldepth {
            write!(f, " seldepth {}", seldepth)?;
        }
        if let Some(time) = self.time {
            write!(f, " time {}", time)?;
        }
        if let Some(nodes) = self.nodes {
            write!(f, " nodes {}", nodes)?;
        }
        if let Some(ref pv) = self.pv {
            write!(f, " pv {}", pv)?;
        }
        if let Some(multipv) = self.multipv {
            write!(f, " multipv {}", multipv)?;
        }
        if let Some(ref score) = self.score {
            write!(f, " {}", score)?;
        }
        if let Some(ref currmove) = self.currmove {
            write!(f, " currmove {}", currmove)?;
        }
        if let Some(currmove_number) = self.currmove_number {
            write!(f, " currmovenumber {}", currmove_number)?;
        }
        if let Some(hashfull) = self.hashfull {
            write!(f, " hashfull {}", hashfull)?;
        }
        if let Some(nps) = self.nps {
            write!(f, " nps {}", nps)?;
        }
        if let Some(tbhits) = self.tbhits {
            write!(f, " tbhits {}", tbhits)?;
        }
        if let Some(sbhits) = self.sbhits {
            write!(f, " sbhits {}", sbhits)?;
        }
        if let Some(cpuload) = self.cpuload {
            write!(f, " cpuload {}", cpuload)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
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

    Info(Info),
    Option(String),
}

pub trait Uci {
    fn send_command(&mut self, command: UciCommand);
    fn read_command(&mut self) -> Option<UciCommand>;
    fn handle_command(&mut self);
}
