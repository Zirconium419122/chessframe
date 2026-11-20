use chessframe::uci::{OptionType, UciCommand, UciOption};

#[test]
fn test_option_parse_spin() {
    let opt: UciOption = "option name Hash type spin default 64 min 1 max 16384"
        .parse()
        .unwrap();
    assert_eq!(opt.name, "Hash");
    assert_eq!(opt.option_type, OptionType::Spin);
    assert_eq!(opt.default, Some("64".to_string()));
    assert_eq!(opt.min, Some(1));
    assert_eq!(opt.max, Some(16384));
    assert!(opt.vars.is_empty());
}

#[test]
fn test_option_parse_check() {
    let opt: UciOption = "option name Ponder type check default true"
        .parse()
        .unwrap();
    assert_eq!(opt.name, "Ponder");
    assert_eq!(opt.option_type, OptionType::Check);
    assert_eq!(opt.default, Some("true".to_string()));
    assert!(opt.min.is_none());
    assert!(opt.max.is_none());
    assert!(opt.vars.is_empty());
}

#[test]
fn test_option_parse_button() {
    let opt: UciOption = "option name Clear Hash type button".parse().unwrap();
    assert_eq!(opt.name, "Clear Hash");
    assert_eq!(opt.option_type, OptionType::Button);
    assert!(opt.default.is_none());
    assert!(opt.vars.is_empty());
}

#[test]
fn test_option_missing_name() {
    let result: Result<UciOption, _> = "option type spin".parse();
    assert!(result.is_err());
}

#[test]
fn test_option_missing_type() {
    let result: Result<UciOption, _> = "option name Hash".parse();
    assert!(result.is_err());
}

#[test]
fn test_option_invalid_type() {
    let result: Result<UciOption, _> = "option name Foo type bar".parse();
    assert!(result.is_err());
}

#[test]
fn test_option_parse_from_uci_command() {
    let cmd: UciCommand = "option name Hash type spin default 64 min 1 max 16384"
        .parse()
        .unwrap();
    match cmd {
        UciCommand::Option(opt) => {
            assert_eq!(opt.name, "Hash");
            assert_eq!(opt.option_type, OptionType::Spin);
        }
        _ => panic!("Expected Option variant"),
    }
}

#[test]
fn test_info_parse_score_cp() {
    let cmd: UciCommand = "info depth 5 score cp 150".parse().unwrap();
    match cmd {
        UciCommand::Info(info) => {
            assert_eq!(info.depth, Some(5));
            let score = info.score.unwrap();
            assert_eq!(score.cp, Some(150));
            assert!(score.mate.is_none());
        }
        _ => panic!("Expected Info variant"),
    }
}

#[test]
fn test_info_parse_score_lowerbound() {
    let cmd: UciCommand = "info depth 10 score cp 50 lowerbound".parse().unwrap();
    match cmd {
        UciCommand::Info(info) => {
            let score = info.score.unwrap();
            assert_eq!(score.cp, Some(50));
            assert!(score.lowerbound);
        }
        _ => panic!("Expected Info variant"),
    }
}

#[test]
fn test_info_parse_pv() {
    let cmd: UciCommand = "info depth 3 pv e2e4 e7e5 g1f3".parse().unwrap();
    match cmd {
        UciCommand::Info(info) => {
            assert_eq!(info.pv, Some("e2e4 e7e5 g1f3".to_string()));
        }
        _ => panic!("Expected Info variant"),
    }
}

#[test]
fn test_info_parse_string() {
    let cmd: UciCommand = "info depth 3 string checking the position at depth 99"
        .parse()
        .unwrap();
    match cmd {
        UciCommand::Info(info) => {
            assert_eq!(info.depth, Some(3));
            assert_eq!(
                info.string.as_deref(),
                Some("checking the position at depth 99")
            );
        }
        _ => panic!("Expected Info variant"),
    }
}

#[test]
fn test_go_parse_all() {
    let cmd: UciCommand =
        "go wtime 5000 btime 3000 winc 100 binc 50 depth 12 nodes 1000000 mate 10 movetime 5000 infinite ponder"
            .parse()
            .unwrap();
    match cmd {
        UciCommand::Go(go) => {
            assert_eq!(go.wtime, Some(5000));
            assert_eq!(go.btime, Some(3000));
            assert_eq!(go.winc, Some(100));
            assert_eq!(go.binc, Some(50));
            assert_eq!(go.depth, Some(12));
            assert_eq!(go.nodes, Some(1000000));
            assert_eq!(go.mate, Some(10));
            assert_eq!(go.move_time, Some(5000));
            assert!(go.infinite);
            assert!(go.ponder);
        }
        _ => panic!("Expected Go variant"),
    }
}
